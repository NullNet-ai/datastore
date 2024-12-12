
import { Injectable } from '@nestjs/common';
import { CreateHypertablesMachine } from '../../machines/create_hypertables/create_hypertables.machine';
import { IGuards } from '../../schemas/create_hypertables/create_hypertables.schema';
import { LoggerService } from '@dna-platform/common';
/**
 * Implementation of guards for the CreateHypertablesMachine.
 */
@Injectable()
export class CreateHypertablesGuardsImplementations {
  constructor(private logger: LoggerService) {}
  public readonly guards: typeof CreateHypertablesMachine.prototype.guards & IGuards = {
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
