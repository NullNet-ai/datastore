import { Injectable } from '@nestjs/common';
import { DownloadMachine } from '../../machines/download/download.machine';
import { IActions } from '../../schemas/download/download.schema';
import { VerifyActionsImplementations } from '../verify';
import * as path from 'path';
import type { Response } from 'express';
import { LoggerService } from '@dna-platform/common';
import { assign } from 'xstate';
import { HttpService } from '@nestjs/axios';
import { lastValueFrom } from 'rxjs';
import { MinioService } from '../../../../providers/files/minio.service';
/**
 * Implementation of actions for the DownloadMachine.
 */
@Injectable()
export class DownloadActionsImplementations {
  constructor(
    private logger: LoggerService,
    private readonly verifyActionsImplementations: VerifyActionsImplementations,
    private readonly axios: HttpService,
    private readonly minio: MinioService,
  ) {
    this.actions.assignResponsibleAccount =
      this.verifyActionsImplementations.actions.assignResponsibleAccount;
  }
  public readonly actions: typeof DownloadMachine.prototype.actions & IActions =
    {
      downloadEntry: () => {
        this.logger.log('downloadEntry is called');
      },
      testing: () => {
        this.logger.log('testing is called');
      },
      assignFileDetailsToControllerArgsRequest: assign({
        controller_args: ({ context, event }) => {
          const [_res, _req, _file] = context.controller_args;
          const data = event.output.payload.data[0];
          _req.params = {
            table: 'files',
            id: data.id,
          };
          _req.body = data;
          return [_res, _req, _file];
        },
      }),
      sendFileToClient: async ({ context, event }) => {
        const [_res] = context.controller_args;
        const [file] = event?.output?.payload?.data as any;
        const [_, file_type] = file.mimetype.split('/');
        const org_id = context.responsible_account.organization.id;
        const bucket_org_name = this.minio.getValidBucketName(
          context.responsible_account.organization.name ||
            process.env.STORAGE_BUCKET_NAME,
          org_id,
        );

        const dataStream = await this.minio.client
          ?.getObject(bucket_org_name, `${file.id}.${file_type}`)
          .catch((err) => {
            this.logger.error(`[ERROR][getObject]: ${err.message}`);
            return;
          });
        dataStream?.pipe(_res);
      },
      sendFileToClientPreview: async ({ context, event }) => {
        const [_res, _req] = context.controller_args;
        const { body } = _req;
        const [file] = event?.output?.payload?.data as any;
        const { mimetype } = file;
        const { originalname, organization_id, id } = body;
        const extention = path.extname(originalname);
        const file_name = `${id}-${organization_id}${extention}`;
        try {
          this.logger.debug(
            `Downloading file ${file_name}: ${file.presignedURL}`,
          );
          const results = await lastValueFrom(
            this.axios.get(file.presignedURL, {
              responseType: 'arraybuffer',
            }),
          ).then(({ data }) => data);
          (_res as Response).set({
            'Content-Type': mimetype,
          });

          (_res as Response).send(Buffer.from(results));
        } catch (error) {
          throw error;
        }
      },
    };
}
