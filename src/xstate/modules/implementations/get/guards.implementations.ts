
import { Injectable, Logger } from '@nestjs/common';
import { GetMachine } from '../../machines/get/get.machine';
import { IGuards } from '../../schemas/get/get.schema';
/**
 * Implementation of guards for the GetMachine.
 */
@Injectable()
export class GetGuardsImplementations {
  constructor(private logger: Logger) {}
  public readonly guards: typeof GetMachine.prototype.guards & IGuards = {
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
