import {
  AnyPgColumn,
  pgTable,
  text,
  primaryKey,
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
  pk: primaryKey({ columns: [table.id] }),
  ...getConfigDefaults.defaultIndexes('device_configurations', table),
});

export const table = pgTable(
  filename,
  {
    ...system_fields,
    id: text('id'),
    device_id: text('device_id').references(() => devices.id as AnyPgColumn),
    digest: text('digest'),
    hostname: text('hostname'),
    raw_content: text('raw_content'),
    config_version: integer('config_version'),
  },
  config,
);
