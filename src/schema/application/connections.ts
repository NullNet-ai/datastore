import {
  pgTable,
  text,
  integer,
  // primaryKey,
  timestamp,
  AnyPgColumn,
  index,
  inet,
  // unique,
} from 'drizzle-orm/pg-core';
import {
  system_fields,
  getConfigDefaults,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';
import { table as devices } from './devices';

const fields = 
  {
    timestamp: timestamp('timestamp', { withTimezone: true }), // Connection Time
    interface_name: text('interface_name'), // Network Interface Name

    total_packet: integer('total_packet'), // Total packets in the connection
    total_byte: integer('total_byte'), // Total bytes in the connection
    device_id: text('device_id').references(() => devices.id as AnyPgColumn), // Device ID

    protocol: text('protocol'), // Protocol (TCP, UDP, ICMP, etc.)
    source_ip: inet('source_ip'), // Source IP Address
    destination_ip: inet('destination_ip'), // Destination IP Address
    remote_ip: inet('remote_ip'), // Remote IP Address

    source_port: integer('source_port'), // Source Port (Optional)
    destination_port: integer('destination_port'), // Destination Port (Optional)
  }


const config = (table) => ({
  // pk: primaryKey({ columns: [table.id, table.timestamp] }),
  ...getConfigDefaults.defaultIndexes('connections', table),
  ...Object.keys(fields).reduce((acc, field) => {
    const index_name = `connections_${field}_idx`;
    return {
      ...acc,
      [index_name]: index(index_name).on(table[field]),
    };
  }, {}),
});

export const table = pgTable(
  'connections',
  {
    ...system_fields,
    ...fields,
    hypertable_timestamp: text('hypertable_timestamp'), // Hypertable timestamp
  },
  config,
);
