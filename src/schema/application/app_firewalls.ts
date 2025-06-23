import { pgTable, text, timestamp } from 'drizzle-orm/pg-core';
import {
  getConfigDefaults,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';
import { primaryKey } from 'drizzle-orm/pg-core';
const config = (table) => ({
  pk: primaryKey({ columns: [table.id, table.timestamp, table.app_id] }),
  ...getConfigDefaults.defaultIndexes('app_firewalls', table),
});
export const table = pgTable(
  'app_firewalls',
  {
    ...system_fields,
    id: text('id'), // Primary key ---
    app_id: text('app_id').unique(),
    timestamp: timestamp('timestamp', { withTimezone: true }),
    firewall: text('firewall'),
  },
  config,
);
