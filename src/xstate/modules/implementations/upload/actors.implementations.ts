import { Injectable } from '@nestjs/common';
import { IResponse, LoggerService } from '@dna-platform/common';
import { fromPromise } from 'xstate';
import { IActors } from '../../schemas/upload/upload.schema';
import { CreateActorsImplementations } from '../create/actors.implementations';
import { VerifyActorsImplementations } from '../verify';
import * as path from 'path';
import { MinioService } from '../../../../providers/files/minio.service';
import { ulid } from 'ulid';
// import storage_policy from './storage_policy';
const { NODE_ENV = 'local', STORAGE_BUCKET_NAME = 'test' } = process.env;
@Injectable()
export class UploadActorsImplementations {
  constructor(
    private readonly minioService: MinioService,
    private readonly createActorsImplementations: CreateActorsImplementations,
    private readonly verifyActorImplementations: VerifyActorsImplementations,
    private readonly logger: LoggerService,
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
    verify: this.verifyActorImplementations.actors.verify,
    create: this.createActorsImplementations.actors.create,
    upload: fromPromise(async ({ input }): Promise<IResponse> => {
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

      const { controller_args, responsible_account } = context;
      const [_res, _req, _file] = controller_args;
      // Set the object metadata
      const metadata = {
        'Content-Type': 'text/plain',
        'X-Amz-Meta-Testing': 1234,
        example: 5678,
      };

      let uploaded_from_remote;
      if (!['local'].includes(NODE_ENV)) {
        const filepath = path.join(process.cwd(), _file.path);
        uploaded_from_remote = await this.minioService.client
          .fPutObject(
            STORAGE_BUCKET_NAME,
            _file.originalname,
            filepath,
            metadata,
          )
          .catch((err) => {
            this.logger.error(`[ERROR][fPutObject]: ${err.message}`);
            return null;
          });

        this.logger.log(`[UPLOADED]: ${JSON.stringify(uploaded_from_remote)}`);
      }

      const download_id = ulid();
      // TODO: create a file copy of the record in the database that has uploaded_by key
      return Promise.resolve({
        payload: {
          success: true,
          message: `File uploaded successfully to ${_req.url}.`,
          count: 1,
          data: [
            {
              ..._file,
              id: download_id,
              uploaded_by: responsible_account.contact.id,
              etag: uploaded_from_remote?.etag,
              versionId: uploaded_from_remote?.versionId,
              download_path: `/api/file/${download_id}/download`,
            },
          ],
        },
      });
    }),
  };
}
