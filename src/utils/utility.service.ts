import { BadRequestException, NotFoundException } from '@nestjs/common';
import * as schema from '../schema';
import { createInsertSchema, createUpdateSchema } from 'drizzle-zod';
import { ulid } from 'ulid';
import { LoggerService, ZodValidationException,  } from '@dna-platform/common';
import {
  EOperator,
  EOrderDirection,
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
  isNotNull,
  isNull,
  like,
  lt,
  lte,
  ne,
  notBetween,
  notIlike,
  notLike,
  or,
  sql,
  SQLWrapper,
  AnyColumn,
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
import * as cache from 'memory-cache';
import sha1 from 'sha1';
import { EDateFormats } from './utility.types';
interface IFilterAnalyzer {
  db: any;
  table_schema: any;
  advance_filters?: IAdvanceFilters[];
  pluck_object?: Record<string, any[]>;
  organization_id: string;
  joins?: any[];
  client_db?: any;
  concatenate_fields?: IParsedConcatenatedFields;
  group_advance_filters?: IGroupAdvanceFilters[];
  request_type?: string;
  encrypted_fields?: string[];
  time_zone?: string;
  table?: string;
  date_format?: EDateFormats;
  pass_field_key?: string;
  parsed_concatenated_fields?: any;
  type?: string;
  permissions?: Record<string, any>;
  concatenated_field_expressions?: Record<string, any>;
}
interface IContructFilters {
  table: any;
  advance_filters?: IAdvanceFilters[];
  table_schema?: any;
  aliased_to_entity?: string;
  aliased_entities?: string[];
  expressions?: any;
  time_zone?: string;
  date_format?: EDateFormats;
  group_advance_filters?: IGroupAdvanceFilters[];
  encrypted_fields?: any[];
  pass_field_key?: string;
  fields?: any[];
  permissions?: Record<string, any>;
  concatenated_field_expressions?: Record<string, any>;
}

interface IEvaluateFilter {
  operator: string;
  table_schema: any;
  field: string;
  values: any[];
  dz_filter_queue: any;
  entity: string;
  aliased_entities: string[];
  expressions?: any;
  case_sensitive: boolean;
  parse_as: string;
  encrypted_fields?: string[];
  fields?: string[];
  time_zone: string;
  date_format: EDateFormats;
  pass_field_key?: string;
  permissions?: Record<string, any>;
  concatenated_field_expressions?: Record<string, any>;
}

interface IAggregationFilterAnalyzer {
  db: any;
  table_schema: any;
  advance_filters: IAdvanceFilters[];
  organization_id: string;
  joins?: IJoins[];
  type?: string;
  time_zone?: string;
  table?: string;
  date_format?: EDateFormats;
  client_db?: any;
}
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

  public static validateFields(table, schema, data) {
    Object.keys(data).forEach((key) => {
      if (!schema[table]?.[key]) {
        throw new BadRequestException(
          `Field ${key} does not exist in ${table}`,
        );
      }
    });
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
    Utility.validateFields(table, schema, data);
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
    date_format = EDateFormats['mm/dd/YYYY'],
    time_zone,
    encrypted_fields,
    fields,
    pass_field_key,
  }: {
    table: string;
    field: string;
    date_format: EDateFormats;
    time_zone?: string;
    encrypted_fields: Array<string>;
    fields: Array<string>;
    pass_field_key?: any;
  }) => {
    const field_prefix = field.replace(/(_date)|(_time)$/, '');
    const date_field = `${field_prefix}_date`;
    const time_field = `${field_prefix}_time`;

    const date_time_field = `(${Utility.decryptField({
      field: `"${table}"."${date_field}"`,
      encrypted_fields,
      table,
      pass_field_key,
    })}::timestamp${
      fields.includes(time_field)
        ? ` + ${Utility.decryptField({
            field: `"${table}"."${time_field}"`,
            encrypted_fields,
            table,
            pass_field_key,
          })}::interval`
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
    date_format: EDateFormats = EDateFormats['mm/dd/YYYY'],
    to_entity,
    fields,
    time_zone,
    pass_field_key,
    is_expression = false,
    field_alias = '',
  ) => {
    if (is_expression) {
      const pattern = /\((.*?)\)\s+AS\s+".*?"/;
      const match = field.match(pattern);
      if (match) {
        const expression = match[1];
        return `'${field_alias}', ${expression}`;
      }
      return `'${field_alias}', ${field}`;
    }
    if (
      field.toLowerCase().endsWith('date') ||
      field.toLowerCase().endsWith('time')
    ) {
      return `'${field}', ${Utility.formatDate({
        table: to_entity,
        field,
        date_format,
        time_zone,
        encrypted_fields: [],
        fields,
        pass_field_key,
      })}`;
    }
    return `'${field}', "${to_entity}"."${field}"`;
  };

  public static parseConcatenateFields = (
    concatenate_fields: IConcatenateField[],
    date_format?: EDateFormats,
    table?: string,
  ) => {
    return concatenate_fields.reduce(
      (
        acc,
        { fields, field_name, separator, entity: _entity, aliased_entity },
      ) => {
        const entity = aliased_entity || pluralize(_entity);
        acc.expressions[entity] = acc.expressions[entity] || [];
        acc.fields[entity] = acc.fields[entity] || [];
        acc.additional_fields[entity] = acc.additional_fields[entity] || [];
        if (!aliased_entity) {
          // Build the concatenated SQL expression
          const concatenatedField = `(${fields
            .map((field) => {
              if (field.endsWith('_date'))
                return `COALESCE(to_char("joined_${entity}"."${field}"::date, '${date_format}'), '')`;
              return `COALESCE("joined_${entity}"."${field}", '')`;
            })
            .join(` || '${separator}' || `)}) AS "${field_name}"`;

          // Store the expression
          acc.expressions[entity].push(concatenatedField);

          // Store the field name in the fields object
          if (!acc.additional_fields[entity].includes(field_name)) {
            acc.fields[entity].push(field_name);
            if (pluralize(_entity) === table)
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
    encrypted_fields = [],
    time_zone,
    pass_field_key,
    request_type,
    concatenated_field_expressions = {},
    organization_id,
  }: {
    table: string;
    pluck_object: Record<string, any>;
    pluck_group_object: Record<string, any>;
    joins: IJoins[];
    date_format: EDateFormats;
    parsed_concatenated_fields: IParsedConcatenatedFields;
    multiple_sort: [{ by_field: string; by_direction: string }];
    encrypted_fields: string[];
    time_zone: string;
    pass_field_key: string;
    request_type?: string;
    aliased_joined_entities?: Record<string, any>[];
    concatenated_field_expressions?: Record<string, any>;
    organization_id: string;
  }): Record<string, string[]> => {
    const pluck_object_keys = Object.keys(pluck_object || {});

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
          pass_field_key,
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
          Utility.decryptField({
            field: `"${table}"."${field}"`,
            encrypted_fields,
            table,
            pass_field_key,
          }),
        ),
      };
    }, {});

    // Handle concatenated fields for main selections
    const main_concatenated_entity =
      concatenated_field_expressions?.[table] || {};
    if (Object.keys(main_concatenated_entity)?.length) {
      Object.entries(main_concatenated_entity)?.forEach(
        ([field_name, concatenated]: any) => {
          mainSelections[field_name] = sql.raw(
            `${Utility.decryptField({
              field: concatenated.expression,
              encrypted_fields,
              table,
              pass_field_key,
            })} AS ${field_name}`,
          );
        },
      );
    }

    // Handle join entity selections
    const joinSelections = joins?.length
      ? joins.reduce((acc, join, index) => {
          const join_type = join.type;
          const toEntity =
            join_type === 'self'
              ? join.field_relation?.from?.entity
              : join.field_relation?.to?.entity;

          const toAlias =
            join_type === 'self'
              ? join.field_relation?.from?.alias || toEntity
              : join.field_relation?.to?.alias || toEntity; // Use alias if provided

          const { from, to } = join.field_relation;
          const { nested = false } = join;
          (join.field_relation.to as any).filters ??= [];
          const join_order_by = to.order_by;
          const join_order_direction =
            to.order_direction || EOrderDirection.ASC;
          const join_is_case_sensitive_sorting =
            to.is_case_sensitive_sorting || false;

          const previous_join = nested ? joins[index - 1] : null;
          const { from: prev_join_from, to: prev_join_to } =
            previous_join?.field_relation ?? {};
          const prev_join_to_entity =
            previous_join?.type === 'self'
              ? prev_join_from?.alias || prev_join_to?.entity
              : prev_join_to?.alias || prev_join_to?.entity;

          // Only process if the entity has pluck_object fields
          if (pluck_object_keys.includes(toAlias)) {
            const fields = pluck_object[toAlias];

            // Dynamically create JSON_AGG with JSON_BUILD_OBJECT
            const jsonAggFields = fields.map((field) =>
              Utility.formatIfDate(
                Utility.decryptField({
                  field,
                  encrypted_fields,
                  table,
                  pass_field_key,
                }),
                date_format,
                toAlias,
                fields,
                time_zone,
                pass_field_key,
              ),
            );
            // Handle concatenated fields selections
            const concatenate_fields_selections = Object.entries(
              concatenated_field_expressions?.[toAlias] ?? {},
            )?.map(([field_name, concatenated]) =>
              Utility.formatIfDate(
                (concatenated as { expression: string; fields: string[] })
                  .expression as string,
                date_format,
                toAlias,
                fields,
                time_zone,
                pass_field_key,
                !!(concatenated as { expression: string; fields: string[] })
                  .expression,
                field_name,
              ),
            );

            const default_filter_clause = `"${toAlias}"."tombstone" = 0 ${
              request_type !== 'root'
                ? `AND "${toAlias}"."organization_id" IS NOT NULL AND "${toAlias}"."organization_id" = '${organization_id}'`
                : ''
            }`;

            let additional_where_and_clause = ` AND "${from.entity}"."${from.field}" = "${toAlias}"."${to.field}"`;
            // Handle additional where clause for nested joins
            if (nested)
              additional_where_and_clause = ` AND "${prev_join_to_entity}"."tombstone" = 0 ${
                request_type !== 'root'
                  ? `AND "${prev_join_to_entity}"."organization_id" IS NOT NULL AND "${prev_join_to_entity}"."organization_id" = '${organization_id}'`
                  : ''
              } AND "${prev_join_from?.entity}"."${
                prev_join_from?.field
              }" = "${prev_join_to_entity}"."${prev_join_to?.field}"`;

            // Handle sorting for data of joined entities
            const joined_sort = [
              ...(join_order_by && join_order_direction
                ? [
                    {
                      by_field: `${toAlias}.${join_order_by}`,
                      by_direction: join_order_direction,
                      is_case_sensitive_sorting: join_is_case_sensitive_sorting,
                    },
                  ]
                : []),
            ]
              .map(
                ({
                  by_direction,
                  by_field,
                  is_case_sensitive_sorting = false,
                }) => {
                  const by_entity_field = by_field.split('.');
                  let sort_entity: any = toEntity;
                  if (by_entity_field.length > 1)
                    sort_entity = by_entity_field[0];
                  if (sort_entity !== toAlias) return null;

                  let sorted_field = `elem->>'${by_field.replace(
                    `${sort_entity}.`,
                    '',
                  )}'`;
                  if (is_case_sensitive_sorting)
                    sorted_field = `LOWER(${sorted_field})`;
                  return `${sorted_field} ${
                    ['asc', 'ascending'].includes(by_direction) ? 'asc' : 'desc'
                  }`;
                },
              )
              .filter(Boolean)
              .join(', ');

            const from_entity = `${nested ? prev_join_to?.entity : toEntity}`;
            const from_alias = `${nested ? prev_join_to_entity : toAlias}`;
            const jsonAggSelection = sql
              .raw(
                `COALESCE(
              (
                SELECT JSONB_AGG(elem${
                  joined_sort.length ? ` ORDER BY ${joined_sort}` : ''
                })
                FROM (
                  SELECT
                    JSONB_BUILD_OBJECT(${[
                      ...jsonAggFields,
                      ...concatenate_fields_selections,
                    ].join(', ')}) AS elem
                  FROM "${from_entity}"${
                  from_entity !== from_alias ? ` "${from_alias}"` : ''
                }
                  ${
                    nested
                      ? `LEFT JOIN "${toEntity}" "${toAlias}" ON "${toAlias}"."id" = "${prev_join_to_entity}"."${from.field}"`
                      : ''
                  }
                  WHERE (${default_filter_clause}${additional_where_and_clause})
                ) sub
            ), '[]')`,
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

    // Handle pluck group object selections
    const groupSelections = Object.entries(pluck_group_object).reduce(
      (acc, [table, fields]) => {
        const plucked_join = joins.find(({ type, field_relation }) => {
          const to = type === 'self' ? field_relation.from : field_relation.to;
          return (to.alias || to.entity) === table;
        });
        if (!plucked_join)
          this.logger.warn(`No join found for pluck group object ${table}`);
        const { to } = plucked_join?.field_relation || {};
        const join_order_by = to?.order_by || null;
        const join_order_direction = to?.order_direction || EOrderDirection.ASC;
        const join_is_case_sensitive_sorting =
          to?.is_case_sensitive_sorting || false;
        return fields.reduce((field_acc, field) => {
          let sorted_field = `"${table}"."${join_order_by}"`;
          if (join_is_case_sensitive_sorting)
            sorted_field = `LOWER(${sorted_field})`;
          const sort_schema = ` ORDER BY ${sorted_field} ${
            ['asc', 'ascending'].includes(join_order_direction) ? 'asc' : 'desc'
          }`;
          const alias = pluralize(field);
          return {
            ...field_acc,
            [`${table}_${alias}`]: sql
              .raw(
                `JSONB_AGG(${Utility.decryptField({
                  field: `"${table}"."${field}"`,
                  encrypted_fields,
                  table,
                  pass_field_key,
                })}${
                  join_order_by && join_order_direction ? sort_schema : ''
                })`,
              )
              .as(`${table}_${alias}`),
          };
        }, acc);
      },
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
    date_format: EDateFormats = EDateFormats['mm/dd/YYYY'],
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
        const field_names = field.fields.map((f) => {
          if (f.endsWith('_date'))
            return `COALESCE(to_char("${table_name}"."${f}"::date, '${date_format}'), '')`;
          return `COALESCE("${table_name}"."${f}", '')`;
        });
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

  public static parsePluckedFields({
    table,
    pluck,
    date_format,
    encrypted_fields = [],
    time_zone,
    pass_field_key,
    permissions = {},
  }: {
    table: string;
    pluck: string[];
    date_format: EDateFormats;
    encrypted_fields?: string[];
    time_zone?: string;
    pass_field_key: string;
    permissions?: Record<string, any>;
  }): Record<string, any> | null {
    const table_schema = this.checkTable(table).table_schema;
    if (!pluck?.length || !pluck) {
      return null;
    }
    const _plucked_fields = pluck.reduce((acc, field) => {
      if (table_schema[field]) {
        const is_date_time_field =
          field.toLowerCase().endsWith('_date') ||
          field.toLowerCase().endsWith('_time');

        return {
          ...acc,
          [field]: is_date_time_field
            ? sql.raw(
                `${Utility.formatDate({
                  table,
                  field,
                  date_format,
                  time_zone,
                  encrypted_fields,
                  fields: pluck,
                  pass_field_key,
                })}
               AS "${field}"`,
              )
            : sql.raw(
                Utility.decryptField({
                  field: `"${table}"."${field}"`,
                  encrypted_fields,
                  table,
                  pass_field_key,
                  permissions,
                }),
              ),
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

  public static FilterAnalyzer = ({
    db,
    table_schema,
    advance_filters,
    pluck_object = {},
    organization_id,
    joins = [],
    client_db,
    concatenate_fields,
    group_advance_filters = [],
    request_type,
    encrypted_fields = [],
    time_zone,
    table,
    date_format,
    pass_field_key,
    permissions,
    concatenated_field_expressions = {},
  }: IFilterAnalyzer) => {
    let _db: any = db;
    const aliased_entities: any = [];
    let expressions = concatenate_fields?.expressions || {};
    const concat_fields = concatenate_fields?.fields || {};

    if (joins.length) {
      joins.forEach(
        (
          { type, field_relation, nested = false }: Record<string, any>,
          index,
        ) => {
          const { from, to } = field_relation;
          const to_entity = to.entity;
          const from_alias = from.alias || from.entity; // Use alias if provided
          const to_alias =
            type === 'self' ? from.alias || from.entity : to.alias || to_entity; // Use alias if provided
          to.filters ??= [];
          const concatenated_entity =
            concatenated_field_expressions[to_alias] || {};
          function constructJoinQuery({ isSelfJoin = false } = {}) {
            if (to.alias) aliased_entities.push(to.alias);
            // Retrieve fields from pluck_object for the specified `to` entity
            const fields = pluck_object[to_alias] || [];
            const to_table_schema = schema[to_entity];
            const aliased_to_entity = aliasedTable(
              to_table_schema,
              `joined_${to_alias}`,
            );

            let sub_query = client_db.select().from(aliased_to_entity);
            let nested_additional_filter: any = [];
            if (nested) {
              const previous_join: any = joins[index - 1];
              const { type, field_relation } = previous_join;
              const { from: prev_join_from, to: prev_join_to } = field_relation;
              const nested_from =
                type === 'self' ? prev_join_from : prev_join_to;

              const parent_entity = nested_from.alias
                ? aliasedTable(schema[nested_from.entity], nested_from.alias)
                : schema[nested_from.entity];

              nested_additional_filter = [
                eq(aliased_to_entity.id, parent_entity[from.field]),
              ];
            }
            sub_query = sub_query
              .where(
                and(
                  eq(aliased_to_entity['tombstone'], 0),
                  ...(request_type !== 'root'
                    ? [
                        isNotNull(aliased_to_entity['organization_id']),
                        eq(
                          aliased_to_entity['organization_id'],
                          organization_id,
                        ),
                      ]
                    : []),
                  ...Utility.constructFilters({
                    table,
                    advance_filters: to.filters,
                    table_schema: aliased_to_entity,
                    aliased_entities: [`joined_${to_alias}`],
                    time_zone,
                    date_format,
                    encrypted_fields,
                    pass_field_key,
                    permissions,
                  }),
                  ...nested_additional_filter,
                ),
              )
              .toSQL();

            const sub_query_from_clause =
              Utility.getPopulatedQueryFrom(sub_query);
            const join_order_direction = to.order_direction || 'ASC';
            let order_by = to.order_by || 'created_date';
            order_by = order_by.replace(`${to_alias}.`, '');
            const is_case_sensitive_sorting =
              to.is_case_sensitive_sorting || false;
            let sorted_field = `"joined_${to_alias}"."${order_by}"`;
            if (is_case_sensitive_sorting)
              sorted_field = `LOWER(${sorted_field})`;

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

            const concatenated_fields = Object.values(concatenated_entity)
              ?.map((concatenated) => (concatenated as any)?.fields)
              .flat();

            const joined_selected_fields = [
              ...new Set([...fields, ...concatenated_fields]),
            ];
            const lateral_join = sql.raw(`
            LATERAL (
              SELECT ${joined_selected_fields
                .map((field) => `"joined_${to_alias}"."${field}"`)
                .join(', ')}
              ${sub_query_from_clause} ${additional_where_and_clause}
              ${
                to.order_by
                  ? `ORDER BY ${sorted_field} ${join_order_direction.toUpperCase()}`
                  : ''
              }
              ${to.limit ? `LIMIT ${to.limit}` : ''}
            ) AS "${isSelfJoin ? from_alias : to_alias}"
          `);

            _db = _db?.leftJoin(lateral_join, sql`TRUE`);
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
        },
      );
    }

    return _db.where(
      and(
        eq(table_schema['tombstone'], 0),
        ...(request_type !== 'root'
          ? [
              isNotNull(table_schema['organization_id']),
              eq(table_schema['organization_id'], organization_id),
            ]
          : []),
        ...Utility.constructFilters({
          table,
          advance_filters,
          table_schema,
          aliased_entities,
          time_zone,
          date_format,
          group_advance_filters,
          encrypted_fields,
          concatenated_field_expressions,
        }),
      ),
    );
  };

  public static AggregationFilterAnalyzer({
    db,
    table_schema,
    advance_filters: _advance_filters = [],
    organization_id,
    joins = [],
    client_db: _client_db,
    type,
    time_zone = '',
    table,
    date_format = EDateFormats['mm/dd/YYYY'],
  }: IAggregationFilterAnalyzer) {
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
        ...Utility.constructFilters({
          table,
          advance_filters: _advance_filters,
          table_schema,
          aliased_entities,
          time_zone,
          date_format,
        }),
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
    case_sensitive,
    parse_as = '',
    encrypted_fields = [],
    fields = [],
    time_zone,
    date_format,
    pass_field_key = '',
    permissions,
    concatenated_field_expressions = {},
  }: IEvaluateFilter) {
    const is_aliased = aliased_entities.includes(entity);
    let _field = `${field}`;
    // if (!table_schema?.[field] && !is_aliased) return null;
    let schema_field;
    // Handle encrypted fields for Permission
    if (encrypted_fields?.length) {
      _field = Utility.decryptField({
        field: _field,
        encrypted_fields,
        table: entity,
        pass_field_key,
        permissions,
      });
    }

    const concatenated_entity = concatenated_field_expressions?.[entity] ?? {};
    // Handle concatenated fields for schema_field
    if (
      !table_schema?.[field] &&
      Object.keys(concatenated_entity)?.length &&
      concatenated_entity?.[field]?.expression
    ) {
      schema_field = sql.raw(
        Utility.decryptField({
          field: concatenated_entity?.[field]?.expression,
          encrypted_fields,
          table: entity,
          pass_field_key,
          permissions,
        }),
      );
    } else {
      // Handle schema fields for date fields
      if (field?.endsWith('_date')) {
        schema_field = sql.raw(
          `to_char(${Utility.decryptField({
            field: `"${entity}"."${field}"`,
            encrypted_fields,
            table: entity,
            pass_field_key,
            permissions,
          })}::date, '${date_format}')`,
        );
      } else
        schema_field = is_aliased
          ? sql.raw(
              Utility.decryptField({
                field: `"${entity}"."${field}"`,
                encrypted_fields,
                table: entity,
                pass_field_key,
                permissions,
              }),
            )
          : sql.raw(
              Utility.decryptField({
                field: `"${entity}"."${field}"`,
                encrypted_fields,
                table: entity,
                pass_field_key,
                permissions,
              }),
            );
    }

    // Handle parsing to text for non text fields in filtering
    if (parse_as === 'text') {
      schema_field = entity
        ? sql.raw(
            `${Utility.decryptField({
              field: `"${entity}"."${field}"`,
              encrypted_fields,
              table: entity,
              pass_field_key,
              permissions,
            })}::TEXT`,
          )
        : sql.raw(`"${_field}"::TEXT`);
    }

    // Handle filtering with consideration of the Time Zone (for date and time fields)
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

      schema_field = sql.raw(`(${Utility.decryptField({
        field: `"${entity}"."${fields[date_field_index]}"`,
        encrypted_fields,
        table: entity,
        pass_field_key,
        permissions,
      })}::timestamp + ${Utility.decryptField({
        field: `"${entity}"."${fields[time_field_index]}"`,
        encrypted_fields,
        table: entity,
        pass_field_key,
        permissions,
      })}::interval) AT TIME ZONE '${TZ}'
      `);
      values = [
        sql.raw(
          `'${values[date_field_index]} ${values[time_field_index]}'::timestamp AT TIME ZONE '${time_zone}'`,
        ),
      ];
    }

    switch (operator) {
      case EOperator.EQUAL:
        // exact value for an array field
        if (pluralize.isPlural(field)) {
          return eq(
            schema_field,
            sql.raw(`ARRAY[${values.map((value) => `'${value}'`).join(', ')}]`),
          );
        }
        return or(...values.map((value) => eq(schema_field, value)));
      case EOperator.NOT_EQUAL:
        if (pluralize.isPlural(field)) {
          return ne(
            schema_field,
            sql.raw(`ARRAY[${values.map((value) => `'${value}'`).join(', ')}]`),
          );
        }
        return and(...values.map((value) => ne(schema_field, value)));
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
        // Can be used on string or array fields (must be parsed as text)
        if (case_sensitive) {
          return or(...values.map((value) => like(schema_field, `%${value}%`)));
        }
        return or(...values.map((value) => ilike(schema_field, `%${value}%`)));
      case EOperator.NOT_CONTAINS:
        if (case_sensitive) {
          return or(
            ...values.map((value) => notLike(schema_field, `%${value}%`)),
          );
        }
        return and(
          ...values.map((value) => notIlike(schema_field, `%${value}%`)),
        );
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
      case EOperator.HAS_NO_VALUE:
        let is_empty_filter = eq(schema_field, '');
        if (pluralize.isPlural(field)) {
          is_empty_filter = sql.raw(
            `ARRAY_LENGTH("${entity}"."${field}", 1) IS NULL OR ARRAY_LENGTH("${entity}"."${field}", 1) = 0`,
          );
        }
        return or(is_empty_filter, isNull(schema_field));
      default:
        return null;
    }
  }

  public static constructFilters({
    table,
    advance_filters = [],
    table_schema,
    aliased_entities = [],
    time_zone = '',
    date_format = EDateFormats['mm/dd/YYYY'],
    group_advance_filters = [],
    encrypted_fields = [],
    pass_field_key = '',
    permissions,
    concatenated_field_expressions = {},
  }: IContructFilters): any[] {
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
            this.constructFilters({
              table,
              advance_filters: filters,
              table_schema,
              aliased_entities,
              time_zone,
              date_format,
              concatenated_field_expressions,
              encrypted_fields,
              pass_field_key,
            }),
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
          field = '',
          values = [],
          type = 'criteria',
          case_sensitive = false,
          parse_as = '',
          fields = [],
        },
      ] = advance_filters as IAdvanceFilters[] as [
        {
          field?: string;
          operator: EOperator;
          values?: string[] | number[] | boolean[] | Date[];
          logical_operator?: 'AND' | 'OR';
          type: 'criteria' | 'operator';
          entity?: string;
          case_sensitive?: boolean;
          parse_as?: 'text';
          fields?: Array<string>;
        },
      ];
      if (typeof values === 'string') {
        values = JSON.parse(values);
      }
      let { entity } = advance_filters[0] as { entity: string };
      if (type === 'operator') {
        throw new BadRequestException(
          `Invalid filter at index 0. Must be a criteria`,
        );
      }

      entity =
        entity &&
        !aliased_entities.includes(entity) &&
        !Object.keys(concatenated_field_expressions?.[entity] ?? {}).length
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
          case_sensitive,
          parse_as,
          encrypted_fields,
          fields,
          time_zone,
          date_format,
          pass_field_key,
          permissions,
          concatenated_field_expressions,
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
        parse_as = '',
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
          values: filter.values ?? [],
          dz_filter_queue,
          entity: entity || table,
          aliased_entities,
          case_sensitive,
          parse_as,
          fields,
          time_zone,
          date_format,
          concatenated_field_expressions,
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
            values: filter.values ?? [],
            dz_filter_queue: where_clause_queue.concat(allowed_to_merged),
            entity: entity || table,
            aliased_entities,
            case_sensitive,
            parse_as,
            fields,
            time_zone,
            date_format,
            concatenated_field_expressions,
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
    Utility.validateFields(table, schema, data);
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
  }) {
    let parsed_body: { records: Record<any, any>[] } = {
      records: [],
    };
    try {
      parsed_body.records = body?.records ? JSON.parse(body.records) : [];
    } catch (error: any) {
      throw new Error(
        'Invalid JSON body, make sure records are provided',
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
    organization_id,
    account_id,
  }) {
    if (encrypted_fields?.length) {
      this.logger.log(`Encrypting data... ${encrypted_fields.join(',')}`);
      const encryption_key = sha1(
        `${organization_id}_${table}_${process.env.PGP_SYM_KEY}`,
      );
      this.logger.debug(`encryption_key: ${encryption_key}`);
      const set_val = `${Object.entries(data)
        .reduce((acc: string[], [key, value]) => {
          let _value = `${typeof value === 'string' ? `'${value}'` : value}`;
          if (Array.isArray(value)) {
            _value = `to_jsonb(ARRAY[${value
              .map((v) => `'${v}'`)
              .join(', ')}])`;
          }

          if (encrypted_fields.includes(`${table}.${key}`)) {
            if (Array.isArray(value))
              _value = `safe_encrypt_array(${_value}, '${encryption_key}')`;
            else
              acc.push(`${key} = safe_encrypt(${_value}, '${encryption_key}')`);
          } else {
            acc.push(`${key} = ${_value}`);
          } // Push the unencrypted value for other field
          return acc;
        }, [])
        .join(',')}`;

      const ek_query = `
          INSERT INTO encryption_keys (id, entity, organization_id, created_by, timestamp, tombstone) 
          VALUES(
            '${encryption_key}', 
            safe_encrypt('${table}', '${process.env.PGP_SYM_KEY}'), 
            safe_encrypt('${organization_id}', '${process.env.PGP_SYM_KEY}'),
            '${account_id}',
            '${new Date().toISOString()}',
            0
          ) ON CONFLICT (id) DO NOTHING;
          `;
      const _values = `(${Object.keys(data)}) VALUES (${Utility.encryptData(
        data,
        encrypted_fields,
        encryption_key,
        table,
      )})`;
      const query = `
      BEGIN;
      ${ek_query}
      INSERT INTO ${table} ${_values} ON CONFLICT (id) DO UPDATE SET ${set_val};
      COMMIT;`;
      this.logger.debug(`Encrypting data: ${query}`);
      return db.execute(sql.raw(query)).then(() => {
        this.logger.debug('Encrypting data completed');
        return data;
      });
    }
    const { table_schema } = query;
    return db
      .insert(query.table_schema)
      .values(data)
      .onConflictDoUpdate({
        target: table_schema.hypertable_timestamp
          ? [query.table_schema.id, query.table_schema.timestamp]
          : query.table_schema.id,
        set: data,
      })
      .returning({ table_schema })
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
    organization_id,
  }) {
    if (encrypted_fields?.length) {
      this.logger.log(`Encrypting data... ${encrypted_fields.join(',')}`);
      const encryption_key = sha1(
        `${organization_id}_${table}_${process.env.PGP_SYM_KEY}`,
      );
      this.logger.debug(`encryption_key: ${encryption_key}`);
      const set_val = `${Object.entries(data)
        .reduce((acc: string[], [key, value]) => {
          let _value = `${typeof value === 'string' ? `'${value}'` : value}`;
          if (Array.isArray(value)) {
            _value = `to_jsonb(ARRAY[${value
              .map((v) => `'${v}'`)
              .join(', ')}])`;
          }

          if (encrypted_fields.includes(`${table}.${key}`)) {
            if (Array.isArray(value))
              _value = `safe_encrypt_array(${_value}, '${encryption_key}')`;
            else
              acc.push(`${key} = safe_encrypt(${_value}, '${encryption_key}')`);
          } else {
            acc.push(`${key} = ${_value}`);
          } // Push the unencrypted value for other field
          return acc;
        }, [])
        .join(',')}`;
      const query = `UPDATE ${table} SET ${set_val} WHERE ${where.join('')}`;
      this.logger.debug(`Encrypting data: ${query}`);
      return db.execute(sql.raw(query)).then(() => {
        this.logger.debug('Encrypting data completed');
        return data;
      });
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
      .execute()
      .then(([{ table_schema }]) => table_schema);
  }

  public static encryptData(
    data: Record<string, any>,
    encryption_keys,
    encrypt_key = '',
    table,
  ) {
    const values = `${Object.entries(data)
      .reduce((encryptedData: any[], [key, value]) => {
        let _value = typeof value === 'string' ? `'${value}'` : value;
        if (Array.isArray(value)) {
          _value = `to_jsonb(ARRAY[${value.map((v) => `'${v}'`).join(', ')}])`;
        }

        if (
          (encryption_keys as string[]).includes(`${table}.${key}`) &&
          encrypt_key
        ) {
          if (Array.isArray(value))
            _value = `safe_encrypt_array(${_value}, '${encrypt_key}')`;
          else _value = `safe_encrypt(${_value}, '${encrypt_key}')`;
          encryptedData.push(_value);
          return encryptedData;
        }

        encryptedData.push(_value !== undefined ? _value : null);
        return encryptedData;
      }, [])
      .join(',')}`;
    return values;
  }
  public static decryptField({
    field,
    encrypted_fields,
    table,
    encryption_key,
    value,
    permissions = {},
    pass_field_key,
  }: {
    field: string;
    encrypted_fields: string[];
    table: string;
    encryption_key?: string;
    value?: any;
    permissions?: Record<string, any>;
    pass_field_key: string;
  }) {
    let _field = field?.replaceAll('"', '');
    let _entity = table;

    const field_parts = _field?.split('.');

    if (field_parts?.length === 2) {
      _field = field_parts[1] as string;
      _entity = field_parts[0] as string;
    } else {
      _field = field_parts?.[0] as string;
    }
    const can_mask = false;
    // const can_mask = !!permissions?.data?.find(
    //   (p) => p.entity === _entity && p.field === _field && p.sensitive === true,
    // );

    const can_decrypt = !!permissions?.data?.find(
      (p) => p.entity === _entity && p.field === _field && p.decrypt === true,
    );
    const can_read = !!permissions?.data?.find(
      (p) => p.entity === _entity && p.field === _field && p.read === true,
    );

    const encrypted_field = `${_entity ? `${_entity}.` : ''}${_field}`;
    let data_type = `${encrypted_field}`;
    if (
      encrypted_fields.includes(encrypted_field) &&
      can_decrypt &&
      can_read &&
      pass_field_key
    ) {
      let decrypted_field = pluralize.isPlural(_field)
        ? `safe_decrypt_array(to_jsonb(${data_type}), '${
            pass_field_key ? pass_field_key : encryption_key
          }')`
        : `safe_decrypt(${data_type}::BYTEA, '${
            pass_field_key ? pass_field_key : encryption_key
          }')`;

      if (value && decrypted_field !== field) {
        return sql.raw(`${decrypted_field}`);
      }
      return decrypted_field;
    }

    if (value) {
      if (!encrypted_fields.includes(field) && can_read) {
        return sql.raw(`${Utility.maskValue({ field: data_type, can_mask })}`);
      }
      return value;
    }

    return field;
  }

  public static maskValue({ field, can_mask }) {
    if (!can_mask) {
      return field;
    }
    return `maskIfBytea(${field})`;
  }

  public static decryptData(
    data: Record<string, any>,
    encrypted_fields: string[],
    table,
    permissions,
    pass_field_key,
    encryption_key = process.env.PGP_SYM_KEY ?? '',
  ) {
    return Object.entries(data).reduce((_data, [key, value]) => {
      return {
        ..._data,
        [key]: Utility.decryptField({
          field: key,
          encrypted_fields,
          table,
          encryption_key,
          value,
          permissions,
          pass_field_key,
        }),
      };
    }, {});
  }

  public static constructPermissionSelectWhereClause({ tables, main_fields }) {
    return `AND (
          data_permissions.tombstone = 0 AND entities.name IN (${tables
            .map((table) => `'${table}'`)
            .join(',')}) AND fields.name IN (${main_fields
      .map((field) => `'${field}'`)
      .join(',')})
        )`;
  }

  public static async isPermitted(account_id, role) {
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

  public static getReadPermittedFields(config) {
    Utility.checkPermissions(config, 'read');
    return {
      ...config,
      metadata: config.metadata,
    };
  }

  public static getWritePermittedFields(config) {
    Utility.checkPermissions(config, 'write');
    return {
      ...config,
      metadata: config.metadata,
    };
  }

  public static checkPermissions(
    { table, schema, permissions, metadata, body, query },
    permission_type: 'read' | 'write' | 'encrypt' | 'decrypt' | 'required',
  ) {
    switch (permission_type) {
      case 'read':
        schema.forEach(
          ({ entity: _entity, field: _field, alias, path, property_name }) => {
            const permission = permissions.data.find(
              (p) => p.read && p.entity === _entity && p.field === _field,
            );
            const hasPermission = !!permission;
            if (!hasPermission) {
              const stack = `[${table}]: Found at ${property_name}${
                alias ? `(${alias})` : ''
              }${path} (${_field})`;
              if (!metadata.find((e) => e.stack === stack)) {
                metadata.push({
                  message: `${_field} is not permitted to access.`,
                  path: stack,
                  entity: _entity,
                  field: _field,
                });
              }

              if (query.pluck) {
                query.pluck = query.pluck
                  .split(',')
                  .filter((f) => f !== _field)
                  .join(',');
              } else {
                // removing data from body
                const cloned_body = { ...body };
                switch (property_name) {
                  case 'joins':
                    cloned_body?.[property_name]?.forEach((join, index) => {
                      if (join?.field_relation?.to?.field === _field) {
                        delete body?.[property_name][index]?.field_relation.to
                          .field;
                      } else if (join?.field_relation?.from?.field === _field) {
                        delete body?.[property_name][index]?.field_relation.from
                          .field;
                      }
                      if (
                        !cloned_body?.[property_name][index]?.field_relation
                          ?.to &&
                        !cloned_body?.[property_name][index]?.field_relation
                          ?.from
                      ) {
                        delete body?.[property_name][index].field_relation;
                      }
                    });
                    break;
                  case 'pluck_object':
                    if (
                      cloned_body?.[property_name][alias ?? _entity].includes(
                        _field,
                      )
                    ) {
                      const index =
                        body?.[property_name][alias ?? _entity]?.indexOf(
                          _field,
                        );
                      if (index > -1) {
                        body?.[property_name][alias ?? _entity]?.splice(
                          index,
                          1,
                        );
                      }
                    }
                    break;
                  case 'multiple_sort':
                    cloned_body?.[property_name]?.forEach((sort, index) => {
                      if (pluralize(sort.by_field) === _field) {
                        delete body?.[property_name][index].by_field;
                      }
                    });
                    break;
                  case 'concatenate_fields':
                    cloned_body?.[property_name]?.forEach((concat, index) => {
                      if (concat?.fields.includes(_field)) {
                        body?.[property_name][index]?.fields.splice(
                          body?.[property_name][index]?.fields.indexOf(_field),
                          1,
                        );
                      }
                    });
                    break;
                  case 'group_by':
                    cloned_body?.[property_name]?.fields.forEach((f) => {
                      const [entity, field] = f.split('.');
                      if (field === _field) {
                        body?.[property_name]?.fields.splice(
                          body?.[property_name]?.fields.indexOf(
                            `${entity}.${_field}`,
                          ),
                          1,
                        );
                      }
                    });
                    break;
                  case 'distinct_by':
                    if (cloned_body?.[property_name] === _field) {
                      delete body?.[property_name];
                    }
                    break;
                  default:
                    if (cloned_body?.[property_name]?.includes(_field)) {
                      if (body?.[property_name]?.includes(_field)) {
                        const index = body[property_name].indexOf(_field);
                        body[property_name].splice(index, 1);
                      }
                    }
                    break;
                }
              }
            }
          },
        );
        break;
      case 'write':
        schema.forEach(
          ({ entity: _entity, field: _field, alias, path, property_name }) => {
            const permission = permissions.data.find(
              (p) => p.write && p.entity === _entity && p.field === _field,
            );
            const hasPermission = !!permission;

            if (!hasPermission) {
              const stack = `[${table}]: Found at ${property_name}${
                alias ? `(${alias})` : ''
              }${path} (${_field})`;
              if (!metadata.find((e) => e.stack === stack)) {
                metadata.push({
                  message: `${permission_type} access to the '${_field}' field is not permitted.`,
                  path: stack,
                  entity: _entity,
                  field: _field,
                });
              }
              delete body[property_name];
            }
          },
        );
        break;
      default:
        break;
    }
  }

  public static getCachedPermissions(
    permission_type: 'read' | 'write',
    {
      data_permissions_query,
      host,
      cookie,
      headers,
      table,
      account_organization_id,
      db,
      body,
      metadata,
      account_id,
      query,
    },
  ) {
    const {
      query: dpquery,
      account_organization_id: account_organization_id_fr_dp,
      schema: _aliased_schema,
      valid_pass_keys_query,
      record_valid_pass_keys_query,
    } = data_permissions_query;
    const custom_suffix = `${host}${cookie}${
      headers?.['user-agent'] ?? ''
    }}${JSON.stringify(query)}`;
    this.logger.debug(`custom_suffix: ${custom_suffix}`);
    const getByQueries = async ({
      type,
      cache_key,
      query: q,
      expiry,
    }): Promise<Record<string, any>> => {
      this.logger.debug(`Getting ${type} permissions`);
      const data = JSON.parse(cache.get(cache_key));
      const cached = data
        ? data
        : await db
            .execute(q.trim())
            .then((response) => ({
              data: response.rows,
              account_organization_id,
              cache: false,
            }))
            .catch(() => []);
      this.logger.debug(`Getting ${type} permissions completed.`);
      if (process.env.DEBUG === 'true') console.table(cached.data);
      if (data === null) {
        this.logger.debug(`${type} cache miss`);
        cache.put(
          cache_key,
          JSON.stringify({
            ...cached,
            cache: true,
          }),
          Utility.getTimeMs(expiry ?? '2d'),
        );
      } else {
        this.logger.debug(`${type} cache hit`);
      }

      switch (type) {
        case 'field_permissions':
          if (cached?.data?.length) {
            switch (permission_type) {
              case 'read':
                const { metadata: acc_read_metadata } =
                  Utility.getReadPermittedFields({
                    body,
                    table,
                    permissions: cached,
                    metadata,
                    schema: _aliased_schema,
                    query,
                  });
                metadata = acc_read_metadata;
                break;
              case 'write':
                const { metadata: acc_write_metadata } =
                  Utility.getWritePermittedFields({
                    body,
                    table,
                    permissions: cached,
                    metadata,
                    schema: _aliased_schema,
                  });
                metadata = acc_write_metadata;
                break;
              default:
                break;
            }
          } else {
            this.logger.warn(
              `No permissions assigned to table:${table} from account_organization_id: ${account_organization_id_fr_dp} | ${account_id}.`,
            );
            // TODO: finalize the role based permissions
            await Utility.isPermitted(account_id, 'Super Admin');
          }
          return cached;
        default:
          return cached;
      }
    };

    return {
      metadata,
      getPermissions: getByQueries({
        type: 'field_permissions',
        cache_key: sha1(`${table}_data_permissions:${custom_suffix}`),
        query: dpquery,
        expiry: process.env.JWT_EXPIRES_IN,
      }),
      getValidPassKeys: getByQueries({
        type: 'valid_pass_keys',
        cache_key: sha1(
          `${table}_valid_pass_keys:${custom_suffix}:${account_organization_id}`,
        ),
        query: valid_pass_keys_query,
        expiry: process.env.JWT_EXPIRES_IN,
      }),
      getRecordPermissions: getByQueries({
        type: 'record_permissions',
        cache_key: sha1(
          `${table}_record_permissions:${custom_suffix}:${account_organization_id}`,
        ),
        query: record_valid_pass_keys_query,
        expiry: process.env.JWT_EXPIRES_IN,
      }),
    };
  }

  public static getSortSchemaAndField({
    table,
    table_schema,
    order_by,
    aliased_entities,
    order_direction = 'asc',
    is_case_sensitive_sorting = false,
    group_by_selections,
    concatenated_field_expressions = {},
  }: {
    table: string;
    table_schema: Record<string, any>;
    order_by: string;
    aliased_entities: Record<string, any>[];
    order_direction: string;
    is_case_sensitive_sorting: boolean;
    group_by_selections: Record<string, any>;
    concatenated_field_expressions: Record<string, any>;
  }) {
    const by_entity_field = order_by.split('.');
    let sort_entity: any = table;
    let sort_field = by_entity_field[0] || 'id';
    let sort_schema = table_schema[sort_field];
    // Handle for dot notation format of field to be sorted
    if (by_entity_field.length > 1) {
      const [_entity = '', by_field = 'id'] = by_entity_field;
      sort_field = by_field;
      sort_entity = _entity;
    }
    const is_aliased = aliased_entities.find(
      ({ alias }) => alias === sort_entity,
    );
    sort_entity = !is_aliased ? pluralize(sort_entity) : sort_entity;

    const concatenated_entity =
      concatenated_field_expressions?.[sort_entity] ?? {};
    const sorted_entity_schema = is_aliased
      ? aliasedTable(schema[is_aliased?.entity], sort_entity)
      : schema[sort_entity];

    // if sorted entity is in the concatenated entities
    // and the field is in the concatenated fields
    if (
      Object.keys(concatenated_entity)?.length &&
      concatenated_entity?.[sort_field]?.expression
    ) {
      const field_concatenated_exp = (concatenated_entity?.[sort_field] ?? {})
        ?.expression;
      sort_schema = field_concatenated_exp;
    }
    // if not concatenated
    // sort_entity is either main or related entity or aliased
    else {
      sort_schema = `"${sort_entity}"."${sort_field}"`;
    }

    if (!is_case_sensitive_sorting) {
      const sorted_field_type = sorted_entity_schema?.[sort_field]?.dataType;
      if (
        sorted_field_type !== 'string' &&
        !concatenated_entity?.[sort_field]?.expression
      ) {
        throw new BadRequestException(
          `Sorted field ${
            by_entity_field[0] || 'id'
          } is of type ${sorted_field_type}. Set is_case_sensitive_sorting to true to sort non-text fields.`,
        );
      }
      sort_schema = `lower(${sort_schema})`;
    }
    if (Object.keys(group_by_selections).length || sort_entity !== table) {
      if (order_direction.toLowerCase() === 'asc') {
        return sql.raw(`MIN(${sort_schema})`);
      } else {
        return sql.raw(`MAX(${sort_schema})`);
      }
    }
    return sql.raw(sort_schema) as SQLWrapper | AnyColumn;
  }

  public static generateConcatenatedExpressions(
    concatenate_fields: IConcatenateField[],
    date_format: EDateFormats = EDateFormats['mm/dd/YYYY'],
    _table?: string,
  ) {
    return concatenate_fields.reduce(
      (
        acc,
        { fields, field_name, separator, entity: _entity, aliased_entity },
      ) => {
        const entity = aliased_entity || pluralize(_entity);

        const concatenated_expression = `(${fields
          .map((field) => {
            if (field.endsWith('_date'))
              return `COALESCE(to_char("${entity}"."${field}"::date, '${date_format}'), '')`;
            return `COALESCE("${entity}"."${field}", '')`;
          })
          .join(` || '${separator}' || `)})`;

        return {
          ...acc,
          [entity]: {
            ...acc[entity],
            [field_name]: { expression: concatenated_expression, fields },
          },
        };
      },
      {},
    );
  }

  public static async generateCode(db, entity: string) {
    const counter_schema = schema['counters'];
    return db
      .insert(counter_schema)
      .values({ entity, counter: 1 })
      .onConflictDoUpdate({
        target: [counter_schema.entity],
        set: {
          counter: sql`${counter_schema.counter} + 1`,
        },
      })
      .returning({
        prefix: counter_schema.prefix,
        default_code: counter_schema.default_code,
        counter: counter_schema.counter,
        digits_number: counter_schema.digits_number,
      })
      .then(([entity_code]) => {
        const { prefix, default_code, counter } = entity_code as Record<
          string,
          any
        >;
        let { digits_number } = entity_code as Record<string, any>;
        const getDigit = (num: number) => {
          return num.toString().length;
        };

        if (digits_number) {
          digits_number = digits_number - getDigit(counter || 0);
          const zero_digits =
            digits_number > 0 ? '0'.repeat(digits_number) : '';
          return prefix + (zero_digits + counter);
        }
        return prefix + (default_code + counter);
      })
      .catch(() => null);
  }

  public static replacePlaceholders(query, values) {
    values.forEach((value, index) => {
      const placeholder = `\\$${index + 1}`;
      const formatted_value = typeof value === 'string' ? `'${value}'` : value;
      query = query.replace(new RegExp(placeholder, 'g'), formatted_value);
    });
    return query;
  }
}
// TODO: dont use past tense in encryption fields
