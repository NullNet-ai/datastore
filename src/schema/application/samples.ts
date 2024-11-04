import { sqliteTable, text } from 'drizzle-orm/sqlite-core';
import * as path from 'path';
import {
  fileRegex,
  getConfigDefaults,
  system_fields,
} from '@dna-platform/crdt-lww/build/schema/system';
const filename = path.basename(__filename).replace(fileRegex, '');
const config = getConfigDefaults.byIndex(filename);
export const table = sqliteTable(
  filename,
  {
    ...system_fields,
    sample_text: text(),
  },
  config,
);
