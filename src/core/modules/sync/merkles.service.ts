import { Injectable } from '@nestjs/common';
import * as schema from '../../../xstate/modules/schemas/drizzle';
import { eq } from 'drizzle-orm';
import { DrizzleService } from '../drizzle/drizzle.service';
@Injectable()
export class MerklesService {
  constructor(private readonly drizzleService: DrizzleService) {}

  async startTransaction<T = any>(fn: (params: any) => Promise<T>): Promise<T> {
    const db = this.drizzleService.getClient();
    return db.transaction(fn as any);
  }

  async getMerklesByGroupId(
    group_id: string,
    db = this.drizzleService.getClient(),
  ) {
    const [row] = await db
      .select()
      .from(schema.merkles)
      .where(eq(schema.merkles.group_id, group_id));

    if (!row) {
      return null;
    }

    return {
      group_id: row.group_id,
      timestamp: JSON.parse(row.timestamp),
      merkle: JSON.parse(row.merkle),
    };
  }
  async setMerklesByGroupId(
    group_id: string,
    merkle: string,
    timestamp: string,
    db = this.drizzleService.getClient(),
  ) {
    const _merkle = JSON.stringify(merkle);
    const _timestamp = JSON.stringify(timestamp);

    await db
      .insert(schema.merkles)
      .values({
        group_id,
        timestamp: _timestamp,
        merkle: _merkle,
      })
      .onConflictDoUpdate({
        target: [schema.merkles.group_id],
        set: {
          timestamp: _timestamp,
          merkle: _merkle,
        },
      });
  }
}
