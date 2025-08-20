import {
  AnyPgColumn, inet, integer,
  pgTable,
  text,
} from 'drizzle-orm/pg-core';
import {
  getConfigDefaults,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';
import { table as aliases } from './aliases';

const table_name = 'ip_aliases';

const fields = {
  alias_id: text('alias_id').references(
    () => aliases.id as AnyPgColumn,
  ),
  ip: inet('ip').unique(),
  prefix: integer('prefix').default(32).unique(),
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
