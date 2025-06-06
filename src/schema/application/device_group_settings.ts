import { pgTable, text } from 'drizzle-orm/pg-core';
import * as path from 'path';
import {
  fileRegex,
  getConfigDefaults,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';
const filename = path.basename(__filename).replace(fileRegex, '');
const config = getConfigDefaults.byIndex(filename);
export const table = pgTable(
  filename,
  {
    ...system_fields,
    id: text('id').primaryKey(),
    name: text('name'),
  },
  config,
);
