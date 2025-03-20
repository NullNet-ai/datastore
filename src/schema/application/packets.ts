import {
  pgTable,
  text,
  integer,
  primaryKey,
  bigint,
  index,
  // unique,
} from 'drizzle-orm/pg-core';
import {
  system_fields,
  getConfigDefaults,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';
import { uuid, timestamp } from 'drizzle-orm/pg-core';

const config = (table) => ({
  pk: primaryKey({ columns: [table.id, table.timestamp] }),
  ...getConfigDefaults.defaultIndexes('packets', table),
  packets_total_length_idx: index('packets_total_length_idx').on(
    table.total_length,
  ),
});

export const table = pgTable(
  'packets',
  {
    ...system_fields,
    id: uuid('id'), // Primary key
    timestamp: timestamp('timestamp', { withTimezone: true }), // Packet Capture Time
    hypertable_timestamp: text('hypertable_timestamp'), // Hypertable timestamp
    interface_name: text('interface_name'), // Network Interface Name

    total_length: integer('total_length'), // Total packet length in bytes

    source_mac: text('source_mac'), // Source MAC Address
    destination_mac: text('destination_mac'), // Destination MAC Address
    ether_type: text('ether_type'), // Ethernet Type

    ip_header_length: integer('ip_header_length'), // IP Header Length
    payload_length: integer('payload_length'), // Payload Length

    protocol: text('protocol'), // Protocol (TCP, UDP, ICMP, etc.)
    source_ip: text('source_ip'), // Source IP Address
    destination_ip: text('destination_ip'), // Destination IP Address

    source_port: integer('source_port'), // Source Port (Optional)
    destination_port: integer('destination_port'), // Destination Port (Optional)

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
    order: text('order'),
  },
  config,
);
