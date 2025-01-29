import { Injectable } from '@nestjs/common';
import { IResponse } from '@dna-platform/common';
import { fromPromise } from 'xstate';
import { IActors } from '../../schemas/create/create.schema';
import { DrizzleService, SyncService } from '@dna-platform/crdt-lww-postgres';
import { Utility } from '../../../../utils/utility.service';
import { pick } from 'lodash';
import { VerifyActorsImplementations } from '../verify';
import { MinioService } from '../../../../providers/files/minio.service';
import { sql } from 'drizzle-orm';
import * as local_schema from '../../../../schema';
import { v4 as uuidv4 } from 'uuid';

@Injectable()
export class CreateActorsImplementations {
  private db;
  constructor(
    private readonly syncService: SyncService,
    private readonly verifyActorImplementations: VerifyActorsImplementations,
    private readonly minioService: MinioService,
    private readonly drizzleService: DrizzleService,
  ) {
    this.db = this.drizzleService.getClient();
    this.actors.verify = this.verifyActorImplementations.actors.verify;
  }
  /**
   * Implementation of actors for the create machine.
   */
  public readonly actors: IActors = {
    create: fromPromise(async ({ input }): Promise<IResponse> => {
      const { context } = input;
      if (!context?.controller_args)
        return Promise.reject({
          payload: {
            success: false,
            message: `No controller args found`,
            count: 0,
            data: [],
          },
        });

      const { controller_args, responsible_account } = context;
      const { organization_id = '', organization } = responsible_account;
      const [_res, _req] = controller_args;
      const { params, body, query } = _req;
      const { table } = params;
      const { pluck = 'id' } = query;
      if (!body?.organization_id) {
        body.organization_id = organization_id;
      }
      if (!body.entity_prefix) {
        return Promise.reject({
          payload: {
            success: false,
            message: `entity_prefix is required [Temporary Fix]`,
            count: 0,
            data: [],
          },
        });
      }
      const prefix = body.entity_prefix;
      delete body.entity_prefix;

      body.created_by = responsible_account.organization_account_id;
      // body.created_by = '01JCSAG79KQ1WM0F9B47Q700P2';

      if (table === 'organizations' && body?.organization_id) {
        await this.minioService.makeBucket(organization.name);
      }
      const table_schema = local_schema[table];
      if (table_schema.hypertable_timestamp) {
        body.hypertable_timestamp = new Date(body.timestamp).toISOString();
      }
      body.timestamp = body?.timestamp
        ? new Date(body?.timestamp)
        : new Date().toISOString();
      body.id = uuidv4();

      const { schema }: any = Utility.checkCreateSchema(
        table,
        undefined as any,
        body,
      );
      //get first three characters of the table name as prefix
      // auto generate code
      if (table !== 'counters') {
        const counter_schema = local_schema['counters'];
        body.code = await this.db
          .insert(counter_schema)
          .values({
            entity: table,
            counter: 1,
            prefix,
            default_code: 100000,
          })
          .onConflictDoUpdate({
            target: [counter_schema.entity],
            set: {
              counter: sql`${counter_schema.counter} + 1`,
            },
          })
          .returning({
            prefix: counter_schema.prefix,
            default_code: counter_schema.default_code,
            counter: counter_schema.counter,
          })
          .then(
            ([{ prefix, default_code, counter }]) =>
              prefix + (default_code + counter),
          )
          .catch(() => null);
      }
      const parsed_data = Utility.createParse({ schema, data: body });
      const results = await this.db
        .insert(table_schema)
        .values(parsed_data)
        .returning({ table_schema })
        .then(([{ table_schema }]) => table_schema);
      await this.syncService.insert(table, parsed_data);

      return Promise.resolve({
        payload: {
          success: true,
          message: `Successfully created in ${table}`,
          count: 1,
          data: [pick(results, pluck.split(','))],
        },
      });
    }),
  };
}
