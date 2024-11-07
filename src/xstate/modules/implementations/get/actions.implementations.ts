import { Injectable, Logger } from '@nestjs/common';
import { GetMachine } from '../../machines/get/get.machine';
import { IActions } from '../../schemas/get/get.schema';
import { VerifyActionsImplementations } from '../verify';
/**
 * Implementation of actions for the GetMachine.
 */
@Injectable()
export class GetActionsImplementations {
  constructor(
    private logger: Logger,
    private readonly verifyActionsImplementations: VerifyActionsImplementations,
  ) {}
  public readonly actions: typeof GetMachine.prototype.actions & IActions = {
    getEntry: () => {
      this.logger.log('getEntry is called');
    },
    assignResponsibleAccount:
      this.verifyActionsImplementations.actions.assignResponsibleAccount,
  };
}
