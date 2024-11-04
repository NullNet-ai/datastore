
import { Injectable, Logger } from '@nestjs/common';
import { CreateMachine } from '../../machines/create/create.machine';
import { IGuards } from '../../schemas/create/create.schema';
/**
 * Implementation of guards for the CreateMachine.
 */
@Injectable()
export class CreateGuardsImplementations {
  constructor(private logger: Logger) {}
  public readonly guards: typeof CreateMachine.prototype.guards & IGuards = {
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
