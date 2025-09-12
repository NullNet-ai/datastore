import { AnyPgColumn, pgTable, text, primaryKey } from 'drizzle-orm/pg-core';
import {
  getConfigDefaults,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';
import { table as device_configurations } from './device_configurations';

const config = (table) => ({
  pk: primaryKey({ columns: [table.id] }),
  ...getConfigDefaults.defaultIndexes('temp_aliases', table),
});

export const table = pgTable(
  'temp_aliases',
  {
    ...system_fields,
    id: text('id'),
    // timestamp: timestamp('timestamp', { withTimezone: true }),
    device_configuration_id: text('device_configuration_id').references(
      () => device_configurations.id as AnyPgColumn,
    ),
    type: text('type'),
    name: text('name').unique(),
    description: text('description'),
    alias_status: text('alias_status'),

    table: text('table'),
    family: text('family'),
  },
  config,
);
