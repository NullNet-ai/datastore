import { pgTable, text, boolean} from 'drizzle-orm/pg-core';
import * as path from 'path';
import {
  fileRegex,
  getConfigDefaults,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';
// import { table as addresses } from './addresses';

const filename = path.basename(__filename).replace(fileRegex, '');
const config = getConfigDefaults.byIndex(filename);
export const table = pgTable(
  filename,
  {
    ...system_fields,
    id: text('id').primaryKey(),
    is_traffic_monitoring_enabled: boolean('is_traffic_monitoring_enabled').default(false),
    is_config_monitoring_enabled: boolean('is_config_monitoring_enabled').default(false),
    is_telemetry_monitoring_enabled: boolean('is_telemetry_monitoring_enabled').default(false),
    is_device_authorized: boolean('is_device_authorized').default(false),
    
    device_uuid: text('device_uuid').default(""),
    device_name: text("device_name").default(""),
    device_category: text("device_category").default(""),
    device_type: text("device_type").default(""),
    device_os: text("device_os").default(""),

    is_device_online: boolean("is_device_online").default(false),
    // address_id: text('address_id').references(
    //   () => addresses.id as AnyPgColumn,
    // ),
    // instance_name: text('instance_name'),
    // is_connection_established: boolean('is_connection_established').default(
    //   false,
    // ),
    // system_id: text('system_id'),
    // device_version: text('device_version'),
    // last_heartbeat: text('last_heartbeat'),
    // is_monitoring_enabled: boolean('is_monitoring_enabled').default(true),
    // is_remote_access_enabled: boolean('is_remote_access_enabled').default(true),
    // is_resource_monitoring_enabled: boolean('is_resource_monitoring_enabled').default(true),
    // is_packet_capture_enabled: boolean('is_packet_capture_enabled').default(true),
    // ip_address: inet('ip_address'),
    // device_status: text('device_status'),
    // device_gui_protocol: text('device_gui_protocol')
  },
  config,
);
