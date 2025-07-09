import { Injectable } from '@nestjs/common';
import { table as streamQueue } from '../schema/stream_queue';
import { table as streamQueueItem } from '../schema/stream_queue_item';
import { asc, eq } from 'drizzle-orm';
import { v4 as uuidv4 } from 'uuid';
import { DrizzleService } from '@dna-platform/crdt-lww-postgres';

@Injectable()
export class QueueService {
  private db;
  constructor(private readonly drizzle: DrizzleService) {
    this.db = this.drizzle.getClient();
  }

  async queueExists(queueName: string): Promise<boolean> {
    const result = await this.db
      .select({ name: streamQueue.name })
      .from(streamQueue)
      .where(eq(streamQueue.name, queueName))
      .limit(1);

    return result.length > 0;
  }

  async getQueueByName(queueName: string) {
    const result = await this.db
      .select()
      .from(streamQueue)
      .where(eq(streamQueue.name, queueName))
      .limit(1);

    return result[0];
  }

  async createQueue(queueName: string) {
    const result = await this.db
      .insert(streamQueue)
      .values({
        id: uuidv4(),
        name: queueName,
      })
      .onConflictDoNothing()
      .returning();

    return result[0];
  }

  async getOrCreateQueue(queueName: string) {
    return this.createQueue(queueName);
  }

  async deleteQueueByName(queueName: string) {
    return this.db
      .delete(streamQueue)
      .where(eq(streamQueue.name, queueName))
      .returning();
  }

  async insertToQueue(queueName: string, content: Record<string, any>) {
    await this.getOrCreateQueue(queueName);
    // Extract the caller function from the stack trace
    const result = await this.db
      .insert(streamQueueItem)
      .values({
        id: uuidv4(),
        queue_name: queueName,
        timestamp: new Date().toISOString(),
        content,
      })
      .returning();

    console.log('inserted to queues');
    return result[0];
  }

  async deleteFromQueue(itemId: string) {
    return this.db
      .delete(streamQueueItem)
      .where(eq(streamQueueItem.id, itemId))
      .returning();
  }

  async getQueueItems(queueName: string, limit = 100) {
    const date = new Date();
    const [queueItems] = await Promise.all([
      this.db
        .select()
        .from(streamQueueItem)
        .where(eq(streamQueueItem.queue_name, queueName))
        .orderBy(asc(streamQueueItem.timestamp))
        .limit(limit),
      this.db
        .update(streamQueue)
        .set({ last_accessed: date })
        .where(eq(streamQueue.name, queueName)),
    ]);
    return queueItems;
  }

  async clearQueue(queueName: string) {
    return this.db
      .delete(streamQueueItem)
      .where(eq(streamQueueItem.queue_name, queueName))
      .returning();
  }
}
