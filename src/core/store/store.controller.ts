import {
  Body,
  Controller,
  Delete,
  Get,
  Inject,
  NotFoundException,
  Param,
  Patch,
  Post,
  Query,
  UseGuards,
} from '@nestjs/common';
import { StoreService } from './store.service';
import { ResponseObject } from './classes/ResponseObject.class';
import { SchemaPipe } from './pipes/schema.pipe';
import { CreateParsePipe } from './pipes/create-parse.pipe';
import { UpdateParsePipe } from './pipes/update-parse.pipe';
import { TablePipe } from './pipes/table.pipe';
import { ExistsPipe } from './pipes/exists.pipe';
import { QueryDriver } from '../modules/drivers/query/query.driver';
import { QueryDto } from '../modules/drivers/query/dto/query.dto';
import { QueryDriverInterface } from '../modules/drivers/query/enums';
import { AuthGuard } from '../modules/auth/auth.guard';

@UseGuards(AuthGuard)
@Controller('api/store')
export class StoreController {
  @Inject(QueryDriverInterface) private readonly queryService: QueryDriver;
  constructor(private readonly storeService: StoreService) {}

  @Get(':table')
  async getAll(
    @Param('table', TablePipe) table,
    @Query() query: QueryDto,
  ): Promise<ResponseObject> {
    const { query: r_query, data } = await this.queryService.find(table, query);

    return {
      status: 'ok',
      query: r_query,
      data,
    };
  }

  @Get(':table/:id')
  async getById(
    @Param('table', TablePipe) table,
    @Param('id') id,
    @Query() query: QueryDto,
  ): Promise<ResponseObject> {
    const data = await this.queryService.get(table, id, query);

    if (data === null) {
      throw new NotFoundException(`Table:${table} id:${id} not found`);
    }

    return {
      status: 'ok',
      params: { query },
      data: [data],
    };
  }

  @Post(':table')
  async create(
    @Param('table', TablePipe) table,
    @Body(SchemaPipe, CreateParsePipe) body,
  ): Promise<ResponseObject> {
    const result = await this.storeService.insert(table, body);
    return {
      status: 'ok',
      params: body,
      data: [result],
    };
  }

  @Patch(':table/:id')
  async update(
    @Param('table', TablePipe) table,
    @Param('id', ExistsPipe) id,
    @Body(SchemaPipe, UpdateParsePipe) body,
  ): Promise<ResponseObject> {
    const result = await this.storeService.update(table, body, id);
    return {
      status: 'ok',
      params: body,
      data: [result],
    };
  }

  @Delete(':table/:id')
  async delete(
    @Param('table', TablePipe) table,
    @Param('id', ExistsPipe) id,
  ): Promise<ResponseObject> {
    const result = await this.storeService.delete(table, id);

    return {
      status: 'ok',
      params: {
        id,
      },
      data: result,
    };
  }
}
