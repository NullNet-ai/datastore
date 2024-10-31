import { sqliteTable, text, primaryKey } from 'drizzle-orm/sqlite-core';

export const messages = sqliteTable(
  'crdt_messages',
  {
    database: text(),
    dataset: text().notNull(),
    group_id: text().notNull(),
    timestamp: text().notNull(),
    row: text().notNull(),
    column: text().notNull(),
    client_id: text().notNull(),
    value: text().notNull(),
  },
  (table) => {
    return {
      pk: primaryKey({
        columns: [table.timestamp, table.group_id, table.row, table.column],
      }),
    };
  },
);
