import { pgTable, text } from 'drizzle-orm/pg-core';
import {
  getConfigDefaults,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';

export enum E_AUDIT_URL_CATEGORY {
  INLUSION = 'INLUSION',
  EXCLUSION = 'EXCLUSION',
}

const config = getConfigDefaults.byIndex('audit_urls');

export const table = pgTable(
  'audit_urls',
  {
    ...system_fields,
    audit_id: text('audit_id'),
    id: text('id'),
    category: text('category'),
    value: text('value'),
  },
  config,
);
