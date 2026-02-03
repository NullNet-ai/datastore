import {
  pgTable,
  text,
  primaryKey,
} from 'drizzle-orm/pg-core';
import * as path from 'path';
import {
  fileRegex,
  getConfigDefaults,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';
const filename = path.basename(__filename).replace(fileRegex, '');

const config = (table) => ({
  pk: primaryKey({ columns: [table.id] }),
  ...getConfigDefaults.defaultIndexes(filename, table),
});

export const table = pgTable(
  filename,
  {
    ...system_fields,
    id: text('id'),
    name: text('name'),
    latest_version: text('latest_version'),
    minimum_version: text('minimum_version'),
    update_type: text('update_type').default('optional'),
    release_notes: text('release_notes'),
  },
  config,
);


