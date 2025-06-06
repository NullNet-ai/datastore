import {
  AnyPgColumn,
  pgTable,
  text,
  primaryKey,
  inet,
} from 'drizzle-orm/pg-core';
import {
  getConfigDefaults,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';
import { table as device_configurations } from './device_configurations';

const config = (table) => ({
  pk: primaryKey({ columns: [table.id] }),
  ...getConfigDefaults.defaultIndexes('temp_device_interfaces', table),
});

export const table = pgTable(
  'temp_device_interfaces',
  {
    ...system_fields,
    id: text('id'),
    device_configuration_id: text('device_configuration_id').references(
      () => device_configurations.id as AnyPgColumn,
    ),
    name: text('name'),
    device: text('device'),
    address: inet('address'), // unused

  },
  config,
);
