import { inet, integer, pgTable, text, timestamp } from 'drizzle-orm/pg-core';
import {
  getConfigDefaults,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';
import { primaryKey } from 'drizzle-orm/pg-core';
const config = (table) => ({
  pk: primaryKey({ columns: [table.id] }),
  ...getConfigDefaults.defaultIndexes('tcp_connections', table),
});
export const table = pgTable(
  'tcp_connections',
  {
    ...system_fields,
    id: text('id'), // Primary key
    timestamp: timestamp('timestamp', { withTimezone: true }), // NOT NULL timestamp
    source: inet('source'),
    sport: integer('sport'),
    dest: inet('dest'),
    dport: integer('dport'),
    proto: text('proto'),
  },
  config,
);
