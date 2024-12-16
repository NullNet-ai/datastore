import { Injectable } from '@nestjs/common';
import { Response, Request, Express } from 'express';
import { Machine } from '@dna-platform/common';
@Injectable()
export class StoreMutationDriver {
  @Machine('create')
  async create(_res: Response, _req: Request) {}

  @Machine('update')
  async update(_res: Response, _req: Request) {}

  @Machine('delete')
  async delete(_res: Response, _req: Request) {}

  @Machine('verify')
  async verify(_res: Response, _req: Request) {}

  @Machine('batchInsert')
  async batchInsert(_res: Response, _req: Request) {}

  @Machine('upload')
  async upload(_res: Response, _req: Request, _file: Express.Multer.File) {}

  @Machine('uploads')
  async uploads(
    _res: Response,
    _req: Request,
    _files: Array<Express.Multer.File>,
  ) {}

  @Machine('download')
  async download(_res: Response, _req: Request) {}

  @Machine('transactions')
  async transactions(_res: Response, _req: Request) {}

  @Machine('createHypertables')
  async createHypertables(_res: Response, _req: Request) {}
}

@Injectable()
export class StoreQueryDriver {
  @Machine('get')
  async get(_res: Response, _req: Request) {}

  @Machine('aggregationFilter')
  async aggregationFilter(_res: Response, _req: Request) {}

  @Machine('find')
  async find(_res: Response, _req: Request) {}

  @Machine('getFileById')
  async getFileById(_res: Response, _req: Request) {}

  @Machine('count')
  async getCount(_res: Response, _req: Request) {}
}
