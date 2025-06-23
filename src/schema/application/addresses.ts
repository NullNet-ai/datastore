import { pgTable, real, text, timestamp } from 'drizzle-orm/pg-core';
import * as path from 'path';
import {
  fileRegex,
  getConfigDefaults,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';
const filename = path.basename(__filename).replace(fileRegex, '');
const config = getConfigDefaults.byIndex(filename);
export const table = pgTable(
  filename,
  {
    ...system_fields,
    id: text('id').primaryKey(),
    timestamp: timestamp('timestamp', { withTimezone: true }),
    address: text('address'),
    address_line_one: text('address_line_one'),
    address_line_two: text('address_line_two'),
    latitude: real('latitude'),
    longitude: real('longitude'),
    place_id: text('place_id'),
    street_number: text('street_number'),
    street: text('street'),
    region: text('region'),
    region_code: text('region_code'),
    country_code: text('country_code'),
    postal_code: text('postal_code'),
    country: text('country'),
    state: text('state'),
    city: text('city'),
  },
  config,
);
