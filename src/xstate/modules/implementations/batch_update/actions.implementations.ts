import { Injectable } from '@nestjs/common';
import { BatchUpdateMachine } from '../../machines/batch_update/batch_update.machine';
import { IActions } from '../../schemas/batch_update/batch_update.schema';
import { LoggerService } from '@dna-platform/common';
import { VerifyActionsImplementations } from '../verify';
/**
 * Implementation of actions for the BatchUpdateMachine.
 */
@Injectable()
export class BatchUpdateActionsImplementations {
  constructor(
    private logger: LoggerService,
    private readonly verifyActionsImplementations: VerifyActionsImplementations,
  ) {
    this.actions.assignResponsibleAccount =
      this.verifyActionsImplementations.actions.assignResponsibleAccount;
  }
  public readonly actions: typeof BatchUpdateMachine.prototype.actions &
    IActions = {
    batchUpdateEntry: () => {
      this.logger.log('batchUpdateEntry is called');
    },
  };
}
