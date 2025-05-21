import { BadRequestException, Injectable } from '@nestjs/common';
import { IResponse } from '@dna-platform/common';
import { fromPromise } from 'xstate';
import { IActors } from '../../schemas/delete/delete.schema';
import { DrizzleService, SyncService } from '@dna-platform/crdt-lww-postgres';
import { and, eq, sql } from 'drizzle-orm';
import { GetActorsImplementations } from '../get';
import * as local_schema from '../../../../schema';
import { VerifyActorsImplementations } from '../verify';
import { Utility } from '../../../../utils/utility.service';
import { LoggerService } from '@dna-platform/common';
import pick from 'lodash.pick';
const { SYNC_ENABLED = 'false' } = process.env;
@Injectable()
export class DeleteActorsImplementations {
  private db;
  constructor(
    private readonly syncService: SyncService,
    private readonly getActorsImplementation: GetActorsImplementations,
    private readonly verifyActorImplementations: VerifyActorsImplementations,
    private readonly drizzleService: DrizzleService,
    private readonly logger: LoggerService,
  ) {
    this.db = this.drizzleService.getClient();
    this.actors.get = this.getActorsImplementation.actors.get;
    this.actors.verify = this.verifyActorImplementations.actors.verify;
  }
  /**
   * Implementation of actors for the delete machine.
   */
  public readonly actors: IActors = {
    delete: fromPromise(async ({ input }): Promise<IResponse> => {
      let metadata: Record<string, any> = [];
      let errors: { message: string; stack: string; status_code: number }[] =
        [];
      try {
        const { context, event } = input;
        const { error } = event;
        if (error) {
          throw error;
        }

        if (!context?.controller_args)
          return Promise.reject({
            payload: {
              success: false,
              message: `No controller args found`,
              count: 0,
              data: [],
            },
          });
        const { controller_args, responsible_account, data_permissions_query } =
          context;
        const {
          organization_id = '',
          is_root_account,
          account_organization_id,
        } = responsible_account;
        const [_res, _req] = controller_args;
        const { params, body, query, headers } = _req;
        const { host, cookie } = headers;
        const { table, id } = params;
        const { is_permanent = 'false', p, rp } = query;
        const date = new Date();
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
        if (body?.organization_id && !is_root_account) {
          body.organization_id = organization_id;
        }

        const { getPermissions, getRecordPermissions } =
          await Utility.getCachedPermissions('write', {
            data_permissions_query,
            host,
            cookie,
            headers,
            table,
            account_organization_id,
            db: this.db,
            body,
            account_id: responsible_account.account_id,
            metadata,
          });
        const permissions = p === 'true' ? await getPermissions : { data: [] };
        const record_permissions =
          rp === 'true' ? await getRecordPermissions : { data: [] };
        const meta_permissions = permissions.data?.map((p) =>
          pick(p, ['entity', 'field', 'write', 'encrypt']),
        );
        const meta_record_permissions = record_permissions.data;
        this.logger.debug(`Soft deleting ${table} record with id: ${id}`);
        const result = await this.db
          .update(table_schema)
          .set({
            tombstone: 1,
            version: sql`${table_schema.version} + 1`,
            deleted_by: account_organization_id,
            updated_date: date.toLocaleDateString(),
            updated_time: Utility.convertTime12to24(date.toLocaleTimeString()),
            status: 'Deleted',
            previous_status: table_schema.status,
          })
          .where(and(eq(table_schema.id, id)))
          .returning({ table_schema })
          .then(([{ table_schema }]) => {
            return {
              tombstone: table_schema.tombstone,
              version: table_schema.version,
              deleted_by: table_schema.deleted_by,
              updated_date: table_schema.updated_date,
              updated_time: table_schema.updated_time,
              status: table_schema.status,
              previous_status: table_schema.previous_status,
              id: table_schema.id,
              hypertable_timestamp: table_schema.hypertable_timestamp ?? null,
            };
          });

        delete result.id;

        if (SYNC_ENABLED === 'true') {
          await this.syncService.delete(table, id, is_permanent === 'true');
        }
        if (meta_record_permissions.length) {
          const [{ write }] = meta_record_permissions;
          if (!write) {
            throw new BadRequestException({
              success: false,
              message: `You do not have permission to delete this record`,
              count: 0,
              data: [],
              metadata,
              errors,
              permissions: meta_permissions,
              record_permissions: meta_record_permissions,
            });
          }
        }
        return Promise.resolve({
          payload: {
            success: true,
            message: `Successfully deleted in ${table}`,
            count: 1,
            data: [{ id }],
            metadata,
            errors,
            permissions: meta_permissions,
            record_permissions: meta_record_permissions,
          },
        });
      } catch (error: any) {
        errors.push({
          message: error?.message,
          stack: error.stack,
          status_code: error.status_code,
        });
        if (error.status !== 400 && error.status < 500) throw error;
        throw new BadRequestException({
          success: false,
          message: `There was an error while creating the new record. Please verify the entered information for completeness and accuracy. If the issue continues, contact your database administrator for further assistance.`,
          count: 0,
          data: [],
          metadata,
          errors,
        });
      }
    }),
  };
}
