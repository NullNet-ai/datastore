import {
  pgTable,
  text,
  primaryKey,
  integer,
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
  ...getConfigDefaults.defaultIndexes('temp_device_tunnels', table),
});

export const table = pgTable(
  filename,
  {
    ...system_fields,
    id: text('id'),
    device_id: text('device_id'),
    tunnel_type: text('tunnel_type'),
    // @TODO:  key to `device_services` id
    service_id: text('service_id'),
    tunnel_status: text('tunnel_status'),
    last_accessed: integer('last_accessed').default(0),
    created_timestamp: integer('created_timestamp').default(0),
  },
  config,
);
