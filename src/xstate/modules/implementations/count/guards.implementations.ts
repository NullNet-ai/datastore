
import { Injectable } from '@nestjs/common';
import { CountMachine } from '../../machines/count/count.machine';
import { IGuards } from '../../schemas/count/count.schema';
import { LoggerService } from '@dna-platform/common';
/**
 * Implementation of guards for the CountMachine.
 */
@Injectable()
export class CountGuardsImplementations {
  constructor(private logger: LoggerService) {}
  public readonly guards: typeof CountMachine.prototype.guards & IGuards = {
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
