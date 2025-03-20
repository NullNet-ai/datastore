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
    record_id: uuid('record_id'),
    created_date: timestamp('created_date').default(sql`now()`),
    table: text('table'),
    prefix: text('prefix'),
    error: text('error'),
  },
  config,
);
