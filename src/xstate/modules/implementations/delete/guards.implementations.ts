import { Injectable, Logger } from '@nestjs/common';
import { DeleteMachine } from '../../machines/delete/delete.machine';
import { IGuards } from '../../schemas/delete/delete.schema';
/**
 * Implementation of guards for the DeleteMachine.
 */
@Injectable()
export class DeleteGuardsImplementations {
  constructor(private logger: Logger) {}
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
