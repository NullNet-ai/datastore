import {
  AnyPgColumn,
  pgTable,
  text,
} from 'drizzle-orm/pg-core';
import {
  getConfigDefaults,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';
import { table as device_configurations } from './device_configurations';

const table_name = 'device_aliases';

const fields = {
  device_configuration_id: text('device_configuration_id').references(
    () => device_configurations.id as AnyPgColumn,
  ),
  type: text('type'),
  name: text('name'),
  value: text('value'),
  description: text('description'),
  // timestamp: timestamp('timestamp', { withTimezone: true }),
  device_alias_status: text('device_alias_status'),
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
