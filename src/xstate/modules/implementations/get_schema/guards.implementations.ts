
import { Injectable, Logger } from '@nestjs/common';
import { GetSchemaMachine } from '../../machines/get_schema/get_schema.machine';
import { IGuards } from '../../schemas/get_schema/get_schema.schema';
/**
 * Implementation of guards for the GetSchemaMachine.
 */
@Injectable()
export class GetSchemaGuardsImplementations {
  constructor(private logger: Logger) {}
  public readonly guards: typeof GetSchemaMachine.prototype.guards & IGuards = {
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
