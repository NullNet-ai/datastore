import { integer, pgTable, text } from 'drizzle-orm/pg-core';
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
    fieldname: text(),
    originalname: text(),
    encoding: text(),
    mimetype: text(),
    destination: text(),
    filename: text(),
    path: text(),
    size: integer(),
    uploaded_by: text(),
    downloaded_by: text(),
    etag: text(),
    versionId: text(),
    download_path: text(),
  },
  config,
);
