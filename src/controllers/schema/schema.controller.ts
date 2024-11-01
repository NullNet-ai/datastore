import { Controller, Get, Req, Res } from '@nestjs/common';
import { Request, Response } from 'express';
import { SchemaService } from '../../providers/schema/schema.service';

@Controller('schema')
export class SchemaController {
  constructor(private schemaService: SchemaService) {}
  @Get('/')
  async getSchema(@Res() _res: Response, @Req() _req: Request) {
    return this.schemaService.getSchema(_res, _req);
  }
}
