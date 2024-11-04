import { Injectable } from '@nestjs/common';
import { IResponse } from '@dna-platform/common';
import { fromPromise } from 'xstate';
import { IActors } from '../../schemas/get/get.schema';

@Injectable()
export class GetActorsImplementations {
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
      console.log('@_reqParams', _req.params);
      return Promise.resolve({
        payload: {
          success: true,
          message: 'get Message',
          count: 0,
          data: [],
        },
      });
    }),
  };
}
