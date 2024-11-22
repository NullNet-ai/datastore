import { Injectable } from '@nestjs/common';
import { VerifyMachine } from '../../machines/verify/verify.machine';
import { IGuards } from '../../schemas/verify/verify.schema';
import { LoggerService } from '@dna-platform/common';
/**
 * Implementation of guards for the VerifyMachine.
 */
@Injectable()
export class VerifyGuardsImplementations {
  constructor(private logger: LoggerService) {}
  public readonly guards: typeof VerifyMachine.prototype.guards & IGuards = {
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
