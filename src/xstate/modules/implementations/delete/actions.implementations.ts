import { Injectable } from '@nestjs/common';
import { DeleteMachine } from '../../machines/delete/delete.machine';
import { IActions } from '../../schemas/delete/delete.schema';
import { VerifyActionsImplementations } from '../verify';
import { LoggerService } from '@dna-platform/common';
/**
 * Implementation of actions for the DeleteMachine.
 */
@Injectable()
export class DeleteActionsImplementations {
  constructor(
    private logger: LoggerService,
    private readonly verifyActionsImplementations: VerifyActionsImplementations,
  ) {}
  public readonly actions: typeof DeleteMachine.prototype.actions & IActions = {
    deleteEntry: () => {
      this.logger.log('deleteEntry is called');
    },
    assignResponsibleAccount:
      this.verifyActionsImplementations.actions.assignResponsibleAccount,
  };
}
