import { Injectable } from '@nestjs/common';
import { UploadsMachine } from '../../machines/uploads/uploads.machine';
import { IActions } from '../../schemas/uploads/uploads.schema';
import { LoggerService } from '@dna-platform/common';
/**
 * Implementation of actions for the UploadsMachine.
 */
@Injectable()
export class UploadsActionsImplementations {
  constructor(private logger: LoggerService) {}
  public readonly actions: typeof UploadsMachine.prototype.actions & IActions =
    {
      uploadsEntry: () => {
        this.logger.log('uploadsEntry is called');
      },
    };
}
