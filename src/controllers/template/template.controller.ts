import { Controller, Get, Req, Res } from '@nestjs/common';
import { Request, Response } from 'express';
import { TemplateService } from '../../providers/template/template.service';
/**
 * GET /template
 *
 * Handles the template route.
 *
 * @param _res - The response object.
 * @param _req - The request object.
 */
@Controller('template')
export class TemplateController {
  constructor(private templateService: TemplateService) {}
  @Get('/')
  async template(@Res() _res: Response, @Req() _req: Request) {
    return this.templateService.getTemplate(_res, _req);
  }
}
