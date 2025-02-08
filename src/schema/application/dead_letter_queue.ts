import {
  pgTable,
  primaryKey,
  text,
  timestamp,
  // unique,
} from 'drizzle-orm/pg-core';
import { uuid } from 'drizzle-orm/pg-core';
import { sql } from 'drizzle-orm';

const config = (table) => ({
  pk: primaryKey({ columns: [table.id] }),
  // uniq_id: unique().on(table.id, table.timestamp),
});

export const table = pgTable(
  'dead_letter_queue',
  {
    id: uuid('id'), // Primary key
    record_id: uuid('record_id').notNull(),
    created_date: timestamp('created_date').default(sql`now()`),
    table: text('table').notNull(),
    prefix: text('prefix').notNull(),
    error: text('error').notNull(),
  },
  config,
);
