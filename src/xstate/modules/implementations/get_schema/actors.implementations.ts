import { Injectable } from '@nestjs/common';
import { IResponse } from '@dna-platform/common';
import { fromPromise } from 'xstate';
import { IActors } from '../../schemas/get_schema/get_schema.schema';
let local_storage = {};
@Injectable()
export class GetSchemaActorsImplementations {
  /**
   * Implementation of actors for the get_schema machine.
   */
  public readonly actors: IActors = {
    /**
     * get_schema step actor implementation.
     * @param input - The input object containing the context.
     * @returns A promise that resolves to an IResponse object.
     */
    getSchema: fromPromise(async ({ input }): Promise<IResponse> => {
      const { context } = input;
      if (!context?.controller_args)
        return Promise.reject({
          payload: {
            success: false,
            message: 'Unable to fetch schema',
            count: 0,
            data: [],
          },
        });

      const [_res, _req] = context?.controller_args;
      console.log('@req', Object.keys(_req));
      console.log('@params', _req.params);
      console.log('@body', _req.body);
      console.log('@local_storage', local_storage);
      return Promise.resolve({
        payload: {
          success: true,
          message: 'getSchema Message',
          count: 0,
          data: [],
        },
      });
    }),
  };
}
