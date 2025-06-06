import { Injectable } from '@nestjs/common';
import { IResponse } from '@dna-platform/common';
import { fromPromise } from 'xstate';
import { IActors } from '../../schemas/pg_listener_get/pg_listener_get.schema';
import { DrizzleService } from '@dna-platform/crdt-lww-postgres';
import { VerifyActorsImplementations } from '../verify';
import { sql } from 'drizzle-orm';

@Injectable()
export class PgListenerGetActorsImplementations {
  private db;
  constructor(
    private readonly drizzleService: DrizzleService,
    private readonly verifyActorImplementations: VerifyActorsImplementations,
  ) {
    this.db = this.drizzleService.getClient();
    this.actors.verify = this.verifyActorImplementations.actors.verify;
  }
  public readonly actors: IActors = {
    pgListenerGet: fromPromise(async ({ input }): Promise<IResponse> => {
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

      //get all the triggers and functions and return back the list
      const { rows: function_rows } = await this.db.execute(
        sql.raw(`
  SELECT p.proname AS name
  FROM pg_proc p
  JOIN pg_namespace n ON n.oid = p.pronamespace
  WHERE n.nspname NOT IN ('pg_catalog', 'information_schema')
    AND pg_function_is_visible(p.oid)
    AND p.prorettype = 'trigger'::regtype
  ORDER BY p.proname;
`),
      );
      const { rows: trigger_rows } = await this.db.execute(
        sql.raw(`
            SELECT
                tg.tgname AS name,
                cls.relname AS table_name
            FROM pg_trigger tg
                JOIN pg_class cls ON cls.oid = tg.tgrelid
                JOIN pg_namespace n ON n.oid = cls.relnamespace
            WHERE tg.tgisinternal = false -- Exclude system triggers
              AND n.nspname NOT IN ('pg_catalog', 'information_schema', 'pg_toast') -- Exclude system schemas
              AND tg.tgname NOT LIKE 'pg_%' -- Exclude auto-generated system triggers
            ORDER BY tg.tgname;
        `),
      );
      const functions = function_rows.map((row) => row.name);
      const result = {
        functions,
        triggers: trigger_rows,
      };

      const [_res, _req] = context?.controller_args;
      return Promise.resolve({
        payload: {
          success: true,
          message: 'pgListenerGet Message',
          count: 0,
          data: [result],
        },
      });
    }),
  };
}
