
import { Injectable, Logger } from '@nestjs/common';
import { SchemaMachine } from '../../machines/schema/schema.machine';
import { IGuards } from '../../schemas/schema/schema.schema';
/**
 * Implementation of guards for the SchemaMachine.
 */
@Injectable()
export class SchemaGuardsImplementations {
  constructor(private logger: Logger) {}
  public readonly guards: typeof SchemaMachine.prototype.guards & IGuards =
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
