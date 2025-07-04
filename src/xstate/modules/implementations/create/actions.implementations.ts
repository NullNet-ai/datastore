import { Injectable } from '@nestjs/common';
import { CreateMachine } from '../../machines/create/create.machine';
import { IActions } from '../../schemas/create/create.schema';
import { VerifyActionsImplementations } from '../verify';
import { LoggerService } from '@dna-platform/common';
/**
 * Implementation of actions for the CreateMachine.
 */
@Injectable()
export class CreateActionsImplementations {
  constructor(
    private logger: LoggerService,
    private readonly verifyActionsImplementations: VerifyActionsImplementations,
  ) {
    this.actions.assignResponsibleAccount =
      this.verifyActionsImplementations.actions.assignResponsibleAccount;
    this.actions.assignQueryDataPermissions =
      this.verifyActionsImplementations.actions.assignQueryDataPermissions;
  }
  public readonly actions: typeof CreateMachine.prototype.actions & IActions = {
    createEntry: () => {
      this.logger.log('createEntry is called');
    },
  };
}
