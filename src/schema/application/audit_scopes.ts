import { pgTable, text } from 'drizzle-orm/pg-core';
import {
  getConfigDefaults,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';

const config = getConfigDefaults.byIndex('crawls');

export const table = pgTable(
  'audit_scopes',
  {
    ...system_fields,
    id: text().unique(),
    name: text('name'),
  },
  config,
);
