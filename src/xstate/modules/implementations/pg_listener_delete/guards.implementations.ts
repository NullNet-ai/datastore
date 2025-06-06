import { Injectable } from '@nestjs/common';
import { PgListenerDeleteMachine } from '../../machines/pg_listener_delete/pg_listener_delete.machine';
import { IGuards } from '../../schemas/pg_listener_delete/pg_listener_delete.schema';
import { LoggerService } from '@dna-platform/common';
/**
 * Implementation of guards for the PgListenerDeleteMachine.
 */
@Injectable()
export class PgListenerDeleteGuardsImplementations {
  constructor(private logger: LoggerService) {}
  public readonly guards: typeof PgListenerDeleteMachine.prototype.guards &
    IGuards = {
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
