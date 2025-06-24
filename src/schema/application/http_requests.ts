import { inet, pgTable, text } from 'drizzle-orm/pg-core';
import {
  getConfigDefaults,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';
import { primaryKey } from 'drizzle-orm/pg-core';
const config = (table) => ({
  pk: primaryKey({ columns: [table.id] }),
  ...getConfigDefaults.defaultIndexes('http_requests', table),
});
export const table = pgTable(
  'http_requests',
  {
    ...system_fields,
    id: text('id'), // Primary key
    // timestamp: timestamp('timestamp', { withTimezone: true }), // NOT NULL timestamp
    fw_policy: text('fw_policy'),
    fw_reasons: text('fw_reasons'),
    ip: inet('ip'),
    original_url: text('original_url'),
    user_agent: text('user_agent'),
    headers: text('headers'),
    method: text('method'),
    body: text('body'),
    query: text('query'),
    cookies: text('cookies'),
    },
  config,
);
