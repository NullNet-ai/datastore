import { BadRequestException, NotFoundException } from '@nestjs/common';
import * as schema from '../schema';
import { createInsertSchema, createUpdateSchema } from 'drizzle-zod';
import { ulid } from 'ulid';
import { LoggerService, ZodValidationException } from '@dna-platform/common';
import {
  EOperator,
  IAdvanceFilters,
  IGroupAdvanceFilters,
  IJoins,
} from '../xstate/modules/schemas/find/find.schema';
import {
  aliasedTable,
  and,
  between,
  eq,
  gt,
  gte,
  ilike,
  inArray,
  isNotNull,
  isNull,
  like,
  lt,
  lte,
  ne,
  notBetween,
  notIlike,
  notInArray,
  notLike,
  or,
  sql,
} from 'drizzle-orm';
import { ZodObject } from 'zod';
import { IAggregationQueryParams } from '../xstate/modules/schemas/aggregation_filter/aggregation_filter.schema';
import {
  IConcatenateField,
  IParsedConcatenatedFields,
} from '../types/utility.types';
import { execSync } from 'child_process';
import {
  locale,
  date_options,
  timezone,
} from '@dna-platform/crdt-lww-postgres/build/modules/constants';

const pluralize = require('pluralize');
const { TZ = 'America/Los_Angeles' } = process.env;

export class Utility {
  private static logger = new LoggerService('Utility');
  public static createParse({
    schema,
    data,
  }: {
    schema: { parse: any };
    data: any;
    meta?: any;
  }) {
    const _data = this.format(data);
    try {
      return schema.parse(_data);
    } catch (error: any) {
      throw new ZodValidationException(error);
    }
  }
  public static updateParse({
    schema,
    data,
  }: {
    schema: { parse: any };
    data: any;
    meta?: any;
  }) {
    const _data = this.format(data, false);
    try {
      return schema.parse(_data);
    } catch (error: any) {
      throw new ZodValidationException(error);
    }
  }

  public static convertTime12to24(time12h: string) {
    const [time = '', modifier = ''] = time12h.split(' ');
    let hours = time.split(':')[0] || '0';
    const minutes = time.split(':')[1] || '0';

    if (hours === '12' && ['AM', 'a.m.'].includes(modifier)) {
      hours = '0';
    }

    if (['PM', 'p.m.'].includes(modifier) && hours !== '12') {
      hours = `${parseInt(hours, 10) + 12}`;
    }

    const formatted_hours = hours.toString().padStart(2, '0');
    const formatted_minutes = minutes.toString().padStart(2, '0');

    return `${formatted_hours}:${formatted_minutes}`;
  }
  public static format(data: any, is_insert = true) {
    if (data?.image_url) {
      const { valid, message } = this.validateUrl(data.image_url);
      if (!valid) throw new BadRequestException(message);
    }
    const date = new Date();
    const formattedDate = date
      .toLocaleDateString(locale, date_options)
      .replace(/-/g, '/');
    const formattedTime = Utility.convertTime12to24(
      date.toLocaleTimeString(locale, {
        timeZone: timezone,
      }),
    );

    const _data = {
      id: data?.id ? data.id : ulid(),
      ...(is_insert
        ? {
            tombstone: 0,
            status: 'Active',
            created_date: formattedDate,
            created_time: formattedTime,
            updated_date: formattedDate,
            updated_time: formattedTime,
            timestamp: date.toISOString(),
          }
        : {
            updated_date: formattedDate,
            updated_time: formattedTime,
          }),
      ...data,
    };

    return _data;
  }
  public static checkCreateSchema(
    table: string,
    meta: Record<string, any>,
    data: Record<string, any>,
  ) {
    const { table_schema: schema_table } = Utility.checkTable(table);
    if (!data) {
      throw new BadRequestException('Data is required in Body');
    }

    return { schema: createInsertSchema(schema_table as any), data, meta };
  }

  public static checkTable(table: string) {
    const table_schema = schema[table];
    if (
      !table_schema ||
      !table ||
      table === 'config_sync' ||
      table.includes('crdt')
    ) {
      throw new NotFoundException(`Table ${table} does not exist`);
    }

    return {
      table_schema,
      schema,
    };
  }
  static formatDate = ({
    table,
    field,
    date_format,
    time_zone,
    encrypted_fields,
    fields,
  }: {
    table: string;
    field: string;
    date_format: string;
    time_zone?: string;
    encrypted_fields: Array<string>;
    fields: Array<string>;
  }) => {
    const field_prefix = field.replace(/(_date)|(_time)$/, '');
    const date_field = `${field_prefix}_date`;
    const time_field = `${field_prefix}_time`;

    const date_time_field = `(${Utility.decryptField(
      `"${table}"."${date_field}"`,
      encrypted_fields,
    )}::timestamp${
      fields.includes(time_field)
        ? ` + ${Utility.decryptField(
            `"${table}"."${time_field}"`,
            encrypted_fields,
          )}::interval`
        : ''
    })`;
    const timezone_query = ` AT TIME ZONE '${TZ}' AT TIME ZONE '${time_zone}'`;
    if (field.toLowerCase().endsWith('_date')) {
      return `to_char((${date_time_field}${
        time_zone ? timezone_query : ''
      })::date, '${date_format}')`;
    } else if (field.toLowerCase().endsWith('_time')) {
      return `(${date_time_field}${time_zone ? timezone_query : ''})::time`;
    }
  };

  static formatIfDate = (
    field: string,
    dateFormat: string = 'MM/DD/YYYY',
    toAlias,
    fields,
    time_zone,
  ) => {
    if (
      field.toLowerCase().endsWith('date') ||
      field.toLowerCase().endsWith('time')
    ) {
      return `'${field}', ${Utility.formatDate({
        table: toAlias,
        field,
        date_format: dateFormat,
        time_zone,
        encrypted_fields: [],
        fields,
      })}`;
    }
    return `'${field}', "${toAlias}"."${field}"`;
  };

  public static parseConcatenateFields = (
    concatenate_fields: IConcatenateField[],
  ) => {
    return concatenate_fields.reduce(
      (
        acc,
        { fields, field_name, separator, entity: _entity, aliased_entity },
      ) => {
        const entity = aliased_entity || _entity;
        acc.expressions[entity] = acc.expressions[entity] || [];
        acc.fields[entity] = acc.fields[entity] || [];
        acc.additional_fields[entity] = acc.additional_fields[entity] || [];
        if (!aliased_entity) {
          // Build the concatenated SQL expression
          const concatenatedField = `(${fields
            .map((field) => `COALESCE("joined_${entity}"."${field}", '')`)
            .join(` || '${separator}' || `)}) AS "${field_name}"`;

          // Store the expression
          acc.expressions[entity].push(concatenatedField);

          // Store the field name in the fields object
          if (!acc.additional_fields[entity].includes(field_name)) {
            acc.fields[entity].push(field_name);
            acc.additional_fields[entity].push(field_name);
          }
        }

        return acc;
      },
      { expressions: {}, fields: {}, additional_fields: {} } as {
        expressions: Record<string, string[]>;
        fields: Record<string, string[]>;
        additional_fields: Record<string, string[]>;
      },
    );
  };

  public static createChildSort = ({
    multiple_sort,
    table,
    parsed_concatenated_fields,
  }) => {
    const { expressions } = parsed_concatenated_fields;
    return multiple_sort
      .map(({ by_field, by_direction }) => {
        const by_entity_field = by_field.split('.');
        let sort_table: any = by_entity_field[0];
        let sort_field: any = by_entity_field[1];
        if (sort_table != table) {
          //check if field name is in the concatenated expressions
          const concat_sort_field = expressions[sort_table]?.find((exp) => {
            return exp.includes(sort_field);
          });
          let sort_query: any = `"${sort_table}"."${sort_field}"`;
          if (concat_sort_field) {
            sort_query = concat_sort_field
              .replaceAll('joined_', '')
              .split(' AS ')[0];
          }

          if (by_direction.toLowerCase() === 'asc') {
            return `ORDER BY MIN(${sort_query}) ASC`;
          } else {
            return `ORDER BY MAX("${sort_table}"."${by_entity_field[1]}") DESC`;
          }
        }
      })
      .filter(Boolean);
  };
  public static createSelections = ({
    table,
    pluck_object,
    pluck_group_object,
    joins,
    date_format,
    parsed_concatenated_fields,
    encrypted_fields = [],
    time_zone,
  }: {
    table: string;
    pluck_object: Record<string, any>;
    pluck_group_object: Record<string, any>;
    joins: IJoins[];
    date_format: string;
    parsed_concatenated_fields: IParsedConcatenatedFields;
    multiple_sort: [{ by_field: string; by_direction: string }];
    encrypted_fields: string[];
    time_zone: string;
  }): Record<string, string[]> => {
    const pluck_object_keys = Object.keys(pluck_object || {});
    const { fields: concatenated_fields, expressions } =
      parsed_concatenated_fields;

    pluck_object_keys.forEach((key) => {
      //check if the value of key is string then parse it to array
      if (typeof pluck_object[key] === 'string') {
        pluck_object[key] = JSON.parse(pluck_object[key]);
      }
    });
    const fields = pluck_object?.[table] || [];
    let mainSelections = fields.reduce((acc, field) => {
      if (
        field.toLowerCase().endsWith('_date') ||
        field.toLowerCase().endsWith('_time')
      ) {
        const formatted_date = Utility.formatDate({
          table,
          field,
          date_format,
          time_zone,
          encrypted_fields,
          fields,
        });
        return {
          ...acc,
          [field]: sql.raw(
            `${formatted_date}
             AS "${field}"`,
          ),
        };
      }
      return {
        ...acc,
        [field]: sql.raw(
          Utility.decryptField(`"${table}"."${field}"`, encrypted_fields),
        ),
      };
    }, {});

    const main_concatenate_selections = expressions[table] || [];
    if (main_concatenate_selections.length) {
      main_concatenate_selections.forEach((selection: any) => {
        const [_expression, field_name] = selection.split(' AS ');
        mainSelections[field_name.replace(/["\/]/g, '')] = sql.raw(
          Utility.decryptField(
            selection.replaceAll('joined_', ''),
            encrypted_fields,
          ),
        );
      });
    }

    // Handle join entity selections
    const joinSelections = joins?.length
      ? joins.reduce((acc, join) => {
          const join_type = join.type;
          const toEntity =
            join_type === 'self'
              ? join.field_relation.from.entity
              : join.field_relation.to.entity;
          const toAlias =
            join_type === 'self'
              ? join.field_relation.from.alias || toEntity
              : join.field_relation.to.alias || toEntity; // Use alias if provided

          // Only process if the entity has pluck_object fields
          const entity_concatenated_fields = concatenated_fields[toAlias] || [];
          if (pluck_object_keys.includes(toAlias)) {
            const fields = [
              ...pluck_object[toAlias],
              ...entity_concatenated_fields,
            ];

            // Dynamically create JSON_AGG with JSON_BUILD_OBJECT
            const jsonAggFields = fields
              .map((field) =>
                Utility.formatIfDate(
                  Utility.decryptField(field, encrypted_fields),
                  date_format,
                  toAlias,
                  fields,
                  time_zone,
                ),
              )
              .join(', ');

            const jsonAggSelection = sql
              .raw(
                `
                  COALESCE(
                    JSONB_AGG( DISTINCT
                      JSONB_BUILD_OBJECT(${jsonAggFields})
                    ) FILTER (WHERE "${toAlias}"."id" IS NOT NULL),
                    '[]'
                  )
                `,
              )
              .as(toAlias);

            return {
              ...acc,
              [toAlias]: jsonAggSelection,
            };
          }
          return acc;
        }, {})
      : {};

    const groupSelections = Object.entries(pluck_group_object).reduce(
      (acc, [table, fields]) =>
        fields.reduce((field_acc, field) => {
          const alias = pluralize(field);
          return {
            ...field_acc,
            [alias]: sql
              .raw(
                `JSONB_AGG(${Utility.decryptField(
                  `"${table}"."${field}"`,
                  encrypted_fields,
                )})`,
              )
              .as(alias),
          };
        }, acc),
      {},
    );

    const selections = {
      ...mainSelections,
      ...joinSelections,
      ...groupSelections,
    };

    return selections;
  };
  public static parseMainConcatenations(
    concatenate_fields: IConcatenateField[],
    table_name: string,
    plucked_fields: Record<string, any> = {},
  ) {
    for (const field of concatenate_fields) {
      if (field.entity !== table_name) {
        continue;
      }
      const all_fields = field.fields;
      const schema_fields = schema[table_name];
      //check if all fields are in the schema and are of type text only
      const all_fields_in_schema = all_fields.every((f) => {
        if (!schema_fields[f]) {
          throw new Error(
            `Field "${f}" doesn't exist in the schema of ${table_name}`,
          );
        }
        return (
          schema_fields[f].dataType === 'string' &&
          // !f.toLowerCase().endsWith('date') &&
          !f.toLowerCase().includes('id')
        );
      });
      if (!all_fields_in_schema) {
        throw new BadRequestException(
          `Concatenated fields must be of type string. Verify the fields in ${table_name}`,
        );
      }
      if (field.entity === table_name) {
        const field_names = field.fields.map(
          (f) => `COALESCE("${table_name}"."${f}", '')`,
        );
        const concatenated = field_names.join(` || '${field.separator}' || `);

        plucked_fields[field.field_name] = sql.raw(`(${concatenated})`);
      }
    }

    return Object.keys(plucked_fields).length > 0 ? plucked_fields : null;
  }
  public static removeJoinedKeyword = (
    expressions: IParsedConcatenatedFields['expressions'],
  ) => {
    const transformedExpressions = {};
    for (const [tableName, tableExpressions] of Object.entries(expressions)) {
      transformedExpressions[tableName] = tableExpressions.map((expr) => {
        return expr.replace(/joined_/g, '');
      });
    }
    return transformedExpressions;
  };

  public static parsePluckedFields(
    table: string,
    pluck: string[],
    date_format: string,
    _is_joined?: boolean,
    encrypted_fields = [],
    time_zone?: string,
  ): Record<string, any> | null {
    const table_schema = this.checkTable(table).table_schema;
    if (!pluck?.length || !pluck) {
      return null;
    }
    const _plucked_fields = pluck.reduce((acc, field) => {
      // const _field = is_joined ? `"${table}"."${field}"` : field;
      if (table_schema[field]) {
        const is_date_time_field =
          field.toLowerCase().endsWith('_date') ||
          field.toLowerCase().endsWith('_time');
        const formatted_date = Utility.formatDate({
          table,
          field,
          date_format,
          time_zone,
          encrypted_fields,
          fields: pluck,
        });
        return {
          ...acc,
          [field]: is_date_time_field
            ? sql.raw(
                `${formatted_date}
               AS "${field}"`,
              )
            : table_schema[field],
        };
      }
      return acc;
    }, {});

    if (Object.keys(_plucked_fields).length === 0) {
      return null;
    }
    return _plucked_fields;
  }

  public static getPopulatedQueryFrom(sql_query: {
    sql: string;
    params: any[];
  }) {
    const { sql, params } = sql_query;

    // Extract the part starting from "FROM"
    const from_index = sql.toLowerCase().indexOf('from');
    if (from_index === -1) {
      throw new Error('The query does not contain a FROM clause.');
    }

    let query_from_part = sql.slice(from_index);

    // Replace placeholders ($1, $2, etc.) with corresponding params
    params.forEach((param, index) => {
      // PostgreSQL-style placeholder ($1, $2, ...)
      const placeholder = new RegExp(`\\$${index + 1}`, 'g');

      // Convert param to a safe SQL string (wrap strings with single quotes)
      const value =
        typeof param === 'string' ? `'${param.replace(/'/g, "''")}'` : param;

      query_from_part = query_from_part.replace(placeholder, value);
    });

    return query_from_part;
  }

  public static FilterAnalyzer = (
    db,
    table_schema,
    advance_filters,
    pluck_object,
    organization_id,
    joins: any = [],
    _client_db: any,
    concatenate_fields?: IParsedConcatenatedFields,
    group_advance_filters: IGroupAdvanceFilters[] = [],
    request_type?: string,
    encrypted_fields = [],
    time_zone?: string,
    table?: string,
    date_format?: string,
  ) => {
    let _db = db;
    const aliased_entities: any = [];
    let expressions = concatenate_fields?.expressions || {};
    const concat_fields = concatenate_fields?.fields || {};

    if (joins.length) {
      joins.forEach(({ type, field_relation, nested = false }) => {
        const { from, to } = field_relation;
        const to_entity = to?.entity;
        const from_alias = from?.alias || from?.entity; // Use alias if provided
        const to_alias = to?.alias || to_entity; // Use alias if provided
        to.filters ??= [];
        if (!from?.entity || !to?.entity || !from.field || !to.field) {
          throw new Error(
            'Invalid join configuration. Ensure both `from` and `to` entities are defined.',
          );
        }
        const concatenate_query = expressions[to_alias] || [];
        function constructJoinQuery({ isSelfJoin = false } = {}) {
          if (to?.alias) aliased_entities.push(to?.alias);
          // Retrieve fields from pluck_object for the specified `to` entity
          const fields = pluck_object[to_alias] || [];
          const to_table_schema = schema[to_entity];
          const aliased_to_entity = aliasedTable(
            to_table_schema,
            `joined_${to_alias}`,
          );

          let sub_query = _client_db.select().from(aliased_to_entity);
          if (nested) {
            sub_query = sub_query.leftJoin(
              schema[from_alias],
              eq(aliased_to_entity.id, schema[from_alias].created_by),
            );
          }
          sub_query = sub_query
            .where(
              and(
                eq(aliased_to_entity['tombstone'], 0),
                ...(request_type !== 'root'
                  ? [
                      isNotNull(aliased_to_entity['organization_id']),
                      eq(aliased_to_entity['organization_id'], organization_id),
                    ]
                  : []),
                ...Utility.constructFilters(
                  table,
                  to.filters,
                  aliased_to_entity,
                  [`joined_${to_alias}`],
                  expressions,
                  time_zone,
                  date_format,
                ),
              ),
            )
            .toSQL();

          const sub_query_from_clause =
            Utility.getPopulatedQueryFrom(sub_query);
          const join_order_direction = to.order_direction || 'ASC';
          let order_by = to.order_by || 'created_date';

          //check if order_by exists in the concatenated_fields
          if (concat_fields[to_alias]?.includes(order_by)) {
            {
              const concatenation = expressions?.to_alias?.find((exp) =>
                exp.includes(order_by),
              );
              order_by = concatenation
                ? concatenation.split(' AS ')[0]
                : 'created_date';
            }
          }

          const additional_where_and_clause = !nested
            ? `AND "${from.entity}"."${from.field}" = "joined_${to_alias}"."${to.field}"`
            : ``;

          const lateral_join = sql.raw(`
            LATERAL (
              SELECT ${fields
                .map((field) => `"joined_${to_alias}"."${field}"`)
                .join(', ')}
                ${
                  concatenate_query.length
                    ? `, ${concatenate_query.join(', ').replace(/,\s*$/, '')}`
                    : ''
                }
              ${sub_query_from_clause} ${additional_where_and_clause}
              ${
                to.order_by
                  ? `ORDER BY "joined_${to_alias}"."${order_by}" ${join_order_direction.toUpperCase()}`
                  : ''
              }
              ${to.limit ? `LIMIT ${to.limit}` : ''}
            ) AS "${isSelfJoin ? from_alias : to_alias}"
          `);

          _db = _db.leftJoin(lateral_join, sql`TRUE`);
          return _db;
        }
        switch (type) {
          case 'left':
            _db = constructJoinQuery();
            break;
          case 'self':
            _db = constructJoinQuery({ isSelfJoin: true });
            break;
          default:
            throw new BadRequestException('Invalid join type');
        }
      });
    }
    const transformed_expressions = Utility.removeJoinedKeyword(expressions);

    //remove joined keyword from every entity in expressions
    return _db.where(
      and(
        eq(table_schema['tombstone'], 0),
        ...(request_type !== 'root'
          ? [
              isNotNull(table_schema['organization_id']),
              eq(table_schema['organization_id'], organization_id),
            ]
          : []),
        // TODO: inject permissions by user_organization_role_id
        // ! testing purpose only
        ...Utility.constructFilters(
          table,
          advance_filters,
          table_schema,
          aliased_entities,
          transformed_expressions,
          time_zone,
          date_format,
          group_advance_filters,
          encrypted_fields,
        ),
      ),
    );
  };

  public static AggregationFilterAnalyzer(
    db,
    table_schema,
    _advance_filters: IAdvanceFilters[],
    organization_id: string,
    joins?: IJoins[],
    _client_db: any = null,
    type?: string,
    time_zone?: string,
    table?: string,
    date_format?: string,
  ) {
    let _db = db;
    const aliased_entities: string[] = [];
    if (joins?.length) {
      joins.forEach(({ type, field_relation }) => {
        const { from, to } = field_relation;
        let _from = from;
        let _to = to;
        switch (type) {
          case 'left':
            if (_to.alias) aliased_entities.push(_to.alias);
            const aliased_schema = aliasedTable(
              schema[_to.entity],
              _to.alias || _to.entity,
            );
            _db = _db.leftJoin(
              aliased_schema,
              eq(schema[_from.entity][_from.field], aliased_schema[_to.field]),
            );
            break;
          case 'self':
            if (!_from.alias) {
              throw new BadRequestException(
                '[from]: Alias are required for self join',
              );
            }
            aliased_entities.push(_from.alias);
            const parent = aliasedTable(schema[_from.entity], _from.alias);
            _db = _db.leftJoin(
              parent,
              eq(parent[_from.field], schema[_to.entity][_to.field]),
            );
            break;
          default:
            throw new BadRequestException('Invalid join type');
        }
      });
    }
    return _db.where(
      and(
        eq(table_schema['tombstone'], 0),
        ...(type !== 'root'
          ? [
              isNotNull(table_schema['organization_id']),
              eq(table_schema['organization_id'], organization_id),
            ]
          : []),
        ...Utility.constructFilters(
          table,
          _advance_filters,
          table_schema,
          aliased_entities,
          {},
          time_zone,
          date_format,
        ),
      ),
    );
  }

  public static evaluateFilter({
    operator,
    table_schema,
    field,
    values,
    dz_filter_queue,
    entity,
    aliased_entities,
    expressions,
    case_sensitive,
    parse_as,
    encrypted_fields = [],
    fields = [],
    time_zone,
    date_format,
  }) {
    const is_aliased = aliased_entities.includes(entity);
    let _field = `${field}`;
    // if (!table_schema?.[field] && !is_aliased) return null;
    let schema_field;
    if (encrypted_fields?.length) {
      _field = Utility.decryptField(_field, encrypted_fields);
    }

    if (!table_schema?.[field] && !is_aliased && expressions[entity]) {
      schema_field = sql.raw(
        Utility.decryptField(
          expressions[entity]
            .find((exp) => exp.includes(field))
            .split(' AS ')[0],
          encrypted_fields,
        ),
      );
    } else {
      if (field.endsWith('_date')) {
        schema_field = sql.raw(
          `to_char(${Utility.decryptField(
            `"${entity}"."${field}"`,
            encrypted_fields,
          )}::date, '${date_format}')`,
        );
      } else
        schema_field = is_aliased
          ? sql.raw(
              Utility.decryptField(`"${entity}"."${field}"`, encrypted_fields),
            )
          : table_schema?.[field];
    }

    if (parse_as === 'text') {
      schema_field = entity
        ? sql.raw(
            `${Utility.decryptField(
              `"${entity}"."${field}"`,
              encrypted_fields,
            )}::TEXT`,
          )
        : sql.raw(`"${_field}"::TEXT`);
    }

    if (fields.length) {
      const date_field_index = (fields as Array<any>).findIndex((f) =>
        f.endsWith('_date'),
      );
      const time_field_index = (fields as Array<any>).findIndex((f) =>
        f.endsWith('_time'),
      );
      if (date_field_index === -1 || time_field_index === -1)
        throw new BadRequestException(
          'Date and Time fields are required for Timezone related filters',
        );

      schema_field = sql.raw(`(${Utility.decryptField(
        `"${entity}"."${fields[date_field_index]}"`,
        encrypted_fields,
      )}::timestamp + ${Utility.decryptField(
        `"${entity}"."${fields[time_field_index]}"`,
        encrypted_fields,
      )}::interval) AT TIME ZONE '${TZ}'
      `);
      values = [
        sql.raw(
          `'${values[date_field_index]} ${values[time_field_index]}'::timestamp AT TIME ZONE '${time_zone}'`,
        ),
      ];
    }

    switch (operator) {
      case EOperator.EQUAL:
        return or(...values.map((value) => eq(schema_field, value)));
      case EOperator.NOT_EQUAL:
        return notInArray(schema_field, values);
      case EOperator.GREATER_THAN:
        return gt(schema_field, values[0]);
      case EOperator.GREATER_THAN_OR_EQUAL:
        return gte(schema_field, values[0]);
      case EOperator.LESS_THAN:
        return lt(schema_field, values[0]);
      case EOperator.LESS_THAN_OR_EQUAL:
        return lte(schema_field, values[0]);
      case EOperator.IS_NULL:
        return isNull(schema_field);
      case EOperator.IS_NOT_NULL:
        return isNotNull(schema_field);
      case EOperator.CONTAINS:
        return inArray(schema_field, [values]);
      case EOperator.NOT_CONTAINS:
        return notInArray(schema_field, [values]);
      case EOperator.IS_BETWEEN:
        return between(schema_field, values[0], values[1]);
      case EOperator.IS_NOT_BETWEEN:
        return notBetween(schema_field, values[0], values[1]);
      case EOperator.IS_EMPTY:
        return eq(schema_field, '');
      case EOperator.IS_NOT_EMPTY:
        return ne(schema_field, '');
      case EOperator.AND:
        return and(...dz_filter_queue);
      case EOperator.OR:
        return or(...dz_filter_queue);
      case EOperator.LIKE:
        if (case_sensitive) {
          return like(schema_field, `%${values[0]}%`);
        }
        return ilike(schema_field, `%${values[0]}%`);
      case EOperator.NOT_LIKE:
        if (case_sensitive) {
          return notLike(schema_field, `%${values[0]}%`);
        }
        return notIlike(schema_field, `%${values[0]}%`);
      default:
        return null;
    }
  }

  public static constructFilters(
    table,
    advance_filters,
    table_schema,
    aliased_entities: string[] = [],
    expressions: any,
    time_zone,
    date_format,
    group_advance_filters: IGroupAdvanceFilters[] = [],
    encrypted_fields = [],
  ): any[] {
    let dz_filter_queue: any[] = [];
    let where_clause_queue: any[] = [];
    let _filter_queue: any[] = [];

    if (group_advance_filters?.length) {
      const group_where_clause_queue: any[] = [];
      const group_criteria_queue: any[] = [];
      const group_operator_queue: any[] = [];
      group_advance_filters.forEach(({ filters, type, operator }) => {
        if (!filters?.length && type === 'criteria') {
          throw new BadRequestException('Grouped filters must not empty');
        }

        if ((filters as any[])?.[filters?.length - 1]?.type === 'operator') {
          throw new BadRequestException(
            'Grouped filters must end with a criteria',
          );
        }

        if (type === 'criteria') {
          group_criteria_queue.push(
            this.constructFilters(
              table,
              filters,
              table_schema,
              aliased_entities,
              expressions,
              time_zone,
              date_format,
            ),
          );
        } else if (type === 'operator') {
          group_operator_queue.push(operator);
        }
      });
      const [group_type] = group_operator_queue;
      if (group_type === EOperator.AND) {
        group_where_clause_queue.push(and(...group_criteria_queue));
      } else if (group_type === EOperator.OR) {
        group_where_clause_queue.push(or(...group_criteria_queue));
      }

      return group_where_clause_queue;
    }

    if (
      advance_filters?.find(({ entity }) => entity) &&
      advance_filters?.filter(
        ({ type = 'criteria', entity }) => type === 'criteria' && !entity,
      ).length
    ) {
      throw new BadRequestException(
        'Invalid filter. "entity" must be defined for all filters',
      );
    }
    advance_filters?.forEach((filter) => {
      if (filter.field && filter.field.toLowerCase().includes('timestamp')) {
        if (typeof filter.values === 'string')
          filter.values = JSON.parse(filter.values);
        filter.values = filter?.values?.map((val) => new Date(val));
      }

      if (filter.fields?.length && filter.field) {
        throw new BadRequestException(
          `Invalid filter. "fields" must not be defined with "field"`,
        );
      }
      if (
        filter.fields?.length &&
        filter.fields?.length !== filter.values?.length
      ) {
        throw new BadRequestException(
          `Invalid filter. "fields" and "values" must have the same length and corresponds to each other`,
        );
      }
      if (
        filter.fields?.some(
          (field) => !(field?.endsWith('_date') || field?.endsWith('_time')),
        )
      ) {
        throw new BadRequestException(
          `Invalid filter. "fields" must be of type date or time for time zone related filters`,
        );
      }
    });
    if (advance_filters?.length === 1) {
      let [
        {
          operator,
          field,
          values,
          type = 'criteria',
          case_sensitive = false,
          parse_as,
          fields = [],
        },
      ] = advance_filters;
      if (typeof values === 'string') {
        values = JSON.parse(values);
      }
      let { entity } = advance_filters[0];
      if (type === 'operator') {
        throw new BadRequestException(
          `Invalid filter at index 0. Must be a criteria`,
        );
      }

      entity =
        entity && !aliased_entities.includes(entity) && !expressions[entity]
          ? pluralize.plural(entity)
          : entity;

      const _table_schema = entity ? schema?.[entity] : table_schema;
      return [
        Utility.evaluateFilter({
          operator,
          table_schema: _table_schema,
          field,
          values,
          dz_filter_queue: [],
          entity: entity || table,
          aliased_entities,
          expressions,
          case_sensitive,
          parse_as,
          encrypted_fields,
          fields,
          time_zone,
          date_format,
        }),
      ];
    }

    advance_filters.forEach((filter, index: number) => {
      const {
        operator,
        type = 'criteria',
        field = '',
        values,
        case_sensitive = false,
        parse_as,
        fields = [],
      } = filter;
      if (typeof values === 'string') {
        filter.values = JSON.parse(values);
      }
      let { entity } = filter;
      entity =
        entity && !aliased_entities.includes(entity)
          ? pluralize.plural(entity)
          : entity;

      const _table_schema = entity ? schema?.[entity] : table_schema;

      if (
        (index % 2 === 0 && type != 'criteria') ||
        (index % 2 === 1 && type != 'operator')
      ) {
        let _type = index % 2 === 0 ? 'a criteria' : 'an operator';
        throw new BadRequestException(
          `Invalid filter at index ${index}. Must be ${_type}`,
        );
      }

      _filter_queue.push(filter);
      dz_filter_queue.push(
        Utility.evaluateFilter({
          operator,
          table_schema: _table_schema,
          field,
          values: filter.values,
          dz_filter_queue,
          entity: entity || table,
          aliased_entities,
          expressions,
          case_sensitive,
          parse_as,
          fields,
          time_zone,
          date_format,
        }),
      );

      if (dz_filter_queue.length > 2) {
        const [_1, _op, _2]: any = _filter_queue;
        const [_c1, _, _c2]: any = dz_filter_queue;
        const allowed_to_merged = _1.operator ? [_c1, _c2] : [_c2];
        where_clause_queue.push(
          Utility.evaluateFilter({
            operator: _op.operator,
            table_schema: _table_schema,
            field,
            values: filter.values,
            dz_filter_queue: where_clause_queue.concat(allowed_to_merged),
            entity: entity || table,
            aliased_entities,
            expressions,
            case_sensitive,
            parse_as,
            fields,
            time_zone,
            date_format,
          }),
        );
        if (where_clause_queue.length > 1) where_clause_queue.shift();
        _filter_queue = [
          // dummy
          {
            type: 'criteria',
            field: '',
            operator: '',
          },
        ];
        dz_filter_queue = [
          // dummy
          {
            type: 'criteria',
            field: '',
            operator: '',
          },
        ];
      }
    });
    return where_clause_queue;
  }

  public static checkUpdateSchema(
    table: string,
    meta: Record<string, any>,
    data: Record<string, any>,
  ) {
    const { table_schema: schema_table } = Utility.checkTable(table);
    if (!data) {
      throw new BadRequestException('Data is required in Body');
    }

    return { schema: createUpdateSchema(schema_table), data, meta };
  }
  public static advanceFilter(advance_filters, organization_id) {
    const supported_operators = {
      equal: '=',
      not_equal: '!=',
      greater_than: '>',
      greater_than_or_equal: '>=',
      less_than: '<',
      less_than_or_equal: '<=',
      is_null: 'IS NULL',
      is_not_null: 'IS NOT NULL',
      is_empty: "=''",
      is_not_empty: "!=''",
      contains: 'IN',
      not_contains: 'NOT IN',
      is_between: 'BETWEEN',
      is_not_between: 'NOT BETWEEN',
    };

    let where_clauses: any = [];

    where_clauses.push(`tombstone = 0`);
    where_clauses.push(`organization_id = '${organization_id}'`);

    if (
      advance_filters &&
      Array.isArray(advance_filters) &&
      advance_filters.length > 0
    ) {
      let advance_filter_clauses: any = [];
      let last_type: any = null;

      for (let i = 0; i < advance_filters.length; i++) {
        if (
          (i % 2 === 0 && advance_filters[i].type != 'criteria') ||
          (i % 2 === 1 && advance_filters[i].type != 'operator')
        ) {
          let _type = i % 2 === 0 ? 'a criteria' : 'an operator';
          throw new BadRequestException(
            `Invalid filter at index ${i}. Must be ${_type}`,
          );
        }

        const filter = advance_filters[i];

        if (filter.type === 'criteria') {
          const { field, operator, values } = filter;

          if (!field || !operator || !supported_operators[operator]) {
            throw new Error(`Unsupported or missing operator: ${operator}`);
          }

          if (
            ['is_null', 'is_not_null', 'is_empty', 'is_not_empty'].includes(
              operator,
            )
          ) {
            advance_filter_clauses.push(
              `${field} ${supported_operators[operator]}`,
            );
          } else if (['contains', 'not_contains'].includes(operator)) {
            if (!Array.isArray(values) || values.length === 0) {
              throw new Error(
                `Values must be a non-empty array for ${operator}`,
              );
            }
            advance_filter_clauses.push(
              `${field} ${supported_operators[operator]} (${values
                .map((v) => `'${v}'`)
                .join(', ')})`,
            );
          } else if (['is_between', 'is_not_between'].includes(operator)) {
            if (!Array.isArray(values) || values.length !== 2) {
              throw new Error(
                `Values must be an array with exactly two elements for ${operator}`,
              );
            }
            advance_filter_clauses.push(
              `${field} ${supported_operators[operator]} '${values[0]}' AND '${values[1]}'`,
            );
          } else {
            if (!values || values.length !== 1) {
              throw new Error(
                `Values must be an array with a single element for ${operator}`,
              );
            }
            advance_filter_clauses.push(
              `${field} ${supported_operators[operator]} '${values[0]}'`,
            );
          }

          last_type = 'criteria';
        } else if (filter.type === 'operator' && last_type === 'criteria') {
          const { operator } = filter;

          if (operator && (operator === 'and' || operator === 'or')) {
            advance_filter_clauses.push(operator.toUpperCase());
          } else {
            throw new Error(`Unsupported logical operator: ${operator}`);
          }

          last_type = 'operator';
        }
      }

      if (last_type === 'operator') {
        advance_filter_clauses.pop();
      }

      if (advance_filter_clauses.length > 0) {
        where_clauses.push(`(${advance_filter_clauses.join(' ')})`);
      }
    }

    return `WHERE ${where_clauses.join(' AND ')}`;
  }
  public static AggregationQueryGenerator(
    params: IAggregationQueryParams,
    from_clause: string,
  ) {
    let {
      entity, // Name of the table
      aggregations, // Array of aggregation objects
      bucket_size, // Time bucket size (e.g., 1 hour, 1 day)
      limit, // Limit the number of results
      order: { order_direction, order_by }, // Column to order by
      timezone = 'UTC', // Timezone to use for time bucketing
    } = params;

    // Validate required parameters
    if (!entity || !bucket_size || !aggregations || aggregations.length === 0) {
      throw new Error(
        'Missing required parameters: entity, bucket_size, or aggregations.',
      );
    }

    // Generate the SELECT clause
    const select_clauses = aggregations.map(
      ({ aggregation, aggregate_on, bucket_name }) => {
        if (!aggregation || !aggregate_on || !bucket_name) {
          throw new Error(
            'Missing aggregation details: aggregation, aggregate_on, or bucket_name.',
          );
        }
        return `${aggregation}(${aggregate_on}) AS ${bucket_name}`;
      },
    );

    const select_clause = `
        SELECT time_bucket('${bucket_size}', ${entity}.timestamp AT TIME ZONE '${timezone}') AS bucket,
               ${select_clauses.join(',\n               ')}
    `;

    //generate the limit clause
    const limit_clause = limit ? `LIMIT ${limit}` : '';

    // Generate the WHERE clause
    const group_by_clause = `GROUP BY bucket`;

    // Generate the ORDER BY clause
    order_direction = order_direction ? order_direction.toUpperCase() : 'asc';
    const order_by_clause = order_by
      ? `ORDER BY ${order_by} ${order_direction}`
      : '';

    // Combine all clauses into the final query
    const query = `
        ${select_clause}
        ${from_clause}
        ${group_by_clause}
        ${order_by_clause}
        ${limit_clause};
        
    `;
    return query.trim();
  }
  static validateZodSchema(
    zodObject: { zod: ZodObject<any> | any; params: any }[],
  ) {
    for (const { zod, params } of zodObject) {
      try {
        zod.parse(params);
      } catch (error: any) {
        throw new Error(`${JSON.stringify(error)}`);
      }
    }
  }
  static processResponseObject(response: any) {
    response.encoding = 'application/json';
    response.data = Utility.stringifyObjects(response.data);
    return response;
  }
  static stringifyObjects(data: []): string {
    if (!data || data?.length === 0) {
      return '[]'; // Return the array as is if empty or not an array
    }
    return JSON.stringify(data);
    // return data.map((item) => JSON.stringify(item));
  }
  static createRequestObject(
    data: any,
    metadata: any,
  ): { headers: any; params: any; query: any; body: any } {
    const _req = {
      headers: {
        authorization: metadata.get('authorization')[0],
      },
      params: data.params || {}, // Ensure params is an object
      query: data.query || {}, // Ensure query is an object
      body: data.body, // Parsed body or null if not present
    };

    return _req;
  }
  static parseRequestBody(body: string) {
    let parsed_body: any;
    try {
      parsed_body = body ? JSON.parse(body) : null;
    } catch (error: any) {
      throw new Error('Invalid JSON body');
    }
    return parsed_body;
  }
  static parseFiltersRequestBody(body: any) {
    let parsed_body: any;
    try {
      if (body && body.advance_filters) {
        parsed_body = {
          ...body,
          advance_filters: this.parseAdvanceFilters(body.advance_filters),
        };
      } else {
        parsed_body = body; // If no advance_filters, set parsed_body to null
      }
    } catch (error: any) {
      throw new Error('Invalid JSON body');
    }
    return parsed_body;
  }
  private static parseAdvanceFilters(advance_filters: any[]) {
    return advance_filters.map((filter: any) => {
      if (filter.values) {
        return {
          ...filter,
          values: filter.values.length ? JSON.parse(filter.values) : [],
        };
      } else {
        return filter;
      }
    });
  }
  static parseBatchRequestBody(body: {
    records: string;
    entity_prefix: string;
  }) {
    let parsed_body: { records: Record<any, any>[]; entity_prefix: string } = {
      records: [],
      entity_prefix: '',
    };
    try {
      parsed_body.records = body?.records ? JSON.parse(body.records) : [];
      parsed_body.entity_prefix = body.entity_prefix;
    } catch (error: any) {
      throw new Error(
        'Invalid JSON body, make sure records and entity_prefix are provided',
      );
    }
    return parsed_body;
  }

  public static execCommand(command: string) {
    try {
      execSync(command);
      return true;
    } catch (error: any) {
      Utility.logger.error(error.stderr.toString() ?? error?.message ?? error);
    }
  }

  private static validateUrl(input_url) {
    try {
      new URL(input_url);
      return { valid: true, message: 'Valid URL' };
    } catch (error: any) {
      return { valid: false, message: error?.message || error };
    }
  }

  public static async encryptCreate({
    encrypted_fields,
    table,
    data,
    db,
    query,
  }) {
    if (encrypted_fields?.length) {
      this.logger.log(`Encrypting data... ${encrypted_fields.join(',')}`);
      const set_val = `${Object.entries(data)
        .reduce((acc: string[], [key, value]) => {
          const _value = `${typeof value === 'string' ? `'${value}'` : value}`;
          if (encrypted_fields.includes(key)) {
            acc.push(
              `${key} = pgp_sym_encrypt(${_value}, '${process.env.PGP_SYM_KEY}')`,
            );
          } else acc.push(`${key} = ${_value}`); // Push the unencrypted value for other fields
          return acc;
        }, [])
        .join(',')}`;
      const _values = `${Object.keys(data)}) VALUES (${Utility.encryptData(
        data,
        encrypted_fields,
      )}`;
      const query = `PREPARE encrypted_insert_raw AS INSERT INTO ${table} (${_values}) ON CONFLICT (id) DO UPDATE SET ${set_val};`;
      this.logger.debug(`Encrypting data: ${query}`);
      return db
        .execute(sql.raw(query))
        .then(() => this.logger.debug('Encrypting data completed'));
    }
    const { table_schema } = query;
    return db
      .insert(query.table_schema)
      .values(data)
      .onConflictDoUpdate({
        target: query.table_schema.id,
        set: data,
      })
      .returning({ table_schema })
      .prepare('encrypted_insert')
      .execute()
      .then(([{ table_schema }]) => table_schema);
  }

  public static async encryptUpdate({
    query,
    encrypted_fields,
    table,
    data,
    db,
    where,
    returning,
  }) {
    if (encrypted_fields?.length) {
      this.logger.log(`Encrypting data... ${encrypted_fields.join(',')}`);
      const set_val = `${Object.entries(data)
        .reduce((acc: string[], [key, value]) => {
          const _value = `${typeof value === 'string' ? `'${value}'` : value}`;
          if (encrypted_fields.includes(key)) {
            acc.push(
              `${key} = pgp_sym_encrypt(${_value}, '${process.env.PGP_SYM_KEY}')`,
            );
          } else acc.push(`${key} = ${_value}`); // Push the unencrypted value for other fields
          return acc;
        }, [])
        .join(',')}`;
      const query = `PREPARE encrypted_update_raw AS UPDATE ${table} SET ${set_val} WHERE ${where.join(
        '',
      )}`;
      this.logger.debug(`Encrypting data: ${query}`);
      return db
        .execute(sql.raw(query))
        .then(() => this.logger.debug('Encrypting data completed'));
    }
    const { table_schema } = query;
    return db
      .update(table_schema)
      .set({
        ...data,
        version: sql`${table_schema.version} + 1`,
      })
      .where(sql.raw(`${where.join(' ')}`))
      .returning(returning)
      .prepare('encrypted_update')
      .execute()
      .then(([{ table_schema }]) => table_schema);
  }

  public static encryptData(data: Record<string, any>, encryption_keys) {
    const values = `${Object.entries(data)
      .reduce((encryptedData: any[], [key, value]) => {
        if ((encryption_keys as string[]).includes(key)) {
          encryptedData.push(
            `pgp_sym_encrypt('${value}', '${process.env.PGP_SYM_KEY}')`,
          );
          return encryptedData;
        }
        const _value = typeof value === 'string' ? `'${value}'` : value;
        encryptedData.push(_value !== undefined ? _value : null);
        return encryptedData;
      }, [])
      .join(',')}`;
    return values;
  }
  public static decryptField(field: string, encrypted_fields: string[]) {
    if (encrypted_fields.includes(field))
      return `pgp_sym_decrypt(${field}::BYTEA, '${process.env.PGP_SYM_KEY}')`;

    return field;
  }
  public static decryptData(
    data: Record<string, any>,
    encrypted_fields: string[],
  ) {
    return Object.entries(data).reduce((_data, [key, value]) => {
      if (encrypted_fields.includes(key)) {
        return {
          ..._data,
          [key]: sql.raw(
            `pgp_sym_decrypt(${key}::BYTEA, '${process.env.PGP_SYM_KEY}')`,
          ),
        };
      }
      return {
        ..._data,
        [key]: value,
      };
    }, {});
  }

  public static constructPermissionSelectWhereClause({ tables, main_fields }) {
    // ! for testing purpose
    return `AND (
          data_permissions.tombstone = 0 AND entities.name IN (${tables
            .map((table) => `'${table}'`)
            .join(',')}) AND fields.name IN (${main_fields
      .map((field) => `'${field}'`)
      .join(',')})
        )`;
  }

  public static getReadPermittedFields(config) {
    Utility.checkPermissions(config, 'read');
    return {
      ...config,
      errors: config.errors,
    };
  }

  public static isPermitted(account_id, role) {
    this.logger.warn(
      `Checking permissions for account_id: ${account_id}, role: ${role}`,
    );
    if (role !== 'Super Admin') {
      throw new BadRequestException({
        success: false,
        message: `Access denied: Although your role is ${role} (${account_id}), you do not have the necessary permissions to access this resource.`,
        count: 0,
        data: [],
      });
    }

    this.logger.warn(
      `As a Role ${role} (${account_id}) has the necessary permissions to access this resource.`,
    );
  }

  public static getTimeMs(timeStr = '1d') {
    const value = parseInt(timeStr);
    const unit = timeStr.slice(-1);
    const multiplier = {
      d: 24 * 60 * 60 * 1000,
      h: 60 * 60 * 1000,
      m: 60 * 1000,
      s: 1000,
    }[unit];

    const ms = value * (multiplier || 0);
    return ms;
  }

  public static checkPermissions(
    { table, schema, permissions, errors, body },
    permission_type: 'read' | 'write' | 'encrypted' | 'decrypted' | 'required',
  ) {
    switch (permission_type) {
      case 'read':
        schema.forEach(
          ({ entity: _entity, field: _field, alias, path, property_name }) => {
            const hasPermission = !!permissions.data.find(
              (p) => p.read && p.entity === _entity && p.field === _field,
            );

            if (!hasPermission) {
              const stack = `[${table}]: Found at ${property_name}${
                alias ? `(${alias})` : ''
              }${path}`;
              if (!errors.find((e) => e.stack === stack)) {
                errors.push({
                  message: `${_field} is not permitted to access.`,
                  stack,
                  status_code: 401,
                });
              }
              body[property_name].pop(_field);
            }
          },
        );
        break;
      default:
        break;
    }
  }
}
