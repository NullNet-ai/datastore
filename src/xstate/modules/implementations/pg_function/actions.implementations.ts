import { Injectable } from '@nestjs/common';
import { PgFunctionMachine } from '../../machines';
import { IActions } from '../../schemas/pg_function/pg_function.schema';
import { LoggerService } from '@dna-platform/common';
import { VerifyActionsImplementations } from '../verify';
import { assign } from 'xstate';
/**
 * Implementation of actions for the PgListenerMachine.
 */
@Injectable()
export class PgFunctionActionsImplementations {
  constructor(
    private logger: LoggerService,
    private readonly verifyActionsImplementations: VerifyActionsImplementations,
  ) {
    this.actions.assignResponsibleAccount =
      this.verifyActionsImplementations.actions.assignResponsibleAccount;
    this.actions.assignQueryDataPermissions =
      this.verifyActionsImplementations.actions.assignQueryDataPermissions;
  }
  public readonly actions: typeof PgFunctionMachine.prototype.actions &
    IActions = {
    pgFunctionEntry: () => {
      this.logger.log('pgFunctionEntry is called');
    },
    updateContext: assign({
      controller_args: ({ context }) => {
        const [_res, _req, _file] = context.controller_args;
        _req.params.table = 'postgres_channels';
        _req.body['timestamp'] = new Date();
        delete _req.body['table_name'];
        return [_res, _req, _file];
      },
    }),
  };
}
