import {
  AnyPgColumn,
  pgTable,
  text,
  primaryKey,
} from 'drizzle-orm/pg-core';
import {
  getConfigDefaults,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';
import { table as device_aliases } from './device_aliases';

const config = (table) => ({
  pk: primaryKey({ columns: [table.id] }),
  ...getConfigDefaults.defaultIndexes('temp_device_aliases_values', table),
});

export const table = pgTable(
  'temp_device_aliases_values',
  {
    ...system_fields,
    id: text('id'),
    // timestamp: timestamp('timestamp', { withTimezone: true }),
    device_alias_id: text('device_alias_id').references(
      () => device_aliases.id as AnyPgColumn,
    ),
    value: text('value'),
  },
  config,
);
