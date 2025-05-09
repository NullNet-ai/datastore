import { Injectable } from '@nestjs/common';
import { VerifyMachine } from '../../machines/verify/verify.machine';
import { IActions } from '../../schemas/verify/verify.schema';
import { assign } from 'xstate';
import { LoggerService } from '@dna-platform/common';
import { Utility } from 'src/utils/utility.service';
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
    assignQueryDataPermissions: assign({
      data_permissions_query: ({ context }) => {
        const { controller_args, responsible_account } = context;
        const [_res, _req] = controller_args;
        const { params, body } = _req;
        const { table } = params;
        const { pluck = [], pluck_object = {}, pluck_group_object = {} } = body;
        const fieldsToUse = pluck_object?.[table]?.length
          ? [...pluck_object[table], ...(pluck_group_object?.[table] || [])]
          : pluck;
        const main_fields = fieldsToUse.filter(
          (field, index, self) => self.indexOf(field) === index,
        );
        const query = `
        SELECT entities.name as entity,fields.name as field,permissions.* FROM data_permissions LEFT JOIN entity_fields on data_permissions.entity_field_id = entity_fields.id LEFT JOIN fields on entity_fields.field_id = fields.id LEFT JOIN entities on entity_fields.entity_id = entities.id LEFT JOIN permissions on data_permissions.inherited_permission_id = permissions.id WHERE data_permissions.account_organization_id = '${
          responsible_account.account_organization_id
        }' ${Utility.constructPermissionSelectWhereClause({
          table,
          main_fields,
        })}`;
        return query;
      },
    }),
  };
}
