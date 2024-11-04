import { BadRequestException, NotFoundException } from '@nestjs/common';
import * as schema from '../schema';
import { createInsertSchema } from 'drizzle-zod';
import { ulid } from 'ulid';
import { ZodValidationException } from '@dna-platform/common';
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
}
