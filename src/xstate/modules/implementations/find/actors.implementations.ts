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

import {
  asc,
  desc,
  sql,
  aliasedTable,
  SQLWrapper,
  AnyColumn,
} from 'drizzle-orm';
import { VerifyActorsImplementations } from '../verify';
const pluralize = require('pluralize');
const pick = require('lodash.pick');
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
      const { table } = params;
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
        pluck_group_object = {},
      } = body;
      let _pluck =
        pluck.length && !pluck.includes('*') ? pluck : ['id', 'code'];
      const { table_schema, schema } = Utility.checkTable(table);
      const _plucked_fields = Utility.parsePluckedFields(table, _pluck);
      const selections = _plucked_fields === null ? undefined : _plucked_fields;
      let _db = this.db;

      let join_keys = Object.keys(pluck_object);
      let aliased_joined_entities: Record<string, any>[] = [];

      if (joins?.length) {
        const aliased_joint_entities = joins.reduce(
          (acc, { type, field_relation }) => {
            const { from, to } = field_relation;
            const aliased = type === 'self' ? from : to;
            if (aliased.alias) {
              const aliased_entity = aliasedTable(
                schema[aliased.entity],
                aliased.alias,
              );
              aliased_joined_entities.push(aliased as Record<string, any>);
              return {
                ...acc,
                [aliased.alias]: pick(
                  aliased_entity,
                  pluck_object[aliased.entity],
                ),
              };
            }
            return acc;
          },
          {},
        );
        const aliased_except_main_entity = aliased_joined_entities.reduce(
          (acc, curr) => {
            if (curr.entity !== table) {
              return {
                ...acc,
                [curr.entity]: curr,
              };
            }
            return acc;
          },
          {},
        );
        const pluck_group_object_keys = Object.keys(pluck_group_object);
        // @ts-ignore
        const _join_selections = join_keys?.length
          ? join_keys.reduce(
              (acc, entity) => {
                return {
                  ...acc,
                  ...(!Object.keys(aliased_except_main_entity).includes(entity)
                    ? {
                        [entity]: pick(
                          Utility.checkTable(entity).table_schema,
                          pluck_object[entity],
                        ),
                      }
                    : {}),
                  ...(pluck_group_object_keys?.length &&
                  pluck_group_object?.[entity]
                    ? pluck_group_object?.[entity].reduce((acc, field) => {
                        let entity_name = entity;
                        let schema_field = schema?.[entity][field];
                        if (
                          Object.keys(aliased_except_main_entity).includes(
                            entity,
                          )
                        ) {
                          const aliased = aliased_except_main_entity[entity];
                          entity_name = aliased.alias;
                          schema_field = sql.raw(`"${entity_name}"."${field}"`);
                        }
                        return {
                          ...acc,
                          [`${entity_name}`]: {
                            ...pluck_object?.[entity].reduce((acc, key) => {
                              return {
                                ...acc,
                                [key]: sql.raw(`"${entity_name}"."${key}"`),
                              };
                            }, {}),
                            ...acc?.[entity_name],
                            ...(schema?.[entity]?.[field]
                              ? {
                                  [pluralize(field)]:
                                    sql`json_group_array(${schema_field})`.mapWith(
                                      {
                                        mapFromDriverValue: (value: any) =>
                                          JSON.parse(value),
                                      },
                                    ),
                                }
                              : null),
                          },
                        };
                      }, {})
                    : {}),
                  // ['contact_phone_numbers.phone_number_raw']: sql<string>`GROUP_CONCAT(${schema['contact_phone_numbers'].phone_number_raw})`,
                };
              },
              {
                ...aliased_joint_entities,
              },
            )
          : {
              [table]: pick(Utility.checkTable(table).table_schema, _pluck),
            };
        _db = _db
          .select(Utility.createSelections({ table, pluck_object, joins }))
          .from(table_schema);
      } else {
        _db = _db.select(selections).from(table_schema);
      }
      _db = Utility.FilterAnalyzer(
        _db,
        table_schema,
        advance_filters,
        pluck_object,
        organization_id,
        joins,
        this.db,
      );

      const getSortSchemaAndField = (
        order_by: string,
        aliased_entities: Record<string, any>,
      ) => {
        const by_entity_field = order_by.split('.');
        let sort_schema = table_schema[by_entity_field[0] || 'id'];
        if (by_entity_field.length > 1) {
          const [_entity = '', by_field = 'id'] = by_entity_field;
          const is_aliased = Object.values(aliased_entities).find(
            ({ alias }) => alias === _entity,
          );
          const entity = !is_aliased ? pluralize(_entity) : _entity;
          if (!join_keys.includes(entity) && !is_aliased)
            throw new BadRequestException({
              success: false,
              message: `Other than main entity, you can only sort by joined entities. ${entity} is not a joined entity nor an aliased joined entity.`,
            });
          sort_schema = is_aliased
            ? sql.raw(`"${entity}"."${by_field}"`)
            : schema[entity][by_field];
        }
        return sort_schema as SQLWrapper | AnyColumn;
      };

      if (multiple_sort.length) {
        _db = _db.orderBy(
          ...multiple_sort.map(({ by_direction, by_field }) => {
            const sort_field_schema = getSortSchemaAndField(
              by_field,
              aliased_joined_entities,
            );
            return ['asc', 'ascending'].includes(by_direction)
              ? asc(sort_field_schema)
              : desc(sort_field_schema);
          }),
        );
      } else if (order_direction && order_by) {
        const sort_field_schema = getSortSchemaAndField(
          order_by,
          aliased_joined_entities,
        );
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
        if (table_schema.hypertable_timestamp) {
          _db = _db.groupBy(table_schema.id, table_schema.timestamp);
        } else {
          _db = _db.groupBy(table_schema.id);
        }
      }
      console.log(_db.toSQL());
      this.logger.debug(`Query: ${JSON.stringify(_db.toSQL())}`);
      let result = await _db;
      console.log(result);
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
}
