import { pgTable, text, date, time } from 'drizzle-orm/pg-core';
import {
  getConfigDefaults,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';

const config = getConfigDefaults.byIndex('accessibility_scans');

export const table = pgTable(
  'accessibility_scans',
  {
    ...system_fields,
    website_id: text('website_id'),
    audit_id: text('audit_id'),
    page_id: text('page_id'),
    id: text('id').primaryKey(),
    accessibility_scan_status: text('accessibility_scan_status'),
    accessibility_scan_start_date: date('accessibility_scan_start_date'),
    accessibility_scan_start_time: time('accessibility_scan_start_time'),
    accessibility_scan_end_date: date('accessibility_scan_end_date'),
    accessibility_scan_end_time: time('accessibility_scan_end_time'),
    wcag_standard: text('wcag_standard'),
  },
  config,
);
