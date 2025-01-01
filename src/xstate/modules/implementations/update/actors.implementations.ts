import { Injectable } from '@nestjs/common';
import { IResponse } from '@dna-platform/common';
import { fromPromise } from 'xstate';
import { IActors } from '../../schemas/update/update.schema';
import { Utility } from '../../../../utils/utility.service';
import { DrizzleService, SyncService } from '@dna-platform/crdt-lww-postgres';
import { pick } from 'lodash';
import { VerifyActorsImplementations } from '../verify';
import * as local_schema from '../../../../schema';
import { and, eq, sql } from 'drizzle-orm';
import { GetActorsImplementations } from '../get';
@Injectable()
export class UpdateActorsImplementations {
  private db;
  constructor(
    private readonly syncService: SyncService,
    private readonly verifyActorImplementations: VerifyActorsImplementations,
    private readonly getActorsImplementation: GetActorsImplementations,
    private readonly drizzleService: DrizzleService,
  ) {
    this.db = this.drizzleService.getClient();
  }
  /**
   * Implementation of actors for the update machine.
   */
  public readonly actors: IActors = {
    get: this.getActorsImplementation.actors.get,
    verify: this.verifyActorImplementations.actors.verify,
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
      const { organization_id = '' } = responsible_account;
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

      if (!body?.organization_id) {
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
      body.updated_by = responsible_account.contact.id;
      const updated_data = Utility.updateParse({ schema, data: body });
      const table_schema = local_schema[table];
      delete body.id;
      const result = await this.db
        .update(table_schema)
        .set({ ...updated_data, version: sql`${table_schema.version} + 1` })
        .where(and(eq(table_schema.id, id), eq(table_schema.tombstone, 0)))
        .returning({ table_schema })
        .then(([{ table_schema }]) => table_schema);

      delete updated_data.id;
      updated_data.version = result?.version;
      await this.syncService.update(table, updated_data, id);
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
