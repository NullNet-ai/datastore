import {
  AnyPgColumn,
  pgTable,
  text,
  primaryKey,
  boolean,
  integer,
  index, timestamp,
} from 'drizzle-orm/pg-core';
import {
  getConfigDefaults,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';
import { table as device_configurations } from './device_configurations';

const table_name = 'device_rules';

const fields = {
  device_configuration_id: text('device_configuration_id').references(
    () => device_configurations.id as AnyPgColumn,
  ),
  disabled: boolean('disabled'),
  type: text('type'),
  policy: text('policy'),
  protocol: text('protocol'),

  source_port: text('source_port'),
  source_addr: text('source_addr'),
  source_type: text('source_type'),

  destination_port: text('destination_port'),
  destination_addr: text('destination_addr'),
  
  description: text('description'),
  device_rule_status: text('device_rule_status'),
  interface: text('interface'),
  order: integer('order'),
  timestamp: timestamp('timestamp', { withTimezone: true }),

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
    destination_inversed: boolean('destination_inversed'),
    destination_type: text('destination_type'),
    source_inversed: boolean('source_inversed'),
  },
  config,
);
