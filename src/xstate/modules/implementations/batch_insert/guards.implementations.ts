
import { Injectable } from '@nestjs/common';
import { BatchInsertMachine } from '../../machines/batch_insert/batch_insert.machine';
import { IGuards } from '../../schemas/batch_insert/batch_insert.schema';
import { LoggerService } from '@dna-platform/common';
/**
 * Implementation of guards for the BatchInsertMachine.
 */
@Injectable()
export class BatchInsertGuardsImplementations {
  constructor(private logger: LoggerService) {}
  public readonly guards: typeof BatchInsertMachine.prototype.guards & IGuards = {
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
