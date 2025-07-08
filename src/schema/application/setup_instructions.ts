import {
  pgTable,
  text,
} from 'drizzle-orm/pg-core';
import {
  getConfigDefaults,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';

const table_name = 'setup_instructions';

const fields = {
  device_category: text('device_category'),
  device_type: text('device_type'),
  markdown: text('markdown'),
}

const config = getConfigDefaults.byIndex(table_name);

export const table = pgTable(
  table_name,
  {
    ...system_fields,
    ...fields,
    id: text('id')
  },
  config,
);
