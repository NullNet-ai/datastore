import { Injectable } from '@nestjs/common';
import { IResponse } from '@dna-platform/common';
import { fromPromise } from 'xstate';
import { IActors } from '../../schemas/batch_insert/batch_insert.schema';
import { DrizzleService } from '@dna-platform/crdt-lww-postgres';
import { map } from 'bluebird';
import { VerifyActorsImplementations } from '../verify';
import * as local_schema from '../../../../schema';
import { v4 as uuidv4 } from 'uuid';
import { sql } from 'drizzle-orm';
import { Utility } from '../../../../utils/utility.service';

@Injectable()
export class BatchInsertActorsImplementations {
  private db;

  constructor(
    private readonly verifyActorImplementations: VerifyActorsImplementations,
    private readonly drizzleService: DrizzleService, // private readonly syncService: SyncService,
  ) {
    this.db = this.drizzleService.getClient();
  }
  public readonly actors: IActors = {
    verify: this.verifyActorImplementations.actors.verify,
    batchInsert: fromPromise(async ({ input }): Promise<IResponse> => {
      const { context } = input;
      if (!context?.controller_args)
        return Promise.reject({
          payload: {
            success: false,
            message: 'No controller args found',
            count: 0,
            data: [],
          },
        });
      const { controller_args, responsible_account } = context;
      // const { organization_id = '', organization } = responsible_account;
      const [_res, _req] = controller_args;
      const { params, body } = _req;
      const { table } = params;
      if (!body.records || !Array.isArray(body.records)) {
        return Promise.reject({
          payload: {
            success: false,
            message: "Invalid payload: 'records' must be an array",
            count: 0,
            data: [],
          },
        });
      }

      const records = await map(body.records, async (record) => {
        const counter_schema = local_schema['counters'];
        record.code = await this.db
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
          })
          .then(
            ([{ prefix, default_code, counter }]) =>
              prefix + (default_code + counter),
          )
          .catch(() => null);
        record = {
          ...record,
          timestamp: new Date(record.timestamp),
          id: uuidv4(),
          created_by: responsible_account.contact.id,
          created_date: new Date().toISOString(),
          organization_id: responsible_account.organization_id,
        };
        const { schema }: any = Utility.checkCreateSchema(
          table,
          undefined as any,
          record,
        );
        Utility.createParse({ schema, data: record });
        // await this.syncService.insert(
        //   table,
        //   Utility.createParse({ schema, data: body }),
        // );
        return record;
      });
      console.log(records);
      const data = local_schema[table];
      const results = await this.db
        .insert(data)
        .values(records)
        .returning({ data })
        .then((inserted) => {
          return inserted;
        });

      return Promise.resolve({
        payload: {
          success: true,
          message: 'Records inserted successfully',
          count: results.length,
          data: results,
        },
      });
    }),
  };
}
