import { BadRequestException, Injectable } from '@nestjs/common';
import { map } from 'bluebird';
import * as local_schema from '../../schema';
import { ulid } from 'ulid';
import { AxonPushService } from '../../providers/axon/axon_push/axon_push.service';
import { Utility } from '../../utils/utility.service';
import { AuthService } from '@dna-platform/crdt-lww-postgres/build/modules/auth/auth.service';
import { LoggerService } from '@dna-platform/common';
import { copyData } from '../../db/raw_batch_query';
import { ConfigService } from '@nestjs/config';
import { ICounterMessage } from '../../providers/axon/types';

// import { insertRecords } from './test';

@Injectable()
export class StoreGrpcService {
  // private db;
  private batch_concurrency: number;

  constructor(
    private readonly pushService: AxonPushService,
    private readonly authService: AuthService,
    private readonly logger: LoggerService,
    private readonly configService: ConfigService,
  ) {
    // this.db = this.drizzleService.getClient();
    this.batch_concurrency = this.configService.get('BATCH_CONCURRENCY', 5);
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

    const {
      organization_id = '',
      is_root_account,
    } = responsible_account;
    const { params, body } = request;
    let { records } = body;

    const { table } = params;
    if (!records || !Array.isArray(body.records)) {
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
    try {
      Utility.checkTable(`temp_${table}`);
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

    records = await map(records, async (record: Record<string, any>) => {
      if (!record?.organization_id && !is_root_account) {
        record.organization_id = organization_id;
      }

      if (table_schema.hypertable_timestamp) {
        record.hypertable_timestamp = new Date(record.timestamp).toISOString();
      }
      record.id = ulid();
      record.version = 1;
      (record.tombstone = 0),
        (record.status = 'Active'),
        (record.created_date = formattedDate),
        (record.created_time = created_time),
        (record.updated_date = formattedDate),
        (record.updated_time = created_time),
        record_ids.push(record.id);
      record.created_by = responsible_account.account_organization_id;
      record.timestamp = record?.timestamp
        ? new Date(record?.timestamp)
        : new Date().toISOString();
      return record;
    });
    const table_columns = Object.keys(table_schema);
    table_columns.pop();
    let batches: Record<any, any>[];
    if (records.length < this.batch_concurrency) {
      batches = [records];
    } else {
      const batch_size = Math.ceil(
        body.records.length / this.batch_concurrency,
      );
      batches = Array.from({ length: this.batch_concurrency }, (_value, i) => {
        const start = i * batch_size;
        const end = start + batch_size;
        return records.slice(start, end);
      }).filter((batch) => batch.length > 0);
    }
    this.logger.debug(`Batches query of ${table}: ${records.length} records`);
    const promises = batches.map((batch: any) =>
      copyData(table, batch, table_columns),
    ); // use map to generate promises
    await Promise.all(promises);
    const message: ICounterMessage = { record_ids, table };
    this.pushService.sender(message);

    const ret_fields = !!request.query.pluck
      ? request.query.pluck.split(',')
      : [];

    let ret_data = [];
    if (ret_fields.length > 0) {
      ret_data = records.map((value) => {
        const object = {};
        for (const field of ret_fields) {
          object[field] = value[field];
        }
        return object;
      });
    }

    return Promise.resolve({
      payload: {
        success: true,
        message: 'Records inserted successfully',
        count: records.length,
        data: ret_data,
      },
    });
  }
}
