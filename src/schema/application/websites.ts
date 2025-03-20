import { pgTable, text, boolean } from 'drizzle-orm/pg-core';
import {
  getConfigDefaults,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';

export enum E_WCAG_STANDARD {
  WCAG_2_0 = 'WCAG_2_0',
  WCAG_2_1 = 'WCAG_2_1',
}

export enum E_WEBSITE_STATUS {
  PENDING = 'PENDING',
  IN_PROGRESS = 'IN_PROGRESS',
  COMPLETED = 'COMPLETED',
  FAILED = 'FAILED',
  COMPLETED_WITH_ERRORS = 'COMPLETED_WITH_ERRORS',
}

const config = getConfigDefaults.byIndex('websites');

export const table = pgTable(
  'websites',
  {
    ...system_fields,
    id: text('id'),
    wcag_standards: text('wcag_standards').array(),
    website_cron_repeat: text('website_cron_repeat'),
    protocol: text('protocol'),
    hostname: text('hostname'),
    accessible_url: text('accessible_url'),
    website_status: text('website_status'),
    website_auto_fixes: boolean('website_auto_fixes').default(false),
  },
  config,
);
