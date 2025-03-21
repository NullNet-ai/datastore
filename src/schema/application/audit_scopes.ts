import { pgTable, text } from 'drizzle-orm/pg-core';
import {
  getConfigDefaults,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';

const config = getConfigDefaults.byIndex('audit_scopes');

export const table = pgTable(
  'audit_scopes',
  {
    ...system_fields,
    id: text('id').primaryKey(),
    name: text('name'),
  },
  config,
);
