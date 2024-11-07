import { Injectable } from '@nestjs/common';
import { IResponse } from '@dna-platform/common';
import { fromPromise } from 'xstate';
import { IActors } from '../../schemas/get_file_by_id/get_file_by_id.schema';

@Injectable()
export class GetFileByIdActorsImplementations {
  /**
   * Implementation of actors for the get_file_by_id machine.
   */
  public readonly actors: IActors = {
    /**
     * Sample step actor implementation.
     * @param input - The input object containing the context.
     * @returns A promise that resolves to an IResponse object.
     */
    getFileById: fromPromise(async ({ input }): Promise<IResponse> => {
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
      const { id: file_id } = _req.params;
      console.log('@file_id', file_id);
      return Promise.resolve({
        payload: {
          success: true,
          message: 'getFileById Message',
          count: 0,
          data: [],
        },
      });
    }),
  };
}
