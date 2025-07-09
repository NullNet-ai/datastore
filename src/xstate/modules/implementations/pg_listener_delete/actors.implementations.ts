import { Injectable } from '@nestjs/common';
import { IResponse } from '@dna-platform/common';
import { fromPromise } from 'xstate';
import { IActors } from '../../schemas/pg_listener_delete/pg_listener_delete.schema';
import { DrizzleService } from '@dna-platform/crdt-lww-postgres';
import { VerifyActorsImplementations } from '../verify';
import * as local_schema from '../../../../schema';
import { and, eq, sql } from 'drizzle-orm';

@Injectable()
export class PgListenerDeleteActorsImplementations {
  // @ts-ignore
  private db;
  constructor(
    private readonly drizzleService: DrizzleService,
    private readonly verifyActorImplementations: VerifyActorsImplementations,
  ) {
    this.db = this.drizzleService.getClient();
    this.actors.verify = this.verifyActorImplementations.actors.verify;
  }
  public readonly actors: IActors = {
    pgListenerDelete: fromPromise(async ({ input }): Promise<IResponse> => {
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

      const [_res, _req] = context?.controller_args;
      const { function_name } = _req.params;
      const { table_name } = _req.query;

      //delete from the postgres_channels function
      //delete the function
      //delete the trigger named as function_name_trigger
      const schema: any = local_schema['postgres_channels'];
      try {
        await this.db.transaction(async (tx) => {
          // Delete from postgres_channels table
          const deletedChannels = await tx
            .delete(schema)
            .where(and(eq(schema.channel_name, function_name)));

          // Delete the function using raw SQL (if needed)
          await tx.execute(
            sql`DROP FUNCTION IF EXISTS ${sql.identifier(
              function_name,
            )} CASCADE`,
          );
          // Delete the trigger if it exists
          await tx.execute(
            sql`DROP TRIGGER IF EXISTS ${sql.identifier(
              function_name + '_trigger',
            )} ON ${sql.identifier(table_name)} CASCADE`,
          );
          return deletedChannels;
        });
      } catch (error: any) {
        return Promise.reject({
          payload: {
            success: false,
            message: `Error deleting pgListener: ${error.message} `,
            count: 0,
            data: [],
          },
        });
      }

      return Promise.resolve({
        payload: {
          success: true,
          message: `Successfully deleted pgListener components for ${table_name}: ${function_name} and trigger ${function_name}_trigger`,
          count: 0,
          data: [],
        },
      });
    }),
  };
}
