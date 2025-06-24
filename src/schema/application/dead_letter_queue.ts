import {
  pgTable,
  primaryKey,
  text,
  timestamp,
  // unique,
} from 'drizzle-orm/pg-core';
import { sql } from 'drizzle-orm';

const config = (table) => ({
  pk: primaryKey({ columns: [table.id] }),
});

export const table = pgTable(
  'dead_letter_queue',
  {
    id: text('id'), // Primary key
    record_id: text('record_id'),
    created_date: timestamp('created_date').default(sql`now()`),
    table: text('table'),
    prefix: text('prefix'),
    error: text('error'),
  },
  config,
);
