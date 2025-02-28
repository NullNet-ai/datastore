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
import * as local_schema from '../../../../schema';
import { asc, desc, sql, SQLWrapper, AnyColumn } from 'drizzle-orm';
import { VerifyActorsImplementations } from '../verify';
import { IParsedConcatenatedFields } from '../../../../types/utility.types';
// const pluralize = require('pluralize');
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
        concatenate_fields = [],
        date_format = 'YYYY-MM-DD',
        // pluck_group_object = {},
      } = body;
      const parsed_concatenated_fields =
        Utility.parseConcatenateFields(concatenate_fields);

      let aliased_joined_entities: Record<string, any>[] = [];
      joins.forEach(({ field_relation }) => {
        const { entity, alias } = field_relation.to;
        if (alias) {
          aliased_joined_entities.push({ alias, entity });
        }
      });

      console.log(aliased_joined_entities);
      Object.keys(pluck_object).forEach((key) => {
        if (!pluck_object[key].includes('id')) {
          throw new BadRequestException({
            success: false,
            message: `pluck_object must have "id" for every entity`,
          });
        }
      });

      multiple_sort.forEach(({ by_field }) => {
        const [entity, field] = by_field.split('.');
        const concat_fields = parsed_concatenated_fields.fields;
        const non_aliased_entity: string =
          aliased_joined_entities.find(({ alias }) => alias === entity)
            ?.entity || entity;
        console.log(non_aliased_entity);
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

        pluck_object[entity] = [
          ...new Set([
            ...pluck_object[entity],
            ...(concat ? concat.fields : [field]),
          ]),
        ];
      });
      let _pluck: string[] =
        pluck.length && !pluck.includes('*') ? pluck : ['id', 'code'];
      const { table_schema, schema } = Utility.checkTable(table);

      let _plucked_fields = Utility.parsePluckedFields(table, _pluck);
      _plucked_fields = Utility.parseMainConcatenations(
        concatenate_fields,
        table,
        _plucked_fields === null ? {} : _plucked_fields,
      );

      const selections = _plucked_fields === null ? undefined : _plucked_fields;

      let _db = this.db;

      // let join_keys: string[] = Object.keys(pluck_object);

      if (joins?.length) {
        _db = _db
          .select(
            Utility.createSelections({
              table,
              pluck_object,
              joins,
              date_format,
              parsed_concatenated_fields,
              multiple_sort,
            }),
          )
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
        parsed_concatenated_fields,
      );

      const getSortSchemaAndField = (
        order_by: string,
        aliased_entities: Record<string, any>,
        transformed_concatenations: IParsedConcatenatedFields['expressions'],
        by_direction: string = 'asc',
      ) => {
        const by_entity_field = order_by.split('.');
        const sort_entity: any = by_entity_field[0];
        let sort_schema = table_schema[by_entity_field[0] || 'id'];
        if (by_entity_field.length > 1) {
          const [_entity = '', by_field = 'id'] = by_entity_field;
          const is_aliased = Object.values(aliased_entities).find(
            ({ alias }) => alias === _entity,
          );
          const entity = _entity;
          // if (!join_keys.includes(entity) && !is_aliased)
          //   throw new BadRequestException({
          //     success: false,
          //     message: `Other than main entity, you can only sort by joined entities. ${entity} is not a joined entity nor an aliased joined entity.`,
          //   });
          if (
            !schema[entity]?.[by_field] &&
            transformed_concatenations[sort_entity] &&
            sort_entity === table
            //if sort_entity is the main table and check if it has any field that is concatenated, and that field doesn't exist in the schema
          ) {
            const concatenation = transformed_concatenations[entity]?.find(
              (exp) => exp.includes(by_field),
            );
            sort_schema = concatenation
              ? sql.raw(concatenation.split(' AS ')[0] as string)
              : undefined;
          } else if (
            !schema[entity]?.[by_field] &&
            transformed_concatenations[sort_entity] &&
            transformed_concatenations[sort_entity].find((exp) =>
              exp.includes(by_field),
            )
            //if entity is not in the schema or its field is not in the schema and it is in the transformed concatenations and the field is in the transformed concatenations
          ) {
            const concat_sort_field: any = transformed_concatenations[
              sort_entity
            ]?.find((exp) => {
              return exp.includes(by_field);
            });
            const sort_query = concat_sort_field.split(' AS ')[0];

            if (by_direction.toLowerCase() === 'asc') {
              return sql.raw(`MIN(${sort_query})`);
            } else {
              return sql.raw(`MAX(${sort_query})`);
            }
          } else if (entity !== table) {
            let sort_query: any = `"${sort_entity}"."${by_field}"`;
            if (by_direction.toLowerCase() === 'asc') {
              return sql.raw(`MIN(${sort_query})`);
            } else {
              return sql.raw(`MAX(${sort_query})`);
            }
          } else {
            sort_schema = is_aliased
              ? sql.raw(`"${entity}"."${by_field}"`)
              : schema[entity][by_field];
          }
        }
        return sort_schema as SQLWrapper | AnyColumn;
      };
      const transformed_concatenations: IParsedConcatenatedFields['expressions'] =
        Utility.removeJoinedKeyword(parsed_concatenated_fields.expressions);
      if (multiple_sort.length) {
        _db = _db.orderBy(
          ...multiple_sort
            .map(({ by_direction, by_field }) => {
              const sort_field_schema = getSortSchemaAndField(
                by_field,
                aliased_joined_entities,
                transformed_concatenations,
                by_direction,
              );
              return ['asc', 'ascending'].includes(by_direction)
                ? asc(sort_field_schema)
                : desc(sort_field_schema);
            })
            .filter(Boolean),
        );
      } else if (order_direction && order_by) {
        const sort_field_schema = getSortSchemaAndField(
          order_by,
          aliased_joined_entities,
          transformed_concatenations,
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
      this.logger.debug(`Query: ${JSON.stringify(_db.toSQL())}`);
      let result = await _db;
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
