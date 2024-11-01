import type { Request } from 'express';
import { REQUEST } from '@nestjs/core';
import {
  Injectable,
  Inject,
  PipeTransform,
  BadRequestException,
} from '@nestjs/common';
import * as schema from '../../../xstate/modules/schemas/drizzle';
import { createInsertSchema } from 'drizzle-zod';
import { ulid } from 'ulid';

@Injectable()
export class SchemaPipe implements PipeTransform {
  constructor(@Inject(REQUEST) protected readonly request: Request) {}

  transform({ meta, data }: { meta?: Record<string, any>; data: any }) {
    const table = this.request.params.table;
    const id = this.request.params.id || ulid();
    if (!table) {
      throw new BadRequestException('Table does not exist');
    }

    const schemaTable = schema[table];

    if (!schemaTable) {
      throw new BadRequestException('Schema Table does not exist');
    }

    if (!data) {
      throw new BadRequestException('Data is required in Body');
    }

    return { schema: createInsertSchema(schemaTable), data, id, meta };
  }
}
