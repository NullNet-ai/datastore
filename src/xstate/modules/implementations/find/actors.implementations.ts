import { Injectable, NotFoundException } from '@nestjs/common';
import { IResponse } from '@dna-platform/common';
import { fromPromise } from 'xstate';
import { IActors } from '../../schemas/find/find.schema';
import { DrizzleService } from '@dna-platform/crdt-lww';
import { Utility } from '../../../../utils/utility.service';
import { asc, desc } from 'drizzle-orm';
// import { pick } from 'lodash';
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
        limit = 50,
        offset = 0,
        pluck = ['id'],
        advance_filters = [],
      } = _req.body;
      const _pluck = pluck;
      const table_schema = Utility.checkTable(table);
      const _plucked_fields = Utility.parsePluckedFields(table, _pluck);
      const selections = _plucked_fields === null ? undefined : _plucked_fields;
      let _db = this.db.select(selections).from(table_schema);

      _db = Utility.sqliteFilterAnalyzer(_db, table_schema, advance_filters);

      if (order_direction && order_by) {
        _db = _db.orderBy(
          order_direction === 'asc'
            ? asc(table_schema[order_by])
            : desc(table_schema[order_by]),
        );
      }

      if (offset) {
        _db = _db.offset(offset);
      }

      if (limit) {
        _db = _db.limit(limit);
      }

      let result = await _db;

      if (!result || !result.length) {
        throw new NotFoundException({
          success: false,
          message: `No data found in ${table}`,
          count: 0,
          data: [],
        });
      }

      return Promise.resolve({
        payload: {
          success: true,
          message: `Successfully fetched ${table} records`,
          count: result.length,
          data: result,
        },
      });
    }),
  };
}
