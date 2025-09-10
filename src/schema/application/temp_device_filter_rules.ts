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
  source_port_value: text('source_port_value'),
  source_port_operator: text('source_port_operator'),
  source_ip_value: text('source_ip_value'),
  source_ip_operator: text('source_ip_operator'),
  source_ip_version: integer('source_ip_version'),
  source_type: text('source_type'),

  destination_inversed: boolean('destination_inversed'),
  destination_port_value: text('destination_port_value'),
  destination_port_operator: text('destination_port_operator'),
  destination_ip_value: text('destination_ip_value'),
  destination_ip_operator: text('destination_ip_operator'),
  destination_ip_version: integer('destination_ip_version'),
  destination_type: text('destination_type'),

  description: text('description'),
  interface: text('interface'),

  id: integer('id'),
  order: integer('order'),

  associated_rule_id: text('associated_rule_id').default(''),

  table: text('table'),
  chain: text('chain'),
  family: text('family'),
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
