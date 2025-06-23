import { boolean, index, jsonb, integer, pgTable, text, timestamp } from 'drizzle-orm/pg-core';
import * as path from 'path';
import {
  fileRegex,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';
import { table as contacts } from './contacts';
const filename = path.basename(__filename).replace(fileRegex, '');

const fields = {
  title: text(),
  description: text(),
  event_timestamp: text(),
  timestamp: timestamp('timestamp', { withTimezone: true }),
  link: text().default(''),
  icon: text().default(''),
  source: text(),
  is_pinned: boolean().default(false),
  recipient_id: text().references(() => (contacts as Record<string, any>).id),
  actions: jsonb("actions").default([]),
  notification_status: text().default('unread'),
  priority_label: text().default('low'),
  priority_level: integer().default(0),
  expiry_date: text().default(''),
  metadata: text(),
};

export const table = pgTable(
  filename,
  {
    ...system_fields,
    ...fields,
  },
  (table: Record<string, any>) => ({
    ...Object.keys({ ...system_fields, ...fields }).reduce((acc, field) => {
      const searchable_fields = [
        ...Object.keys(system_fields),
        'title',
        'description',
        'timestamp',
        'link',
        'icon',
        'source',
        'is_pinned',
        'recipient_id',
        'notification_status',
        'priority_label',
        'priority_level',
        'expiry_date',
      ];
      if (!searchable_fields.includes(field)) return acc;
      const index_name = `notifications_${field}_idx`;
      if (index_name.includes('_time')) return acc;
      return {
        ...acc,
        [index_name]: index(index_name).on(table[field]),
      };
    }, {}),
  }),
);
