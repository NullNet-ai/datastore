import { pgTable, text, timestamp } from 'drizzle-orm/pg-core';
import * as path from 'path';
import {
  fileRegex,
  getConfigDefaults,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';
import { primaryKey } from 'drizzle-orm/pg-core/index';
const filename = path.basename(__filename).replace(fileRegex, '');
const config = (table) => ({
  pk: primaryKey({ columns: [table.id, table.timestamp] }),
  ...getConfigDefaults.defaultIndexes('wallguard_logs', table),
  // uniq_id: unique().on(table.id, table.timestamp),
});
export const table = pgTable(
  filename,
  {
    ...system_fields,
    timestamp: timestamp('timestamp', { withTimezone: true }).notNull(), // NOT NULL timestamp
    level: text('level'),
    message: text('message'),
  },
  config,
);
