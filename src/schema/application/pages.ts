import { pgTable, text } from 'drizzle-orm/pg-core';
import {
  getConfigDefaults,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';
import { numeric } from 'drizzle-orm/pg-core';

export enum E_PAGE_CATEGORY {
  ORIGINAL = 'ORIGINAL',
  ACCESSIBLE = 'ACCESSIBLE',
}

const config = getConfigDefaults.byIndex('pages');

export const table = pgTable(
  'pages',
  {
    ...system_fields,
    website_id: text('website_id'),
    audit_id: text('audit_id'),
    crawl_id: text('crawl_id'),
    id: text('id').primaryKey(),
    page_title: text('page_title'),
    page_url: text('page_url'),
    page_links: text('page_links'),
    page_headers: text('page_headers'),
    screenshot_url: text('screenshot_url'),
    category: text('category'),
    page_hash: text('page_hash'),
    page_accessible_url: text('page_accessible_url'),
    page_accessibility_score: numeric('page_accessibility_score'),
  },
  config,
);
