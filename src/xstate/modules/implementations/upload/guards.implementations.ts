import { Injectable } from '@nestjs/common';
import { UploadMachine } from '../../machines/upload/upload.machine';
import { IGuards } from '../../schemas/upload/upload.schema';
import { LoggerService } from '@dna-platform/common';
/**
 * Implementation of guards for the UploadMachine.
 */
@Injectable()
export class UploadGuardsImplementations {
  constructor(private logger: LoggerService) {}
  public readonly guards: typeof UploadMachine.prototype.guards & IGuards = {
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
