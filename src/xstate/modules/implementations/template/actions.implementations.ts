import { Injectable, Logger } from '@nestjs/common';
import { TemplateMachine } from '../../machines/template/template.machine';
import { IActions } from '../../schemas/template/template.schema';
/**
 * Implementation of actions for the TemplateMachine.
 */
@Injectable()
export class TemplateActionsImplementations {
  constructor(private logger: Logger) {}
  public readonly actions: typeof TemplateMachine.prototype.actions & IActions =
    {
      sampleAction: () => {
        this.logger.log('Sample action is called');
      },
    };
}
