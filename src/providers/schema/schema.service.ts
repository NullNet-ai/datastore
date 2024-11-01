import { Injectable } from '@nestjs/common';
import { Response, Request } from 'express';
import { Machine } from '@dna-platform/common';
@Injectable()
export class SchemaService {
  @Machine('schema')
  getSchema(_res: Response, _req: Request) {}
}
