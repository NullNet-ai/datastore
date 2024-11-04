import {
  Controller,
  Delete,
  Get,
  Inject,
  Patch,
  Post,
  Req,
  Res,
} from '@nestjs/common';
import { Request, Response } from 'express';
import {
  StoreMutationDriver,
  StoreQueryDriver,
} from '../../providers/store/store.service';
@Controller('/api/store')
export class StoreController {
  constructor(
    @Inject('QueryDriverInterface')
    private storeQuery: StoreQueryDriver,
    private storeMutation: StoreMutationDriver,
  ) {}
  @Get('/:table/:id')
  async get(@Res() _res: Response, @Req() _req: Request) {
    return this.storeQuery.get(_res, _req);
  }

  @Get('/:table')
  async find(@Res() _res: Response, @Req() _req: Request) {
    return this.storeQuery.find(_res, _req);
  }

  @Post('/:table')
  async create(@Res() _res: Response, @Req() _req: Request) {
    return this.storeMutation.create(_res, _req);
  }

  @Patch('/:table/:id')
  async update(@Res() _res: Response, @Req() _req: Request) {
    return this.storeMutation.update(_res, _req);
  }

  @Delete('/:table/:id')
  async delete(@Res() _res: Response, @Req() _req: Request) {
    return this.storeMutation.delete(_res, _req);
  }
}
