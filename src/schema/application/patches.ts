import { pgTable, text } from 'drizzle-orm/pg-core';
import {
  getConfigDefaults,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';

export enum E_PATCH_CATEGORY {
  AI = 'AI',
  MANUAL = 'MANUAL',
  AUTOMATED = 'AUTOMATED',
}

export enum E_PATCH_STATUS {
  SCHEDULED = 'SCHEDULED',
  PENDING = 'PENDING',
  IN_PROGRESS = 'IN_PROGRESS',
  COMPLETED = 'COMPLETED',
  FAILED = 'FAILED',
  COMPLETED_WITH_ERRORS = 'COMPLETED_WITH_ERRORS',
}
const config = getConfigDefaults.byIndex('patches');

export const table = pgTable(
  'patches',
  {
    ...system_fields,
    id: text('id').primaryKey(),
    website_id: text('website_id'),
    audit_id: text('audit_id'),
    page_id: text('page_id'),
    accessibility_report_id: text('accessibility_report_id'),
    patch_status: text('patch_status'),
    patch_start_date: text('patch_start_date'),
    patch_start_time: text('patch_start_time'),
    patch_end_date: text('patch_end_date'),
    patch_end_time: text('patch_end_time'),
  },
  config,
);
