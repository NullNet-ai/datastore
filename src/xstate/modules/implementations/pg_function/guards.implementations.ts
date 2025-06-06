import { Injectable } from '@nestjs/common';
import { PgFunctionMachine } from '../../machines/pg_function/pg_function.machine';
import { IGuards } from '../../schemas/pg_function/pg_function.schema';
import { LoggerService } from '@dna-platform/common';
/**
 * Implementation of guards for the PgListenerMachine.
 */
@Injectable()
export class PgFunctionGuardsImplementations {
  constructor(private logger: LoggerService) {}
  public readonly guards: typeof PgFunctionMachine.prototype.guards & IGuards =
    {
      hasControllerArgs: ({ context }) => {
        if (!context.controller_args) return false;
        const hasNoControllerArgs = !!context.controller_args.length;
        this.logger.log(
          `[hasNoControllerArgs:${hasNoControllerArgs}] gaurd is called.`,
        );
        return hasNoControllerArgs;
      },
    };
}
