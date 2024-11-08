
import { Injectable, Logger } from '@nestjs/common';
import { UploadsMachine } from '../../machines/uploads/uploads.machine';
import { IActions } from '../../schemas/uploads/uploads.schema';
/**
 * Implementation of actions for the UploadsMachine.
 */
@Injectable()
export class UploadsActionsImplementations {
  constructor(private logger: Logger) {}
  public readonly actions: typeof UploadsMachine.prototype.actions & IActions =
    {
      uploadsEntry: () => {
        this.logger.log('uploadsEntry is called');
      },
    };
}
