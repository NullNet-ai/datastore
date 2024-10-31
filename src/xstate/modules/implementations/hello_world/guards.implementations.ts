import { Injectable, Logger } from '@nestjs/common';
import { HelloWorldMachine } from '../../machines/hello_world/hello_world.machine';
import { IGuards } from '../../schemas/hello_world/hello_world.schema';
/**
 * Implementation of guards for the HelloWorldMachine.
 */
@Injectable()
export class HelloWorldGuardsImplementations {
  constructor(private logger: Logger) {}
  public readonly guards: typeof HelloWorldMachine.prototype.guards & IGuards =
    {
      hasParamArgs: ({ context }) => {
        if (!context.param_args) return false;
        const hasNoParamArgs = !!context.param_args.length;
        this.logger.log(
          `Sample guard is called [hasNoParamArgs:${hasNoParamArgs}]`,
        );
        return hasNoParamArgs;
      },
    };
}
