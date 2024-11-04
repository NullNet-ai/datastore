import { Injectable } from '@nestjs/common';
import { IResponse } from '@dna-platform/common';
import { fromPromise } from 'xstate';
import { IActors } from '../../schemas/create/create.schema';
import { SyncService } from '@dna-platform/crdt-lww';
import { Utility } from 'src/utils/utility.service';

@Injectable()
export class CreateActorsImplementations {
  constructor(private readonly syncService: SyncService) {}
  /**
   * Implementation of actors for the create machine.
   */
  public readonly actors: IActors = {
    /**
     * Sample step actor implementation.
     * @param input - The input object containing the context.
     * @returns A promise that resolves to an IResponse object.
     */
    create: fromPromise(async ({ input }): Promise<IResponse> => {
      const { context } = input;
      if (!context?.controller_args)
        return Promise.reject({
          payload: {
            success: false,
            message: `Failed to get controller args in create actor`,
            count: 0,
            data: [],
          },
        });

      const [_res, _req] = context?.controller_args;
      const { params, body } = _req;
      const { meta, data } = body;
      const { table } = params;
      const { schema } = Utility.checkCreateSchema(table, meta, data);
      const result = await this.syncService.insert(
        table,
        Utility.createParse({ schema, data }),
      );
      console.log('@table after', table);
      return Promise.resolve({
        payload: {
          success: true,
          message: `Successfully created in ${table}`,
          count: 1,
          data: [result],
        },
      });
    }),
  };
}
