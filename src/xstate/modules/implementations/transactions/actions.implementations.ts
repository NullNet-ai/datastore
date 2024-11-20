
import { Injectable, Logger } from '@nestjs/common';
import { TransactionsMachine } from '../../machines/transactions/transactions.machine';
import { IActions } from '../../schemas/transactions/transactions.schema';
/**
 * Implementation of actions for the TransactionsMachine.
 */
@Injectable()
export class TransactionsActionsImplementations {
  constructor(private logger: Logger) {}
  public readonly actions: typeof TransactionsMachine.prototype.actions & IActions =
    {
      transactionsEntry: () => {
        this.logger.log('transactionsEntry is called');
      },
    };
}
