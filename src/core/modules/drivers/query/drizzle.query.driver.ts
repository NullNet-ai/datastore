import {
  BadRequestException,
  Injectable,
  NotFoundException,
} from '@nestjs/common';
import { eq, ne, and, asc, desc, SQL } from 'drizzle-orm';
import * as schema from '../../../../xstate/modules/schemas/drizzle';
import { DrizzleService } from '../../../modules/drizzle/drizzle.service';
import { QueryDriver } from './query.driver';
import { QueryDto } from './dto/query.dto';

/*
  @class DrizzleQueryDriver
  @implements QueryDriver
  @description Implements the QueryDriver interface for the Drizzle supoorted databases
*/
@Injectable()
export class DrizzleQueryDriver implements QueryDriver {
  private db;
  constructor(private readonly drizzleService: DrizzleService) {
    this.db = this.drizzleService.getClient();
  }

  private parsePluckedFields(
    table: string,
    pluck?: string,
  ): Record<string, any> | null {
    const table_schema = schema[table];

    if (pluck === '' || !pluck) {
      return null;
    }
    const _plucked_fields = pluck.split(',').reduce((acc, field) => {
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
  async get(table: string, id: string, query: QueryDto): Promise<any | null> {
    const table_schema = schema[table];

    if (!table_schema) {
      throw new NotFoundException('Table not found');
    }

    const { pluck = '' } = query;
    const _plucked_fields = this.parsePluckedFields(table, pluck);

    const selection = _plucked_fields === null ? undefined : _plucked_fields;
    const [row = null] = await this.db
      .select(selection)
      .from(table_schema)
      .where(and(eq(table_schema.id, id), ne(table_schema.tombstone, 1)));

    return row;
  }

  async find(
    table: string,
    query: QueryDto,
  ): Promise<{ query: QueryDto; data: Array<any> }> {
    const table_schema = schema[table];
    if (!table_schema) {
      throw new NotFoundException('Table not found');
    }
    const {
      order_direction = 'asc',
      order_by = 'id',
      limit = '100',
      offset = '0',
      pluck = '',
      ..._query
    } = query;

    const _plucked_fields = this.parsePluckedFields(table, pluck);

    const where_clause =
      Object.keys(_query).length > 0
        ? Object.keys(_query).reduce(
            (acc, key) => {
              const column = table_schema[key];
              if (!column) {
                throw new BadRequestException(
                  `Column ${key} not found in table ${table}`,
                );
              }
              return [...acc, eq(table_schema[key], query[key])];
            },
            [eq(table_schema.tombstone, 0)],
          )
        : eq(table_schema.tombstone, 0);

    const selections = _plucked_fields === null ? undefined : _plucked_fields;

    const result = await this.db
      .select(selections)
      .from(table_schema)
      .where(where_clause as SQL<unknown>)
      .orderBy(
        order_direction === 'asc'
          ? asc(table_schema[order_by])
          : desc(table_schema[order_by]),
      )
      .offset(Number(offset))
      .limit(Number(limit));
    return {
      query: {
        order_direction,
        order_by,
        limit,
        offset,
        pluck,
        ..._query,
      },
      data: result,
    };
  }
}
