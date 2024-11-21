import { Injectable, Logger } from '@nestjs/common';
import { DownloadMachine } from '../../machines/download/download.machine';
import { IGuards } from '../../schemas/download/download.schema';
/**
 * Implementation of guards for the DownloadMachine.
 */
@Injectable()
export class DownloadGuardsImplementations {
  constructor(private logger: Logger) {}
  public readonly guards: typeof DownloadMachine.prototype.guards & IGuards = {
    hasControllerArgs: ({ context }) => {
      if (!context.controller_args) return false;
      const hasNoControllerArgs = !!context.controller_args.length;
      this.logger.log(
        `[hasNoControllerArgs:${hasNoControllerArgs}] guard is called.`,
      );
      return hasNoControllerArgs;
    },
  };
}
