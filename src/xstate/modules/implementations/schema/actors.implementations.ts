import { Injectable } from '@nestjs/common';
import { IResponse } from '@dna-platform/common';
import { fromPromise } from 'xstate';
import { IActors } from '../../schemas/schema/schema.schema';

@Injectable()
export class SchemaActorsImplementations {
  /**
   * Implementation of actors for the schema machine.
   */
  public readonly actors: IActors = {
    /**
     * Sample step actor implementation.
     * @param input - The input object containing the context.
     * @returns A promise that resolves to an IResponse object.
     */
    schema: fromPromise(async (): Promise<IResponse> => {
      // return Promise.reject({
      //   payload: {
      //     success: false,
      //     message: 'schema Message fail',
      //     count: 0,
      //     data: [],
      //   },
      // });
      return Promise.resolve({
        payload: {
          success: true,
          message: 'schema Message',
          count: 0,
          data: [],
        },
      });
    }),
  };
}
