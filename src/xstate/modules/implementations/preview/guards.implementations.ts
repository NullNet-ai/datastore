import { Injectable } from '@nestjs/common';
import { PreviewMachine } from '../../machines/preview/preview.machine';
import { IGuards } from '../../schemas/preview/preview.schema';
import { LoggerService } from '@dna-platform/common';
/**
 * Implementation of guards for the PreviewMachine.
 */
@Injectable()
export class PreviewGuardsImplementations {
  constructor(private logger: LoggerService) {}
  public readonly guards: typeof PreviewMachine.prototype.guards & IGuards = {
    hasControllerArgs: ({ context }) => {
      if (!context.controller_args) return false;
      const hasNoControllerArgs = !!context.controller_args.length;
      this.logger.log(
        `Sample guard is called [hasNoControllerArgs:${hasNoControllerArgs}]`,
      );
      return hasNoControllerArgs;
    },
    isFileAlreadyHavesPresignedURL: ({ event }) => {
      return event.output.payload.hasPresignedURL;
    },
  };
}
