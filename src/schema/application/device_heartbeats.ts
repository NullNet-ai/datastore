import {
  AnyPgColumn,
  pgTable,
  text,
  timestamp,
  primaryKey,
} from 'drizzle-orm/pg-core';
import * as path from 'path';
import {
  fileRegex,
  getConfigDefaults,
  // getConfigDefaults,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system'; //convert to hypertable
import { table as devices } from './devices';
const filename = path.basename(__filename).replace(fileRegex, '');

// @TODO: Merge `config1` into `config`

// const config1 = getConfigDefaults.byIndex(filename);
const config = (table) => ({
  pk: primaryKey({ columns: [table.id, table.timestamp] }),
  ...getConfigDefaults.defaultIndexes('device_heartbeats', table),
  // uniq_id: unique().on(table.id, table.timestamp),
});

export const table = pgTable(
  filename,
  {
    ...system_fields,
    id: text('id'),
    device_id: text('device_id').references(() => devices.id as AnyPgColumn),
    timestamp: timestamp('timestamp', { withTimezone: true }), // Epoch timestamp when the packet was captured

    hypertable_timestamp: text('hypertable_timestamp'), // Hypertable timestamp
  },
  config,
);
