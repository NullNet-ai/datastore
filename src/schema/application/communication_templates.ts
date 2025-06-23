import { index, pgTable, text, timestamp } from 'drizzle-orm/pg-core';
import * as path from 'path';
import {
  fileRegex,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';
const filename = path.basename(__filename).replace(fileRegex, '');

const fields = {
  name: text('name'),
  communication_template_status: text('communication_template_status'),
  event: text('event'),
  content: text('content'),
  subject: text('subject'),
  timestamp: timestamp('timestamp', { withTimezone: true }),
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
        'name',
        'communication_template_status',
        'event',
        'content',
        'subject',
      ];
      if (!searchable_fields.includes(field)) return acc;
      const index_name = `communication_templates_${field}_idx`;
      if (index_name.includes('_time')) return acc;
      return {
        ...acc,
        [index_name]: index(index_name).on(table[field]),
      };
    }, {}),
  }),
);
