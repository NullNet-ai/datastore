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
import { inet, integer } from 'drizzle-orm/pg-core';

const config = (table) => ({
  pk: primaryKey({ columns: [table.id] }),
  ...getConfigDefaults.defaultIndexes('temp_ip_aliases', table),
});

export const table = pgTable(
  'temp_ip_aliases',
  {
    ...system_fields,
    id: text('id'),
    alias_id: text('alias_id').references(
      () => aliases.id as AnyPgColumn,
    ),
    ip: inet('ip').default("0.0.0.0"),
    prefix: integer('prefix').default(32),
  },
  config,
);
