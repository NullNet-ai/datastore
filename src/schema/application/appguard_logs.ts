import { pgTable, text, timestamp } from 'drizzle-orm/pg-core';
import {
  getConfigDefaults,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';
import { primaryKey } from 'drizzle-orm/pg-core';
const config = (table) => ({
  pk: primaryKey({ columns: [table.id, table.timestamp] }),
  ...getConfigDefaults.defaultIndexes('appguard_logs', table),
  // uniq_id: unique().on(table.id, table.timestamp),
});
export const table = pgTable(
  'appguard_logs',
  {
    ...system_fields,
    id: text('id'), // Primary key
    timestamp: timestamp('timestamp', { withTimezone: true }), // NOT NULL timestamp
    level: text('level'),
    message: text('message'),

  },
  config,
);
