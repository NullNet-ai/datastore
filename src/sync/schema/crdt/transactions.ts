import { sqliteTable, text, integer } from 'drizzle-orm/sqlite-core';

export const transactions = sqliteTable('transactions', {
  id: text().primaryKey(),
  timestamp: text().notNull(),
  status: text().notNull().default('Active'),
  expiry: integer(),
});
