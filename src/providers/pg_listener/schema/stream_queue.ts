import {
  pgTable,
  text,
  primaryKey,
  timestamp,
} from 'drizzle-orm/pg-core';

const config = (table) => ({
  pk: primaryKey({ columns: [table.id] }),
});

export const table = pgTable(
  'stream_queue',
  {
    id: text('id'), // Primary key
    name: text('name').notNull().unique(), // Queue name
    created_at: timestamp('created_at', {
      withTimezone: true,
      mode: 'string',
    }).defaultNow(),
    last_accessed: timestamp('last_accessed'),
  },
  config,
);
