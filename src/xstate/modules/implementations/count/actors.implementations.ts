import { Injectable } from '@nestjs/common';
import { IResponse } from '@dna-platform/common';
import { fromPromise } from 'xstate';
import { IActors } from '../../schemas/count/count.schema';
import { DrizzleService } from '@dna-platform/crdt-lww-postgres';
import { sql } from 'drizzle-orm';
import * as local_schema from '../../../../schema';

@Injectable()
export class CountActorsImplementations {
  private db;
  constructor(private readonly drizzleService: DrizzleService) {
    this.db = this.drizzleService.getClient();
  }
  /**
   * Implementation of actors for the count machine.
   */
  public readonly actors: IActors = {
    /**
     * Sample step actor implementation.
     * @param input - The input object containing the context.
     * @returns A promise that resolves to an IResponse object.
     */
    count: fromPromise(async ({ input }): Promise<IResponse> => {
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

      const [_res, _req] = context?.controller_args;
      const { table } = _req.params;
      const { count } = _req.query;
      const [{ count: result_count }] = await this.db
        .select({ count: sql<number>`count(${count})` })
        .from(local_schema[table]);

      return Promise.resolve({
        payload: {
          success: true,
          message: 'count Message',
          count: result_count,
          data: [],
        },
      });
    }),
  };
}
