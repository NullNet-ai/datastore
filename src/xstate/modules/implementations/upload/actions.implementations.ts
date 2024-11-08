import { Injectable, Logger } from '@nestjs/common';
import { UploadMachine } from '../../machines/upload/upload.machine';
import { IActions } from '../../schemas/upload/upload.schema';
import { assign } from 'xstate';
import { VerifyActionsImplementations } from '../verify';
/**
 * Implementation of actions for the UploadMachine.
 */
@Injectable()
export class UploadActionsImplementations {
  constructor(
    private logger: Logger,
    private readonly verifyActionsImplementations: VerifyActionsImplementations,
  ) {}
  public readonly actions: typeof UploadMachine.prototype.actions & IActions = {
    uploadEntry: () => {
      this.logger.log('uploadEntry is called');
    },
    assignResponsibleAccount:
      this.verifyActionsImplementations.actions.assignResponsibleAccount,
    assignFileDetailsToControllerArgsRequest: assign({
      controller_args: ({ context, event }) => {
        const [_res, _req, _file] = context.controller_args;
        const { organization_id } = context.responsible_account;
        const data = event.output.payload.data[0];
        _req.params = {
          table: 'files',
        };
        _req.body = {
          ...data,
          organization_id,
        };
        return [_res, _req, _file];
      },
    }),
  };
}
