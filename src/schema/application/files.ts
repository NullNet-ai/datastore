import { integer, sqliteTable, text } from 'drizzle-orm/sqlite-core';
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
    fieldname: text(),
    originalname: text(),
    encoding: text(),
    mimetype: text(),
    destination: text(),
    filename: text(),
    path: text(),
    size: integer(),
  },
  config,
);
