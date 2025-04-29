import {
  Injectable,
  NotFoundException,
  BadRequestException,
  // NotImplementedException,
} from '@nestjs/common';
import { IResponse, LoggerService } from '@dna-platform/common';
import { fromPromise } from 'xstate';
import { IActors } from '../../schemas/find/find.schema';
import { DrizzleService } from '@dna-platform/crdt-lww-postgres';
import { Utility } from '../../../../utils/utility.service';
import * as local_schema from '../../../../schema';
import {
  asc,
  desc,
  sql,
  SQLWrapper,
  AnyColumn,
  aliasedTable,
} from 'drizzle-orm';
import pick from 'lodash.pick';
import omit from 'lodash.omit';
import { VerifyActorsImplementations } from '../verify';
import { IParsedConcatenatedFields } from '../../../../types/utility.types';
import { EDateFormats } from 'src/utils/utility.types';
const pluralize = require('pluralize');
@Injectable()
export class FindActorsImplementations {
  private db;
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
      const { controller_args, responsible_account } = context;
      const { organization_id = '' } = responsible_account;
      const [_res, _req] = controller_args;
      const { params, body } = _req;
      const { table, type } = params;
      const {
        order_direction = 'asc',
        order_by = 'id',
        limit = 50,
        offset = 0,
        pluck = ['id'],
        advance_filters = [],
        joins = [],
        multiple_sort = [],
        pluck_object = {},
        concatenate_fields = [],
        date_format = 'YYYY-MM-DD',
        group_advance_filters = [],
        distinct_by = '',
        group_by = {},
        is_case_sensitive_sorting = false,
        pluck_group_object = {},
      } = body;

      if (group_advance_filters.length && advance_filters.length) {
        throw new BadRequestException({
          success: false,
          message: `You can either use [advance_filters] or [group_advance_filters] but not both.`,
          count: 0,
          data: [],
        });
      }

      if (group_advance_filters.length && advance_filters.length.length <= 1) {
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

      const parsed_concatenated_fields =
        Utility.parseConcatenateFields(concatenate_fields);
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

      joins.forEach(({ field_relation }) => {
        const { entity, alias } = field_relation.to;
        if (alias) {
          aliased_joined_entities.push({ alias, entity });
        }
      });

      multiple_sort.forEach(({ by_field }) => {
        //check if by_field is separated by a dot if not then throw an error
        if (!by_field.includes('.')) {
          by_field = `${table}.${by_field}`;
        }
        const [entity, field] = by_field.split('.');
        const concat_fields = parsed_concatenated_fields.fields;
        const non_aliased_entity: string =
          aliased_joined_entities.find(({ alias }) => alias === entity)
            ?.entity || entity;
        const field_exists =
          local_schema[non_aliased_entity]?.[field] ||
          concat_fields[entity]?.find((exp) => exp.includes(field));
        if (!field_exists) {
          let message;
          if (non_aliased_entity === entity) {
            message = `Field ${field} does not exist in ${entity}`;
          } else {
            message = `Field ${field} does not exist in ${entity} which is alias of ${non_aliased_entity}`;
          }
          throw new BadRequestException({
            success: false,
            message,
          });
        }
        const concat = concatenate_fields.find(
          (concat_entity) =>
            concat_entity.field_name === field &&
            concat_entity.entity === entity,
        );

        // put fields from order into pluck_object
        if (joins.length) {
          pluck_object[entity] = [
            ...new Set([
              ...pluck_object[entity],
              ...(concat ? concat.fields : [field]),
            ]),
          ];
        }
      });
      const requested_date_format: string =
        EDateFormats[date_format] || '%m/%d/%Y';

      let _pluck: string[] =
        pluck.length && !pluck.includes('*') ? pluck : ['id', 'code'];
      const { table_schema, schema } = Utility.checkTable(table);
      let _plucked_fields = Utility.parsePluckedFields(
        table,
        _pluck,
        requested_date_format,
      );
      _plucked_fields = Utility.parseMainConcatenations(
        concatenate_fields,
        table,
        _plucked_fields === null ? {} : _plucked_fields,
      );

      const selections = _plucked_fields === null ? undefined : _plucked_fields;

      let _db = this.db;

      let join_keys: string[] = Object.keys(pluck_object);
      let group_by_selections = {};
      // let group_by_agg_selections = {};
      let group_by_fields: Record<string, any> = {};
      let group_by_entities: Array<string> = [];
      if (group_by?.fields?.length) {
        const { fields = [], has_count = false } = group_by;
        group_by_selections = fields.reduce(
          (acc, field) => {
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
            if (
              parsed_concatenated_fields.fields[group_by_entity]?.includes(
                group_by_field,
              )
            )
              throw new BadRequestException({
                success: false,
                message: `You can't group by concatenated fields`,
              });
            else group_by_fields[group_field] = group_field;
            if (!group_field_schema)
              throw new BadRequestException({
                success: false,
                message: `you can only group results by main valid fields. ['${group_field}'] is not a valid entity field, nor a concatenated field.`,
              });
            // if (multiple_sort.length)
            //   throw new BadRequestException({
            //     success: false,
            //     message: `You can't group by fields if you have multiple sorting of fields`,
            //   });
            // const order_by_schema = grouped_entity_schema[order_by];
            // if (!order_by_schema)
            //   throw new BadRequestException({
            //     success: false,
            //     message: `Order by field ${order_by} does not exist in ${group_by_entity}`,
            //   });
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
                ...acc[group_by_entity],
                [group_by_field]: sql.raw(
                  `${group_by_entity}.${group_by_field}`,
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
        const join_selections = Utility.createSelections({
          table,
          pluck_object,
          pluck_group_object,
          joins,
          date_format,
          parsed_concatenated_fields,
          multiple_sort,
        });
        // const is_grouping_joined_entity = group_by_entities.some((key) =>
        //   Object.keys(join_selections ?? {}).includes(key),
        // );

        // if (is_grouping_joined_entity)
        //   throw new NotImplementedException({
        //     success: false,
        //     message: `Grouping joint entity is not allowed. Please group it with ${table} main fields.`,
        //   });

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
                delete (join_selections as Record<string, any> | undefined)?.id;
              return {
                ...acc,
                ...count_selection,
                [entity]: {
                  ...(fields as Record<string, any>),
                  ...join_selections,
                },
              };
            },
            {},
          ),
        };
        _db = _db
          .select({
            ...(Object.keys(group_by_selections).length
              ? join_selections_with_group_by
              : join_selections),
          })
          .from(table_schema);
      } else {
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
        const selections_with_group_by = {
          ...count_selection,
          [table]: group_by_selections?.[table] ?? {},
        };
        _db = _db
          .select({
            ...(Object.keys(group_by_selections).length
              ? selections_with_group_by
              : selections),
          })
          .from(table_schema);
      }

      _db = Utility.FilterAnalyzer(
        _db,
        table_schema,
        advance_filters,
        pluck_object,
        organization_id,
        joins,
        this.db,
        parsed_concatenated_fields,
        group_advance_filters,
        type,
      );

      const getSortSchemaAndField = (
        order_by: string,
        aliased_entities: Record<string, any>,
        transformed_concatenations: IParsedConcatenatedFields['expressions'],
        by_direction: string = 'asc',
        is_case_sensitive_sorting: boolean = false,
        group_by_selections: Record<string, any>,
      ) => {
        const by_entity_field = order_by.split('.');
        let sort_entity: any = table;
        let sort_schema = table_schema[by_entity_field[0] || 'id'];
        if (by_entity_field.length > 1) {
          const [_entity = '', by_field = 'id'] = by_entity_field;
          const is_aliased = Object.values(aliased_entities).find(
            ({ alias }) => alias === _entity,
          );
          sort_entity = !is_aliased ? pluralize(_entity) : _entity;
          // if (!join_keys.includes(entity) && !is_aliased)
          //   throw new BadRequestException({
          //     success: false,
          //     message: `Other than main entity, you can only sort by joined entities. ${entity} is not a joined entity nor an aliased joined entity.`,
          //   });
          if (
            !schema[sort_entity]?.[by_field] &&
            transformed_concatenations[sort_entity] &&
            sort_entity === table
            //if sort_entity is the main table and check if it has any field that is concatenated, and that field doesn't exist in the schema
          ) {
            const concatenation = transformed_concatenations[sort_entity]?.find(
              (exp) => exp.includes(by_field),
            );
            sort_schema = concatenation
              ? sql.raw(concatenation.split(' AS ')[0] as string)
              : undefined;
          } else if (
            !schema[sort_entity]?.[by_field] &&
            transformed_concatenations[sort_entity] &&
            transformed_concatenations[sort_entity]?.find((exp) =>
              exp.includes(by_field),
            )
            //if entity is not in the schema or its field is not in the schema and it is in the transformed concatenations and the field is in the transformed concatenations
          ) {
            const concat_sort_field: any = transformed_concatenations[
              sort_entity
            ]?.find((exp) => {
              return exp.includes(by_field);
            });
            let sort_query = concat_sort_field.split(' AS ')[0];
            if (!is_case_sensitive_sorting) {
              sort_query = `lower(${sort_query})`;
            }

            if (by_direction.toLowerCase() === 'asc') {
              return sql.raw(`MIN(${sort_query})`);
            } else {
              return sql.raw(`MAX(${sort_query})`);
            }
          } else if (sort_entity !== table) {
            let sort_query: any = `"${sort_entity}"."${by_field}"`;
            if (!is_case_sensitive_sorting) {
              sort_query = `lower(${sort_query})`;
            }
            if (by_direction.toLowerCase() === 'asc') {
              return sql.raw(`MIN(${sort_query})`);
            } else {
              return sql.raw(`MAX(${sort_query})`);
            }
          } else {
            let sort_query: any = `"${sort_entity}"."${by_field}"`;
            if (Object.keys(group_by_selections).length) {
              if (by_direction.toLowerCase() === 'asc') {
                return sql.raw(`MIN(${sort_query})`);
              } else {
                return sql.raw(`MAX(${sort_query})`);
              }
            }

            sort_schema = is_aliased
              ? sql.raw(sort_query)
              : schema[sort_entity][by_field];
          }
        }
        if (Object.keys(group_by_selections).length) {
          let sort_query: any = `"${sort_entity}"."${
            by_entity_field[0] || 'id'
          }"`;
          if (by_direction.toLowerCase() === 'asc') {
            return sql.raw(`MIN(${sort_query})`);
          } else {
            return sql.raw(`MAX(${sort_query})`);
          }
        }
        return sort_schema as SQLWrapper | AnyColumn;
      };
      const transformed_concatenations: IParsedConcatenatedFields['expressions'] =
        Utility.removeJoinedKeyword(parsed_concatenated_fields.expressions);
      // if (group_by_agg_selections[order_by]) {
      //   _db = _db.orderBy(
      //     ['asc', 'ascending'].includes(order_direction)
      //       ? asc(group_by_agg_selections[order_by])
      //       : desc(group_by_agg_selections[order_by]),
      //   );
      // } else
      if (multiple_sort.length) {
        _db = _db.orderBy(
          ...multiple_sort
            .map(
              ({
                by_direction,
                by_field,
                is_case_sensitive_sorting = false,
              }) => {
                let sort_field_schema = getSortSchemaAndField(
                  by_field,
                  aliased_joined_entities,
                  transformed_concatenations,
                  by_direction,
                  is_case_sensitive_sorting,
                  group_by_selections,
                );
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
                return ['asc', 'ascending'].includes(by_direction)
                  ? asc(sort_field_schema)
                  : desc(sort_field_schema);
              },
            )
            .filter(Boolean),
        );
      } else if (order_direction && order_by) {
        let sort_field_schema = getSortSchemaAndField(
          order_by,
          aliased_joined_entities,
          transformed_concatenations,
          order_direction,
          is_case_sensitive_sorting,
          group_by_selections,
        );
        const is_query_already_lowered = (() => {
          try {
            return JSON.stringify(sort_field_schema, null, 2).includes('lower');
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
      // group by main table if joins are present and check if table is hypertable or not
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
        let grouped: Array<any> = [];
        grouped = Object.keys(group_by_fields).map((group_by) =>
          sql.raw(group_by_fields[group_by]),
        );
        _db = _db.groupBy(grouped);
      }
      this.logger.debug(`Query: ${_db.toSQL().sql}`);
      this.logger.debug(`Params: ${_db.toSQL().params}`);
      const result = joins.length
        ? this.transformer(
            await _db,
            table,
            pluck_object,
            pluck_group_object,
            joins,
            concatenate_fields,
            group_by,
          )
        : await _db;

      if (!result || !result.length) {
        throw new NotFoundException({
          success: false,
          message: `No data found in ${table}`,
          count: 0,
          data: [],
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
        },
      });
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
  ) {
    return results?.map((main_item) => {
      let cloned_item = { ...main_item };
      if (group_by.fields?.length) {
        cloned_item = {
          ...cloned_item[table],
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
            const contactinated_related_fields = _concatenate_fields.find(
              (f) => f.aliased_entity === name,
            );

            const _item = Array.isArray(cloned_item?.[name] ?? [])
              ? cloned_item?.[name]?.reduce((__acc, item) => {
                  if (contactinated_related_fields) {
                    item = {
                      ...item,
                      [contactinated_related_fields.field_name]:
                        contactinated_related_fields.fields
                          .map(
                            (field) =>
                              acc[contactinated_related_fields.entity]?.[
                                field
                              ] ?? '',
                          )
                          .join(contactinated_related_fields?.separator ?? ''),
                    };
                  }

                  if (!_pluck_group_object[name]?.length) {
                    return item;
                  }
                  return Object.entries(item).reduce((_acc, [key]) => {
                    if (_pluck_group_object[name]) {
                      if (_pluck_group_object[name].includes(key)) {
                        _acc[pluralize(key)] = _acc?.[key] ?? [];
                        _acc[pluralize(key)].push(item[key]);
                      } else if (pluck_object[name].includes(key)) {
                        _acc[key] = cloned_item[name][0][key];
                      }
                      return _acc;
                    }

                    return {
                      ..._acc,
                      // by default always the 1st index
                      [key]: cloned_item[name][0][key],
                    };
                  }, __acc);
                }, {}) ?? null
              : {};
            const keys = Object.keys(_item ?? {});
            const l = keys.length;
            if (l === 1) {
              acc[table][name] = keys.reduce(
                (acc, key) => acc + cloned_item[name][0][key],
                '',
              );
            }

            return {
              ...acc,
              [name]: keys.length ? _item : null,
            };
          },
          {
            ...pick(main_item, ['count', 'total_group_count']),
            [table]: pick(
              this.reducer(cloned_item, pluck_object, table),
              pluck_object[table],
            ),
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
