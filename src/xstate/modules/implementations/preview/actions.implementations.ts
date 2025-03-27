import { Injectable } from '@nestjs/common';
import { PreviewMachine } from '../../machines/preview/preview.machine';
import { IActions } from '../../schemas/preview/preview.schema';
import { LoggerService } from '@dna-platform/common';
// import * as fs from 'fs/promises';
import * as path from 'path';
import type { Response } from 'express';
import { VerifyActionsImplementations } from '../verify';
import { assign } from 'xstate';
import { HttpService } from '@nestjs/axios';
import { lastValueFrom } from 'rxjs';
/**
 * Implementation of actions for the PreviewMachine.
 */
@Injectable()
export class PreviewActionsImplementations {
  constructor(
    private logger: LoggerService,
    private readonly verifyActionsImplementations: VerifyActionsImplementations,
    private readonly axios: HttpService,
  ) {
    this.actions.assignResponsibleAccount =
      this.verifyActionsImplementations.actions.assignResponsibleAccount;
  }
  public readonly actions: typeof PreviewMachine.prototype.actions & IActions =
    {
      previewEntry: () => {
        this.logger.log('previewEntry is called');
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
      sendFileToClientPreview: async ({ context, event }) => {
        const [_res, _req] = context.controller_args;
        const [file] = event?.output?.payload?.data as any;
        const { mimetype, originalname, id, organization_id } = file;
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
