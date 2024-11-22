import { Injectable } from '@nestjs/common';
import { UploadsMachine } from '../../machines/uploads/uploads.machine';
import { IGuards } from '../../schemas/uploads/uploads.schema';
import { LoggerService } from '@dna-platform/common';
/**
 * Implementation of guards for the UploadsMachine.
 */
@Injectable()
export class UploadsGuardsImplementations {
  constructor(private logger: LoggerService) {}
  public readonly guards: typeof UploadsMachine.prototype.guards & IGuards = {
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
