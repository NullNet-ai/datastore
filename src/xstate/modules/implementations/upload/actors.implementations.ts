import { Injectable } from '@nestjs/common';
import { IResponse } from '@dna-platform/common';
import { fromPromise } from 'xstate';
import { IActors } from '../../schemas/upload/upload.schema';
import { CreateActorsImplementations } from '../create/actors.implementations';
@Injectable()
export class UploadActorsImplementations {
  constructor(
    private readonly createActorsImplementations: CreateActorsImplementations,
  ) {}
  /**
   * Implementation of actors for the upload machine.
   */
  public readonly actors: IActors = {
    /**
     * Sample step actor implementation.
     * @param input - The input object containing the context.
     * @returns A promise that resolves to an IResponse object.
     */
    create: this.createActorsImplementations.actors.create,
    upload: fromPromise(async ({ input }): Promise<IResponse> => {
      const { context } = input;
      if (!context?.controller_args)
        return Promise.reject({
          payload: {
            success: false,
            message: `No file uploaded`,
            count: 0,
            data: [],
          },
        });
      const [_res, _req, _file] = context?.controller_args;
      return Promise.resolve({
        payload: {
          success: true,
          message: `File uploaded successfully to ${_req.url}`,
          count: 1,
          data: [_file],
        },
      });
    }),
  };
}
