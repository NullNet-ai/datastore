import {
  AnyPgColumn,
  pgTable,
  text,
  primaryKey,
  boolean,
  integer,
  index,
} from 'drizzle-orm/pg-core';
import {
  getConfigDefaults,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';
import { table as device_configurations } from './device_configurations';

const table_name = 'temp_device_filter_rules';

const fields = {
  device_configuration_id: text('device_configuration_id').references(
    () => device_configurations.id as AnyPgColumn,
  ),
  disabled: boolean('disabled'),

  policy: text('policy'),
  protocol: text('protocol'),

  source_inversed: boolean('source_inversed'),
  source_port: text('source_port'),
  source_addr: text('source_addr'),
  source_type: text('source_type'),

  destination_inversed: boolean('destination_inversed'),
  destination_port: text('destination_port'),
  destination_addr: text('destination_addr'),
  destination_type: text('destination_type'),

  description: text('description'),
  interface: text('interface'),

  order: integer('order'),
  id: integer('id'),

  assosiated_rule_id: text('assosiated_rule_id').default(''),
};

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
  },
  config,
);
