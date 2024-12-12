import {
  pgTable,
  text,
  integer,
  primaryKey,
  serial,
} from 'drizzle-orm/pg-core';
import { uuid, timestamp } from 'drizzle-orm/pg-core';

const config = (table) => ({
  pk: primaryKey({ columns: [table.id, table.timestamp] }),
});

export const table = pgTable(
  'packets',
  {
    id: serial('id'),
    firewall_uuid: uuid('firewall_uuid'), // Firewall UUID String
    network_interface: text('network_interface'), // Network Interface name
    timestamp: timestamp('timestamp', { withTimezone: true }).notNull(), // Epoch timestamp when the packet was captured, // Epoch timestamp (int32)
    mac_source: text('mac_source'), // Source MAC address
    mac_destination: text('mac_destination'), // Destination MAC address
    eth_type: text('eth_type'), // Ethernet type (String)
    ip_version: integer('ip_version'), // IP version (Number)
    header_length: integer('header_length'), // Header Length (bytes)
    total_length: integer('total_length'), // Total Length (bytes)
    protocol: text('protocol'), // Protocol (String, e.g., TCP/UDP)
    ip_source: text('ip_source'), // Source IP address
    ip_destination: text('ip_destination'), // Destination IP address
    source_port: integer('source_port'), // Source Port (Number)
    destination_port: integer('destination_port'), // Destination Port (Number)
    tombstone: integer('tombstone').default(0), // Tombstone flag
  },
  config,
);
