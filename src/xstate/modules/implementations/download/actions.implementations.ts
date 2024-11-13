import { Injectable, Logger } from '@nestjs/common';
import { DownloadMachine } from '../../machines/download/download.machine';
import { IActions } from '../../schemas/download/download.schema';
import { VerifyActionsImplementations } from '../verify';
import * as path from 'path';
import * as fs from 'fs';
import type { Response } from 'express';
/**
 * Implementation of actions for the DownloadMachine.
 */
@Injectable()
export class DownloadActionsImplementations {
  constructor(
    private logger: Logger,
    private readonly verifyActionsImplementations: VerifyActionsImplementations,
  ) {}
  public readonly actions: typeof DownloadMachine.prototype.actions & IActions =
    {
      downloadEntry: () => {
        this.logger.log('downloadEntry is called');
      },
      assignResponsibleAccount:
        this.verifyActionsImplementations.actions.assignResponsibleAccount,
      sendToImageToClient: ({ context, event }) => {
        const [_res] = context.controller_args;
        const [file] = event?.output?.payload?.data as any;
        const {
          merged_chunked: actual_file,
          mimetype,
          path: _file_path,
          originalname,
          id,
          organization_id,
        } = file;
        const file_path = path.join(process.cwd(), _file_path || '');
        const extention = path.extname(originalname);
        const file_name = `${id}-${organization_id}${extention}`;
        switch (typeof actual_file) {
          case 'string':
            const img = Buffer.from(actual_file, 'base64');
            (_res as Response).set({
              'Content-Type': mimetype,
              'Content-Disposition': `attachment; filename="${file_name}"`,
            });
            const tmpdir = 'tmp';
            const temp_file_path = path.join(
              process.cwd(),
              `${tmpdir}/temp-${file_name}`,
            );
            fs.mkdirSync(`${tmpdir}`, { recursive: true });
            fs.writeFileSync(temp_file_path, img);
            (_res as Response).sendFile(temp_file_path);
            break;
          default:
            (_res as Response).set({
              'Content-Type': mimetype,
              'Content-Disposition': `attachment; filename="${file_name}"`,
            });
            if (actual_file?.is_temp)
              (_res as Response).sendFile(actual_file?.temp_file_path);
            else (_res as Response).sendFile(file_path);
            break;
        }
      },
    };
}
