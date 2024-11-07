import { Injectable, Logger } from '@nestjs/common';
import { CreateMachine } from '../../machines/create/create.machine';
import { IActions } from '../../schemas/create/create.schema';
import { VerifyActionsImplementations } from '../verify';
/**
 * Implementation of actions for the CreateMachine.
 */
@Injectable()
export class CreateActionsImplementations {
  constructor(
    private logger: Logger,
    private readonly verifyActionsImplementations: VerifyActionsImplementations,
  ) {}
  public readonly actions: typeof CreateMachine.prototype.actions & IActions = {
    createEntry: () => {
      this.logger.log('createEntry is called');
    },
    assignResponsibleAccount:
      this.verifyActionsImplementations.actions.assignResponsibleAccount,
  };
}
