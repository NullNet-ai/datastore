import type { Request } from 'express';
import { REQUEST } from '@nestjs/core';
import {
  Injectable,
  Inject,
  PipeTransform,
  BadRequestException,
} from '@nestjs/common';
import * as schema from '../../schema';

@Injectable()
export class TablePipe implements PipeTransform {
  constructor(@Inject(REQUEST) protected readonly request: Request) {}

  transform(table: string) {
    const schemaTable = schema[table];
    if (!table || !schemaTable || !table.includes('config')) {
      throw new BadRequestException('Table does not exist');
    }

    return schema[table];
  }
}
