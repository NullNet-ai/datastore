import {
  AnyPgColumn,
  boolean,
  pgTable,
  primaryKey,
  text,
} from 'drizzle-orm/pg-core';
import {
  getConfigDefaults,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';
import { table as devices } from './devices';
import { sql } from 'drizzle-orm';

const table_name = 'installation_codes';

const fields = {
  device_id: text('device_id').references(
    () => devices.id as AnyPgColumn,
  ),
  device_code: text('device_code'),
  redeemed: boolean("redeemed").default(false),
  auto_authorization: boolean("auto_authorization").default(false)
}

const config = (table) => ({
  pk: primaryKey({ columns: [table.id] }),
  ...getConfigDefaults.defaultIndexes(table_name, table),
});

export const table = pgTable(
  table_name,
  {
    ...system_fields,
    ...fields,
    id: text('id'),
    token: text("token")
      .default(sql`substring(md5(random()::text) FROM 1 FOR 8)`)
      .unique(),
  },
  config,
);
