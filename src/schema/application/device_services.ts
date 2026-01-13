import {
  AnyPgColumn,
  pgTable,
  text,
  primaryKey,
  inet,
  integer,
} from 'drizzle-orm/pg-core';
import * as path from 'path';
import {
  fileRegex,
  getConfigDefaults,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';
import { table as devices } from './devices';
const filename = path.basename(__filename).replace(fileRegex, '');

const config = (table) => ({
  pk: primaryKey({ columns: [table.id, table.timestamp, table.device_id] }),
  ...getConfigDefaults.defaultIndexes('device_services', table),
});

export const table = pgTable(
  filename,
  {
    ...system_fields,
    id: text('id'),
    device_id: text('device_id').references(() => devices.id as AnyPgColumn),
    address: inet('address'),
    port: integer('port'),
    protocol: text('protocol')
  },
  config,
);
