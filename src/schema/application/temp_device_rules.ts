import {
  AnyPgColumn,
  pgTable,
  text,
  primaryKey,
  boolean,
  integer, timestamp,
} from 'drizzle-orm/pg-core';
import {
  getConfigDefaults,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';
import { table as device_configurations } from './device_configurations';

const config = (table) => ({
  pk: primaryKey({ columns: [table.id] }),
  ...getConfigDefaults.defaultIndexes('temp_device_rules', table),
});

export const table = pgTable(
  'temp_device_rules',
  {
    ...system_fields,
    id: text('id'),
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
    source_inversed: boolean('source_inversed'),

    destination_port: text('destination_port'),
    destination_addr: text('destination_addr'),
    destination_type: text('destination_type'),
    destination_inversed: boolean('destination_inversed'),
    timestamp: timestamp('timestamp', { withTimezone: true }),
    

    description: text('description'),
    device_rule_status: text('device_rule_status'),
    interface: text('interface'),
    order: integer('order'),
  },
  config,
);
