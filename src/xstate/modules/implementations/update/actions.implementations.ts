import { Injectable } from '@nestjs/common';
import { UpdateMachine } from '../../machines/update/update.machine';
import { IActions } from '../../schemas/update/update.schema';
import { VerifyActionsImplementations } from '../verify';
import { LoggerService } from '@dna-platform/common';
import { assign } from 'xstate';
/**
 * Implementation of actions for the UpdateMachine.
 */
@Injectable()
export class UpdateActionsImplementations {
  constructor(
    private logger: LoggerService,
    private readonly verifyActionsImplementations: VerifyActionsImplementations,
  ) {
    this.actions.assignResponsibleAccount =
      this.verifyActionsImplementations.actions.assignResponsibleAccount;
    this.actions.assignQueryDataPermissions =
      this.verifyActionsImplementations.actions.assignQueryDataPermissions;
  }
  public readonly actions: typeof UpdateMachine.prototype.actions & IActions = {
    updateEntry: () => {
      this.logger.log('updateEntry is called');
    },
    updateContext: assign({
      controller_args: ({ context }) => {
        const [_res, _req, _file] = context.controller_args;
        const pluck_fields = [
          ...new Set([
            'id',
            ...(_req.query.pluck?.split(',') || []),
            ...(Object.keys(_req.body) || []),
          ]),
        ];
        _req.query.pluck = pluck_fields.join(',');
        return [_res, _req, _file];
      },
    }),
  };
}
