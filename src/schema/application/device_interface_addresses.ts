import {
  AnyPgColumn,
  pgTable,
  text,
  primaryKey,
  integer,
  index,
  inet,
} from 'drizzle-orm/pg-core';
import {
  getConfigDefaults,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';
import { table as device_interfaces } from './device_interfaces';

const table_name = 'device_interface_addresses';

const fields = {
  device_interface_id: text('device_interface_id').references(
    () => device_interfaces.id as AnyPgColumn,
  ),
  address: inet('address'),
  version: integer('version'),
}

const config = (table) => ({
  pk: primaryKey({ columns: [table.id] }),
  ...getConfigDefaults.defaultIndexes(table_name, table),
  ...Object.keys(fields).reduce((acc, field) => {
    const index_name = `${table_name}_${field}_idx`;
    return {
      ...acc,
      [index_name]: index(index_name).on(table[field]),
    };
  }, {}),
});

export const table = pgTable(
  table_name,
  {
    ...system_fields,
    ...fields,
    // timestamp: timestamp('timestamp', { withTimezone: true }),
    id: text('id'),
  },
  config,
);
