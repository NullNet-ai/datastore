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
        const { params, body, method, url, query: req_query } = _req;
        const { table } = params;
        let tables = [];
        let main_fields = [];
        const schema: {
          entity: string;
          alias: string;
          field: string;
          property_name: string;
          path: string;
        }[] = [];
        const request_info = `${method}:${url}`;
        const query_string = Object.entries(req_query)
          .map(([key, val], index) => {
            if (index === 0) return `?${key}=${val}`;
            return `${key}=${val}`;
          })
          .join('&');
          const write_method = _req.method === 'POST' ? 'POST' : 'PATCH';
          const single_record = _req.method === 'PATCH' ? params.id : '';
          const write_endpoint = `${write_method}:/api/store/${table}${
            single_record ? `/${single_record}` : ``
          }${query_string}`;

          switch (request_info) {
            case `POST:/api/store/${table}/filter${query_string}`:
              const { main_fields: read_main_fields, tables: read_tables } =
                this.accumulateReadInformation.bind(this)({
                  body,
                  table,
                  tables,
                  main_fields,
                  schema,
                });
              tables = read_tables;
              main_fields = read_main_fields;
              break;
            case write_endpoint:
              const { main_fields: write_main_fields, tables: write_tables } =
                this.accumulateWriteInformation.bind(this)({
                  body,
                  table,
                  tables,
                  main_fields,
                  schema,
                });
              tables = write_tables;
              main_fields = write_main_fields;
              break;
            default:
              break;
          }

          const query = `
        SELECT entities.name as entity,fields.name as field,permissions.sensitive as sensitive,permissions.read as read,permissions.write as write,permissions.encrypt as encrypt,permissions.decrypt as decrypt,permissions.required as required,permissions.archive as archive,permissions.delete as delete, data_permissions.account_organization_id as account_organization_id, permissions.id as pid FROM data_permissions LEFT JOIN entity_fields on data_permissions.entity_field_id = entity_fields.id LEFT JOIN fields on entity_fields.field_id = fields.id LEFT JOIN entities on entity_fields.entity_id = entities.id LEFT JOIN permissions on data_permissions.inherited_permission_id = permissions.id WHERE data_permissions.account_organization_id = '${
          responsible_account.account_organization_id
        }' ${Utility.constructPermissionSelectWhereClause({
            tables,
            main_fields,
          })}`;
          const valid_pass_keys_query = `
        SELECT id FROM encryption_keys WHERE safe_decrypt(organization_id::BYTEA,'${process.env.PGP_SYM_KEY}') = '${responsible_account.organization_id}' AND safe_decrypt(entity::BYTEA,'${process.env.PGP_SYM_KEY}') = '${table}'
        `;
          const record_valid_pass_keys_query = `
          SELECT 
          entities.name as entity,
          count(fields.name) AS total_fields,
          COUNT(*) FILTER (WHERE permissions.write IS TRUE) as total_fields_with_write,
          CASE WHEN COUNT(*) FILTER (WHERE permissions.sensitive IS TRUE) != count(fields.name) THEN false ELSE true END AS sensitive,
          CASE WHEN COUNT(*) FILTER (WHERE permissions.read IS TRUE) != count(fields.name) THEN false ELSE true END AS read,
          CASE WHEN COUNT(*) FILTER (WHERE permissions.write IS TRUE) != count(fields.name) THEN false ELSE true END AS write,
          CASE WHEN COUNT(*) FILTER (WHERE permissions.encrypt IS TRUE) != count(fields.name) THEN false ELSE true END AS encrypt,
          CASE WHEN COUNT(*) FILTER (WHERE permissions.decrypt IS TRUE) != count(fields.name) THEN false ELSE true END AS decrypt,
          CASE WHEN COUNT(*) FILTER (WHERE permissions.required IS TRUE) != count(fields.name) THEN false ELSE true END AS required,
          CASE WHEN COUNT(*) FILTER (WHERE permissions.archive IS TRUE) != count(fields.name) THEN false ELSE true END AS archive,
          CASE WHEN COUNT(*) FILTER (WHERE permissions.delete IS TRUE) != count(fields.name) THEN false ELSE true END AS delete
          FROM data_permissions 
          LEFT JOIN entity_fields on data_permissions.entity_field_id = entity_fields.id 
          LEFT JOIN fields on entity_fields.field_id = fields.id 
          LEFT JOIN entities on entity_fields.entity_id = entities.id 
          LEFT JOIN permissions on data_permissions.inherited_permission_id = permissions.id 
          WHERE entities.name = '${table}'
          GROUP BY entities.name;
        `;

          return {
            query,
            account_organization_id:
              responsible_account.account_organization_id,
            schema,
            valid_pass_keys_query,
            record_valid_pass_keys_query,
          };
      },
    }),
  };

  private accumulateReadInformation({
    body,
    table,
    tables,
    schema,
    main_fields,
  }) {
    this.logger.debug(`accumulateReadInformation`);
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
      distinct_by,
    } = body;

    const join_fields: string[] = [];
    tables = joins.reduce(
      (acc, join, index) => {
        const { to, from } = join.field_relation;
        if (to) {
          schema.push({
            entity: to.entity,
            alias: to.alias,
            field: to.field,
            property_name: `joins`,
            path: `[${index}].field_relation.to.field`,
          });
          if (!acc.includes(to?.entity)) {
            acc.push(to.entity);
          }
          if (!join_fields.includes(to?.field)) {
            join_fields.push(to.field);
          }
        }

        if (from) {
          schema.push({
            entity: from.entity,
            alias: from.alias,
            field: from.field,
            property_name: `joins`,
            path: `[${index}].field_relation.from.field`,
          });
          if (!acc.includes(from?.entity)) {
            acc.push(from.entity);
          }

          if (!join_fields.includes(from?.field)) {
            join_fields.push(from.field);
          }
        }

        return acc;
      },
      [table],
    );
    const common_fields_from_query = [
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
      ...[distinct_by]
        .map((field) => {
          const split_field = field?.split('.') ?? [];
          const _field =
            split_field.length > 1 ? split_field[1] : split_field[0];

          if (_field) {
            const split_entity = pluralize(
              split_field.length > 1
                ? pluralize(split_field[0])
                : pluralize(table),
            );
            tables.push(split_entity);
            schema.push({
              entity: split_entity,
              alias: split_entity,
              field: _field,
              property_name: `distinct_by`,
              path: ``,
            });
          }

          return _field;
        })
        .filter(Boolean),
    ];
    const fieldsToUse = pluck_object?.[table]?.length
      ? [
          ...pluck_object[table],
          ...common_fields_from_query,
          ...(pluck_group_object?.[table] || []),
          ...join_fields,
        ]
      : [...pluck, ...common_fields_from_query];

    main_fields = fieldsToUse.filter(
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

    return {
      main_fields,
      tables,
    };
  }
  private accumulateWriteInformation({
    body,
    table,
    tables,
    schema,
    main_fields,
  }) {
    this.logger.debug(`accumulateWriteInformation`);
    tables.push(table);
    main_fields = Object.keys(body);
    main_fields.forEach((field) => {
      schema.push({
        entity: table,
        alias: '',
        field,
        property_name: ``,
        path: `${field} = ${body?.[field]}`,
      });
    });
    return {
      tables,
      main_fields,
    };
  }
}
