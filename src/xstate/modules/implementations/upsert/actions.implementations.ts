import { Injectable } from '@nestjs/common';
import { UpsertMachine } from '../../machines/upsert/upsert.machine';
import { IActions } from '../../schemas/upsert/upsert.schema';
import { LoggerService } from '@dna-platform/common';
import { VerifyActionsImplementations } from '../verify';
/**
 * Implementation of actions for the UpsertMachine.
 */
@Injectable()
export class UpsertActionsImplementations {
  constructor(
    private logger: LoggerService,
    private readonly verifyActionsImplementations: VerifyActionsImplementations,
  ) {
    this.actions.assignResponsibleAccount =
      this.verifyActionsImplementations.actions.assignResponsibleAccount;
    this.actions.assignQueryDataPermissions =
      this.verifyActionsImplementations.actions.assignQueryDataPermissions;
  }
  public readonly actions: typeof UpsertMachine.prototype.actions & IActions = {
    upsertEntry: () => {
      this.logger.log('upsertEntry is called');
    },
  };
}
