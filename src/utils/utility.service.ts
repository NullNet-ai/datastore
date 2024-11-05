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
    outer_logic_operator: 'AND' | 'OR ',
    advance_filters: IAdvanceFilters[],
  ) {
    let _db = db;
    let where_classes: any = [];
    let outer_lo: any = outer_logic_operator === 'AND' ? and : or;

    advance_filters.forEach((filters: IAdvanceFilters) => {
      const { field, operator, values = [] } = filters;

      switch (operator) {
        case EOperator.EQUAL:
          where_classes.push(eq(table_schema[field], values[0]));
          break;
        case EOperator.NOT_EQUAL:
          where_classes.push(ne(table_schema[field], values[0]));
          break;
        case EOperator.GREATER_THAN:
          where_classes.push(gt(table_schema[field], values[0]));
          break;
        case EOperator.GREATER_THAN_OR_EQUAL:
          where_classes.push(gte(table_schema[field], values[0]));
          break;
        case EOperator.LESS_THAN:
          where_classes.push(lt(table_schema[field], values[0]));
          break;
        case EOperator.LESS_THAN_OR_EQUAL:
          where_classes.push(lte(table_schema[field], values[0]));
          break;
        case EOperator.IS_NULL:
          where_classes.push(isNull(table_schema[field]));
          break;
        case EOperator.IS_NOT_NULL:
          where_classes.push(isNotNull(table_schema[field]));
          break;
        case EOperator.CONTAINS:
          where_classes.push(inArray(table_schema[field], values));
          break;
        case EOperator.NOT_CONTAINS:
          where_classes.push(notInArray(table_schema[field], values));
          break;
        case EOperator.IS_BETWEEN:
          where_classes.push(
            between(table_schema[field], values[0], values[1]),
          );
          break;
        case EOperator.IS_NOT_BETWEEN:
          where_classes.push(
            notBetween(table_schema[field], values[0], values[1]),
          );
          break;
        case EOperator.IS_EMPTY:
          where_classes.push(eq(table_schema[field], ''));
          break;
        case EOperator.IS_NOT_EMPTY:
          where_classes.push(ne(table_schema[field], ''));
          break;
        default:
          throw new BadRequestException('Invalid Operator');
      }
    });
    return _db.where(outer_lo(...where_classes));
  }
}
