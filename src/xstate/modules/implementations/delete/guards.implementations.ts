import { Injectable } from '@nestjs/common';
import { DeleteMachine } from '../../machines/delete/delete.machine';
import { IGuards } from '../../schemas/delete/delete.schema';
import { LoggerService } from '@dna-platform/common';
/**
 * Implementation of guards for the DeleteMachine.
 */
@Injectable()
export class DeleteGuardsImplementations {
  constructor(private logger: LoggerService) {}
  public readonly guards: typeof DeleteMachine.prototype.guards & IGuards = {
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
