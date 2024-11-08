import {
  BadRequestException,
  Injectable,
  StreamableFile,
} from '@nestjs/common';
import { IResponse } from '@dna-platform/common';
import { fromPromise } from 'xstate';
import { IActors } from '../../schemas/download/download.schema';
import * as fs from 'fs';
import * as path from 'path';

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
      const file_path =
        process.cwd() + '/upload/' + '1b951babbc6852a3ed97fa76471001cb';
      if (!fs.existsSync(file_path))
        throw new BadRequestException({
          statusCode: 400,
          success: false,
          message: `File not found at :${file_path}`,
          count: 0,
          data: [],
        });

      const file = fs.createReadStream(path.resolve(file_path));
      return Promise.resolve({
        payload: {
          success: true,
          message: `Downloaded File found at :${file_path}`,
          count: 1,
          data: [
            new StreamableFile(file, {
              type: 'application/json',
              disposition: 'attachment; filename="package.json"',
              // If you want to define the Content-Length value to another value instead of file's length:
              // length: 123,
            }),
          ],
        },
      });
    }),
  };
}
