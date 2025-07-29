-- Your SQL goes here
ALTER TABLE "user_roles" 
ADD COLUMN "sync_status" text DEFAULT 'in-process';
ALTER TABLE "user_roles" 
ADD COLUMN "is_batch" boolean DEFAULT false;

ALTER TABLE "external_contacts" 
ADD COLUMN "sync_status" text DEFAULT 'in-process';
ALTER TABLE "external_contacts" 
ADD COLUMN "is_batch" boolean DEFAULT false;

ALTER TABLE "organizations" 
ADD COLUMN "sync_status" text DEFAULT 'in-process';
ALTER TABLE "organizations" 
ADD COLUMN "is_batch" boolean DEFAULT false;

ALTER TABLE "organization_contacts" 
ADD COLUMN "sync_status" text DEFAULT 'in-process';
ALTER TABLE "organization_contacts" 
ADD COLUMN "is_batch" boolean DEFAULT false;

ALTER TABLE "organization_accounts" 
ADD COLUMN "sync_status" text DEFAULT 'in-process';
ALTER TABLE "organization_accounts" 
ADD COLUMN "is_batch" boolean DEFAULT false;

ALTER TABLE "account_organizations" 
ADD COLUMN "sync_status" text DEFAULT 'in-process';
ALTER TABLE "account_organizations" 
ADD COLUMN "is_batch" boolean DEFAULT false;

ALTER TABLE "account_profiles" 
ADD COLUMN "sync_status" text DEFAULT 'in-process';
ALTER TABLE "account_profiles" 
ADD COLUMN "is_batch" boolean DEFAULT false;

ALTER TABLE "organization_domains" 
ADD COLUMN "sync_status" text DEFAULT 'in-process';
ALTER TABLE "organization_domains" 
ADD COLUMN "is_batch" boolean DEFAULT false;

ALTER TABLE "addresses" 
ADD COLUMN "sync_status" text DEFAULT 'in-process';
ALTER TABLE "addresses" 
ADD COLUMN "is_batch" boolean DEFAULT false;

ALTER TABLE "app_firewalls" 
ADD COLUMN "sync_status" text DEFAULT 'in-process';
ALTER TABLE "app_firewalls" 
ADD COLUMN "is_batch" boolean DEFAULT false;

ALTER TABLE "appguard_logs" 
ADD COLUMN "sync_status" text DEFAULT 'in-process';
ALTER TABLE "appguard_logs" 
ADD COLUMN "is_batch" boolean DEFAULT false;

ALTER TABLE "temp_appguard_logs" 
ADD COLUMN "sync_status" text DEFAULT 'in-process';
ALTER TABLE "temp_appguard_logs" 
ADD COLUMN "is_batch" boolean DEFAULT false;

ALTER TABLE "device_aliases" 
ADD COLUMN "sync_status" text DEFAULT 'in-process';
ALTER TABLE "device_aliases" 
ADD COLUMN "is_batch" boolean DEFAULT false;

ALTER TABLE "temp_device_aliases" 
ADD COLUMN "sync_status" text DEFAULT 'in-process';
ALTER TABLE "temp_device_aliases" 
ADD COLUMN "is_batch" boolean DEFAULT false;

ALTER TABLE "device_configurations" 
ADD COLUMN "sync_status" text DEFAULT 'in-process';
ALTER TABLE "device_configurations" 
ADD COLUMN "is_batch" boolean DEFAULT false;

ALTER TABLE "device_interface_addresses" 
ADD COLUMN "sync_status" text DEFAULT 'in-process';
ALTER TABLE "device_interface_addresses" 
ADD COLUMN "is_batch" boolean DEFAULT false;

ALTER TABLE "temp_device_interface_addresses" 
ADD COLUMN "sync_status" text DEFAULT 'in-process';
ALTER TABLE "temp_device_interface_addresses" 
ADD COLUMN "is_batch" boolean DEFAULT false;

ALTER TABLE "device_interfaces" 
ADD COLUMN "sync_status" text DEFAULT 'in-process';
ALTER TABLE "device_interfaces" 
ADD COLUMN "is_batch" boolean DEFAULT false;

ALTER TABLE "temp_device_interfaces" 
ADD COLUMN "sync_status" text DEFAULT 'in-process';
ALTER TABLE "temp_device_interfaces" 
ADD COLUMN "is_batch" boolean DEFAULT false;

ALTER TABLE "device_remote_access_sessions" 
ADD COLUMN "sync_status" text DEFAULT 'in-process';
ALTER TABLE "device_remote_access_sessions" 
ADD COLUMN "is_batch" boolean DEFAULT false;

ALTER TABLE "temp_device_remote_access_sessions" 
ADD COLUMN "sync_status" text DEFAULT 'in-process';
ALTER TABLE "temp_device_remote_access_sessions" 
ADD COLUMN "is_batch" boolean DEFAULT false;

ALTER TABLE "device_rules" 
ADD COLUMN "sync_status" text DEFAULT 'in-process';
ALTER TABLE "device_rules" 
ADD COLUMN "is_batch" boolean DEFAULT false;

ALTER TABLE "temp_device_rules" 
ADD COLUMN "sync_status" text DEFAULT 'in-process';
ALTER TABLE "temp_device_rules" 
ADD COLUMN "is_batch" boolean DEFAULT false;

ALTER TABLE "packets" 
ADD COLUMN "sync_status" text DEFAULT 'in-process';
ALTER TABLE "packets" 
ADD COLUMN "is_batch" boolean DEFAULT false;

ALTER TABLE "temp_packets" 
ADD COLUMN "sync_status" text DEFAULT 'in-process';
ALTER TABLE "temp_packets" 
ADD COLUMN "is_batch" boolean DEFAULT false;

ALTER TABLE "device_ssh_keys" 
ADD COLUMN "sync_status" text DEFAULT 'in-process';
ALTER TABLE "device_ssh_keys" 
ADD COLUMN "is_batch" boolean DEFAULT false;

ALTER TABLE "devices" 
ADD COLUMN "sync_status" text DEFAULT 'in-process';
ALTER TABLE "devices" 
ADD COLUMN "is_batch" boolean DEFAULT false;

ALTER TABLE "ip_infos" 
ADD COLUMN "sync_status" text DEFAULT 'in-process';
ALTER TABLE "ip_infos" 
ADD COLUMN "is_batch" boolean DEFAULT false;

ALTER TABLE "postgres_channels" 
ADD COLUMN "sync_status" text DEFAULT 'in-process';
ALTER TABLE "postgres_channels" 
ADD COLUMN "is_batch" boolean DEFAULT false;

ALTER TABLE "resolutions" 
ADD COLUMN "sync_status" text DEFAULT 'in-process';
ALTER TABLE "resolutions" 
ADD COLUMN "is_batch" boolean DEFAULT false;

ALTER TABLE "wallguard_logs" 
ADD COLUMN "sync_status" text DEFAULT 'in-process';
ALTER TABLE "wallguard_logs" 
ADD COLUMN "is_batch" boolean DEFAULT false;

ALTER TABLE "temp_wallguard_logs" 
ADD COLUMN "sync_status" text DEFAULT 'in-process';
ALTER TABLE "temp_wallguard_logs" 
ADD COLUMN "is_batch" boolean DEFAULT false;

ALTER TABLE "device_group_settings" 
ADD COLUMN "sync_status" text DEFAULT 'in-process';
ALTER TABLE "device_group_settings" 
ADD COLUMN "is_batch" boolean DEFAULT false;

ALTER TABLE "contacts" 
ADD COLUMN "sync_status" text DEFAULT 'in-process';
ALTER TABLE "contacts" 
ADD COLUMN "is_batch" boolean DEFAULT false;

ALTER TABLE "contact_phone_numbers" 
ADD COLUMN "sync_status" text DEFAULT 'in-process';
ALTER TABLE "contact_phone_numbers" 
ADD COLUMN "is_batch" boolean DEFAULT false;

ALTER TABLE "contact_emails" 
ADD COLUMN "sync_status" text DEFAULT 'in-process';
ALTER TABLE "contact_emails" 
ADD COLUMN "is_batch" boolean DEFAULT false;