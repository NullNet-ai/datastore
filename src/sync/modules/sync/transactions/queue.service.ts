import { Injectable } from '@nestjs/common';
import * as schema from '../../../schema';
import { v4 as uuidv4 } from 'uuid';
import { and, eq } from 'drizzle-orm';
import { DrizzleService } from '../../../modules/drizzle/drizzle.service';

@Injectable()
export class QueueService {
  constructor(private readonly drizzleService: DrizzleService) {}

  async onModuleInit() {
    const db = this.drizzleService.getClient();

    await db.transaction(async (tx) => {
      return tx
        .insert(schema.queues)
        .values({
          id: '1',
          name: 'test',
          size: 0,
          count: 0,
        })
        .onConflictDoNothing();
    });
  }

  async size(queue_name = 'test') {
    const db = this.drizzleService.getClient();

    const [queue = null] = await db
      .select()
      .from(schema.queues)
      .where(eq(schema.queues.name, queue_name));

    if (!queue) {
      throw new Error('Queue not found');
    }

    return queue.size - queue.count;
  }

  async enqueue(item: any, queue_name = 'test') {
    const db = this.drizzleService.getClient();

    return db.transaction(async (tx) => {
      const [queue = null] = await tx
        .select()
        .from(schema.queues)
        .where(eq(schema.queues.name, queue_name));

      if (!queue) {
        throw new Error('Queue not found');
      }

      const queue_item: typeof schema.queue_items.$inferInsert = {
        id: uuidv4(),
        order: queue.size + 1,
        queue_id: queue.id,
        value: JSON.stringify(item),
      };

      await tx.insert(schema.queue_items).values(queue_item);
      await tx
        .update(schema.queues)
        .set({ size: queue.size + 1 })
        .where(eq(schema.queues.id, queue.id));

      return queue_item.order;
    });
  }

  async dequeue(queue_name = 'test') {
    const db = this.drizzleService.getClient();

    return db.transaction(async (tx) => {
      const [queue = null] = await tx
        .select()
        .from(schema.queues)
        .where(eq(schema.queues.name, queue_name));

      if (!queue) {
        throw new Error('Queue not found');
      }

      if (queue.count === queue.size) {
        return null;
      }

      const [queue_item = null] = await tx
        .select()
        .from(schema.queue_items)
        .where(
          and(
            eq(schema.queue_items.queue_id, queue.id),
            eq(schema.queue_items.order, queue.count),
          ),
        )
        .orderBy(schema.queue_items.order)
        .limit(1);

      if (!queue_item) {
        return null;
      }

      return JSON.parse(queue_item.value);
    });
  }
  async ack(queue_name = 'test') {
    const db = this.drizzleService.getClient();

    return db.transaction(async (tx) => {
      const [queue = null] = await tx
        .select()
        .from(schema.queues)
        .where(eq(schema.queues.name, queue_name));

      if (!queue) {
        throw new Error('Queue not found');
      }

      await tx
        .update(schema.queues)
        .set({ count: queue.count + 1 })
        .where(eq(schema.queues.id, queue.id));
    });
  }
}
