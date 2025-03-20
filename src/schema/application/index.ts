// define the schema for the application
export { table as samples } from '../application/samples';
export { table as files } from '../application/files';
export { table as packets } from '../application/packets'; //hypertable
export { table as temp_packets } from '../application/temp_packets';
export { table as dead_letter_queue } from '../application/dead_letter_queue';
export { table as wallguard_logs } from '../application/wallguard_logs';
export { table as temp_wallguard_logs } from '../application/temp_wallguard_logs';
export { table as websites } from '../application/websites';
export { table as page_fixes } from '../application/page_fixes';
export { table as approvals } from '../application/approvals';
export { table as audit_urls } from '../application/audit_urls';
export { table as crawls } from '../application/crawls';
export { table as audits } from '../application/audits';
export { table as audit_scopes } from '../application/audit_scopes';
export { table as accessibility_reports } from '../application/accessibility_reports';
export { table as accessibility_scans } from '../application/accessibility_scans';
export { table as pages } from '../application/pages';
export { table as patches } from '../application/patches';
