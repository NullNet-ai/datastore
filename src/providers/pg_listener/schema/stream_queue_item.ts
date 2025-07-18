import {
  pgTable,
  primaryKey,
  uuid,
  timestamp,
  jsonb,
  text,
} from 'drizzle-orm/pg-core';
import { table as stream_queue } from './stream_queue';

const config = (table) => ({
  pk: primaryKey({ columns: [table.id] }),
});

export const table = pgTable(
  'stream_queue_item',
  {
    id: uuid('id'),
    queue_name: text('queue_name')
      .notNull()
      .references(() => stream_queue.name, { onDelete: 'cascade' }),
    content: jsonb('content').notNull(), // JSON content
    timestamp: timestamp('timestamp', {
      withTimezone: true,
      mode: 'string',
    }).defaultNow(),
  },
  config,
);
