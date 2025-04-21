import { Injectable } from '@nestjs/common';
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
      const { controller_args, responsible_account } = context;
      const { organization_id = '', is_root_account } = responsible_account;
      const [_res, _req] = controller_args;
      const { params, body, query } = _req;
      const { table, id } = params;
      const { is_permanent = 'false' } = query;
      const date = new Date();
      const table_schema = local_schema[table];
      if (body?.organization_id && !is_root_account) {
        body.organization_id = organization_id;
      }
      this.logger.debug(`Soft deleting ${table} record with id: ${id}`);
      const result = await this.db
        .update(table_schema)
        .set({
          tombstone: 1,
          version: sql`${table_schema.version} + 1`,
          deleted_by: is_root_account
            ? responsible_account?.organization_account?.id
            : responsible_account.organization_account_id,
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

      return Promise.resolve({
        payload: {
          success: true,
          message: `Successfully deleted in ${table}`,
          count: 1,
          data: [{ id }],
        },
      });
    }),
  };
}
