import {
  AnyPgColumn,
  pgTable,
  text,
  primaryKey,
  index,
  inet,
} from 'drizzle-orm/pg-core';
import {
  getConfigDefaults,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';
import { table as device_configurations } from './device_configurations';

const table_name = 'device_interfaces';

const fields = {
  device_configuration_id: text('device_configuration_id').references(
    () => device_configurations.id as AnyPgColumn,
  ),
  name: text('name'),
  device: text('device'),
  // timestamp: timestamp('timestamp', { withTimezone: true }),
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
    id: text('id'),
    address: inet('address'), // unused
  },
  config,
);
