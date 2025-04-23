import { boolean, index, pgTable, text } from 'drizzle-orm/pg-core';
import * as path from 'path';
import {
  fileRegex,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';
const filename = path.basename(__filename).replace(fileRegex, '');
import { organizations, external_contacts } from '..';
import { table as contacts } from './contacts';
import { table as organization_contacts } from './organization_contacts';
import { table as user_roles } from './user_roles';

const fields = {
  organization_contact_id: text().references(
    () => (organization_contacts as Record<string, any>).id,
  ),
  organization_id: text().references(() => organizations.id),
  contact_id: text().references(() => (contacts as Record<string, any>).id),
  email: text(),
  password: text(),
  account_id: text(),
  account_secret: text(),
  role_id: text().references(() => (user_roles as Record<string, any>).id),
  account_organization_id: text().references(
    () => (organizations as Record<string, any>).id,
  ),
  is_new_user: boolean().default(false),
  account_status: text(),
  external_contact_id: text().references(
    () => (external_contacts as Record<string, any>).id,
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
      const index_name = `organization_accounts_${field}_idx`;
      if (index_name.includes('_time')) return acc;
      return {
        ...acc,
        [index_name]: index(index_name).on(table[field]),
      };
    }, {}),
  }),
);
