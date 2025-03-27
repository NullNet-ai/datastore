import { Injectable, NotFoundException } from '@nestjs/common';
import { IActors } from '../../schemas/preview/preview.schema';
import { MinioService } from '../../../../providers/files/minio.service';
import { GetFileByIdActorsImplementations } from '../get_file_by_id';
import { IResponse, LoggerService } from '@dna-platform/common';
import { fromPromise } from 'xstate';
import { VerifyActorsImplementations } from '../verify';
import { UpdateActorsImplementations } from '../update';
const {
  STORAGE_BUCKET_NAME = '',
  // 7days
  STORAGE_FILE_EXPIRES_SECONDS = '604800',
} = process.env;
@Injectable()
export class PreviewActorsImplementations {
  constructor(
    private readonly minioService: MinioService,
    private readonly getFileByIdActorImplementations: GetFileByIdActorsImplementations,
    private readonly verifyActorImplementations: VerifyActorsImplementations,
    private readonly logger: LoggerService,
    private readonly updateActorImplementations: UpdateActorsImplementations,
  ) {
    this.actors.getFileById =
      this.getFileByIdActorImplementations.actors.getFileById;
    this.actors.verify = this.verifyActorImplementations.actors.verify;
    this.actors.update = this.updateActorImplementations.actors.update;
  }
  /**
   * Implementation of actors for the preview mac
   * hine.
   */
  public readonly actors: IActors = {
    /**
     * Sample step actor implementation.
     * @param input - The input object containing the context.
     * @returns A promise that resolves to an IResponse object.
     */
    prepreview: fromPromise(async ({ input }): Promise<IResponse> => {
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

      const [_res, _req] = context?.controller_args;
      const { params } = _req;
      const { data = [], success, message } = event?.output?.payload;
      if (!success) throw new NotFoundException(message);
      const [_file] = data;
      const hasPresignedURL = !!_file.presignedURL;
      if (hasPresignedURL) {
        return Promise.resolve({
          payload: {
            hasPresignedURL,
            success: true,
            message:
              'Presigned URL already exists for [${STORAGE_BUCKET_NAME}/${_file.originalname}]',
            count: 1,
            data: [
              {
                id: params.id,
                originalname: _file.originalname,
                presignedURL: _file.presignedURL,
                presignedURLExpires: _file.presignedURLExpires,
              },
            ],
          },
        } as IResponse);
      }

      this.logger.log(
        `Creating a presigned URL for [${STORAGE_BUCKET_NAME}/${_file.originalname}]...`,
      );
      const presignedURLExpires = +STORAGE_FILE_EXPIRES_SECONDS;
      const presignedURL = await this.minioService.client?.presignedGetObject(
        STORAGE_BUCKET_NAME,
        _file.originalname,
        presignedURLExpires,
      );
      if (!presignedURL)
        throw new Error(
          `No presigned URL created for [${STORAGE_BUCKET_NAME}/${_file.originalname}]`,
        );

      return Promise.resolve({
        payload: {
          hasPresignedURL,
          success: true,
          message: 'Successfully created a presigned URL',
          count: 1,
          data: [
            {
              id: params.id,
              originalname: _file.originalname,
              presignedURL,
              presignedURLExpires,
            },
          ],
        },
      } as IResponse);
    }),
  };
}
