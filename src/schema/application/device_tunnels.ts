import {
  AnyPgColumn,
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
import { table as devices } from './devices';

const filename = path.basename(__filename).replace(fileRegex, '');

const config = (table) => ({
  pk: primaryKey({ columns: [table.id, table.timestamp, table.device_id] }),
  ...getConfigDefaults.defaultIndexes('device_tunnels', table),
});

export const table = pgTable(
  filename,
  {
    ...system_fields,
    id: text('id'),
    device_id: text('device_id').references(() => devices.id as AnyPgColumn),
    tunnel_type: text('tunnel_type'),
    // @TODO:  key to `device_services` id
    service_id: text('service_id'),
  },
  config,
);
