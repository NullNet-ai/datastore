import {
  Injectable,
  NotFoundException,
  BadRequestException,
} from '@nestjs/common';
import { IResponse, LoggerService } from '@dna-platform/common';
import { fromPromise } from 'xstate';
import { IActors } from '../../schemas/find/find.schema';
import { DrizzleService } from '@dna-platform/crdt-lww-postgres';
import { Utility } from '../../../../utils/utility.service';
import { EDateFormats } from '../../../../utils/utility.types';
import * as local_schema from '../../../../schema';
import { asc, desc, sql, aliasedTable } from 'drizzle-orm';
import pick from 'lodash.pick';
import omit from 'lodash.omit';
import { VerifyActorsImplementations } from '../verify';
const pluralize = require('pluralize');
import sha1 from 'sha1';
@Injectable()
export class FindActorsImplementations {
  private db;
  private not_allowed_entities = [
    'data_permissions',
    'entities',
    'fields',
    'permissions',
    'entity_fields',
  ];

  constructor(
    private readonly drizzleService: DrizzleService,
    private readonly verifyActorImplementations: VerifyActorsImplementations,
    private readonly logger: LoggerService,
  ) {
    this.db = this.drizzleService.getClient();
    this.actors.verify = this.verifyActorImplementations.actors.verify;
  }
  public readonly actors: IActors = {
    find: fromPromise(async ({ input }): Promise<IResponse> => {
      let metadata: Record<string, any> = [];
      let errors: { message: string; stack: string; status_code: number }[] =
        [];
      try {
        const { context } = input;
        if (!context?.controller_args)
          return Promise.reject({
            payload: {
              success: false,
              message: `No controller args found`,
              count: 0,
              data: [],
            },
          });
        const { controller_args, responsible_account, data_permissions_query } =
          context;
        const { organization_id = '', account_organization_id } =
          responsible_account;
        const [_res, _req] = controller_args;
        const { params, headers, query } = _req;
        const { p, rp } = query;

        let { body } = _req;
        const { time_zone, host, cookie } = headers;
        const { table, type } = params;
        if (this.not_allowed_entities.includes(table)) {
          return Promise.reject({
            payload: {
              success: false,
              message: `Table ${table} is not allowed.`,
              count: 0,
              data: [],
            },
          });
        }
        const {
          metadata: _metadata,
          getValidPassKeys,
          getPermissions,
          getRecordPermissions,
        } = Utility.getCachedPermissions('read', {
          data_permissions_query,
          host,
          cookie,
          headers,
          table,
          account_organization_id,
          db: this.db,
          body,
          account_id: responsible_account.account_id,
          metadata,
          query,
        });
        const permissions = p === 'true' ? await getPermissions : { data: [] };
        const record_permissions =
          rp === 'true' ? await getRecordPermissions : { data: [] };
        let { data: valid_pass_keys } = await getValidPassKeys;
        valid_pass_keys = valid_pass_keys?.map((key) => key.id);
        const pass_field_key = !query?.pfk
          ? valid_pass_keys?.[0] ?? ''
          : query?.pfk;
        const meta_permissions = permissions.data.map((p) =>
          pick(p, [
            'entity',
            'field',
            'read',
            'write',
            'encrypt',
            'decrypt',
            'sensitive',
            'archive',
            'delete',
          ]),
        );
        const meta_record_permissions = record_permissions.data;
        const {
          order_direction = 'asc',
          order_by = 'id',
          limit = 50,
          offset = 0,
          pluck = [],
          advance_filters = [],
          joins = [],
          multiple_sort = [],
          pluck_object = {},
          concatenate_fields = [],
          date_format = EDateFormats['mm/dd/YYYY'],
          group_advance_filters = [],
          distinct_by = '',
          group_by = {},
          is_case_sensitive_sorting = false,
          pluck_group_object = {},
          encrypted_fields = permissions.data
            .filter((p) => p.decrypt)
            .map((p) => `${p.entity}.${p.field}`),
        } = body;

        if (!valid_pass_keys.includes(pass_field_key) && pass_field_key) {
          throw new BadRequestException({
            success: false,
            message: `Pass field key is not valid.`,
            count: 0,
            data: [],
          });
        }

        if (group_advance_filters.length && advance_filters.length) {
          throw new BadRequestException({
            success: false,
            message: `You can either use [advance_filters] or [group_advance_filters] but not both.`,
            count: 0,
            data: [],
          });
        }

        if (
          group_advance_filters.length &&
          advance_filters.length.length <= 1
        ) {
          throw new BadRequestException({
            success: false,
            message: `Group advance filters must be more than 1. Use the [advance_filters] instead.`,
            count: 0,
            data: [],
          });
        }
        if (
          (Object.keys(group_by).length || group_by?.fields?.length) &&
          distinct_by
        ) {
          throw new BadRequestException({
            success: false,
            message: `You can only use one of the [group_by] or [distinct_by].`,
            count: 0,
            data: [],
          });
        }

        // Create the concatenated expressions for all the fields to concatenate, without the aliasing ('AS')
        const concatenated_field_expressions =
          Utility.generateConcatenatedExpressions(
            concatenate_fields,
            date_format,
            table,
          );

        const parsed_concatenated_fields: any = Utility.parseConcatenateFields(
          concatenate_fields,
          date_format,
          table,
        );
        let aliased_joined_entities: Record<string, any>[] = [];
        Object.keys(pluck_object).forEach((key) => {
          pluck_object[key] = [
            ...new Set([
              ...pluck_object[key],
              ...(parsed_concatenated_fields?.additional_fields?.[key] ?? []),
              'id',
            ]),
          ];
        });

        // assign all aliased entity/ies
        joins.forEach(({ field_relation }) => {
          const { entity, alias } = field_relation.to;
          if (alias) {
            aliased_joined_entities.push({ alias, entity });
          }
        });

        // Modify the pluck object in consideration with sorting and concatenating
        multiple_sort.forEach(({ by_field }) => {
          let entity = table;
          let field = by_field.split('.')[0];
          if (by_field.split('.').length > 1) {
            entity = by_field.split('.')[0];
            field = by_field.split('.')[1];
          }
          const is_alias = aliased_joined_entities.find(
            ({ alias }) => alias === entity,
          );
          const sorted_entity: string = is_alias?.entity || pluralize(entity);
          const concatenated_fields =
            Object.keys(
              concatenated_field_expressions?.[
                is_alias ? entity : pluralize(entity)
              ] || {},
            ) || [];
          const field_exists =
            local_schema?.[sorted_entity]?.[field] ||
            concatenated_fields.includes(field);
          if (!field_exists) {
            let message;
            if (sorted_entity === entity) {
              message = `Field ${field} does not exist in ${entity}`;
            } else {
              message = `Field ${field} does not exist in ${entity} which is alias of ${sorted_entity}`;
            }
            throw new BadRequestException({
              success: false,
              message,
            });
          }
          const concat = concatenate_fields.find(
            (concat_entity) =>
              concat_entity.field_name === field &&
              concat_entity.entity === sorted_entity,
          );

          // put fields from order into pluck_object
          if (
            (!Object.keys(group_by).length || !group_by?.fields?.length) &&
            joins.length
          ) {
            pluck_object[sorted_entity] = [
              ...new Set([
                ...pluck_object?.[sorted_entity],
                ...(concat ? concat.fields : [field]),
              ]),
            ];
          }
        });

        let _pluck: string[] =
          pluck.length && !pluck.includes('*') ? pluck : ['id', 'code'];
        const { table_schema, schema } = Utility.checkTable(table);
        let _plucked_fields = Utility.parsePluckedFields({
          table,
          pluck: _pluck,
          date_format,
          encrypted_fields,
          time_zone,
          pass_field_key,
        });

        _plucked_fields = Utility.parseMainConcatenations(
          concatenate_fields,
          table,
          _plucked_fields === null ? {} : _plucked_fields,
          date_format,
        );

        const selections =
          _plucked_fields === null ? undefined : _plucked_fields;

        let _db = this.db;

        let join_keys: string[] = Object.keys(pluck_object);
        let group_by_selections = {};
        // let group_by_agg_selections = {};
        let group_by_fields: Record<string, any> = {};
        let group_by_entities: Array<string> = [];
        if (group_by?.fields?.length) {
          const { fields = [], has_count = false } = group_by;
          const temp_pluck_object = {};
          // create the group by selections (including count and overall group count
          // and modify pluck objects accordingly
          // get the grouped fields to be used on groupBy
          group_by_selections = fields.reduce(
            (acc, field, index) => {
              let group_by_entity = table;
              const _field = field.split('.');
              let group_by_field = _field[0];
              if (_field.length > 1) {
                const [entity, field] = _field;
                group_by_entity = entity;
                group_by_field = field;
              }
              const alias = aliased_joined_entities?.find(
                ({ alias }) => alias === group_by_entity,
              );
              group_by_entity = alias
                ? group_by_entity
                : pluralize(group_by_entity || table);

              if (
                table !== group_by_entity &&
                !join_keys.includes(group_by_entity) &&
                !alias
              )
                throw new BadRequestException({
                  success: false,
                  message: `Other than main entity, you can only group results by fields of joined entities. ${group_by_entity} is not a joined entity nor an aliased joined entity.`,
                });
              const grouped_entity_schema = alias
                ? aliasedTable(schema[alias?.entity], group_by_entity)
                : schema[group_by_entity];
              let group_field_schema = grouped_entity_schema[group_by_field];
              const group_field = `${group_by_entity}.${group_by_field}`;

              const group_concatenated_field_exp =
                concatenated_field_expressions?.[group_by_entity]?.[
                  group_by_field
                ]?.expression;

              group_by_fields[group_field] = group_concatenated_field_exp
                ? group_concatenated_field_exp
                : group_field;

              if (!group_field_schema && !group_concatenated_field_exp)
                throw new BadRequestException({
                  success: false,
                  message: `you can only group results by main valid fields. ['${group_field}'] is not a valid entity field, nor a concatenated field.`,
                });

              if (!temp_pluck_object?.[group_by_entity])
                temp_pluck_object[group_by_entity] = ['id'];

              if (!group_concatenated_field_exp)
                temp_pluck_object[group_by_entity].push(group_by_field);

              if (fields.length - 1 === index) {
                pluck_object[group_by_entity] =
                  temp_pluck_object[group_by_entity];
                pluck_group_object[group_by_entity] = ['id'];

                if (group_by_entity !== table) {
                  pluck_object[table] = ['id'];
                  parsed_concatenated_fields.expressions[table] = [];
                  parsed_concatenated_fields.fields[table] = [];
                  parsed_concatenated_fields.additional_fields[table] = [];
                }
              }
              if (parsed_concatenated_fields.fields[group_by_entity]?.length) {
                parsed_concatenated_fields.expressions[group_by_entity] = [];
                parsed_concatenated_fields.fields[group_by_entity] = [];
                parsed_concatenated_fields.additional_fields[group_by_entity] =
                  [];
              }
              // const order_by_schema = grouped_entity_schema[order_by];
              // group_by_agg_selections = !group_by?.fields?.includes(order_by)
              //   ? {
              //       [order_by_schema.name]: sql.raw(
              //         `${
              //           ['asc', 'ascending'].includes(order_direction)
              //             ? 'MIN'
              //             : 'MAX'
              //         }("${table}"."${order_by_schema.name}")`,
              //       ),
              //     }
              //   : {};

              group_by_entities.push(group_by_entity);
              return {
                ...acc,
                [table]: {
                  // ...group_by_agg_selections,
                },
                [group_by_entity]: {
                  ...(acc?.[group_by_entity] ?? {}),
                  [group_by_field]: sql.raw(
                    `${
                      group_concatenated_field_exp?.length
                        ? `${group_concatenated_field_exp} AS ${group_by_field}`
                        : `${group_by_entity}.${group_by_field}`
                    }`,
                  ),
                },
              };
            },
            {
              ...(has_count
                ? {
                    count: sql.raw('COUNT(*)'),
                    total_group_count: sql.raw('COUNT(*) OVER ()'),
                  }
                : {}),
            },
          );
        }
        // Adding/Modifying selections for distinct
        // Distinct feature is not tested properly as it's not used on portal
        if (distinct_by) {
          const _distinct = distinct_by.split('.');
          let distinct_entity = table;
          let distinct_field = _distinct[0];
          if (_distinct.length > 1) {
            const [entity, field] = _distinct;
            distinct_entity = entity;
            distinct_field = field;
          }
          const alias = aliased_joined_entities?.find(
            ({ alias }) => alias === distinct_entity,
          );
          distinct_entity = alias
            ? distinct_entity
            : pluralize(distinct_entity || table);

          if (
            table !== distinct_entity &&
            !join_keys.includes(distinct_entity) &&
            !alias
          )
            throw new BadRequestException({
              success: false,
              message: `Other than main entity, you can only distinct a field of joined entities. ${distinct_entity} is not a joined entity nor an aliased joined entity.`,
            });

          _db = _db
            .select({
              [`${distinct_entity}`]: {
                [`${distinct_field}`]: sql.raw(
                  `DISTINCT ${distinct_entity}.${distinct_field}`,
                ),
              },
            })
            .from(table_schema);
        } else if (!distinct_by && joins?.length) {
          // Creating selections with Join
          const join_selections = Utility.createSelections({
            table,
            pluck_object,
            pluck_group_object,
            joins,
            date_format,
            parsed_concatenated_fields,
            encrypted_fields,
            time_zone,
            request_type: type,
            aliased_joined_entities,
            concatenated_field_expressions,
            pass_field_key,
            multiple_sort,
            organization_id,
          });
          // const is_grouping_joined_entity = group_by_entities.some((key) =>
          //   Object.keys(join_selections ?? {}).includes(key),
          // );

          // if (is_grouping_joined_entity)
          //   throw new NotImplementedException({
          //     success: false,
          //     message: `Grouping joint entity is not allowed. Please group it with ${table} main fields.`,
          //   });

          // Modifying selections if results are being grouped
          let count_selection = {};
          if ((group_by_selections as Record<string, any>)?.count)
            count_selection = {
              count: (group_by_selections as Record<string, any>).count,
              total_group_count: (group_by_selections as Record<string, any>)
                .total_group_count,
            };
          const join_selections_with_group_by = {
            ...Object.entries(group_by_selections).reduce(
              (acc, [entity, fields]) => {
                if (!Object.keys(fields as Record<string, any>).includes('id'))
                  delete (join_selections as Record<string, any> | undefined)
                    ?.id;
                return {
                  ...acc,
                  ...count_selection,
                  [entity]: {
                    ...(fields as Record<string, any>),
                    // ...join_selections,
                  },
                };
              },
              {},
            ),
          };
          _db = _db
            .select(
              Utility.decryptData(
                {
                  ...(Object.keys(group_by_selections).length
                    ? join_selections_with_group_by
                    : join_selections),
                },
                encrypted_fields,
                table,
                permissions,
                pass_field_key,
              ),
            )

            .from(table_schema);
        } else {
          // Selections with no join
          let count_selection = {};
          if ((group_by_selections as Record<string, any>)?.count)
            count_selection = {
              count: (group_by_selections as Record<string, any>).count,
              total_group_count: (group_by_selections as Record<string, any>)
                .total_group_count,
            };
          const has_plucked_not_grouped_fields = Object.keys(
            selections ?? {},
          ).some((key) => !group_by_selections?.[table]?.[key]);
          if (
            Object.keys(group_by_selections).length &&
            has_plucked_not_grouped_fields
          )
            throw new BadRequestException({
              success: false,
              message: `You can only select fields that are in the group_by fields.`,
            });
          // Modifying selections if being grouped
          const selections_with_group_by = {
            ...count_selection,
            [table]: group_by_selections?.[table] ?? {},
          };
          _db = _db
            .select(
              Utility.decryptData(
                {
                  ...(Object.keys(group_by_selections).length
                    ? selections_with_group_by
                    : selections),
                },
                encrypted_fields,
                table,
                permissions,
                pass_field_key,
              ),
            )
            .from(table_schema);
        }

        // Adding the left join and where clauses
        _db = Utility?.FilterAnalyzer({
          db: _db,
          table_schema,
          advance_filters,
          pluck_object,
          organization_id,
          joins,
          client_db: this.db,
          group_advance_filters,
          encrypted_fields,
          time_zone,
          table,
          date_format,
          pass_field_key,
          permissions,
          concatenated_field_expressions,
          concatenate_fields,
          request_type: type,
        });

        // if (group_by_agg_selections[order_by]) {
        //   _db = _db.orderBy(
        //     ['asc', 'ascending'].includes(order_direction)
        //       ? asc(group_by_agg_selections[order_by])
        //       : desc(group_by_agg_selections[order_by]),
        //   );
        // } else

        // Adding the multiple sorting
        if (multiple_sort.length) {
          _db = _db.orderBy(
            ...multiple_sort
              .map(
                ({
                  by_direction,
                  by_field,
                  is_case_sensitive_sorting = false,
                }) => {
                  let sort_field_schema = Utility.getSortSchemaAndField({
                    table,
                    table_schema,
                    order_by: by_field,
                    aliased_entities: aliased_joined_entities,
                    order_direction: by_direction,
                    is_case_sensitive_sorting,
                    group_by_selections,
                    concatenated_field_expressions,
                  });
                  const is_query_already_lowered = (() => {
                    try {
                      return JSON.stringify(
                        sort_field_schema,
                        null,
                        2,
                      ).includes('lower');
                    } catch {
                      return false; // If JSON.stringify fails (circular structure), return false
                    }
                  })();
                  if (!is_case_sensitive_sorting && !is_query_already_lowered) {
                    const sorted_field_type = (sort_field_schema as any)
                      .dataType;
                    if (sorted_field_type && sorted_field_type !== 'string')
                      throw new BadRequestException(
                        `Sorted field ${by_field} is of type ${sorted_field_type}. Set is_case_sensitive_sorting to true to sort non-text fields.`,
                      );
                    sort_field_schema = sql`lower(${sort_field_schema})`;
                  }
                  return ['asc', 'ascending'].includes(by_direction)
                    ? asc(sort_field_schema)
                    : desc(sort_field_schema);
                },
              )
              .filter(Boolean),
          );
        } else if (order_direction && order_by) {
          // If only simple sorting
          let sort_field_schema = Utility.getSortSchemaAndField({
            table,
            table_schema,
            order_by,
            aliased_entities: aliased_joined_entities,
            order_direction,
            is_case_sensitive_sorting,
            group_by_selections,
            concatenated_field_expressions,
          });
          const is_query_already_lowered = (() => {
            try {
              return JSON.stringify(sort_field_schema, null, 2).includes(
                'lower',
              );
            } catch {
              return false; // If JSON.stringify fails (circular structure), return false
            }
          })();
          if (!is_case_sensitive_sorting && !is_query_already_lowered) {
            sort_field_schema = sql`lower(${sort_field_schema})`;
          }
          _db = _db.orderBy(
            ['asc', 'ascending'].includes(order_direction)
              ? asc(sort_field_schema)
              : desc(sort_field_schema),
          );
        }
        if (offset) {
          _db = _db.offset(offset);
        }

        if (limit) {
          _db = _db.limit(limit);
        }

        // Groupings
        // group by main table if joins are present and check if table is hypertable or not
        // If with Join
        if (joins?.length) {
          if (group_by?.fields?.length) {
            let grouped: Array<any> = [];
            grouped = Object.keys(group_by_fields).map((group_by) => {
              return sql.raw(group_by_fields[group_by]);
            });

            _db = _db.groupBy(grouped);
          } else if (table_schema.hypertable_timestamp) {
            _db = _db.groupBy(table_schema.id, table_schema.timestamp);
          } else {
            _db = _db.groupBy(table_schema.id);
          }
        } else if (group_by?.fields?.length) {
          // If Grouping without Join
          let grouped: Array<any> = [];
          grouped = Object.keys(group_by_fields).map((group_by) =>
            sql.raw(group_by_fields[group_by]),
          );
          _db = _db.groupBy(grouped);
        }
        this.logger.debug(`Query: ${_db.toSQL().sql}`);
        this.logger.debug(`Params: ${_db.toSQL().params}`);
        const prepared_query_key = sha1(_db.toSQL().sql + _db.toSQL().params);
        const db_results: any = await _db.prepare(prepared_query_key).execute();

        // Transforming the results
        const result = joins.length
          ? this.transformer(
              db_results,
              table,
              pluck_object,
              pluck_group_object,
              joins,
              concatenate_fields,
              group_by,
              aliased_joined_entities,
            )
          : db_results;

        if (!result || !result.length) {
          throw new NotFoundException({
            success: true,
            message: `No data found in ${table}`,
            count: 0,
            data: [],
            metadata,
            errors,
            permissions: meta_permissions,
            record_permissions: meta_record_permissions,
          });
        }

        return Promise.resolve({
          payload: {
            success: true,
            message: `Successfully fetched ${table} records`,
            count: result.length,
            data: result.reduce((acc, item) => {
              const _item = { ...item };
              acc.push(_item);
              return acc;
            }, []),
            metadata,
            errors,
            permissions: meta_permissions,
            record_permissions: meta_record_permissions,
          },
        });
      } catch (error: any) {
        errors.push({
          message: error?.message,
          stack: error.stack,
          status_code: error.status_code,
        });
        if (error.status !== 400 && error.status < 500) throw error;
        throw new BadRequestException({
          success: false,
          message: `An error occurred while processing your request. Please review your query for any incorrect assignments. If the issue persists, contact your database administrator for further assistance.`,
          count: 0,
          data: [],
          metadata,
          errors,
        });
      }
    }),
  };

  private transformer(
    results,
    table,
    pluck_object,
    _pluck_group_object,
    joins,
    _concatenate_fields,
    group_by,
    aliased_joined_entities,
  ) {
    // get the concatenated main fields
    const main_fields_concatenated =
      _concatenate_fields
        .filter((f) => (f.aliased_entity || pluralize(f.entity)) === table)
        ?.map((f) => f.field_name) ?? [];

    return results.map((main_item) => {
      let cloned_item = { ...main_item };
      if (group_by.fields?.length) {
        // if there's a grouping,
        // the result is the grouped field inside the object value and with the entity as key
        // assign it to the cloned item as an array of object
        cloned_item = {
          ...cloned_item[table],
          ...group_by.fields?.reduce((acc, field) => {
            let group_by_entity = table;
            const _field = field.split('.');
            if (_field.length > 1) {
              const [entity] = _field;
              group_by_entity = entity;
            }
            const alias = aliased_joined_entities?.find(
              ({ alias }) => alias === group_by_entity,
            );
            group_by_entity = alias
              ? group_by_entity
              : pluralize(group_by_entity || table);

            return {
              ...acc,
              [group_by_entity]: [
                {
                  ...acc?.[group_by_entity]?.[0],
                  ...(main_item[group_by_entity] && main_item[group_by_entity]),
                },
              ],
            };
          }, {}),
        };
      }

      return joins
        .map((join) => {
          const isSelfJoin = join.type === 'self';
          const prop = isSelfJoin
            ? join.field_relation.from?.alias ||
              join.field_relation.from?.entity
            : join.field_relation.to?.alias || join.field_relation.to?.entity;
          return prop;
        })
        .reduce(
          (acc, name) => {
            const concatenated_related_fields = _concatenate_fields.find(
              (f) => (f.aliased_entity || pluralize(f.entity)) === name,
            );
            let [item = {}] = cloned_item?.[name] ?? [];
            // to add the concatenated fields to the item if it's not concatenated yet
            // (most likely it's already concatenated, this is before handling the concatenated fields on the db)
            if (concatenated_related_fields) {
              item = {
                ...item,
                ...(!item[concatenated_related_fields.field_name] && {
                  [concatenated_related_fields.field_name]:
                    concatenated_related_fields.fields
                      .map((field) => item?.[field] ?? '')
                      .join(concatenated_related_fields?.separator ?? ''),
                }),
              };
            }

            // this is group the fields indicated on the pluck_group_object
            item = {
              ...item,
              ...(_pluck_group_object[name]?.length && {
                ..._pluck_group_object[name].reduce((acc, key) => {
                  const grouped_field_key = `${name}_${pluralize(key)}`;
                  return {
                    ...acc,
                    ...(cloned_item[grouped_field_key] && {
                      [pluralize(key)]: cloned_item[grouped_field_key],
                    }),
                  };
                }, {}),
              }),
            };
            const keys = Object.keys(item ?? {});

            return {
              ...acc,
              [name]: keys.length ? item : null,
            };
          },
          {
            ...pick(main_item, ['count', 'total_group_count']),
            [table]: pick(this.reducer(cloned_item, pluck_object, table), [
              ...pluck_object[table],
              ...main_fields_concatenated,
            ]),
          },
        );
    });
  }

  private reducer(data, _pluck_object = {}, table) {
    const cloned_data = { ...data };
    return Object.entries(cloned_data).reduce((acc, [key, value]) => {
      const isSingular = pluralize.isSingular(key);
      const _val = Array.isArray(value) ? value[0] : value;
      if (_pluck_object?.[table]?.includes(key) && _pluck_object?.[key]) {
        return omit(acc, key);
      }
      return {
        ...acc,
        [key]: isSingular ? _val : value,
      };
    }, {});
  }
}
