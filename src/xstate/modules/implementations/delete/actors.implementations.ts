import { Injectable } from '@nestjs/common';
import { IResponse } from '@dna-platform/common';
import { fromPromise } from 'xstate';
import { IActors } from '../../schemas/delete/delete.schema';
import { DrizzleService, SyncService } from '@dna-platform/crdt-lww-postgres';
import { and, eq, sql } from 'drizzle-orm';
import { GetActorsImplementations } from '../get';
import * as local_schema from '../../../../schema';
import { VerifyActorsImplementations } from '../verify';

@Injectable()
export class DeleteActorsImplementations {
  private db;
  constructor(
    private readonly syncService: SyncService,
    private readonly getActorsImplementation: GetActorsImplementations,
    private readonly verifyActorImplementations: VerifyActorsImplementations,
    private readonly drizzleService: DrizzleService,
  ) {
    this.db = this.drizzleService.getClient();
  }
  /**
   * Implementation of actors for the delete machine.
   */
  public readonly actors: IActors = {
    get: this.getActorsImplementation.actors.get,
    verify: this.verifyActorImplementations.actors.verify,

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
      const { organization_id = '' } = responsible_account;
      const [_res, _req] = controller_args;
      const { params, body } = _req;
      const { table, id } = params;
      const table_schema = local_schema[table];
      const result = await this.db
        .update(table_schema)
        .set({ tombstone: 1, version: sql`${table_schema.version} + 1` })
        .where(and(eq(table_schema.id, id)))
        .returning({ table_schema })
        .then(([{ table_schema }]) => {
          return { id: table_schema.id };
        });

      if (body?.organization_id) {
        body.organization_id = organization_id;
      }
      // TODO: update deleted_by to responsible_account.contact.id
      body.deleted_by = responsible_account.contact.id;
      await this.syncService.delete(table, id);
      return Promise.resolve({
        payload: {
          success: true,
          message: `Successfully deleted in ${table}`,
          count: 1,
          data: [result],
        },
      });
    }),
  };
}
