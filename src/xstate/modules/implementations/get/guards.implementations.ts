import { Injectable } from '@nestjs/common';
import { GetMachine } from '../../machines/get/get.machine';
import { IGuards } from '../../schemas/get/get.schema';
import { LoggerService } from '@dna-platform/common';
/**
 * Implementation of guards for the GetMachine.
 */
@Injectable()
export class GetGuardsImplementations {
  constructor(private logger: LoggerService) {}
  public readonly guards: typeof GetMachine.prototype.guards & IGuards = {
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
