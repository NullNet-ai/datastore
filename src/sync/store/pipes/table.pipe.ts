import type { Request } from 'express';
import { REQUEST } from '@nestjs/core';
import {
  Injectable,
  Inject,
  PipeTransform,
  NotFoundException,
} from '@nestjs/common';
import * as schema from '../../schema';

@Injectable()
export class TablePipe implements PipeTransform {
  constructor(@Inject(REQUEST) protected readonly request: Request) {}

  transform(table: string) {
    const schemaTable = schema[table];

    if (
      !table ||
      !schemaTable ||
      table === 'config_sync' ||
      table.includes('crdt')
    ) {
      throw new NotFoundException('Table does not exist');
    }

    return table;
  }
}
