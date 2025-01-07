import { BadRequestException, NotFoundException } from '@nestjs/common';
import { ZodObject } from 'zod';
import * as schema from '../schema';
import { createInsertSchema, createUpdateSchema } from 'drizzle-zod';
import { ulid } from 'ulid';
import { ZodValidationException } from '@dna-platform/common';
import {
  EOperator,
  IAdvanceFilters,
  IJoins,
} from '../xstate/modules/schemas/find/find.schema';
import {
  aliasedTable,
  and,
  between,
  eq,
  gt,
  gte,
  inArray,
  isNotNull,
  isNull,
  lt,
  lte,
  ne,
  notBetween,
  notInArray,
  or,
} from 'drizzle-orm';

export class Utility {
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
    } catch (error) {
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
    } catch (error) {
      throw new ZodValidationException(error);
    }
  }

  public static convertTime12to24(time12h: string) {
    const [time = '', modifier] = time12h.split(' ');

    let [hours = '', minutes] = time.split(':');

    if (hours === '12') {
      hours = '00';
    }

    if (modifier === 'PM') {
      hours = `${parseInt(hours, 10) + 12}`;
    }

    if (parseInt(hours) < 10) {
      hours = `0${hours}`;
    }

    return `${hours}:${minutes}`;
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

  public static queryGenerator(params, organization_id: string) {
    const {
      entity, // Name of the table
      aggregations, // Array of aggregation objects
      bucket_size, // Time bucket size (e.g., 1 hour, 1 day)
      advance_filters,
      order: { order_direction, order_by }, // Column to order by
    } = params;

    // Validate required parameters
    if (!entity || !bucket_size || !aggregations || aggregations.length === 0) {
      throw new Error(
        'Missing required parameters: entity, bucket_size, or aggregations.',
      );
    }

    // Generate the SELECT clause
    const selectClauses = aggregations.map(
      ({ aggregation, aggregate_on, bucket_name }) => {
        if (!aggregation || !aggregate_on || !bucket_name) {
          throw new Error(
            'Missing aggregation details: aggregation, aggregate_on, or bucket_name.',
          );
        }
        return `${aggregation}(${aggregate_on}) AS ${bucket_name}`;
      },
    );

    const selectClause = `
        SELECT time_bucket('${bucket_size}', timestamp) AS bucket,
               ${selectClauses.join(',\n               ')}
    `;

    // Generate the FROM clause
    const fromClause = `FROM ${entity}`;

    // Generate the WHERE clause
    let whereClause = this.advanceFilter(advance_filters, organization_id);
    console.log(whereClause);
    // Generate the GROUP BY clause
    const groupByClause = `GROUP BY bucket`;

    // Generate the ORDER BY clause
    const orderDirection = order_direction
      ? order_direction.toUpperCase()
      : 'asc';
    const orderByClause = order_by
      ? `ORDER BY ${order_by} ${orderDirection}`
      : '';

    // Combine all clauses into the final query
    const query = `
        ${selectClause}
        ${fromClause}
        ${whereClause}
        ${groupByClause}
        ${orderByClause};
    `;

    return query.trim();
  }
  public static format(data: any, is_insert = true) {
    const date = new Date();
    const _data = {
      id: data?.id ? data.id : ulid(),
      ...(is_insert
        ? {
            tombstone: 0,
            status: 'Active',
            created_date: date.toLocaleDateString(),
            created_time: Utility.convertTime12to24(date.toLocaleTimeString()),
            updated_date: date.toLocaleDateString(),
            updated_time: Utility.convertTime12to24(date.toLocaleTimeString()),
          }
        : {
            updated_date: date.toLocaleDateString(),
            updated_time: Utility.convertTime12to24(date.toLocaleTimeString()),
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
    const schema_table = Utility.checkTable(table);
    if (!data) {
      throw new BadRequestException('Data is required in Body');
    }

    return { schema: createInsertSchema(schema_table), data, meta };
  }

  public static checkUpdateSchema(
    table: string,
    meta: Record<string, any>,
    data: Record<string, any>,
  ) {
    const schema_table = Utility.checkTable(table);
    if (!data) {
      throw new BadRequestException('Data is required in Body');
    }

    return { schema: createUpdateSchema(schema_table), data, meta };
  }
  public static checkTable(table: string) {
    const table_schema = schema[table];
    if (
      !table ||
      !table_schema ||
      table === 'config_sync' ||
      table.includes('crdt')
    ) {
      throw new NotFoundException('Table does not exist');
    }

    return table_schema;
  }
  public static parsePluckedFields(
    table: string,
    pluck: string[],
  ): Record<string, any> | null {
    const table_schema = schema[table];

    if (!pluck?.length || !pluck) {
      return null;
    }
    const _plucked_fields = pluck.reduce((acc, field) => {
      if (table_schema[field]) {
        return {
          ...acc,
          [field]: table_schema[field],
        };
      }
      return acc;
    }, {});

    if (Object.keys(_plucked_fields).length === 0) {
      return null;
    }
    return _plucked_fields;
  }
  public static sqliteFilterAnalyzer(
    db,
    table_schema,
    _advance_filters: IAdvanceFilters[],
    organization_id,
    joins?: IJoins[],
  ) {
    let _db = db;
    const get_all_special_conditions_for_where = _advance_filters.filter(
      (filter) => filter.entity === undefined,
    );

    if (joins?.length) {
      const get_all_special_conditions_for_join = _advance_filters.filter(
        (filter) => filter?.entity,
      );
      joins.forEach(({ type, field_relation }) => {
        const { from, to } = field_relation;
        let _from = from;
        let _to = to;

        switch (type) {
          case 'left':
            _db = _db.leftJoin(
              schema[_to.entity],
              ...Utility.constructFilters(
                get_all_special_conditions_for_join,
                schema[_to.entity],
              ),
              eq(
                schema[_from.entity][_from.field],
                schema[_to.entity][_to.field],
              ),
            );
            break;
          case 'self':
            if (!_from.alias) {
              throw new BadRequestException(
                '[from]: Alias are required for self join',
              );
            }
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
        isNotNull(table_schema['organization_id']),
        eq(table_schema['organization_id'], organization_id),
        ...Utility.constructFilters(
          get_all_special_conditions_for_where,
          table_schema,
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
  }) {
    switch (operator) {
      case EOperator.EQUAL:
        return eq(table_schema[field], values[0]);
      case EOperator.NOT_EQUAL:
        return ne(table_schema[field], values[0]);
      case EOperator.GREATER_THAN:
        return gt(table_schema[field], values[0]);
      case EOperator.GREATER_THAN_OR_EQUAL:
        return gte(table_schema[field], values[0]);
      case EOperator.LESS_THAN:
        return lt(table_schema[field], values[0]);
      case EOperator.LESS_THAN_OR_EQUAL:
        return lte(table_schema[field], values[0]);
      case EOperator.IS_NULL:
        return isNull(table_schema[field]);
      case EOperator.IS_NOT_NULL:
        return isNotNull(table_schema[field]);
      case EOperator.CONTAINS:
        return inArray(table_schema[field], values);
      case EOperator.NOT_CONTAINS:
        return notInArray(table_schema[field], values);
      case EOperator.IS_BETWEEN:
        return between(table_schema[field], values[0], values[1]);
      case EOperator.IS_NOT_BETWEEN:
        return notBetween(table_schema[field], values[0], values[1]);
      case EOperator.IS_EMPTY:
        return eq(table_schema[field], '');
      case EOperator.IS_NOT_EMPTY:
        return ne(table_schema[field], '');
      case EOperator.AND:
        return and(...dz_filter_queue);
      case EOperator.OR:
        return or(...dz_filter_queue);
      default:
        return null;
    }
  }
  static validateZodSchema(
    zodObject: { zod: ZodObject<any> | any; params: any }[],
  ) {
    for (const { zod, params } of zodObject) {
      try {
        zod.parse(params);
      } catch (error) {
        throw new Error(`${JSON.stringify(error)}`);
      }
    }
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
    } catch (error) {
      throw new Error('Invalid JSON body');
    }
    return parsed_body;
  }
  static parseBatchRequestBody(body: { records: string }) {
    let parsed_body: { records: Record<any, any>[] } = { records: [] };
    try {
      parsed_body.records = body?.records ? JSON.parse(body.records) : [];
    } catch (error) {
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
    } catch (error) {
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

  public static constructFilters(advance_filters, table_schema): any[] {
    let dz_filter_queue: any[] = [];
    let where_clause_queue: any[] = [];
    let _filter_queue: any[] = [];
    if (advance_filters.length === 1) {
      const [{ operator, field, values, type, entity }] = advance_filters;
      if (type === 'operator') {
        throw new BadRequestException(
          `Invalid filter at index 0. Must be a criteria`,
        );
      }
      const _table_schema = entity ? schema[entity] : table_schema;
      return [
        Utility.evaluateFilter({
          operator,
          table_schema: _table_schema,
          field,
          values,
          dz_filter_queue: [],
        }),
      ];
    }

    advance_filters.forEach((filter, index: number) => {
      const { operator, type = 'criteria', field = '' } = filter;
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
          table_schema,
          field,
          values: filter.values,
          dz_filter_queue,
        }),
      );

      if (dz_filter_queue.length > 2) {
        const [_1, _op, _2]: any = _filter_queue;
        const [_c1, _, _c2]: any = dz_filter_queue;
        const allowed_to_merged = _1.operator ? [_c1, _c2] : [_c2];
        where_clause_queue.push(
          Utility.evaluateFilter({
            operator: _op.operator,
            table_schema,
            field,
            values: filter.values,
            dz_filter_queue: where_clause_queue.concat(allowed_to_merged),
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
}
