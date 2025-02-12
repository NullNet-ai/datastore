import {
  pgTable,
  text,
  integer,
  primaryKey,
  // unique,
} from 'drizzle-orm/pg-core';
import { system_fields } from '@dna-platform/crdt-lww-postgres/build/schema/system';
import { uuid, timestamp } from 'drizzle-orm/pg-core';
import { bigint } from 'drizzle-orm/pg-core/index';

const config = (table) => ({
  pk: primaryKey({ columns: [table.id] }),
});

export const table = pgTable(
  'temp_packets',
  {
    ...system_fields,
    id: uuid('id'), // Primary key
    timestamp: timestamp('timestamp', { withTimezone: true }).notNull(), // Packet Capture Time
    hypertable_timestamp: text('hypertable_timestamp'), // Hypertable timestamp
    interface_name: text('interface_name').notNull(), // Network Interface Name

    total_length: integer('total_length'), // Total packet length in bytes
    device_id: text(), // Device ID

    source_mac: text('source_mac'), // Source MAC Address
    destination_mac: text('destination_mac'), // Destination MAC Address
    ether_type: text('ether_type'), // Ethernet Type

    ip_header_length: integer('ip_header_length').notNull(), // IP Header Length
    payload_length: integer('payload_length').notNull(), // Payload Length

    protocol: text('protocol').notNull(), // Protocol (TCP, UDP, ICMP, etc.)
    source_ip: text('source_ip').notNull(), // Source IP Address
    destination_ip: text('destination_ip').notNull(), // Destination IP Address

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
  },
  config,
);
