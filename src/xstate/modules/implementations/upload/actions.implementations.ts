import { Injectable, Logger } from '@nestjs/common';
import { UploadMachine } from '../../machines/upload/upload.machine';
import { IActions } from '../../schemas/upload/upload.schema';
import { assign } from 'xstate';
/**
 * Implementation of actions for the UploadMachine.
 */
@Injectable()
export class UploadActionsImplementations {
  constructor(private logger: Logger) {}
  public readonly actions: typeof UploadMachine.prototype.actions & IActions = {
    uploadEntry: () => {
      this.logger.log('uploadEntry is called');
    },
    assignFileDetailsToControllerArgsRequest: assign({
      controller_args: ({ context, event }) => {
        const [_res, _req, _file] = context.controller_args;
        const data = event.output.payload.data[0];
        _req.params = {
          table: 'files',
        };
        _req.body = data;
        return [_res, _req, _file];
      },
    }),
  };
}
