import {
  pgTable,
  text,
  integer,
  primaryKey,
  timestamp,
  AnyPgColumn,
  bigint,
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
  ...getConfigDefaults.defaultIndexes('temp_packets', table),
});

export const table = pgTable(
  'temp_packets',
  {
    ...system_fields,
    id: text('id'), // Primary key
    timestamp: timestamp('timestamp', { withTimezone: true }), // Packet Capture Time
    hypertable_timestamp: text('hypertable_timestamp'), // Hypertable timestamp
    interface_name: text('interface_name'), // Network Interface Name

    total_length: integer('total_length'), // Total packet length in bytes
    device_id: text('device_id').references(() => devices.id as AnyPgColumn), // Device ID

    source_mac: text('source_mac'), // Source MAC Address
    destination_mac: text('destination_mac'), // Destination MAC Address
    ether_type: text('ether_type'), // Ethernet Type

    protocol: text('protocol'), // Protocol (TCP, UDP, ICMP, etc.)
    source_ip: inet('source_ip'), // Source IP Address
    destination_ip: inet('destination_ip'), // Destination IP Address

    source_port: integer('source_port'), // Source Port (Optional)
    destination_port: integer('destination_port'), // Destination Port (Optional)
    remote_ip: text('remote_ip'), // Remote IP Address

    tcp_header_length: integer('tcp_header_length'), // TCP Header Length
    tcp_sequence_number: bigint('tcp_sequence_number', { mode: 'number' }), // TCP Sequence Number
    tcp_acknowledgment_number: bigint('tcp_acknowledgment_number', {
      mode: 'number',
    }), // TCP Acknowledgment Number
    tcp_data_offset: integer('tcp_data_offset'), // TCP Data Offset
    tcp_flags: integer('tcp_flags'), // TCP Flags
    tcp_window_size: integer('tcp_window_size'), // TCP Window Size
    tcp_urgent_pointer: integer('tcp_urgent_pointer'), // TCP Urgent Pointer

    icmp_type: integer('icmp_type'), // ICMP Type
    icmp_code: integer('icmp_code'), // ICMP Code
  },
  config,
);
