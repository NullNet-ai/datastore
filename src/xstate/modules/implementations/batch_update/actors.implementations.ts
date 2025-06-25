import { Injectable } from '@nestjs/common';
import { IResponse } from '@dna-platform/common';
import { fromPromise } from 'xstate';
import { IActors } from '../../schemas/batch_update/batch_update.schema';
import { VerifyActorsImplementations } from '../verify';
import { DrizzleService } from '@dna-platform/crdt-lww-postgres';
import { AxonPushService } from '../../../../providers/axon/axon_push/axon_push.service';
import { Utility } from '../../../../utils/utility.service';
import * as local_schema from '../../../../schema';
import { sql } from 'drizzle-orm';
import { IUpdateMessage } from '../../../../providers/axon/types';
import { ConfigService } from '@nestjs/config';
import { LoggerService } from '@dna-platform/common';

@Injectable()
export class BatchUpdateActorsImplementations {
  private readonly db;
  private readonly batch_updates_sync_enabled: string;

  constructor(
    private readonly verifyActorImplementations: VerifyActorsImplementations,
    private readonly drizzleService: DrizzleService,
    private readonly pushService: AxonPushService,
    private readonly configService: ConfigService,
    private readonly logger: LoggerService,
  ) {
    this.db = this.drizzleService.getClient();
    this.actors.verify = this.verifyActorImplementations.actors.verify;
    this.batch_updates_sync_enabled = this.configService.get(
      'BATCH_UPDATES_SYNC_ENABLED',
      'true',
    );
  }

  public readonly actors: IActors = {
    batchUpdate: fromPromise(async ({ input }): Promise<IResponse> => {
      const { context } = input;
      if (!context?.controller_args)
        return Promise.reject({
          payload: {
            success: false,
            message: 'sampleStep fail Message',
            count: 0,
            data: [],
          },
        });

      const { controller_args, responsible_account } = context;
      const { organization_id = '', account_organization_id } =
        responsible_account;
      const [_res, _req] = controller_args;
      const { params, body } = _req;
      const { table, type } = params;
      let { advance_filters, updates } = body;
      const table_schema = local_schema[table];

      Utility.checkTable(table);
      updates = {
        ...updates,
        id: '56ab2a2c-b498-43e0-884f-a61beb93e56e',
        timestamp: new Date(),
        updated_by: account_organization_id,
      };
      if (updates.tombstone && updates.tombstone === 1) {
        updates.deleted_by = account_organization_id;
      }
      const { schema } = Utility.checkUpdateSchema(
        table,
        undefined as any,
        updates,
      );
      this.logger.debug(
        `Batch update filters: ${JSON.stringify(advance_filters)}`,
      );
      const parsed_updates = Utility.updateParse({ schema, data: updates });
      parsed_updates.version = sql`${table_schema.version} + 1`;
      delete parsed_updates.id;
      delete parsed_updates.timestamp;
      let _db = this.db.update(table_schema).set(parsed_updates);
      const return_data = {};
      Object.keys(parsed_updates).forEach((key) => {
        return_data[key] = table_schema[key];
      });
      _db = Utility.FilterAnalyzer({
        db: _db,
        table_schema,
        advance_filters,
        organization_id,
        client_db: this.db,
        request_type: type
      });
      const result = await _db
        .returning({
          id: table_schema.id,
          version: table_schema.version,
          updated_date: table_schema.updated_date,
          updated_time: table_schema.updated_time,
          updated_by: table_schema.updated_by,
          ...(table_schema.hypertable_timestamp && {
            hypertable_timestamp: table_schema.hypertable_timestamp,
          }),
          ...return_data,
        })
        .then((data) => data);
      const axon_update_message: IUpdateMessage = {
        table,
        records: result,
      };
      const count = result.length;

      const message =
        updates.tombstone && updates.tombstone === 1
          ? `${count} records deleted successfully`
          : `${count} records updated successfully`;
      if (this.batch_updates_sync_enabled === 'true') {
        this.pushService.pushToUpdateQueue(axon_update_message);
      }
      return Promise.resolve({
        payload: {
          success: true,
          message,
          count,
          data: [],
        },
      });
    }),
  };
}
