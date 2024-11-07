import { Injectable, Logger } from '@nestjs/common';
import { FindMachine } from '../../machines/find/find.machine';
import { IActions } from '../../schemas/find/find.schema';
import { VerifyActionsImplementations } from '../verify';
/**
 * Implementation of actions for the FindMachine.
 */
@Injectable()
export class FindActionsImplementations {
  constructor(
    private logger: Logger,
    private readonly verifyActionsImplementations: VerifyActionsImplementations,
  ) {}
  public readonly actions: typeof FindMachine.prototype.actions & IActions = {
    findEntry: () => {
      this.logger.log('findEntry is called');
    },
    assignResponsibleAccount:
      this.verifyActionsImplementations.actions.assignResponsibleAccount,
  };
}
