import { index, pgTable, text, boolean } from 'drizzle-orm/pg-core';
import * as path from 'path';
import {
  fileRegex,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';
const filename = path.basename(__filename).replace(fileRegex, '');
import { organizations } from '@dna-platform/crdt-lww-postgres/build/schema';
import { table as contacts } from './contacts';

const fields = {
  contact_organization_id: text().references(() => organizations.id),
  contact_id: text().references(() => (contacts as Record<string, any>).id),
  is_primary: boolean().default(false),
};

export const table = pgTable(
  filename,
  {
    ...system_fields,
    ...fields,
  },
  (table: Record<string, any>) => ({
    ...Object.keys({ ...system_fields, ...fields }).reduce((acc, field) => {
      const index_name = `organization_contacts_${field}_idx`;
      if (index_name.includes('_time')) return acc;
      return {
        ...acc,
        [index_name]: index(index_name).on(table[field]),
      };
    }, {}),
  }),
);
