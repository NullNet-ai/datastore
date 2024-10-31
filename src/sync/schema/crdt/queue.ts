import { sqliteTable, text, integer } from 'drizzle-orm/sqlite-core';

export const queues = sqliteTable('queue', {
  id: text().primaryKey(),
  name: text().notNull(),
  count: integer().notNull(),
  size: integer().notNull(),
});
