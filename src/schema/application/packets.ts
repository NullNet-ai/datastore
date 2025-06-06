import {
  pgTable,
  text,
  integer,
  primaryKey,
  timestamp,
  AnyPgColumn,
  bigint,
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
    timestamp: timestamp('timestamp', { withTimezone: true }), // Packet Capture Time
    interface_name: text('interface_name'), // Network Interface Name

    total_length: integer('total_length'), // Total packet length in bytes
    device_id: text('device_id').references(() => devices.id as AnyPgColumn), // Device ID

    ether_type: text('ether_type'), // Ethernet Type

    protocol: text('protocol'), // Protocol (TCP, UDP, ICMP, etc.)
    source_ip: inet('source_ip'), // Source IP Address
    destination_ip: inet('destination_ip'), // Destination IP Address
    remote_ip: inet('remote_ip'), // Remote IP Address

    source_port: integer('source_port'), // Source Port (Optional)
    destination_port: integer('destination_port'), // Destination Port (Optional)
  }


const config = (table) => ({
  pk: primaryKey({ columns: [table.id, table.timestamp] }),
  ...getConfigDefaults.defaultIndexes('packets', table),
  // packets_total_length_idx: index('packets_total_length_idx').on(
  //   table.total_length,
  // ),
  ...Object.keys(fields).reduce((acc, field) => {
    const index_name = `packets_${field}_idx`;
    return {
      ...acc,
      [index_name]: index(index_name).on(table[field]),
    };
  }, {}),
});

export const table = pgTable(
  'packets',
  {
    ...system_fields,
    ...fields,
    id: text('id'), // Primary key
    hypertable_timestamp: text('hypertable_timestamp'), // Hypertable timestamp

    source_mac: text('source_mac'), // Source MAC Address
    destination_mac: text('destination_mac'), // Destination MAC Address

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
