// define the schema for the application
export { table as files } from './files';
export { table as contacts } from './contacts';
export { table as contact_phone_numbers } from './contact_phone_numbers';
export { table as user_roles } from './user_roles';
export { table as organization_contacts } from './organization_contacts';
export { table as organization_contact_user_roles } from './organization_contact_user_roles';
// export { table as organization_accounts } from './organization_accounts';
export { table as notifications } from './notifications';
export { table as invitations } from './invitations';
export { table as grid_filters } from './grid_filters';
export { table as communication_templates } from './communication_templates';
export { table as locations } from './locations';
// define the schema for the application
export { table as devices } from "./devices";
export { table as samples } from '../application/samples';
export { table as connections } from '../application/connections'; //hypertable
export { table as packets } from '../application/packets'; //hypertable
export { table as dummy_packets } from '../application/dummy_packets'; //hypertable
export { table as temp_packets } from '../application/temp_packets';
export { table as temp_connections } from '../application/temp_connections';
export { table as device_group_settings } from '../application/device_group_settings';
export { table as device_groups } from '../application/device_groups';
export { table as device_heartbeats } from '../application/device_heartbeats'; //hypertable
export { table as device_configurations } from '../application/device_configurations';

export { table as device_filter_rules } from '../application/device_filter_rules';
export { table as temp_device_filter_rules } from '../application/temp_device_filter_rules';

export { table as device_nat_rules } from '../application/device_nat_rules';
export { table as temp_device_nat_rules } from '../application/temp_device_nat_rules';

export { table as aliases } from '../application/aliases';
export { table as temp_aliases } from '../application/temp_aliases';
export { table as ip_aliases } from '../application/ip_aliases';
export { table as temp_ip_aliases } from '../application/temp_ip_aliases';
export { table as port_aliases } from '../application/port_aliases';
export { table as temp_port_aliases } from '../application/temp_port_aliases';
export { table as device_interfaces } from '../application/device_interfaces';
export { table as temp_device_interfaces } from '../application/temp_device_interfaces';
export { table as dead_letter_queue } from '../application/dead_letter_queue';
export { table as wallguard_logs } from '../application/wallguard_logs';
export { table as temp_wallguard_logs } from '../application/temp_wallguard_logs';
export { table as appguard_logs } from '../application/appguard_logs';
export { table as appguard_configs } from '../application/appguard_configs';
export { table as temp_appguard_logs } from '../application/temp_appguard_logs';
export { table as device_interface_addresses } from '../application/device_interface_addresses';
export { table as temp_device_interface_addresses } from '../application/temp_device_interface_addresses';
export { table as device_remote_access_sessions } from '../application/device_remote_access_sessions';
export { table as temp_device_remote_access_sessions } from '../application/temp_device_remote_access_sessions';
export { table as device_services } from "../application/device_services";
export { table as temp_device_services } from "../application/temp_device_services";
export { table as tcp_connections } from '../application/tcp_connections';
export { table as ip_infos } from './ip_infos';
export { table as app_firewalls } from './app_firewalls';
export { table as http_requests } from '../application/http_requests';
export { table as http_responses } from '../application/http_responses';
export { table as smtp_requests } from '../application/smtp_requests';
export { table as smtp_responses } from '../application/smtp_responses';
export { table as postgres_channels } from '../application/postgres_channels';
export { table as resolutions } from '../application/resolutions'; // new entity
export { table as device_ssh_keys } from '../application/device_ssh_keys';
export { table as system_resources } from './system_resources';
export { table as temp_system_resources } from './temp_system_resources';
export { table as setup_instructions } from "./setup_instructions";
export { table as installation_codes } from "./installation_codes";
export { table as device_instances } from "./device_instances"; 
// export { table as data_permissions } from '../application/data_permissions';
