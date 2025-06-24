import { bigint, integer, pgTable, text, doublePrecision } from 'drizzle-orm/pg-core';
import {
  getConfigDefaults,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';
import { primaryKey } from 'drizzle-orm/pg-core';
const config = (table) => ({
  pk: primaryKey({ columns: [table.id] }),
  ...getConfigDefaults.defaultIndexes('system_resources', table),
});
export const table = pgTable(
  'system_resources',
  {
    ...system_fields,
    id: text('id'), // Primary key
    // timestamp: timestamp('timestamp', { withTimezone: true }), // NOT NULL timestamp

    // CPU
    num_cpus: integer('num_cpus'),
    global_cpu_usage: doublePrecision('global_cpu_usage'),
    cpu_usages: text('cpu_usages'), // JSON string

    // RAM
    total_memory: bigint('total_memory', { mode: 'number' }),
    used_memory: bigint('used_memory', { mode: 'number' }),

    // Disk
    total_disk_space: bigint('total_disk_space', { mode: 'number' }),
    available_disk_space: bigint('available_disk_space', { mode: 'number' }),

    // I/O
    read_bytes: bigint('read_bytes', { mode: 'number' }),
    written_bytes: bigint('written_bytes', { mode: 'number' }),

    // Temperature
    temperatures: text('temperatures'), // JSON string
  },
  config,
);
