import { Injectable } from '@nestjs/common';
import { DownloadMachine } from '../../machines/download/download.machine';
import { IGuards } from '../../schemas/download/download.schema';
import { LoggerService } from '@dna-platform/common';
/**
 * Implementation of guards for the DownloadMachine.
 */
@Injectable()
export class DownloadGuardsImplementations {
  constructor(private logger: LoggerService) {}
  public readonly guards: typeof DownloadMachine.prototype.guards & IGuards = {
    hasControllerArgs: ({ context }) => {
      if (!context.controller_args) return false;
      const hasNoControllerArgs = !!context.controller_args.length;
      this.logger.log(
        `[hasNoControllerArgs:${hasNoControllerArgs}] guard is called.`,
      );
      return hasNoControllerArgs;
    },
    isPreviewEnabled: ({ context }) => {
      return context.controller_args[1].query.p === '1';
    },
  };
}
