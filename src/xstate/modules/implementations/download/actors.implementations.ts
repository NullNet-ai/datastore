import { Injectable } from '@nestjs/common';
import { IResponse, LoggerService } from '@dna-platform/common';
import { fromPromise } from 'xstate';
import { VerifyActorsImplementations } from '../verify';
import { GetFileByIdActorsImplementations } from '../get_file_by_id';
import { IActors } from '../../schemas/download/download.schema';
import { UploadActorsImplementations } from '../upload';
import * as Minio from 'minio';
import * as fs from 'fs';
import * as path from 'path';
const { UPLOAD_BUCKET_NAME = 'test', NODE_ENV = 'local' } = process.env;
@Injectable()
export class DownloadActorsImplementations {
  private client: Minio.Client;
  private size = 0;
  private file;
  private chunks: any[] = [];
  constructor(
    private readonly verifyActorImplementations: VerifyActorsImplementations,
    private readonly getFileByIdActorImplementations: GetFileByIdActorsImplementations,
    private readonly uploadActorsImplementations: UploadActorsImplementations,
    private readonly logger: LoggerService,
  ) {
    this.onData = this.onData.bind(this);
  }
  /**
   * Implementation of actors for the create machine.
   */
  public readonly actors: IActors = {
    getFileById: this.getFileByIdActorImplementations.actors.getFileById,
    verify: this.verifyActorImplementations.actors.verify,
    download: fromPromise(async ({ input }): Promise<IResponse> => {
      const { context, event } = input;
      if (!context?.controller_args)
        return Promise.reject({
          payload: {
            success: false,
            message: `No controller args found`,
            count: 0,
            data: [],
          },
        });
      this.client = this.uploadActorsImplementations.client;
      const { controller_args, responsible_account } = context;
      const { organization_id = '' } = responsible_account;
      const [_res, _req] = controller_args;
      const { body } = _req;
      if (!body?.organization_id) {
        body.organization_id = organization_id;
      }
      const [_file] = event?.output?.payload?.data;
      this.file = _file;
      this.logger.debug(`Downloading file ${_file.originalname}`);

      let merged_chunked;
      if (!['local'].includes(NODE_ENV)) {
        const extention = path.extname(this.file.originalname);
        const file_name = `${this.file.id}-${organization_id}${extention}`;
        const temp_file_path = path.join(process.cwd(), 'tmp', file_name);
        // Check if the file or directory exists synchronously
        if (fs.existsSync(temp_file_path)) {
          merged_chunked = {
            is_temp: true,
            temp_file_path,
          };
        }

        if (!merged_chunked) {
          const dataStream = await this.client.getObject(
            UPLOAD_BUCKET_NAME,
            _file.originalname,
          );
          merged_chunked = await new Promise((resolve, reject) => {
            dataStream.on('data', this.onData);
            dataStream.on('end', () => {
              resolve(Buffer.concat(this.chunks).toString('base64'));
            });
            dataStream.on('error', reject);
          });
        }
      } else {
        // In local environment, we don't have Minio, so we just return the file as is
        merged_chunked = _file;
      }

      const data = [
        {
          ...body,
          ..._file,
          downloaded_by: responsible_account.contact.id,
          merged_chunked,
        },
      ];
      // TODO: update the file to bed downloaded by the responsible account
      return Promise.resolve({
        payload: {
          success: true,
          message: `Downloaded File found at `,
          count: 1,
          data,
        },
      });
    }),
  };

  private onData(chunk: any) {
    this.chunks.push(chunk);
    this.size += chunk.length;
    this.logger.debug(
      'Data [' +
        this.file.originalname +
        ']: ' +
        chunk.length +
        '/' +
        this.size,
    );
  }
}
