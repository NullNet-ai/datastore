
import { Injectable, Logger } from '@nestjs/common';
import { DownloadMachine } from '../../machines/download/download.machine';
import { IActions } from '../../schemas/download/download.schema';
/**
 * Implementation of actions for the DownloadMachine.
 */
@Injectable()
export class DownloadActionsImplementations {
  constructor(private logger: Logger) {}
  public readonly actions: typeof DownloadMachine.prototype.actions & IActions =
    {
      downloadEntry: () => {
        this.logger.log('downloadEntry is called');
      },
    };
}
