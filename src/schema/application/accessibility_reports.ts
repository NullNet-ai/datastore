import { pgTable, text, json, numeric } from 'drizzle-orm/pg-core';
import {
  getConfigDefaults,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';

export enum E_WCAG_VALUE {
  VIOLATION = 'VIOLATION',
  PASS = 'PASS',
}

const config = getConfigDefaults.byIndex('accessibility_reports');

export const table = pgTable(
  'accessibility_reports',
  {
    ...system_fields,
    website_id: text('website_id'),
    audit_id: text('audit_id'),
    page_id: text('page_id'),
    accessibility_scan_id: text('accessibility_scan_id'),
    id: text('id').primaryKey(),
    wcag_rule_id: text('wcag_rule_id'),
    wcag_value: text('wcag_value'),
    path: json('path'),
    wcag_rule_time: numeric('wcag_rule_time'),
    wcag_reason_id: text('wcag_reason_id'),
    wcag_message: text('wcag_message'),
    wcag_message_args: text('wcag_message_args').array(),
    wcag_api_args: text('wcag_api_args').array(),
    wcag_snippet: text('wcag_snippet'),
    wcag_category: text('wcag_category'),
    wcag_help_link: text('wcag_help_link'),
    node_config: json('node_config'),
    wcag_snippet_path: text('wcag_snippet_path'),
  },
  config,
);
