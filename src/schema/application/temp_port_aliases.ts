import {
  AnyPgColumn,
  pgTable,
  text,
  primaryKey,
} from 'drizzle-orm/pg-core';
import {
  getConfigDefaults,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';
import { table as aliases } from './aliases';
import { integer } from 'drizzle-orm/pg-core';

const config = (table) => ({
  pk: primaryKey({ columns: [table.id] }),
  ...getConfigDefaults.defaultIndexes('temp_port_aliases', table),
});

export const table = pgTable(
  'temp_port_aliases',
  {
    ...system_fields,
    id: text('id'),
    // timestamp: timestamp('timestamp', { withTimezone: true }),
    alias_id: text('alias_id').references(
      () => aliases.id as AnyPgColumn,
    ),
    lower_port: integer('lower_port'),
    upper_port: integer('upper_port'),
  },
  config,
);
