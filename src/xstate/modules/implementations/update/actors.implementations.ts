import { Injectable } from '@nestjs/common';
import { IResponse } from '@dna-platform/common';
import { fromPromise } from 'xstate';
import { IActors } from '../../schemas/update/update.schema';
import { Utility } from '../../../../utils/utility.service';
import { DrizzleService, SyncService } from '@dna-platform/crdt-lww-postgres';
import { pick } from 'lodash';
import { VerifyActorsImplementations } from '../verify';
import * as local_schema from '../../../../schema';
import { eq } from 'drizzle-orm';
import { GetActorsImplementations } from '../get';
import { LoggerService } from '@dna-platform/common';
const { SYNC_ENABLED = 'false' } = process.env;
@Injectable()
export class UpdateActorsImplementations {
  private db;
  constructor(
    private readonly syncService: SyncService,
    private readonly verifyActorImplementations: VerifyActorsImplementations,
    private readonly getActorsImplementation: GetActorsImplementations,
    private readonly drizzleService: DrizzleService,
    private readonly logger: LoggerService,
  ) {
    this.db = this.drizzleService.getClient();
    this.actors.get = this.getActorsImplementation.actors.get;
    this.actors.verify = this.verifyActorImplementations.actors.verify;
  }

  public readonly actors: IActors = {
    update: fromPromise(async ({ input }): Promise<IResponse> => {
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
        is_root_account,
        account_organization_id,
      } = responsible_account;
      const [_res, _req] = controller_args;
      const { params, body, query } = _req;
      const { table, id } = params;
      let { pluck = 'id' } = query;

      if (table === 'counters') {
        pluck = 'entity,default_code,prefix';
        const [result] = await this.db
          .update(local_schema[table])
          .set(body)
          .where(eq(local_schema[table].entity, id))
          .returning({
            entity: local_schema[table].entity,
            default_code: local_schema[table].default_code,
            prefix: local_schema[table].prefix,
          });

        return Promise.resolve({
          payload: {
            success: true,
            message: `Successfully updated in ${table}`,
            count: 1,
            data: [pick(result, pluck.split(','))],
          },
        });
      }

      if (!body?.organization_id && !is_root_account) {
        body.organization_id = organization_id;
      }
      body.id = id;
      const { schema } = Utility.checkUpdateSchema(
        table,
        undefined as any,
        body,
      );
      if (body?.timestamp) {
        return Promise.reject({
          payload: {
            success: false,
            message: `Timestamp is not allowed in update`,
            count: 0,
            data: [],
          },
        });
      }

      body.updated_by = account_organization_id;
      const updated_data = Utility.updateParse({ schema, data: body });
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
      delete body.id;
      let result;
      this.logger.debug(`Update request for ${table}: ${id}`);

      if (body.status) {
        result = await Utility.encryptUpdate({
          query: {
            table_schema,
          },
          encrypted_fields: body.encrypted_fields,
          table,
          db: this.db,
          data: {
            ...updated_data,
            previous_status: updated_data.status,
          },
          where: [`id = '${id}'`, 'AND', `tombstone = 0`],
          returning: {
            table_schema,
          },
        });
        updated_data.previous_status = result?.previous_status;
      } else {
        result = await Utility.encryptUpdate({
          query: {
            table_schema,
          },
          encrypted_fields: body.encrypted_fields,
          table,
          db: this.db,
          data: {
            ...updated_data,
          },
          where: [`id = '${id}'`],
          returning: {
            table_schema,
          },
        });
      }

      delete updated_data.id;
      updated_data.version = result?.version;
      if (table_schema.hypertable_timestamp) {
        updated_data.hypertable_timestamp = result.hypertable_timestamp;
      }
      //this.syncService.update(table, updated_data, id);
      if (SYNC_ENABLED === 'true') {
        this.syncService.update(table, updated_data, id);
      }
      return Promise.resolve({
        payload: {
          success: true,
          message: `Successfully updated in ${table}`,
          count: 1,
          data: [pick(result, pluck.split(','))],
        },
      });
    }),
  };
}
