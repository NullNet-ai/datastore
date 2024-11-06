import { sqliteTable, text } from 'drizzle-orm/sqlite-core';
import * as path from 'path';
import {
  fileRegex,
  getConfigDefaults,
  system_fields,
} from '@dna-platform/crdt-lww/build/schema/system';
const filename = path.basename(__filename).replace(fileRegex, '');

const config = getConfigDefaults.byIndex(filename);
export const contacts = sqliteTable(
  filename,
  {
    ...system_fields,
    first_name: text(),
    middle_name: text(),
    last_name: text(),
    date_of_birth: text(),
  },
  config,
);
