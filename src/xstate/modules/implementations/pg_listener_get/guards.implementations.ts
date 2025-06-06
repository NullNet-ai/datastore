import { Injectable } from '@nestjs/common';
import { PgListenerGetMachine } from '../../machines';
import { IGuards } from '../../schemas/pg_listener_get/pg_listener_get.schema';
import { LoggerService } from '@dna-platform/common';
@Injectable()
export class PgListenerGetGuardsImplementations {
  constructor(private logger: LoggerService) {}
  public readonly guards: typeof PgListenerGetMachine.prototype.guards &
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
