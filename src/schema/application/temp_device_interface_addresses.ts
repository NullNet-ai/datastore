import {
  AnyPgColumn,
  pgTable,
  text,
  primaryKey,
  integer,
  inet,
} from 'drizzle-orm/pg-core';
import {
  getConfigDefaults,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';
import { table as device_interfaces } from './device_interfaces';

const config = (table) => ({
  pk: primaryKey({ columns: [table.id] }),
  ...getConfigDefaults.defaultIndexes('temp_device_interface_addresses', table),
});

export const table = pgTable(
  'temp_device_interface_addresses',
  {
    ...system_fields,
    id: text('id'),
    device_interface_id: text('device_interface_id').references(
      () => device_interfaces.id as AnyPgColumn,
    ),
    address: inet('address'),
    version: integer('version'),
  },
  config,
);
