import { BadRequestException, Injectable } from '@nestjs/common';
import { DrizzleService } from '@dna-platform/crdt-lww-postgres';
import { map } from 'bluebird';
import * as local_schema from '../../schema';
import { v4 as uuidv4 } from 'uuid';
import { AxonPushService } from '../../providers/axon/axon_push/axon_push.service';
import { Utility } from '../../utils/utility.service';
import { AuthService } from '@dna-platform/crdt-lww-postgres/build/modules/auth/auth.service';
import { LoggerService } from '@dna-platform/common';

// import { insertRecords } from './test';

@Injectable()
export class StoreService {
  private db;

  constructor(
    private readonly drizzleService: DrizzleService,
    private readonly pushService: AxonPushService,
    private readonly authService: AuthService,
    private readonly logger: LoggerService,
  ) {
    this.db = this.drizzleService.getClient();
  }

  async batchInsert(request) {
    const { query, headers } = request;
    const { authorization } = headers;
    const { t = '' } = query;
    const { account: responsible_account } = await this.authService
      .verify(t || authorization?.replace('Bearer ', ''))
      .catch((err) => {
        this.logger.debug(err.message);
        throw new BadRequestException(
          `Token Verification Failed: ${err.message}`,
        );
      });

    const { organization_id = '' } = responsible_account;
    const { params, body } = request;
    const prefix = body.entity_prefix;
    if (!prefix) {
      return Promise.reject({
        payload: {
          success: false,
          message: 'entity_prefix is required [Temporary Fix]',
          count: 0,
          data: [],
        },
      });
    }

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

    // @ts-ignore
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
    const record_ids: string[] = [];
    const table_schema = local_schema[table];
    const records = await map(
      body.records,
      async (record: Record<string, any>) => {
        record.organization_id = organization_id;

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
        record_ids.push(record.id);
        record.created_by = responsible_account.organization_account_id;
        record.timestamp = record?.timestamp
          ? new Date(record?.timestamp)
          : new Date().toISOString();
        record = Utility.createParse({ schema, data: record });
        return record;
      },
    );

    // const data = await insertRecords('packets', 'temp_packets', records);

    const results = await this.db.transaction(async (trx) => {
      // Prepare both insert operations
      const main_table_insert = trx
        .insert(table_schema)
        .values(records)
        .returning({ table_schema });
      const temp_table_insert = trx.insert(temp_schema).values(records);

      // Execute both inserts in parallel
      const [results_main_table, _] = await Promise.all([
        main_table_insert,
        temp_table_insert,
      ]);

      return results_main_table;
    });

    this.pushService.sender({ table, prefix, record_ids });

    return Promise.resolve({
      payload: {
        success: true,
        message: 'Records inserted successfully',
        count: 0,
        data: [results],
      },
    });
  }
}
