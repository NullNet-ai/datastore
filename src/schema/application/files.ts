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
    fieldname: text('fieldname'),
    originalname: text('originalname'),
    encoding: text('encoding'),
    mimetype: text('mimetype'),
    destination: text('destination'),
    filename: text('filename'),
    path: text('path'),
    size: integer('size'),
    uploaded_by: text('uploaded_by'),
    downloaded_by: text('downloaded_by'),
    etag: text('etag'),
    versionId: text('versionId'),
    download_path: text('download_path'),
    presignedURL: text('presignedURL'),
    presignedURLExpires: integer('presignedURLExpires'),
  },
  config,
);
