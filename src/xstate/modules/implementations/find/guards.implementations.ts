
import { Injectable, Logger } from '@nestjs/common';
import { FindMachine } from '../../machines/find/find.machine';
import { IGuards } from '../../schemas/find/find.schema';
/**
 * Implementation of guards for the FindMachine.
 */
@Injectable()
export class FindGuardsImplementations {
  constructor(private logger: Logger) {}
  public readonly guards: typeof FindMachine.prototype.guards & IGuards = {
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
