import { index, pgTable, text } from 'drizzle-orm/pg-core';
import * as path from 'path';
import {
  fileRegex,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';
const filename = path.basename(__filename).replace(fileRegex, '');
import { account_organizations, organization_accounts } from '..';

const fields = {
  account_id: text().references(
    () => (organization_accounts as Record<string, any>).id,
  ),
  expiration_date: text(),
  expiration_time: text(),
  // timestamp: timestamp('timestamp', { withTimezone: true }),
  account_organization_id: text('account_organization_id').references(
    () => (account_organizations as Record<string, any>).id,
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
      const searchable_fields = [
        ...Object.keys(system_fields),
        'account_id',
        'expiration_date',
      ];
      if (!searchable_fields.includes(field)) return acc;
      const index_name = `invitations_${field}_idx`;
      if (index_name.includes('_time')) return acc;
      return {
        ...acc,
        [index_name]: index(index_name).on(table[field]),
      };
    }, {}),
  }),
);
