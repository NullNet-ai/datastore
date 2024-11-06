import {
  Controller,
  Delete,
  Get,
  Inject,
  Patch,
  Post,
  Req,
  Res,
  UseGuards,
} from '@nestjs/common';
import { Request, Response } from 'express';
import {
  StoreMutationDriver,
  StoreQueryDriver,
} from '../../providers/store/store.service';
import { AuthGuard } from '@dna-platform/crdt-lww';
@UseGuards(AuthGuard)
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

  @Post('/:table/filter')
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

@Controller('/api/token')
export class TokenController {
  constructor(private storeMutation: StoreMutationDriver) {}
  @Post('/verify')
  async create(@Res() _res: Response, @Req() _req: Request) {
    return this.storeMutation.verify(_res, _req);
  }
}
