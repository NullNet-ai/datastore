import {
  pgTable,
  text,
  primaryKey,
  AnyPgColumn,
  index,
} from 'drizzle-orm/pg-core';
import {
  system_fields,
  getConfigDefaults,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';
import { table as devices } from './devices';

const fields = {
  public_key: text('public_key'),
  private_key: text('private_key'),
  passphrase: text('passphrase'),
  device_id: text('device_id').references(() => devices.id as AnyPgColumn),
};

const config = (table) => ({
  pk: primaryKey({ columns: [table.id] }),
  ...getConfigDefaults.defaultIndexes('device_ssh_keys', table),
  ...Object.keys(fields).reduce((acc, field) => {
    const index_name = `device_ssh_keys_${field}_idx`;
    return {
      ...acc,
      [index_name]: index(index_name).on(table[field]),
    };
  }, {}),
});

export const table = pgTable(
  'device_ssh_keys',
  {
    ...system_fields,
    ...fields,
    // timestamp: timestamp('timestamp', { withTimezone: true }),
    id: text('id'),
  },
  config,
);
