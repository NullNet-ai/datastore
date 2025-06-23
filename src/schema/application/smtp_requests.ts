import { inet, pgTable, text, timestamp } from 'drizzle-orm/pg-core';
import {
  getConfigDefaults,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';
import { primaryKey } from 'drizzle-orm/pg-core';
const config = (table) => ({
  pk: primaryKey({ columns: [table.id] }),
  ...getConfigDefaults.defaultIndexes('smtp_requests', table),
});
export const table = pgTable(
  'smtp_requests',
  {
    ...system_fields,
    id: text('id'), // Primary key
    timestamp: timestamp('timestamp', { withTimezone: true }), // NOT NULL timestamp
    fw_policy: text('fw_policy'),
    fw_reasons: text('fw_reasons'),
    ip: inet('ip'),
    user_agent: text('user_agent'),
    headers: text('headers'),
    body: text('body'),
    },
  config,
);
