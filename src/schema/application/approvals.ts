import { pgTable, text } from 'drizzle-orm/pg-core';
import {
  getConfigDefaults,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';

const config = getConfigDefaults.byIndex('approvals');

export const table = pgTable(
  'approvals',
  {
    ...system_fields,
    website_id: text('website_id'),
    audit_id: text('audit_id'),
    page_id: text('page_id'),
    accessibility_report_id: text('accessibility_report_id'),
    page_fix_id: text('page_fix_id'),
    id: text('id').primaryKey(),
    approved_by: text('approved_by'),
  },
  config,
);
