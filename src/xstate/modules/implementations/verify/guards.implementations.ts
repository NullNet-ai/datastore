
import { Injectable, Logger } from '@nestjs/common';
import { VerifyMachine } from '../../machines/verify/verify.machine';
import { IGuards } from '../../schemas/verify/verify.schema';
/**
 * Implementation of guards for the VerifyMachine.
 */
@Injectable()
export class VerifyGuardsImplementations {
  constructor(private logger: Logger) {}
  public readonly guards: typeof VerifyMachine.prototype.guards & IGuards = {
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
