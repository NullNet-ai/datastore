import { Injectable } from '@nestjs/common';
import { IResponse } from '@dna-platform/common';
import { fromPromise } from 'xstate';
import { IActors } from '../../schemas/create/create.schema';
import { DrizzleService, SyncService } from '@dna-platform/crdt-lww-postgres';
import { Utility } from '../../../../utils/utility.service';
import { eq, pick } from 'lodash';
import { VerifyActorsImplementations } from '../verify';
import { MinioService } from '../../../../providers/files/minio.service';
import { sql } from 'drizzle-orm';
import { ulid } from 'ulid';
import * as local_schema from '../../../../schema';
import { LoggerService } from '@dna-platform/common';
const { SYNC_ENABLED = 'false' } = process.env;

@Injectable()
export class CreateActorsImplementations {
  private db;
  constructor(
    private readonly syncService: SyncService,
    private readonly verifyActorImplementations: VerifyActorsImplementations,
    private readonly minioService: MinioService,
    private readonly drizzleService: DrizzleService,
    private readonly logger: LoggerService,
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
      const {
        organization_id = '',
        organization,
        is_root_account,
        account_organization_id,
        account_id,
      } = responsible_account;
      const [_res, _req] = controller_args;
      const { params, body, query } = _req;
      const { table } = params;
      const { pluck = 'id' } = query;
      if (!body?.organization_id && !is_root_account) {
        body.organization_id = organization_id;
      }
      // if (!body.entity_prefix && table != 'files') {
      //   return Promise.reject({
      //     payload: {
      //       success: false,
      //       message: `entity_prefix is required [Temporary Fix]`,
      //       count: 0,
      //       data: [],
      //     },
      //   });
      // }
      // const prefix = body.entity_prefix;
      // delete body.entity_prefix;

      body.created_by = account_organization_id;
      // body.created_by = '01JCSAG79KQ1WM0F9B47Q700P2';

      if (table === 'organizations' && body?.organization_id) {
        await this.minioService.makeBucket(
          organization.name,
          body?.organization_id,
        );
      }
      const table_schema = local_schema[table];
      if (!table_schema) {
        return Promise.reject({
          payload: {
            success: false,
            message: `Table ${table} does not exist`,
            count: 0,
            data: [],
          },
        });
      }
      if (table_schema?.hypertable_timestamp) {
        body.hypertable_timestamp = new Date(body.timestamp).toISOString();
      }
      body.timestamp = body?.timestamp
        ? new Date(body?.timestamp)
        : new Date().toISOString();
      body.id = body.id === undefined ? ulid() : body.id;

      const { schema }: any = Utility.checkCreateSchema(
        table,
        undefined as any,
        body,
      );
      //get first three characters of the table name as prefix
      // auto generate code
      if (table !== 'counters') {
        //! TODO: refactor this, incrementing counter should be parallel to inserting record
        let exist = null;
        if (body.id) {
          exist = await this.db
            .select({ id: local_schema[table].id })
            .from(local_schema[table])
            .where(eq(local_schema[table].id, body.id))
            .prepare('existing_record')
            .execute()[0];
        }
        if (!exist) {
          const counter_schema = local_schema['counters'];
          const code = await this.db
            .insert(counter_schema)
            .values({ entity: table, counter: 1 })
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
              digits_number: counter_schema.digits_number,
            })
            .prepare('insert_counter')
            .execute();

          function constructCode([
            { prefix, default_code, counter, digits_number },
          ]) {
            const getDigit = (num: number) => {
              return num.toString().length;
            };

            if (digits_number) {
              digits_number = digits_number - getDigit(counter);
              const zero_digits =
                digits_number > 0 ? '0'.repeat(digits_number) : '';
              return prefix + (zero_digits + counter);
            }
            return prefix + (default_code + counter);
          }

          body.code = constructCode(code);
        }
      }
      const { encrypted_fields = [], ..._body } = body;
      let parsed_data = Utility.createParse({ schema, data: _body });
      this.logger.debug(`Create request for ${table}: ${body.id}`);

      const results = await Utility.encryptCreate({
        query: {
          table_schema,
        },
        table,
        data: parsed_data,
        encrypted_fields,
        db: this.db,
        organization_id,
        account_id,
      });

      if (SYNC_ENABLED === 'true') {
        this.syncService.insert(table, parsed_data);
      }

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
