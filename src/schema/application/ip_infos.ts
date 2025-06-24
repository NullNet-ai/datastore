import { boolean, pgTable, text } from 'drizzle-orm/pg-core';
import {
  getConfigDefaults,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';
import { primaryKey } from 'drizzle-orm/pg-core';
const config = (table) => ({
  pk: primaryKey({ columns: [table.id] }),
  ...getConfigDefaults.defaultIndexes('ip_infos', table),
});
export const table = pgTable(
  'ip_infos',
  {
    ...system_fields,
    id: text('id'), // Primary key ---
    // timestamp: timestamp('timestamp', { withTimezone: true }), // NOT NULL timestamp
    ip: text('ip').unique(),
    country: text('country'),
    asn: text('asn'),
    org: text('org'),
    continent_code: text('continent_code'),
    city: text('city'),
    region: text('region'),
    postal: text('postal'),
    timezone: text('timezone'),
    blacklist: boolean('blacklist'),
  },
  config,
);
