import { BadRequestException, Injectable } from '@nestjs/common';
// import { DrizzleService } from '@dna-platform/crdt-lww-postgres';
import { map } from 'bluebird';
import * as local_schema from '../../schema';
import { v4 as uuidv4 } from 'uuid';
// import { AxonPushService } from '../../providers/axon/axon_push/axon_push.service';
import { Utility } from '../../utils/utility.service';
import { AuthService } from '@dna-platform/crdt-lww-postgres/build/modules/auth/auth.service';
import { LoggerService } from '@dna-platform/common';
import { copyData } from './raw_query';
import { ConfigService } from '@nestjs/config';

// import { insertRecords } from './test';

@Injectable()
export class StoreService {
  // private db;
  private batch_concurrency: number;

  constructor(
    // private readonly drizzleService: DrizzleService,
    // private readonly pushService: AxonPushService,
    private readonly authService: AuthService,
    private readonly logger: LoggerService,
    private readonly configService: ConfigService,
  ) {
    // this.db = this.drizzleService.getClient();
    this.batch_concurrency = this.configService.get('BATCH_CONCURRENCY', 5);
  }

  async batchInsert(request) {
    const batch_insert_start = new Date().getTime();
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
    const record_ids: string[] = [];
    const table_schema = local_schema[table];
    const date = new Date();
    const options: Intl.DateTimeFormatOptions = {
      year: 'numeric',
      month: '2-digit',
      day: '2-digit',
    };
    const formattedDate = date
      .toLocaleDateString('en-CA', options)
      .replace(/-/g, '/');
    const created_time = Utility.convertTime12to24(date.toLocaleTimeString());

    const records = await map(
      body.records,
      async (record: Record<string, any>) => {
        record.organization_id = organization_id;

        if (table_schema.hypertable_timestamp) {
          record.hypertable_timestamp = new Date(
            record.timestamp,
          ).toISOString();
        }

        record.id = uuidv4();
        (record.tombstone = 0),
          (record.status = 'Active'),
          (record.created_date = formattedDate),
          (record.created_time = created_time),
          (record.updated_date = formattedDate),
          (record.updated_time = created_time),
          record_ids.push(record.id);
        record.created_by = responsible_account.organization_account_id;
        record.timestamp = record?.timestamp
          ? new Date(record?.timestamp)
          : new Date().toISOString();
        return record;
      },
    );
    const table_columns = Object.keys(table_schema);
    table_columns.pop();
    console.log(
      'Time taken before copy query:',
      new Date().getTime() - batch_insert_start,
    );
    const batch_size = Math.ceil(body.records.length / this.batch_concurrency);
    let batches: Record<any, any>[] = Array.from(
      { length: this.batch_concurrency },
      (_value, i) => {
        const start = i * batch_size;
        const end = start + batch_size;
        return records.slice(start, end);
      },
    ).filter((batch) => batch.length > 0);
    const start_time = Date.now(); // Use Date.now() for better performance
    const promises = batches.map((batch: any) =>
      copyData(table, batch, table_columns),
    ); // use map to generate promises
    await Promise.all(promises);

    console.log('Time taken:', new Date().getTime() - start_time);

    // await this.db.transaction(async (trx) => {
    //   // Prepare both insert operations
    //   const main_table_insert = trx.insert(table_schema).values(records);
    //   const temp_table_insert = trx.insert(temp_schema).values(records);
    //
    //   // Execute both inserts in parallel
    //   await Promise.all([main_table_insert, temp_table_insert]);
    // });

    // this.pushService.sender({ table, prefix, record_ids });

    return Promise.resolve({
      payload: {
        success: true,
        message: 'Records inserted successfully',
        count: 0,
        data: [{ data: 'data inserted' }],
      },
    });
  }
}
