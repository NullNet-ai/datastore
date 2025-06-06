import {
  pgTable,
  text,
  integer,
  primaryKey,
  timestamp,
  AnyPgColumn,
  inet,
  // unique,
} from 'drizzle-orm/pg-core';
import {
  system_fields,
  getConfigDefaults,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';
import { table as devices } from './devices';

const config = (table) => ({
  pk: primaryKey({ columns: [table.id, table.timestamp] }),
  ...getConfigDefaults.defaultIndexes('temp_connections', table),
});

export const table = pgTable(
  'temp_connections',
  {
    ...system_fields,
    id: text('id'), // Primary key
    timestamp: timestamp('timestamp', { withTimezone: true }), // Connection Time
    hypertable_timestamp: text('hypertable_timestamp'), // Hypertable timestamp
    interface_name: text('interface_name'), // Network Interface Name

    total_packet: integer('total_packet'), // Total packets in the connection
    total_byte: integer('total_byte'), // Total bytes in the connection
    device_id: text('device_id').references(() => devices.id as AnyPgColumn), // Device ID

    protocol: text('protocol'), // Protocol (TCP, UDP, ICMP, etc.)
    source_ip: inet('source_ip'), // Source IP Address
    destination_ip: inet('destination_ip'), // Destination IP Address

    source_port: integer('source_port'), // Source Port (Optional)
    destination_port: integer('destination_port'), // Destination Port (Optional)
    remote_ip: text('remote_ip'), // Remote IP Address
  },
  config,
);
