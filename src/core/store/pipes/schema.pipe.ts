import type { Request } from 'express';
import { REQUEST } from '@nestjs/core';
import {
  Injectable,
  Inject,
  PipeTransform,
  BadRequestException,
  NotFoundException,
} from '@nestjs/common';
import * as schema from '../../../xstate/modules/schemas/drizzle';
import { createInsertSchema } from 'drizzle-zod';

@Injectable()
export class SchemaPipe implements PipeTransform {
  constructor(@Inject(REQUEST) protected readonly request: Request) {}

  transform({ meta, data }: { meta?: Record<string, any>; data: any }) {
    const table = this.request.params.table;
    if (!table) {
      throw new BadRequestException('Table does not exist');
    }
    const schemaTable = schema[table];

    if (
      !table ||
      !schemaTable ||
      table === 'config_sync' ||
      table.includes('crdt')
    ) {
      throw new NotFoundException('Table does not exist');
    }

    if (!data) {
      throw new BadRequestException('Data is required in Body');
    }

    return { schema: createInsertSchema(schemaTable), data, meta };
  }
}
