import { Injectable } from '@nestjs/common';
import { UploadMachine } from '../../machines/upload/upload.machine';
import { IActions } from '../../schemas/upload/upload.schema';
import { assign } from 'xstate';
import { VerifyActionsImplementations } from '../verify';
import { LoggerService } from '@dna-platform/common';
/**
 * Implementation of actions for the UploadMachine.
 */
@Injectable()
export class UploadActionsImplementations {
  constructor(
    private logger: LoggerService,
    private readonly verifyActionsImplementations: VerifyActionsImplementations,
  ) {
    this.actions.assignResponsibleAccount =
      this.verifyActionsImplementations.actions.assignResponsibleAccount;
  }
  public readonly actions: typeof UploadMachine.prototype.actions & IActions = {
    uploadEntry: () => {
      this.logger.log('uploadEntry is called');
    },

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
