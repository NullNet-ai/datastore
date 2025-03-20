import { pgTable, text, date, time, json } from 'drizzle-orm/pg-core';
import {
  getConfigDefaults,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';

const config = getConfigDefaults.byIndex('crawls');

export const table = pgTable(
  'crawls',
  {
    ...system_fields,
    website_id: text('website_id'),
    audit_id: text('audit_id'),
    id: text('id'),
    crawl_status: text('crawl_status'),
    crawl_start_date: date('crawl_start_date'),
    crawl_start_time: time('crawl_start_time'),
    crawl_end_date: date('crawl_end_date'),
    crawl_end_time: time('crawl_end_time'),
    crawl_result: json('crawl_result'),
    meta_server_hostname: text('meta_server_hostname'),
    meta_cpu: text('meta_cpu'),
    meta_memory: text('meta_memory'),
  },
  config,
);
