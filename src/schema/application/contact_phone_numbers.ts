import { boolean, pgTable, text } from 'drizzle-orm/pg-core';
import * as path from 'path';
import {
  fileRegex,
  getConfigDefaults,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';
import { table as contacts } from './contacts';
const filename = path.basename(__filename).replace(fileRegex, '');
const config = getConfigDefaults.byIndex(filename);
export const table = pgTable(
  filename,
  {
    ...system_fields,
    contact_id: text('contact_id').references(
      () => (contacts as Record<string, any>).id,
    ),
    raw_phone_number: text('raw_phone_number'),
    iso_code: text('iso_code'),
    country_code: text('country_code'),
    is_primary: boolean('is_primary').default(false),
  },
  config,
);
