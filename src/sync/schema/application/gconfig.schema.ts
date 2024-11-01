import { sqliteTable, text, integer } from 'drizzle-orm/sqlite-core';

export const class_type = sqliteTable(`class_type`, {
  id: text().primaryKey(),
  type: text(),
  value: text(),
  tombstone: integer(),
  status: text().default('Active'),
  created_date: text(),
  created_time: text(),
  updated_date: text(),
  updated_time: text(),
});
