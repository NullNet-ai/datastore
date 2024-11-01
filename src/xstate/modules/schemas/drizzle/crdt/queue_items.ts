import { sqliteTable, text, integer } from 'drizzle-orm/sqlite-core';

export const queue_items = sqliteTable('queue_items', {
  id: text().primaryKey(),
  order: integer().notNull(),
  queue_id: text().notNull(),
  value: text().notNull(),
});
