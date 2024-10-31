import { Body, Controller, Get, Param, Post, UseGuards } from '@nestjs/common';
import { DrizzleService } from '../modules/drizzle/drizzle.service';
import { TablePipe } from './pipes/table.pipe';
import { SchemaPipe } from './pipes/schema.pipe';
import { CreateParsePipe } from './pipes/create-parse.pipe';
import { AuthGuard } from '../modules/auth/auth.guard';

@UseGuards(AuthGuard)
@Controller('config')
export class ConfigController {
  constructor(private readonly drizzleService: DrizzleService) {}

  @Get('/:table')
  async getAllSync(@Param('table', TablePipe) table) {
    const db = this.drizzleService.getClient();
    const results = await db.select().from(table);
    return {
      status: 'ok',
      data: results,
    };
  }

  @Post('/:table/:id')
  async insertSync(
    @Param('table', TablePipe) table,
    @Param('id') id,
    @Body(SchemaPipe, CreateParsePipe) body,
  ) {
    const db = this.drizzleService.getClient();
    await db
      .insert(table)
      .values(body)
      .onConflictDoUpdate({
        target: [table.id],
        set: body,
      });

    return {
      status: 'ok',
      params: {
        id,
        body,
      },
      data: [body],
    };
  }
}
