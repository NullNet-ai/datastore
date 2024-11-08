import { Injectable } from '@nestjs/common';
import { IResponse } from '@dna-platform/common';
import { fromPromise } from 'xstate';
import { IActors } from '../../schemas/download/download.schema';

@Injectable()
export class DownloadActorsImplementations {
  /**
   * Implementation of actors for the download machine.
   */
  public readonly actors: IActors = {
    /**
     * Sample step actor implementation.
     * @param input - The input object containing the context.
     * @returns A promise that resolves to an IResponse object.
     */
    download: fromPromise(async ({ input }): Promise<IResponse> => {
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
      return Promise.resolve({
        payload: {
          success: true,
          message: 'download Message',
          count: 1,
          data: [],
        },
      });
    }),
  };
}
