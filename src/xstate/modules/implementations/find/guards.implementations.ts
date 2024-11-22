import { Injectable } from '@nestjs/common';
import { FindMachine } from '../../machines/find/find.machine';
import { IGuards } from '../../schemas/find/find.schema';
import { LoggerService } from '@dna-platform/common';
/**
 * Implementation of guards for the FindMachine.
 */
@Injectable()
export class FindGuardsImplementations {
  constructor(private logger: LoggerService) {}
  public readonly guards: typeof FindMachine.prototype.guards & IGuards = {
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
