import { pgTable, text, timestamp,  bigint, inet } from 'drizzle-orm/pg-core';
import {
  getConfigDefaults,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';
import { primaryKey } from 'drizzle-orm/pg-core';
const config = (table) => ({
  pk: primaryKey({ columns: [table.id, table.timestamp] }),
  ...getConfigDefaults.defaultIndexes('smtp_responses', table),
});
export const table = pgTable(
  'smtp_responses',
  {
    ...system_fields,
    id: text('id'), // Primary key
    timestamp: timestamp('timestamp', { withTimezone: true }), // NOT NULL timestamp
    fw_policy: text('fw_policy'),
    fw_reasons: text('fw_reasons'),
    ip: inet('ip'),
    response_code: bigint('response_code', { mode: 'number' }),
    time: bigint('time', { mode: 'number' }),
    },
  config,
);
