import {
  pgTable,
  text,
  date,
  time,
  boolean,
  numeric,
} from 'drizzle-orm/pg-core';
import {
  getConfigDefaults,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';

export enum E_AUDIT_SCOPES {
  FULL = 'FULL',
  CRAWL = 'CRAWL',
  ACCESSIBILITY_SCAN = 'ACCESSIBILITY_SCAN',
  GENERATE_FIX = 'GENERATE_FIX',
}

export enum E_TASK_STATUS {
  QUEUED = 'QUEUED',
  IN_PROGRESS = 'IN_PROGRESS',
  COMPLETED = 'COMPLETED',
  FAILED = 'FAILED',
}

export enum E_AUDIT_SCHEDULED_TYPE {
  NOW = 'NOW',
  LATER = 'LATER',
  SKIP = 'SKIP',
}

export enum E_AUDIT_TYPE {
  Detailed_Audit = 'Detailed_Audit',
  Page = 'Page',
}

const config = getConfigDefaults.byIndex('audits');

export const table = pgTable(
  'audits',
  {
    ...system_fields,
    website_id: text('website_id'),
    id: text('id'),
    audit_scopes: text('audit_scopes'),
    audit_scheduled_date: date('audit_scheduled_date'),
    audit_scheduled_time: time('audit_scheduled_time'),
    audit_end_date: date('audit_end_date'),
    audit_end_time: time('audit_end_time'),
    audit_schedule_type: text('audit_schedule_type'),
    audit_status: text('audit_status'),
    audit_auto_fixes: boolean('audit_auto_fixes').default(false),
    audit_wcag_standards: text('audit_wcag_standards').array(),
    audit_accessibility_score: numeric('audit_accessibility_score'),
    audit_type: text('audit_type'),
  },
  config,
);
