import { Injectable } from '@nestjs/common';
import { TransactionsMachine } from '../../machines/transactions/transactions.machine';
import { IGuards } from '../../schemas/transactions/transactions.schema';
import { LoggerService } from '@dna-platform/common';
/**
 * Implementation of guards for the TransactionsMachine.
 */
@Injectable()
export class TransactionsGuardsImplementations {
  constructor(private logger: LoggerService) {}
  public readonly guards: typeof TransactionsMachine.prototype.guards &
    IGuards = {
    hasControllerArgs: ({ context }) => {
      if (!context.controller_args) return false;
      const hasNoControllerArgs = !!context.controller_args.length;
      this.logger.log(
        `[hasNoControllerArgs:${hasNoControllerArgs}] guard is called.`,
      );
      return hasNoControllerArgs;
    },
  };
}
