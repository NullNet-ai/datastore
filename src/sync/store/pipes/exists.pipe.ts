import type { Request } from 'express';
import { REQUEST } from '@nestjs/core';
import {
  Injectable,
  Inject,
  PipeTransform,
  BadRequestException,
} from '@nestjs/common';
import * as schema from '../../schema';
import { QueryDriver } from '../../modules/drivers/query/query.driver';
import { QueryDriverInterface } from '../../modules/drivers/query/enums';

@Injectable()
export class ExistsPipe implements PipeTransform {
  constructor(
    @Inject(REQUEST) protected readonly request: Request,
    @Inject(QueryDriverInterface) private readonly queryService: QueryDriver,
  ) {}

  async transform(id: string) {
    const table = this.request.params.table;

    if (!table) {
      throw new BadRequestException('Table does not exist');
    }

    const schemaTable = schema[table];

    if (!schemaTable) {
      throw new BadRequestException('Schema Table does not exist');
    }

    const row = await this.queryService.get(table, id);

    if (!row) {
      throw new BadRequestException(`${table} with id:${id} does not exist`);
    }
    return id;
  }
}
