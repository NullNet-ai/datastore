import { pgTable, text, timestamp } from 'drizzle-orm/pg-core';
import {
  getConfigDefaults,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';
const config = (table) => ({
  ...getConfigDefaults.defaultIndexes('postgres_channels', table),
});
export const table = pgTable(
  'postgres_channels',
  {
    ...system_fields,
    id: text('id').primaryKey(),
    channel_name: text('channel_name').unique(),
    timestamp: timestamp('timestamp', { withTimezone: true }),
    function: text('function'),
  },
  config,
);
