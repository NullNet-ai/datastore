import { Injectable, Logger } from '@nestjs/common';
import { UpdateMachine } from '../../machines/update/update.machine';
import { IActions } from '../../schemas/update/update.schema';
import { VerifyActionsImplementations } from '../verify';
/**
 * Implementation of actions for the UpdateMachine.
 */
@Injectable()
export class UpdateActionsImplementations {
  constructor(
    private logger: Logger,
    private readonly verifyActionsImplementations: VerifyActionsImplementations,
  ) {}
  public readonly actions: typeof UpdateMachine.prototype.actions & IActions = {
    updateEntry: () => {
      this.logger.log('updateEntry is called');
    },
    assignResponsibleAccount:
      this.verifyActionsImplementations.actions.assignResponsibleAccount,
  };
}
