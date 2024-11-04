import { Injectable } from '@nestjs/common';
import { Response, Request } from 'express';
import { Machine } from '@dna-platform/common';
@Injectable()
export class StoreMutationDriver {
  @Machine('create')
  async create(_res: Response, _req: Request) {}

  @Machine('update')
  async update(_res: Response, _req: Request) {}

  @Machine('delete')
  async delete(_res: Response, _req: Request) {}
}

@Injectable()
export class StoreQueryDriver {
  @Machine('get')
  async get(_res: Response, _req: Request) {}

  @Machine('find')
  async find(_res: Response, _req: Request) {}
}
