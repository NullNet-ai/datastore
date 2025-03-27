import { Injectable, NotFoundException } from '@nestjs/common';
import { IResponse, LoggerService } from '@dna-platform/common';
import { fromPromise } from 'xstate';
import { VerifyActorsImplementations } from '../verify';
import { GetFileByIdActorsImplementations } from '../get_file_by_id';
import { IActors } from '../../schemas/download/download.schema';
import { PreviewActorsImplementations } from '../preview';
import { UpdateActorsImplementations } from '../update';
@Injectable()
export class DownloadActorsImplementations {
  private size = 0;
  private file;
  private chunks: any[] = [];
  constructor(
    private readonly verifyActorImplementations: VerifyActorsImplementations,
    private readonly getFileByIdActorImplementations: GetFileByIdActorsImplementations,
    private readonly logger: LoggerService,
    private readonly previewActorImplementations: PreviewActorsImplementations,
    private readonly updateActorImplementations: UpdateActorsImplementations,
  ) {
    this.onData = this.onData.bind(this);
    this.actors.getFileById =
      this.getFileByIdActorImplementations.actors.getFileById;
    this.actors.verify = this.verifyActorImplementations.actors.verify;
    this.actors.prepreview = this.previewActorImplementations.actors.prepreview;
    this.actors.update = this.updateActorImplementations.actors.update;
  }
  /**
   * Implementation of actors for the create machine.
   */
  public readonly actors: IActors = {
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
      const { controller_args, responsible_account } = context;
      const { organization_id = '' } = responsible_account;
      const [_res, _req] = controller_args;
      const { body, query } = _req;

      if (!body?.organization_id) {
        body.organization_id = organization_id;
      }
      const [_file] = event?.output?.payload?.data;
      if (!_file) throw new NotFoundException('File not found');
      const hasPresignedURL = !!_file.presignedURL;
      if (query.p === '1') {
        return Promise.resolve({
          payload: {
            hasPresignedURL,
            success: true,
            message: `Preview File found at `,
            count: 1,
            data: [this.file],
          },
        });
      }
      // download the file from file storage
      this.file = _file;
      return {
        payload: {
          hasPresignedURL,
          success: true,
          message: `File found at `,
          count: 1,
          data: [this.file],
        },
      };
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
