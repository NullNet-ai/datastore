import { Injectable } from '@nestjs/common';
import { UpdateMachine } from '../../machines/update/update.machine';
import { IGuards } from '../../schemas/update/update.schema';
import { LoggerService } from '@dna-platform/common';
/**
 * Implementation of guards for the UpdateMachine.
 */
@Injectable()
export class UpdateGuardsImplementations {
  constructor(private logger: LoggerService) {}
  public readonly guards: typeof UpdateMachine.prototype.guards & IGuards = {
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
