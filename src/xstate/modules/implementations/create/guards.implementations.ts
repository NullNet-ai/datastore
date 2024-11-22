import { Injectable } from '@nestjs/common';
import { CreateMachine } from '../../machines/create/create.machine';
import { IGuards } from '../../schemas/create/create.schema';
import { LoggerService } from '@dna-platform/common';
/**
 * Implementation of guards for the CreateMachine.
 */
@Injectable()
export class CreateGuardsImplementations {
  constructor(private logger: LoggerService) {}
  public readonly guards: typeof CreateMachine.prototype.guards & IGuards = {
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
