import { Injectable } from '@nestjs/common';
import { UpsertMachine } from '../../machines/upsert/upsert.machine';
import { IGuards } from '../../schemas/upsert/upsert.schema';
import { LoggerService } from '@dna-platform/common';
/**
 * Implementation of guards for the UpsertMachine.
 */
@Injectable()
export class UpsertGuardsImplementations {
  constructor(private logger: LoggerService) {}
  public readonly guards: typeof UpsertMachine.prototype.guards & IGuards = {
    hasControllerArgs: ({ context }) => {
      if (!context.controller_args) return false;
      const hasNoControllerArgs = !!context.controller_args.length;
      this.logger.log(
        `Sample guard is called [hasNoControllerArgs:${hasNoControllerArgs}]`,
      );
      return hasNoControllerArgs;
    },
    recordExists: ({ context }) => {
      this.logger.log(`Record exists check: ${context.recordExists}`);
      console.log(context.recordExists);
      return context.recordExists === true;
    },
  };
}
