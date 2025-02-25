// define the schema for the application
export { table as samples } from '../application/samples';
export { table as files } from '../application/files';
export { table as packets } from '../application/packets'; //hypertable
export { table as temp_packets } from '../application/temp_packets';
export { table as dead_letter_queue } from '../application/dead_letter_queue';
export { table as wallguard_logs } from '../application/wallguard_logs';