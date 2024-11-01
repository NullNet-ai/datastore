import { sqliteTable, text } from 'drizzle-orm/sqlite-core';

export const merkles = sqliteTable(`crdt_merkles`, {
  group_id: text().notNull().unique().primaryKey(),
  timestamp: text().notNull(),
  merkle: text().notNull(),
});
