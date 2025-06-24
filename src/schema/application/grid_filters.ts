import { boolean, index, jsonb, pgTable, text } from 'drizzle-orm/pg-core';
import * as path from 'path';
import {
  fileRegex,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';
import { table as contacts } from './contacts';
const filename = path.basename(__filename).replace(fileRegex, '');

const fields = {
  name: text(),
  grid_id: text(),
  link: text().default(''),
  is_current: boolean().default(false),
  is_default: boolean().default(false),
  contact_id: text().references(() => (contacts as Record<string, any>).id),
  account_organization_id : text(),
  entity: text(),
  columns: jsonb("columns").default([]),
  groups: jsonb("groups").default([]),
  sorts: jsonb("sorts").default([]),
  // timestamp: timestamp('timestamp', { withTimezone: true }),
  default_sorts:jsonb("default_sorts").default([]),
  advance_filters: jsonb("advance_filters").default([]),
  group_advance_filters: jsonb("group_advance_filters").default([]),
  filter_groups: jsonb("filter_groups").default([]),
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
        'grid_id',
        'link',
        'is_current',
        'is_default',
        'contact_id',
        'entity',
      ];
      if (!searchable_fields.includes(field)) return acc;
      const index_name = `grid_filters_${field}_idx`;
      if (index_name.includes('_time')) return acc;
      return {
        ...acc,
        [index_name]: index(index_name).on(table[field]),
      };
    }, {}),
  }),
);
