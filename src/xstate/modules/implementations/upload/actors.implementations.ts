import { Injectable, NotFoundException } from '@nestjs/common';
import { IResponse, LoggerService } from '@dna-platform/common';
import { fromPromise } from 'xstate';
import { IActors } from '../../schemas/upload/upload.schema';
import { CreateActorsImplementations } from '../create/actors.implementations';
import { VerifyActorsImplementations } from '../verify';
import * as path from 'path';
import { MinioService } from '../../../../providers/files/minio.service';
import { ulid } from 'ulid';
import { ObjectMetaData } from 'minio/dist/main/internal/type';
// import storage_policy from './storage_policy';
const { NODE_ENV = 'local', STORAGE_BUCKET_NAME = 'test' } = process.env;
@Injectable()
export class UploadActorsImplementations {
  constructor(
    private readonly minioService: MinioService,
    private readonly createActorsImplementations: CreateActorsImplementations,
    private readonly verifyActorImplementations: VerifyActorsImplementations,
    private readonly logger: LoggerService,
  ) {
    this.actors.verify = this.verifyActorImplementations.actors.verify;
    this.actors.create = this.createActorsImplementations.actors.create;
  }

  /**
   * Implementation of actors for the upload machine.
   */
  public readonly actors: IActors = {
    /**
     * Sample step actor implementation.
     * @param input - The input object containing the context.
     * @returns A promise that resolves to an IResponse object.
     */
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
      if (!_file) throw new NotFoundException('File not found');
      // Set the object metadata
      const metadata: ObjectMetaData = {
        'Content-Type': _file.mimetype,
        // 'X-Amz-Meta-Testing': 1234,
        // example: 5678,
      };
      const download_id = ulid();
      _file.id = download_id;

      let uploaded_from_remote;
      if (!['local'].includes(NODE_ENV)) {
        const [_, file_type] = _file.mimetype.split('/');
        const filepath = path.join(process.cwd(), _file.path);
        const org_id = responsible_account.organization.id;
        const bucket_org_name = this.minioService.getValidBucketName(
          responsible_account.organization.name || STORAGE_BUCKET_NAME,
          org_id,
        );
        uploaded_from_remote = await this.minioService.client
          ?.fPutObject(
            bucket_org_name,
            `${_file.id}.${file_type}`,
            filepath,
            metadata,
          )
          .catch((err) => {
            this.logger.error(`[ERROR][fPutObject]: ${err.message}`);
            return null;
          });

        this.logger.log(`[UPLOADED]: ${JSON.stringify(uploaded_from_remote)}`);
      }

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
