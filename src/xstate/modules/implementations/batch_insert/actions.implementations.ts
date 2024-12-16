import { Injectable } from '@nestjs/common';
import { BatchInsertMachine } from '../../machines';
import { IActions } from '../../schemas/batch_insert/batch_insert.schema';
import { LoggerService } from '@dna-platform/common';
import { VerifyActionsImplementations } from '../verify';
/**
 * Implementation of actions for the BatchInsertMachine.
 */
@Injectable()
export class BatchInsertActionsImplementations {
  constructor(
    private logger: LoggerService,
    private readonly verifyActionsImplementations: VerifyActionsImplementations,
  ) {}
  public readonly actions: typeof BatchInsertMachine.prototype.actions &
    IActions = {
    batchInsertEntry: () => {
      this.logger.log('batchInsertEntry is called');
    },
    assignResponsibleAccount:
      this.verifyActionsImplementations.actions.assignResponsibleAccount,
  };
}
