import { Injectable } from '@nestjs/common';
import { VerifyMachine } from '../../machines/verify/verify.machine';
import { IActions } from '../../schemas/verify/verify.schema';
import { assign } from 'xstate';
import { LoggerService } from '@dna-platform/common';
/**
 * Implementation of actions for the VerifyMachine.
 */
@Injectable()
export class VerifyActionsImplementations {
  constructor(private logger: LoggerService) {}
  public readonly actions: typeof VerifyMachine.prototype.actions & IActions = {
    verifyEntry: () => {
      this.logger.log('verifyEntry is called');
    },
    assignResponsibleAccount: assign({
      responsible_account: ({ event }) => {
        const [{ account }] = event?.output?.payload?.data || [];
        return account;
      },
    }),
  };
}
