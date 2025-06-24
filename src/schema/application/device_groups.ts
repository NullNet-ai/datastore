import { AnyPgColumn, pgTable, text } from 'drizzle-orm/pg-core';
import * as path from 'path';
import {
  fileRegex,
  getConfigDefaults,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';
import { table as device_group_settings } from './device_group_settings';
import { table as devices } from './devices';
const filename = path.basename(__filename).replace(fileRegex, '');
const config = getConfigDefaults.byIndex(filename);

export const table = pgTable(
  filename,
  {
    ...system_fields,
    id: text('id').primaryKey(),
    // timestamp: timestamp('timestamp', { withTimezone: true }),
    device_id: text('device_id').references(() => devices.id as AnyPgColumn),
    device_group_setting_id: text('device_group_setting_id').references(
      () => device_group_settings.id as AnyPgColumn,
    ),
  },
  config,
);
