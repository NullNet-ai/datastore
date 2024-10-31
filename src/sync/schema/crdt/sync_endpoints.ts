import { sqliteTable, text } from 'drizzle-orm/sqlite-core';

export const sync_endpoints = sqliteTable('sync_endpoints', {
  id: text().primaryKey(),
  name: text(),
  url: text(),
  group_id: text(),
  username: text(),
  password: text(),
  status: text(),
});
