import { Injectable } from '@nestjs/common';
import { CreateHypertablesMachine } from '../../machines';
import { IActions } from '../../schemas/create_hypertables/create_hypertables.schema';
import { LoggerService } from '@dna-platform/common';
import { VerifyActionsImplementations } from '../verify';
/**
 * Implementation of actions for the CreateHypertablesMachine.
 */
@Injectable()
export class CreateHypertablesActionsImplementations {
  constructor(
    private logger: LoggerService,
    private readonly verifyActionsImplementations: VerifyActionsImplementations,
  ) {}
  public readonly actions: typeof CreateHypertablesMachine.prototype.actions &
    IActions = {
    createHypertablesEntry: () => {
      this.logger.log('createHypertablesEntry is called');
    },
    assignResponsibleAccount:
      this.verifyActionsImplementations.actions.assignResponsibleAccount,
  };
}
