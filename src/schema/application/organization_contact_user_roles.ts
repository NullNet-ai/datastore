import { pgTable, text } from 'drizzle-orm/pg-core';
import * as path from 'path';
import {
  fileRegex,
  system_fields,
  getConfigDefaults,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';
const filename = path.basename(__filename).replace(fileRegex, '');
import { table as organization_contacts } from './organization_contacts';
import { table as user_roles } from './user_roles';

const config = getConfigDefaults.byIndex(filename);

const fields = {
  organization_contact_id: text().references(
    () => (organization_contacts as Record<string, any>).id,
  ),
  user_role_id: text().references(() => (user_roles as Record<string, any>).id),
};

export const table = pgTable(
  filename,
  {
    ...system_fields,
    ...fields,
  },
  config,
);
