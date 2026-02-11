import {
  pgTable,
  text,
  primaryKey,
} from 'drizzle-orm/pg-core';
import * as path from 'path';
import {
  fileRegex,
  getConfigDefaults,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';

const filename = path.basename(__filename).replace(fileRegex, '');

const config = (table) => ({
  pk: primaryKey({ columns: [table.id] }),
  ...getConfigDefaults.defaultIndexes('temp_device_tty_sessions', table),
});

export const table = pgTable(
  filename,
  {
    ...system_fields,
    id: text('id'),
    device_tunnel_id: text("device_tunnel_id"),
    device_id: text("device_id"),
    instance_id: text("instance_id"),
    session_status: text("session_status"),
    // username: text("username"),
  },
  config,
);
