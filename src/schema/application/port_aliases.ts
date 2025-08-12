import {
  AnyPgColumn, integer,
  pgTable,
  text,
} from 'drizzle-orm/pg-core';
import {
  getConfigDefaults,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';
import { table as aliases } from './aliases';

const table_name = 'port_aliases';

const fields = {
  alias_id: text('alias_id').references(
    () => aliases.id as AnyPgColumn,
  ),
  lower_port: integer('lower_port'),
  upper_port: integer('upper_port'),
  // timestamp: timestamp('timestamp', { withTimezone: true }),
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
