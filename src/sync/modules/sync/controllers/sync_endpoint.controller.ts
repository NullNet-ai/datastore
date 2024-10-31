import { Body, Controller, Get, Post, Put } from '@nestjs/common';
import { SyncEndpointsService } from '../sync_endpoints.service';
import { PostOpts } from '../../../modules/drivers/transport/transport.driver';
import { sync_endpoints } from '../../../schema';
export type ResponsePackage = {
  data: Array<PostOpts>;
};
@Controller('sync_endpoints')
export class SyncEndpointsController {
  constructor(private readonly syncEndpointsService: SyncEndpointsService) {}

  @Get()
  async getSyncEndpoints(): Promise<ResponsePackage> {
    const data = await this.syncEndpointsService.getAllSyncEndpoints();
    return {
      data,
    };
  }

  @Post()
  @Put()
  async createEndpoint(
    @Body('endpoint') endpoint: typeof sync_endpoints.$inferInsert,
  ): Promise<{ message: 'ok' }> {
    await this.syncEndpointsService.createEndpoint(endpoint);
    return {
      message: 'ok',
    };
  }
}
