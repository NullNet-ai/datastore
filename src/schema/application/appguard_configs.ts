import { boolean, integer, pgTable, text } from 'drizzle-orm/pg-core';
import {
  getConfigDefaults,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';
import { primaryKey } from 'drizzle-orm/pg-core';
const config = (table) => ({
  pk: primaryKey({ columns: [table.id] }),
  ...getConfigDefaults.defaultIndexes('appguard_configs', table),
});
export const table = pgTable(
  'appguard_configs',
  {
    ...system_fields,
    id: text('id'), // Primary key
    // timestamp: timestamp('timestamp', { withTimezone: true }), // NOT NULL timestamp
    active: boolean('active'),
    log_request: boolean('log_request'),
    log_response: boolean('log_response'),
    retention_sec: integer('retention_sec'),
    ip_info_cache_size: integer('ip_info_cache_size'),
  },
  config,
);
