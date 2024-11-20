import { BadRequestException, NotFoundException } from '@nestjs/common';
import * as schema from '../schema';
import { createInsertSchema } from 'drizzle-zod';
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
