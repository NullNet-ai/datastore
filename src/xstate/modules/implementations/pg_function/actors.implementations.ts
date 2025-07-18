import { Injectable } from '@nestjs/common';
import { IResponse, LoggerService } from '@dna-platform/common';
import { fromPromise } from 'xstate';
import { IActors } from '../../schemas/pg_function/pg_function.schema';
import { DrizzleService } from '@dna-platform/crdt-lww-postgres';
import { VerifyActorsImplementations } from '../verify';
import { sql } from 'drizzle-orm';
import { CreateActorsImplementations } from '../create';

@Injectable()
export class PgFunctionActorsImplementations {
  private db;
  constructor(
    private readonly drizzleService: DrizzleService,
    private readonly verifyActorImplementations: VerifyActorsImplementations,
    private readonly createActorsImplementations: CreateActorsImplementations,
    private readonly logger: LoggerService,
  ) {
    this.db = this.drizzleService.getClient();
    this.actors.verify = this.verifyActorImplementations.actors.verify;
    this.actors.create = this.createActorsImplementations.actors.create;
  }
  public readonly actors: IActors = {
    pgFunction: fromPromise(async ({ input }): Promise<IResponse> => {
      const { context, event } = input;
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
      const { body, query } = _req;
      const { function: function_string, table_name } = body;
      if (!function_string)
        return Promise.reject({
          payload: {
            success: false,
            message: 'No function string found',
            count: 0,
            data: [],
          },
        });

      if (!table_name) {
        return Promise.reject({
          payload: {
            success: false,
            message: 'No table name found, it is used to create trigger',
            count: 0,
            data: [],
          },
        });
      }
      //check if function_string and channel_name are not empty
      const function_name = function_string.match(
        /CREATE\s+OR\s+REPLACE\s+FUNCTION\s+([a-zA-Z0-9_]+)/i,
      )[1];
      body.channel_name = function_name;

      const channel_name = function_string.match(
        /channel\s+text\s*:=\s*'([a-zA-Z0-9_]+)'\s*;/i,
      )[1];

      const json_args = function_string.match(
        /SELECT\s+json_build_object\s*\(([\s\S]+?)\)::text/i,
      )[1];
      const type_channel_match = json_args.match(/['"]type['"]\s*,\s*channel/i);
      if (function_name !== channel_name || !type_channel_match) {
        return Promise.reject({
          payload: {
            success: false,
            message:
              'Function name and channel name should be the same and type should be channel in json_build_object',
            count: 0,
            data: [],
          },
        });
      }
      try {
        let trigger_instance_statement = 'AFTER INSERT';
        if (query?.type === 'timeline')
          trigger_instance_statement += ' OR UPDATE OR DELETE';

        await this.db.execute(sql.raw(function_string));
        await this.db.execute(
          sql.raw(`DO $$
        BEGIN
          IF NOT EXISTS (
              SELECT 1
              FROM pg_available_extensions
              WHERE name = 'hstore'
          ) THEN
              CREATE EXTENSION hstore;
          END IF;

          IF NOT EXISTS (
            SELECT 1 FROM pg_trigger WHERE tgname = '${channel_name}_trigger'
          ) THEN
            CREATE TRIGGER ${channel_name}_trigger
            ${trigger_instance_statement} ON ${table_name}
            FOR EACH ROW EXECUTE FUNCTION ${channel_name}();
          END IF;
        END;
        $$;`),
        );
      } catch (err: any) {
        this.logger.error(err.message);
        return Promise.reject({
          payload: {
            success: false,
            message: `Error executing function string:  ${err.message}`,
            count: 0,
            data: [],
          },
        });
      }
      return Promise.resolve({
        payload: {
          success: true,
          message: 'pgFunction Message',
          count: 0,
          data: event.output.payload.data,
        },
      });
    }),
  };
}
