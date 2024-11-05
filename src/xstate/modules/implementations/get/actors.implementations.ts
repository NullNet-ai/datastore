import { Injectable, NotFoundException } from '@nestjs/common';
import { IResponse } from '@dna-platform/common';
import { fromPromise } from 'xstate';
import { IActors } from '../../schemas/get/get.schema';
import { DrizzleService } from '@dna-platform/crdt-lww';
import { Utility } from 'src/utils/utility.service';
import { eq } from 'drizzle-orm';

@Injectable()
export class GetActorsImplementations {
  private db;
  constructor(private readonly drizzleService: DrizzleService) {
    this.db = this.drizzleService.getClient();
  }
  /**
   * Implementation of actors for the get machine.
   */
  public readonly actors: IActors = {
    /**
     * Sample step actor implementation.
     * @param input - The input object containing the context.
     * @returns A promise that resolves to an IResponse object.
     */
    get: fromPromise(async ({ input }): Promise<IResponse> => {
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
      const { table, id } = _req.params;
      const { pluck = '' } = _req.query;

      const table_schema = Utility.checkTable(table);
      const _plucked_fields = Utility.parsePluckedFields(
        table,
        pluck.split(','),
      );
      const selections = _plucked_fields === null ? undefined : _plucked_fields;

      const result = await this.db
        .select(selections)
        .from(table_schema)
        .where(eq(table_schema.id, id));

      if (!result || !result.length) {
        throw new NotFoundException({
          success: false,
          message: `No data [${id}] found in ${table}`,
          count: 0,
          data: [],
        });
      }

      return Promise.resolve({
        payload: {
          success: true,
          message: `Successfully got data [${id}] from ${table}`,
          count: result.length,
          data: result,
        },
      });
    }),
  };
}
