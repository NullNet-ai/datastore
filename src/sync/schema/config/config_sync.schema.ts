import { integer, sqliteTable, text } from 'drizzle-orm/sqlite-core';

export const config_sync = sqliteTable(`config_sync`, {
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
