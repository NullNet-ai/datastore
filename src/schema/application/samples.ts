import {
  pgTable,
  timestamp,
  serial,
  doublePrecision,
  primaryKey,
} from 'drizzle-orm/pg-core';
import * as path from 'path';
import {
  fileRegex,
  // getConfigDefaults,
  // system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';
const filename = path.basename(__filename).replace(fileRegex, '');
const config = (table) => ({
  pk: primaryKey({ columns: [table.id, table.time] }),
});
export const table = pgTable(
  filename,
  {
    id: serial('id'), // Auto-increment primary key
    time: timestamp('time', { withTimezone: true }).notNull(), // NOT NULL timestamp
    temperature: doublePrecision('temperature').notNull(), // NOT NULL double precision
    humidity: doublePrecision('humidity').notNull(), // NOT NULL double precision
  },
  config,
);
