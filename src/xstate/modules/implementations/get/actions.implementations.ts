import { Injectable } from '@nestjs/common';
import { GetMachine } from '../../machines/get/get.machine';
import { IActions } from '../../schemas/get/get.schema';
import { VerifyActionsImplementations } from '../verify';
import { LoggerService } from '@dna-platform/common';
/**
 * Implementation of actions for the GetMachine.
 */
@Injectable()
export class GetActionsImplementations {
  constructor(
    private logger: LoggerService,
    private readonly verifyActionsImplementations: VerifyActionsImplementations,
  ) {
    this.actions.assignResponsibleAccount =
      this.verifyActionsImplementations.actions.assignResponsibleAccount;
    this.actions.assignQueryDataPermissions =
      this.verifyActionsImplementations.actions.assignQueryDataPermissions;
  }
  public readonly actions: typeof GetMachine.prototype.actions & IActions = {
    getEntry: () => {
      this.logger.log('getEntry is called');
    },
  };
}
