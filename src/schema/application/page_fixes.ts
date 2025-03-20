import { pgTable, text } from 'drizzle-orm/pg-core';
import {
  getConfigDefaults,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';

export enum E_PAGE_FIX_STATUS {
  PENDING = 'PENDING',
  APPROVED = 'APPROVED',
}

const config = getConfigDefaults.byIndex('page_fixes');

export const table = pgTable(
  'page_fixes',
  {
    ...system_fields,
    website_id: text('website_id'),
    audit_id: text('audit_id'),
    page_id: text('page_id'),
    accessibility_report_id: text('accessibility_report_id'),
    id: text('id'),
    approval_id: text('approval_id'),
    patch_id: text('patch_id'),
    selector: text('selector'),
    replacement: text('replacement'),
    page_fix_status: text('page_fix_status'),
  },
  config,
);
