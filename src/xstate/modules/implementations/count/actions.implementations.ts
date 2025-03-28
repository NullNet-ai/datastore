import { Injectable } from '@nestjs/common';
import { CountMachine } from '../../machines/count/count.machine';
import { IActions } from '../../schemas/count/count.schema';
import { LoggerService } from '@dna-platform/common';
import { VerifyActionsImplementations } from '../verify';
/**
 * Implementation of actions for the CountMachine.
 */
@Injectable()
export class CountActionsImplementations {
  constructor(
    private logger: LoggerService,
    private readonly verifyActionsImplementations: VerifyActionsImplementations,
  ) {
    this.actions.assignResponsibleAccount =
      this.verifyActionsImplementations.actions.assignResponsibleAccount;
  }
  public readonly actions: typeof CountMachine.prototype.actions & IActions = {
    countEntry: () => {
      this.logger.log('countEntry is called');
    },
  };
}
