import { Injectable, Logger } from '@nestjs/common';
import { TemplateMachine } from '../../machines/template/template.machine';
import { IGuards } from '../../schemas/template/template.schema';
/**
 * Implementation of guards for the TemplateMachine.
 */
@Injectable()
export class TemplateGuardsImplementations {
  constructor(private logger: Logger) {}
  public readonly guards: typeof TemplateMachine.prototype.guards & IGuards = {
    hasControllerArgs: ({ context }) => {
      if (!context.controller_args) return false;
      const hasControllerArgs = !!context.controller_args.length;
      this.logger.log(
        `Sample guard is called [hasControllerArgs:${hasControllerArgs}]`,
      );
      return hasControllerArgs;
    },
  };
}
