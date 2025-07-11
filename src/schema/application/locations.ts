import { index, pgTable, text } from 'drizzle-orm/pg-core';
import * as path from 'path';
import {
  fileRegex,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';
import { addresses } from '..';

const filename = path.basename(__filename).replace(fileRegex, '');

const fields = {
  location_name : text(),
  // timestamp: timestamp('timestamp', { withTimezone: true }),
  address_id : text().references(
    () => (addresses as Record<string, any>).id,
  ),
};

export const table = pgTable(
  filename,
  {
    ...system_fields,
    ...fields,
  },
  (table: Record<string, any>) => ({
    ...Object.keys({ ...system_fields, ...fields }).reduce((acc, field) => {
      const index_name = `locations_${field}_idx`;
      if (index_name.includes('_time')) return acc;
      return {
        ...acc,
        [index_name]: index(index_name).on(table[field]),
      };
    }, {}),
  }),
);
