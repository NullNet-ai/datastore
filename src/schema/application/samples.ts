import { pgTable, text, jsonb } from 'drizzle-orm/pg-core';
import * as path from 'path';
import {
  fileRegex,
  getConfigDefaults,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';
import { sql } from 'drizzle-orm';

const filename = path.basename(__filename).replace(fileRegex, '');
const config = getConfigDefaults.byIndex(filename);
export const table = pgTable(
  filename,
  {
    ...system_fields,
    id: text('id').primaryKey(),
    name: text('name'),
    sample_text: text('sample_text'),
    test_obj: jsonb('test_obj').default(sql`'{}'::jsonb`),
    // time: timestamp('time', { withTimezone: true }), // NOT NULL timestamp
    // temperature: doublePrecision('temperature'), // NOT NULL double precision
    // humidity: doublePrecision('humidity'), // NOT NULL double precision
  },
  config,
);
