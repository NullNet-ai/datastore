import { pgTable, text } from 'drizzle-orm/pg-core';
import {
  getConfigDefaults,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';

const config = getConfigDefaults.byIndex('crawls');

export const table = pgTable(
  'crawls',
  {
    ...system_fields,
    id: text('id').primaryKey(),
    website_id: text('website_id'),
    audit_id: text('audit_id'),
    crawl_status: text('crawl_status'),
    crawl_start_date: text('crawl_start_date'),
    crawl_start_time: text('crawl_start_time'),
    crawl_end_date: text('crawl_end_date'),
    crawl_end_time: text('crawl_end_time'),
    crawl_result: text('crawl_result'),
    meta_server_hostname: text('meta_server_hostname'),
    meta_cpu: text('meta_cpu'),
    meta_memory: text('meta_memory'),
  },
  config,
);
