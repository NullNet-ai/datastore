-- This file should undo anything in `up.sql`
ALTER TABLE "user_roles" DROP COLUMN "sync_status";
ALTER TABLE "user_roles" DROP COLUMN "is_batch";

ALTER TABLE "external_contacts" DROP COLUMN "sync_status";
ALTER TABLE "external_contacts" DROP COLUMN "is_batch";

ALTER TABLE "organizations" DROP COLUMN "sync_status";
ALTER TABLE "organizations" DROP COLUMN "is_batch";

ALTER TABLE "organization_contacts" DROP COLUMN "sync_status";
ALTER TABLE "organization_contacts" DROP COLUMN "is_batch";

ALTER TABLE "organization_accounts" DROP COLUMN "sync_status";
ALTER TABLE "organization_accounts" DROP COLUMN "is_batch";

ALTER TABLE "account_organizations" DROP COLUMN "sync_status";
ALTER TABLE "account_organizations" DROP COLUMN "is_batch";

ALTER TABLE "account_profiles" DROP COLUMN "sync_status";
ALTER TABLE "account_profiles" DROP COLUMN "is_batch";

ALTER TABLE "organization_domains" DROP COLUMN "sync_status";
ALTER TABLE "organization_domains" DROP COLUMN "is_batch";

ALTER TABLE "addresses" DROP COLUMN "sync_status";
ALTER TABLE "addresses" DROP COLUMN "is_batch";

ALTER TABLE "app_firewalls" DROP COLUMN "sync_status";
ALTER TABLE "app_firewalls" DROP COLUMN "is_batch";

ALTER TABLE "appguard_logs" DROP COLUMN "sync_status";
ALTER TABLE "appguard_logs" DROP COLUMN "is_batch";

ALTER TABLE "temp_appguard_logs" DROP COLUMN "sync_status";
ALTER TABLE "temp_appguard_logs" DROP COLUMN "is_batch";

ALTER TABLE "device_aliases" DROP COLUMN "sync_status";
ALTER TABLE "device_aliases" DROP COLUMN "is_batch";

ALTER TABLE "temp_device_aliases" DROP COLUMN "sync_status";
ALTER TABLE "temp_device_aliases" DROP COLUMN "is_batch";

ALTER TABLE "device_configurations" DROP COLUMN "sync_status";
ALTER TABLE "device_configurations" DROP COLUMN "is_batch";

ALTER TABLE "device_interface_addresses" DROP COLUMN "sync_status";
ALTER TABLE "device_interface_addresses" DROP COLUMN "is_batch";

ALTER TABLE "temp_device_interface_addresses" DROP COLUMN "sync_status";
ALTER TABLE "temp_device_interface_addresses" DROP COLUMN "is_batch";

ALTER TABLE "device_interfaces" DROP COLUMN "sync_status";
ALTER TABLE "device_interfaces" DROP COLUMN "is_batch";

ALTER TABLE "temp_device_interfaces" DROP COLUMN "sync_status";
ALTER TABLE "temp_device_interfaces" DROP COLUMN "is_batch";

ALTER TABLE "device_remote_access_sessions" DROP COLUMN "sync_status";
ALTER TABLE "device_remote_access_sessions" DROP COLUMN "is_batch";

ALTER TABLE "temp_device_remote_access_sessions" DROP COLUMN "sync_status";
ALTER TABLE "temp_device_remote_access_sessions" DROP COLUMN "is_batch";

ALTER TABLE "device_rules" DROP COLUMN "sync_status";
ALTER TABLE "device_rules" DROP COLUMN "is_batch";

ALTER TABLE "temp_device_rules" DROP COLUMN "sync_status";
ALTER TABLE "temp_device_rules" DROP COLUMN "is_batch";

ALTER TABLE "packets" DROP COLUMN "sync_status";
ALTER TABLE "packets" DROP COLUMN "is_batch";

ALTER TABLE "temp_packets" DROP COLUMN "sync_status";
ALTER TABLE "temp_packets" DROP COLUMN "is_batch";

ALTER TABLE "device_ssh_keys" DROP COLUMN "sync_status";
ALTER TABLE "device_ssh_keys" DROP COLUMN "is_batch";

ALTER TABLE "devices" DROP COLUMN "sync_status";
ALTER TABLE "devices" DROP COLUMN "is_batch";

ALTER TABLE "ip_infos" DROP COLUMN "sync_status";
ALTER TABLE "ip_infos" DROP COLUMN "is_batch";

ALTER TABLE "postgres_channels" DROP COLUMN "sync_status";
ALTER TABLE "postgres_channels" DROP COLUMN "is_batch";

ALTER TABLE "resolutions" DROP COLUMN "sync_status";
ALTER TABLE "resolutions" DROP COLUMN "is_batch";

ALTER TABLE "wallguard_logs" DROP COLUMN "sync_status";
ALTER TABLE "wallguard_logs" DROP COLUMN "is_batch";

ALTER TABLE "temp_wallguard_logs" DROP COLUMN "sync_status";
ALTER TABLE "temp_wallguard_logs" DROP COLUMN "is_batch";

ALTER TABLE "device_group_settings" DROP COLUMN "sync_status";
ALTER TABLE "device_group_settings" DROP COLUMN "is_batch";

ALTER TABLE "contacts" DROP COLUMN "sync_status";
ALTER TABLE "contacts" DROP COLUMN "is_batch";

ALTER TABLE "contact_phone_numbers" DROP COLUMN "sync_status";
ALTER TABLE "contact_phone_numbers" DROP COLUMN "is_batch";

ALTER TABLE "contact_emails" DROP COLUMN "sync_status";
ALTER TABLE "contact_emails" DROP COLUMN "is_batch";