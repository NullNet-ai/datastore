import { AnyPgColumn, pgTable, text, primaryKey, timestamp } from 'drizzle-orm/pg-core';
import * as path from 'path';
import {
  fileRegex,
  getConfigDefaults,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';
import { table as devices } from './devices';
const filename = path.basename(__filename).replace(fileRegex, '');

const config = (table) => ({
  pk: primaryKey({ columns: [table.id] }),
  ...getConfigDefaults.defaultIndexes('device_remote_access_sessions', table),
});

export const table = pgTable(
  filename,
  {
    ...system_fields,
    id: text('id'),
    device_id: text('device_id').references(() => devices.id as AnyPgColumn),
    remote_access_type: text('remote_access_type'),
    remote_access_session: text('remote_access_session'),
    remote_access_status: text('remote_access_status'),

    // ?????
    remote_access_category: text('remote_access_category'),

    timestamp: timestamp('timestamp', {
      withTimezone: true,
      mode: 'string',
    }).defaultNow(),
  },
  config,
);
