import { Injectable } from '@nestjs/common';
import { FindMachine } from '../../machines/find/find.machine';
import { IActions } from '../../schemas/find/find.schema';
import { VerifyActionsImplementations } from '../verify';
import { LoggerService } from '@dna-platform/common';
/**
 * Implementation of actions for the FindMachine.
 */
@Injectable()
export class FindActionsImplementations {
  constructor(
    private logger: LoggerService,
    private readonly verifyActionsImplementations: VerifyActionsImplementations,
  ) {
    this.actions.assignResponsibleAccount =
      this.verifyActionsImplementations.actions.assignResponsibleAccount;
    this.actions.assignQueryDataPermissions =
      this.verifyActionsImplementations.actions.assignQueryDataPermissions;
  }
  public readonly actions: typeof FindMachine.prototype.actions & IActions = {
    findEntry: () => {
      this.logger.log('findEntry is called');
    },
  };
}
