import { inet, pgTable, text } from 'drizzle-orm/pg-core';
import {
  getConfigDefaults,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';
import { primaryKey } from 'drizzle-orm/pg-core';
const config = (table) => ({
  pk: primaryKey({ columns: [table.id] }),
  ...getConfigDefaults.defaultIndexes('ip_blacklists', table),
});
export const table = pgTable(
  'ip_blacklists',
  {
    ...system_fields,
    id: text('id'), // Primary key
    ip: inet('ip'),
    // timestamp: timestamp('timestamp', { withTimezone: true }),

  },
  config,
);
