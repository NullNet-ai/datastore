import { Injectable } from '@nestjs/common';
import { VerifyMachine } from '../../machines/verify/verify.machine';
import { IActions } from '../../schemas/verify/verify.schema';
import { assign } from 'xstate';
import { LoggerService } from '@dna-platform/common';
import { Utility } from 'src/utils/utility.service';
import pluralize from 'pluralize';
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
        const {
          pluck = [],
          pluck_object = {},
          pluck_group_object = {},
          joins = [],
          group_by,
          concatenate_fields = [],
          multiple_sort = [],
          advance_filters = [],
          group_advance_filters = [],
        } = body;
        const schema: {
          entity: string;
          alias: string;
          field: string;
          property_name: string;
          path: string;
        }[] = [];
        const join_fields: string[] = [];
        const tables = joins.reduce(
          (acc, join, index) => {
            const { to, from } = join.field_relation;
            schema.push({
              entity: to.entity,
              alias: to.alias,
              field: to.field,
              property_name: `joins`,
              path: `[${index}].field_relation.to.field`,
            });
            schema.push({
              entity: from.entity,
              alias: from.alias,
              field: from.field,
              property_name: `joins`,
              path: `[${index}].field_relation.from.field`,
            });
            if (!acc.includes(to.entity)) {
              acc.push(to.entity);
            }
            if (!acc.includes(from.entity)) {
              acc.push(from.entity);
            }
            if (!join_fields.includes(to.field)) {
              join_fields.push(to.field);
            }
            if (!join_fields.includes(from.field)) {
              join_fields.push(from.field);
            }
            return acc;
          },
          [table],
        );

        const fieldsToUse = pluck_object?.[table]?.length
          ? [
              ...(group_by?.fields ?? []).reduce((acc, field, index) => {
                const [table, _field] = field.split('.');
                const _table = pluralize(table);
                if (!acc.includes(_field)) {
                  acc.push(_field);
                }
                if (!tables.includes(_table)) {
                  tables.push(_table);
                }
                schema.push({
                  entity: _table,
                  alias: _table,
                  field: _field,
                  property_name: `group_by`,
                  path: `[${index}]fields`,
                });
                return acc;
              }, []),
              ...pluck_object[table],
              ...(pluck_group_object?.[table] || []),
              ...join_fields,
              ...concatenate_fields.reduce((acc, cc, index) => {
                if (!tables.includes(cc.entity)) {
                  tables.push(cc.entity);
                }
                cc.fields.forEach((field) => {
                  schema.push({
                    entity: cc.entity,
                    alias: cc.entity,
                    field,
                    property_name: `concatenate_fields`,
                    path: `[${index}]fields`,
                  });
                });

                return [...acc, ...cc.fields];
              }, []),
              ...multiple_sort.reduce((acc, ms, index) => {
                const by = ms.by_field.split('.');
                const _table = by.length > 1 ? by[0] : table;
                if (!tables.includes(_table)) {
                  tables.push(_table);
                }
                const field = by.length > 1 ? by[1] : by[0];
                const _field = pluralize(field);
                if (!acc.includes(_field) && _field) {
                  acc.push(_field);
                }

                schema.push({
                  entity: _table,
                  alias: _table,
                  field: _field,
                  property_name: `multiple_sort`,
                  path: `[${index}].by_field`,
                });
                return acc;
              }, []),
              ...advance_filters.reduce((acc, af, index) => {
                if (af?.entity) {
                  const _table = pluralize(af.entity);
                  if (!tables.includes(_table)) {
                    tables.push(_table);
                  }
                }
                if (af?.field) {
                  if (!acc.includes(af.field)) {
                    acc.push(af.field);
                  }
                }
                schema.push({
                  entity: af?.entity ?? table,
                  alias: af?.entity ?? table,
                  field: af?.field,
                  property_name: `advance_filters`,
                  path: `[${index}].field`,
                });
                return acc;
              }, []),
              ...group_advance_filters.reduce((acc, gaf, gaf_index) => {
                const { filters = [] } = gaf;
                return acc.concat(
                  filters.reduce((_acc, af, af_index) => {
                    if (af?.entity) {
                      const _table = pluralize(af.entity);
                      if (!tables.includes(_table)) {
                        tables.push(_table);
                      }
                    }
                    if (af?.field) {
                      if (!_acc.includes(af.field)) {
                        _acc.push(af.field);
                      }
                    }
                    schema.push({
                      entity: af?.entity ?? table,
                      alias: af?.entity ?? table,
                      field: af?.field,
                      property_name: `group_advance_filters`,
                      path: `[${gaf_index}].filters[${af_index}].field`,
                    });
                    return _acc;
                  }, []),
                );
              }, []),
            ]
          : pluck;
        const main_fields = fieldsToUse.filter(
          (field, index, self) => self.indexOf(field) === index,
        );

        Object.keys(pluck_object).forEach((aliased_entity: string) => {
          if (pluck_object?.[aliased_entity]) {
            pluck_object?.[aliased_entity].forEach((key, index) => {
              schema.push({
                entity: aliased_entity,
                alias: aliased_entity,
                field: key,
                property_name: `pluck_object`,
                path: `[${index}]`,
              });
            });
          }
        });

        pluck.forEach((key, index) => {
          schema.push({
            entity: table,
            alias: '',
            field: key,
            property_name: `pluck`,
            path: `[${index}]`,
          });
        });

        Object.keys(pluck_group_object).forEach((aliased_entity: string) => {
          if (pluck_group_object?.[aliased_entity]) {
            pluck_group_object?.[aliased_entity].forEach((key, index) => {
              schema.push({
                entity: aliased_entity,
                alias: aliased_entity,
                field: key,
                property_name: `pluck_group_object`,
                path: `[${index}]`,
              });
            });
          }
        });

        const query = `
        SELECT entities.name as entity,fields.name as field,permissions.read as read,permissions.write as write,permissions.encrypted as encrypted,permissions.decrypted as decrypted,permissions.required as required, data_permissions.account_organization_id as account_organization_id FROM data_permissions LEFT JOIN entity_fields on data_permissions.entity_field_id = entity_fields.id LEFT JOIN fields on entity_fields.field_id = fields.id LEFT JOIN entities on entity_fields.entity_id = entities.id LEFT JOIN permissions on data_permissions.inherited_permission_id = permissions.id WHERE data_permissions.account_organization_id = '${
          responsible_account.account_organization_id
        }' ${Utility.constructPermissionSelectWhereClause({
          tables,
          main_fields,
        })}`;
        return {
          query,
          account_organization_id: responsible_account.account_organization_id,
          schema,
        };
      },
    }),
  };
}
