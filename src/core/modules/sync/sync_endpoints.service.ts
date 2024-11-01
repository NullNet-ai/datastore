import { BadRequestException, Injectable } from '@nestjs/common';
import { DrizzleService } from '../drizzle/drizzle.service';
import { PostOpts } from '../drivers/transport/transport.driver';
import * as schema from '../../../xstate/modules/schemas/drizzle';
import { eq } from 'drizzle-orm';

@Injectable()
export class SyncEndpointsService {
  constructor(private readonly drizzleService: DrizzleService) {}

  async getAllSyncEndpoints(): Promise<PostOpts[]> {
    const db = this.drizzleService.getClient();
    const result = (await db
      .select()
      .from(schema.sync_endpoints)) as PostOpts[];

    if (!result) throw new BadRequestException('No Sync Endpoints found');
    return result;
  }

  async createEndpoint(
    endpoint: typeof schema.sync_endpoints.$inferInsert,
  ): Promise<{ message: 'ok' }> {
    const db = await this.drizzleService.getClient();
    console.log('@schema.sync_endpoints', endpoint);
    await db.insert(schema.sync_endpoints).values(endpoint).onConflictDoUpdate({
      target: schema.sync_endpoints.id,
      set: endpoint,
    });
    return {
      message: 'ok',
    };
  }

  async getSyncEndpoints(): Promise<Array<PostOpts>> {
    const db = await this.drizzleService.getClient();

    const endpoints = (await db
      .select({
        url: schema.sync_endpoints.url,
        username: schema.sync_endpoints.username,
        password: schema.sync_endpoints.password,
      })
      .from(schema.sync_endpoints)
      .where(eq(schema.sync_endpoints.status, 'Active'))) as PostOpts[];

    return endpoints;
  }
}
