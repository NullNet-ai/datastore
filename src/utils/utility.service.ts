import { BadRequestException, NotFoundException } from '@nestjs/common';
import * as schema from '../schema';
import { createInsertSchema } from 'drizzle-zod';
import { ulid } from 'ulid';
import { ZodValidationException } from '@dna-platform/common';
import {
  EOperator,
  IAdvanceFilters,
} from '../xstate/modules/schemas/find/find.schema';
import {
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
            timestamp: date.toISOString(),
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
    advance_filters: IAdvanceFilters[],
  ) {
    return db.where(
      and(
        eq(table_schema['tombstone'], 0),
        isNull(table_schema['organization_id']),
        ...Utility.constructFilters(advance_filters, table_schema),
      ),
    );
  }

  public static evaluateFilter({
    operator,
    table_schema,
    field,
    values,
    filter_stack,
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
        return and(...filter_stack);
      case EOperator.OR:
        return or(...filter_stack);
      default:
        return null;
    }
  }

  public static constructFilters(advance_filters, table_schema): any[] {
    let filter_stack: any[] = [];
    let where_clause_stack: any[] = [];

    advance_filters.forEach((filter, index: number) => {
      const { operator, type = 'criteria', field = '' } = filter;
      if (type === 'criteria') {
        filter_stack.push(
          Utility.evaluateFilter({
            operator,
            table_schema,
            field,
            values: filter.values,
            filter_stack,
          }),
        );
      } else {
        if (filter_stack.length > 0) {
          where_clause_stack.push(
            Utility.evaluateFilter({
              operator,
              table_schema,
              field,
              values: filter.values,
              filter_stack: where_clause_stack.concat(filter_stack),
            }),
          );
          if (where_clause_stack?.length > 1) where_clause_stack.shift();
        }

        filter_stack = [];
      }

      // last iteration
      if (index === advance_filters.length - 1) {
        where_clause_stack.push(
          Utility.evaluateFilter({
            operator,
            table_schema,
            field,
            values: filter.values,
            filter_stack: [where_clause_stack[0]],
          }),
        );
        where_clause_stack.shift();
        if (type !== 'operator') {
          throw new Error(
            'Invalid Advance Filter. Please add an Operator [And | OR] at the end of the filter list',
          );
        }
      }
    });
    return where_clause_stack;
  }
}
