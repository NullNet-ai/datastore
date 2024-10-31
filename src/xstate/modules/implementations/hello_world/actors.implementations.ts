import { Injectable } from '@nestjs/common';
import { IResponse } from '@dna-platform/common';
import { fromPromise } from 'xstate';
import { IActors } from '../../schemas/hello_world/hello_world.schema';

@Injectable()
export class HelloWorldActorsImplementations {
  /**
   * Implementation of actors for the hello_world machine.
   */
  public readonly actors: IActors = {
    /**
     * Sample step actor implementation.
     * @param input - The input object containing the context.
     * @returns A promise that resolves to an IResponse object.
     */
    helloWorld: fromPromise(async (): Promise<IResponse> => {
      return Promise.reject({
        payload: {
          success: false,
          message: 'helloWorld Message fail',
          count: 0,
          data: [],
        },
      });
      return Promise.resolve({
        payload: {
          success: true,
          message: 'helloWorld Message',
          count: 0,
          data: [],
        },
      });
    }),
  };
}
