import {
  BadRequestException,
  Injectable,
  NotFoundException,
} from '@nestjs/common';
import { IResponse } from '@dna-platform/common';
import { fromPromise } from 'xstate';
import { IActors } from '../../schemas/find/find.schema';
import { DrizzleService } from '@dna-platform/crdt-lww';
import * as schema from '../../../../schema';
import { Utility } from '../../../../utils/utility.service';
import { eq, asc, desc, SQL } from 'drizzle-orm';
@Injectable()
export class FindActorsImplementations {
  private db;
  constructor(private readonly drizzleService: DrizzleService) {
    this.db = this.drizzleService.getClient();
  }
  /**
   * Implementation of actors for the find machine.
   */
  public readonly actors: IActors = {
    /**
     * Sample step actor implementation.
     * @param input - The input object containing the context.
     * @returns A promise that resolves to an IResponse object.
     */
    find: fromPromise(async ({ input }): Promise<IResponse> => {
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

      const [_res, _req] = context?.controller_args;

      const { table } = _req.params;
      const {
        order_direction = 'asc',
        order_by = 'id',
        limit = '100',
        offset = '0',
        pluck = '',
        ..._query
      } = _req.query;
      const table_schema = schema[table];
      if (!table_schema) {
        throw new NotFoundException('Table not found');
      }

      const _plucked_fields = Utility.parsePluckedFields(table, pluck);
      const where_clause =
        Object.keys(_query).length > 0
          ? Object.keys(_query).reduce(
              (acc, key) => {
                const column = table_schema[key];
                if (!column) {
                  throw new BadRequestException(
                    `Column ${key} not found in table ${table}`,
                  );
                }
                return [...acc, eq(table_schema[key], _req.query[key])];
              },
              [eq(table_schema.tombstone, 0)],
            )
          : eq(table_schema.tombstone, 0);

      const selections = _plucked_fields === null ? undefined : _plucked_fields;

      const result = await this.db
        .select(selections)
        .from(table_schema)
        .where(where_clause as SQL<unknown>)
        .orderBy(
          order_direction === 'asc'
            ? asc(table_schema[order_by])
            : desc(table_schema[order_by]),
        )
        .offset(Number(offset))
        .limit(Number(limit));

      return Promise.resolve({
        payload: {
          success: true,
          message: `Successfully fetched ${table} records`,
          count: 0,
          data: result,
        },
      });
    }),
  };
}
