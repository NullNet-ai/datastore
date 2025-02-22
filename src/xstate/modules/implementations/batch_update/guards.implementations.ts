
import { Injectable } from '@nestjs/common';
import { BatchUpdateMachine } from '../../machines/batch_update/batch_update.machine';
import { IGuards } from '../../schemas/batch_update/batch_update.schema';
import { LoggerService } from '@dna-platform/common';
/**
 * Implementation of guards for the BatchUpdateMachine.
 */
@Injectable()
export class BatchUpdateGuardsImplementations {
  constructor(private logger: LoggerService) {}
  public readonly guards: typeof BatchUpdateMachine.prototype.guards & IGuards = {
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
