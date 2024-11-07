
import { Injectable, Logger } from '@nestjs/common';
import { UploadsMachine } from '../../machines/uploads/uploads.machine';
import { IGuards } from '../../schemas/uploads/uploads.schema';
/**
 * Implementation of guards for the UploadsMachine.
 */
@Injectable()
export class UploadsGuardsImplementations {
  constructor(private logger: Logger) {}
  public readonly guards: typeof UploadsMachine.prototype.guards & IGuards = {
    hasControllerArgs: ({ context }) => {
      if (!context.controller_args) return false;
      const hasNoControllerArgs = !!context.controller_args.length;
      this.logger.log(
        `Sample guard is called [hasNoControllerArgs:${hasNoControllerArgs}]`,
      );
      return hasNoControllerArgs;
    },
  };
}
