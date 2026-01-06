import { AnyPgColumn, pgTable, primaryKey, text } from 'drizzle-orm/pg-core';
import {
  getConfigDefaults,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';
import { table as device_configurations } from './device_configurations';

const table_name = 'aliases';

const fields = {
  device_configuration_id: text('device_configuration_id').references(
    () => device_configurations.id as AnyPgColumn,
  ),
  type: text('type'),
  name: text('name'),
  description: text('description'),
  // timestamp: timestamp('timestamp', { withTimezone: true }),
  alias_status: text('alias_status'),

  table: text('table'),
  family: text('family'),
};

const config = (table) => ({
  pk: primaryKey({ columns: [table.id] }),
  ...getConfigDefaults.defaultIndexes('aliases', table),
});

export const table = pgTable(
  table_name,
  {
    ...system_fields,
    ...fields,
    id: text('id'),
  },
  config,
);
