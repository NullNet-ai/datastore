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
    this.actors.verify = this.verifyActorImplementations.actors.verify;
  }
  public readonly actors: IActors = {
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
      let temp_schema;
      try {
        temp_schema = Utility.checkTable(`temp_${table}`);
      } catch (e) {
        return Promise.reject({
          payload: {
            success: false,
            message: `Table not found: temp_${table}, for batch insert create a temp table first e.g for table ${table} create temp_${table}`,
            count: 0,
            data: [],
          },
        });
      }
      temp_schema = local_schema[`temp_${table}`];
      const table_schema = local_schema[table];
      const records = await map(
        body.records,
        async (record: Record<string, any>) => {
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
          if (table_schema.hypertable_timestamp) {
            record.hypertable_timestamp = new Date(
              record.timestamp,
            ).toISOString();
          }
          const { schema }: any = Utility.checkCreateSchema(
            table,
            undefined as any,
            record,
          );
          record.id = uuidv4();
          record.created_by = responsible_account.organization_account_id;
          record.timestamp = record?.timestamp
            ? new Date(record?.timestamp)
            : new Date().toISOString();
          record = Utility.createParse({ schema, data: record });
          // await this.syncService.insert(
          //   table,
          //   Utility.createParse({ schema, data: body }),
          // );
          return record;
        },
      );
      const check_records = [...records];
      const results = await this.db.transaction(async (trx) => {
        // Insert into the main table
        const results_main_table = await trx
          .insert(table_schema)
          .values(records)
          .returning({ table_schema })
          .then((inserted) => {
            return inserted;
          });

        await trx
          .insert(temp_schema)
          .values(check_records)
          .returning({ table_schema })
          .then((inserted) => {
            return inserted;
          });

        return results_main_table;
      });

      //todo: insert data into temp table as well, and write a seprarate compaction service which cleans up data everyday
      //todo: create temp tables for table which are required for batch queries
      //todo: create a separate column for hypertable timestamp which indentifies if the table is a hypertable
      //todo: update the create message method to check for hypertable and then do the conflict update
      //todo: in crdt server change the column type of expiry insync_transactions to bigint

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
