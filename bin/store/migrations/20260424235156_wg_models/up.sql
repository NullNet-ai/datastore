-- Your SQL goes here

CREATE TABLE "ip_aliases" (
    "alias_id" TEXT,
    "ip" TEXT,
    "prefix" INTEGER,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_ip_aliases_tombstone" ON "ip_aliases" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_ip_aliases_status" ON "ip_aliases" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_ip_aliases_previous_status" ON "ip_aliases" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_ip_aliases_version" ON "ip_aliases" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_ip_aliases_created_date" ON "ip_aliases" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_ip_aliases_updated_date" ON "ip_aliases" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_ip_aliases_organization_id" ON "ip_aliases" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_ip_aliases_created_by" ON "ip_aliases" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_ip_aliases_updated_by" ON "ip_aliases" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_ip_aliases_deleted_by" ON "ip_aliases" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_ip_aliases_requested_by" ON "ip_aliases" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_ip_aliases_tags" ON "ip_aliases" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_ip_aliases_categories" ON "ip_aliases" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_ip_aliases_code" ON "ip_aliases" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_ip_aliases_sensitivity_level" ON "ip_aliases" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE TABLE "smtp_responses" (
    "fw_policy" TEXT,
    "fw_reasons" TEXT,
    "ip" TEXT,
    "response_code" BIGINT,
    "time" BIGINT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_smtp_responses_tombstone" ON "smtp_responses" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_smtp_responses_status" ON "smtp_responses" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_smtp_responses_previous_status" ON "smtp_responses" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_smtp_responses_version" ON "smtp_responses" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_smtp_responses_created_date" ON "smtp_responses" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_smtp_responses_updated_date" ON "smtp_responses" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_smtp_responses_organization_id" ON "smtp_responses" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_smtp_responses_created_by" ON "smtp_responses" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_smtp_responses_updated_by" ON "smtp_responses" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_smtp_responses_deleted_by" ON "smtp_responses" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_smtp_responses_requested_by" ON "smtp_responses" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_smtp_responses_tags" ON "smtp_responses" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_smtp_responses_categories" ON "smtp_responses" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_smtp_responses_code" ON "smtp_responses" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_smtp_responses_sensitivity_level" ON "smtp_responses" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE TABLE "appguard_logs" (
    "level" TEXT,
    "message" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_appguard_logs_tombstone" ON "appguard_logs" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_appguard_logs_status" ON "appguard_logs" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_appguard_logs_previous_status" ON "appguard_logs" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_appguard_logs_version" ON "appguard_logs" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_appguard_logs_created_date" ON "appguard_logs" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_appguard_logs_updated_date" ON "appguard_logs" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_appguard_logs_organization_id" ON "appguard_logs" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_appguard_logs_created_by" ON "appguard_logs" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_appguard_logs_updated_by" ON "appguard_logs" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_appguard_logs_deleted_by" ON "appguard_logs" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_appguard_logs_requested_by" ON "appguard_logs" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_appguard_logs_tags" ON "appguard_logs" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_appguard_logs_categories" ON "appguard_logs" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_appguard_logs_code" ON "appguard_logs" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_appguard_logs_sensitivity_level" ON "appguard_logs" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE TABLE "packets" (
    "interface_name" TEXT,
    "total_length" INTEGER,
    "device_id" TEXT,
    "ether_type" TEXT,
    "protocol" TEXT,
    "source_ip" TEXT,
    "destination_ip" TEXT,
    "remote_ip" TEXT,
    "source_port" INTEGER,
    "destination_port" INTEGER,
    "hypertable_timestamp" TEXT,
    "source_mac" TEXT,
    "destination_mac" TEXT,
    "tcp_header_length" INTEGER,
    "tcp_sequence_number" BIGINT,
    "tcp_acknowledgment_number" BIGINT,
    "tcp_data_offset" INTEGER,
    "tcp_flags" INTEGER,
    "tcp_window_size" INTEGER,
    "tcp_urgent_pointer" INTEGER,
    "icmp_type" INTEGER,
    "icmp_code" INTEGER,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_packets_tombstone" ON "packets" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_packets_status" ON "packets" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_packets_previous_status" ON "packets" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_packets_version" ON "packets" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_packets_created_date" ON "packets" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_packets_updated_date" ON "packets" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_packets_organization_id" ON "packets" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_packets_created_by" ON "packets" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_packets_updated_by" ON "packets" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_packets_deleted_by" ON "packets" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_packets_requested_by" ON "packets" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_packets_tags" ON "packets" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_packets_categories" ON "packets" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_packets_code" ON "packets" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_packets_sensitivity_level" ON "packets" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_packets_total_length" ON "packets" USING btree("total_length");
--> statement-breakpoint
CREATE TABLE "temp_device_interface_addresses" (
    "device_interface_id" TEXT,
    "address" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_temp_device_interface_addresses_tombstone" ON "temp_device_interface_addresses" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_interface_addresses_status" ON "temp_device_interface_addresses" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_interface_addresses_previous_status" ON "temp_device_interface_addresses" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_interface_addresses_version" ON "temp_device_interface_addresses" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_interface_addresses_created_date" ON "temp_device_interface_addresses" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_interface_addresses_updated_date" ON "temp_device_interface_addresses" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_interface_addresses_organization_id" ON "temp_device_interface_addresses" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_interface_addresses_created_by" ON "temp_device_interface_addresses" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_interface_addresses_updated_by" ON "temp_device_interface_addresses" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_interface_addresses_deleted_by" ON "temp_device_interface_addresses" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_interface_addresses_requested_by" ON "temp_device_interface_addresses" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_interface_addresses_tags" ON "temp_device_interface_addresses" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_interface_addresses_categories" ON "temp_device_interface_addresses" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_interface_addresses_code" ON "temp_device_interface_addresses" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_interface_addresses_sensitivity_level" ON "temp_device_interface_addresses" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE TABLE "http_responses" (
    "fw_policy" TEXT,
    "fw_reasons" TEXT,
    "ip" TEXT,
    "response_code" BIGINT,
    "headers" TEXT,
    "time" BIGINT,
    "size" BIGINT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_http_responses_tombstone" ON "http_responses" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_http_responses_status" ON "http_responses" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_http_responses_previous_status" ON "http_responses" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_http_responses_version" ON "http_responses" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_http_responses_created_date" ON "http_responses" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_http_responses_updated_date" ON "http_responses" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_http_responses_organization_id" ON "http_responses" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_http_responses_created_by" ON "http_responses" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_http_responses_updated_by" ON "http_responses" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_http_responses_deleted_by" ON "http_responses" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_http_responses_requested_by" ON "http_responses" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_http_responses_tags" ON "http_responses" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_http_responses_categories" ON "http_responses" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_http_responses_code" ON "http_responses" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_http_responses_sensitivity_level" ON "http_responses" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE TABLE "system_resources" (
    "num_cpus" INTEGER,
    "global_cpu_usage" TEXT,
    "cpu_usages" TEXT,
    "total_memory" BIGINT,
    "used_memory" BIGINT,
    "total_disk_space" BIGINT,
    "available_disk_space" BIGINT,
    "read_bytes" BIGINT,
    "written_bytes" BIGINT,
    "temperatures" TEXT,
    "device_id" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_system_resources_tombstone" ON "system_resources" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_system_resources_status" ON "system_resources" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_system_resources_previous_status" ON "system_resources" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_system_resources_version" ON "system_resources" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_system_resources_created_date" ON "system_resources" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_system_resources_updated_date" ON "system_resources" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_system_resources_organization_id" ON "system_resources" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_system_resources_created_by" ON "system_resources" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_system_resources_updated_by" ON "system_resources" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_system_resources_deleted_by" ON "system_resources" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_system_resources_requested_by" ON "system_resources" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_system_resources_tags" ON "system_resources" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_system_resources_categories" ON "system_resources" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_system_resources_code" ON "system_resources" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_system_resources_sensitivity_level" ON "system_resources" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE TABLE "organization_contact_user_roles" (
    "organization_contact_id" TEXT,
    "user_role_id" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_organization_contact_user_roles_tombstone" ON "organization_contact_user_roles" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_organization_contact_user_roles_status" ON "organization_contact_user_roles" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_organization_contact_user_roles_previous_status" ON "organization_contact_user_roles" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_organization_contact_user_roles_version" ON "organization_contact_user_roles" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_organization_contact_user_roles_created_date" ON "organization_contact_user_roles" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_organization_contact_user_roles_updated_date" ON "organization_contact_user_roles" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_organization_contact_user_roles_organization_id" ON "organization_contact_user_roles" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_organization_contact_user_roles_created_by" ON "organization_contact_user_roles" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_organization_contact_user_roles_updated_by" ON "organization_contact_user_roles" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_organization_contact_user_roles_deleted_by" ON "organization_contact_user_roles" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_organization_contact_user_roles_requested_by" ON "organization_contact_user_roles" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_organization_contact_user_roles_tags" ON "organization_contact_user_roles" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_organization_contact_user_roles_categories" ON "organization_contact_user_roles" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_organization_contact_user_roles_code" ON "organization_contact_user_roles" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_organization_contact_user_roles_sensitivity_level" ON "organization_contact_user_roles" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE TABLE "temp_system_resources" (
    "num_cpus" INTEGER,
    "global_cpu_usage" TEXT,
    "cpu_usages" TEXT,
    "total_memory" BIGINT,
    "used_memory" BIGINT,
    "total_disk_space" BIGINT,
    "available_disk_space" BIGINT,
    "read_bytes" BIGINT,
    "written_bytes" BIGINT,
    "temperatures" TEXT,
    "device_id" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_temp_system_resources_tombstone" ON "temp_system_resources" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_temp_system_resources_status" ON "temp_system_resources" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_temp_system_resources_previous_status" ON "temp_system_resources" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_temp_system_resources_version" ON "temp_system_resources" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_temp_system_resources_created_date" ON "temp_system_resources" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_temp_system_resources_updated_date" ON "temp_system_resources" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_temp_system_resources_organization_id" ON "temp_system_resources" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_temp_system_resources_created_by" ON "temp_system_resources" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_system_resources_updated_by" ON "temp_system_resources" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_system_resources_deleted_by" ON "temp_system_resources" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_system_resources_requested_by" ON "temp_system_resources" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_system_resources_tags" ON "temp_system_resources" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_temp_system_resources_categories" ON "temp_system_resources" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_temp_system_resources_code" ON "temp_system_resources" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_temp_system_resources_sensitivity_level" ON "temp_system_resources" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE TABLE "device_configurations" (
    "device_id" TEXT,
    "digest" TEXT,
    "hostname" TEXT,
    "raw_content" TEXT,
    "config_version" INTEGER,
    "tables" TEXT[],
    "chains" TEXT[],
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_device_configurations_tombstone" ON "device_configurations" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_device_configurations_status" ON "device_configurations" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_device_configurations_previous_status" ON "device_configurations" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_device_configurations_version" ON "device_configurations" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_device_configurations_created_date" ON "device_configurations" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_device_configurations_updated_date" ON "device_configurations" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_device_configurations_organization_id" ON "device_configurations" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_device_configurations_created_by" ON "device_configurations" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_device_configurations_updated_by" ON "device_configurations" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_device_configurations_deleted_by" ON "device_configurations" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_device_configurations_requested_by" ON "device_configurations" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_device_configurations_tags" ON "device_configurations" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_device_configurations_categories" ON "device_configurations" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_device_configurations_code" ON "device_configurations" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_device_configurations_sensitivity_level" ON "device_configurations" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE TABLE "grid_filters" (
    "name" TEXT,
    "grid_id" TEXT,
    "link" TEXT,
    "is_current" BOOLEAN,
    "is_default" BOOLEAN,
    "contact_id" TEXT,
    "account_organization_id" TEXT,
    "entity" TEXT,
    "columns" JSONB,
    "groups" JSONB,
    "sorts" JSONB,
    "default_sorts" JSONB,
    "advance_filters" JSONB,
    "group_advance_filters" JSONB,
    "filter_groups" JSONB,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_grid_filters_tombstone" ON "grid_filters" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_grid_filters_status" ON "grid_filters" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_grid_filters_previous_status" ON "grid_filters" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_grid_filters_version" ON "grid_filters" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_grid_filters_created_date" ON "grid_filters" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_grid_filters_updated_date" ON "grid_filters" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_grid_filters_organization_id" ON "grid_filters" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_grid_filters_created_by" ON "grid_filters" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_grid_filters_updated_by" ON "grid_filters" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_grid_filters_deleted_by" ON "grid_filters" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_grid_filters_requested_by" ON "grid_filters" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_grid_filters_tags" ON "grid_filters" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_grid_filters_categories" ON "grid_filters" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_grid_filters_code" ON "grid_filters" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_grid_filters_sensitivity_level" ON "grid_filters" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_grid_filters_name" ON "grid_filters" USING btree("name");
--> statement-breakpoint
CREATE INDEX "idx_grid_filters_grid_id" ON "grid_filters" USING btree("grid_id");
--> statement-breakpoint
CREATE INDEX "idx_grid_filters_link" ON "grid_filters" USING btree("link");
--> statement-breakpoint
CREATE INDEX "idx_grid_filters_is_current" ON "grid_filters" USING btree("is_current");
--> statement-breakpoint
CREATE INDEX "idx_grid_filters_is_default" ON "grid_filters" USING btree("is_default");
--> statement-breakpoint
CREATE INDEX "idx_grid_filters_contact_id" ON "grid_filters" USING btree("contact_id");
--> statement-breakpoint
CREATE INDEX "idx_grid_filters_entity" ON "grid_filters" USING btree("entity");
--> statement-breakpoint
CREATE TABLE "connections" (
    "interface_name" TEXT,
    "total_packet" INTEGER,
    "total_byte" INTEGER,
    "device_id" TEXT,
    "protocol" TEXT,
    "source_ip" TEXT,
    "destination_ip" TEXT,
    "remote_ip" TEXT,
    "source_port" INTEGER,
    "destination_port" INTEGER,
    "hypertable_timestamp" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_connections_tombstone" ON "connections" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_connections_status" ON "connections" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_connections_previous_status" ON "connections" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_connections_version" ON "connections" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_connections_created_date" ON "connections" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_connections_updated_date" ON "connections" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_connections_organization_id" ON "connections" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_connections_created_by" ON "connections" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_connections_updated_by" ON "connections" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_connections_deleted_by" ON "connections" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_connections_requested_by" ON "connections" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_connections_tags" ON "connections" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_connections_categories" ON "connections" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_connections_code" ON "connections" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_connections_sensitivity_level" ON "connections" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_connections_device_id" ON "connections" USING btree("device_id");
--> statement-breakpoint
CREATE INDEX "idx_connections_device_id_source_ip" ON "connections" USING btree("device_id", "source_ip");
--> statement-breakpoint
CREATE TABLE "dummy_packets" (
    "interface_name" TEXT,
    "total_length" INTEGER,
    "device_id" TEXT,
    "ether_type" TEXT,
    "protocol" TEXT,
    "source_ip" TEXT,
    "destination_ip" TEXT,
    "remote_ip" TEXT,
    "source_port" INTEGER,
    "destination_port" INTEGER,
    "hypertable_timestamp" TEXT,
    "source_mac" TEXT,
    "destination_mac" TEXT,
    "tcp_header_length" INTEGER,
    "tcp_sequence_number" BIGINT,
    "tcp_acknowledgment_number" BIGINT,
    "tcp_data_offset" INTEGER,
    "tcp_flags" INTEGER,
    "tcp_window_size" INTEGER,
    "tcp_urgent_pointer" INTEGER,
    "icmp_type" INTEGER,
    "icmp_code" INTEGER,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_dummy_packets_tombstone" ON "dummy_packets" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_dummy_packets_status" ON "dummy_packets" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_dummy_packets_previous_status" ON "dummy_packets" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_dummy_packets_version" ON "dummy_packets" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_dummy_packets_created_date" ON "dummy_packets" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_dummy_packets_updated_date" ON "dummy_packets" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_dummy_packets_organization_id" ON "dummy_packets" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_dummy_packets_created_by" ON "dummy_packets" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_dummy_packets_updated_by" ON "dummy_packets" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_dummy_packets_deleted_by" ON "dummy_packets" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_dummy_packets_requested_by" ON "dummy_packets" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_dummy_packets_tags" ON "dummy_packets" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_dummy_packets_categories" ON "dummy_packets" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_dummy_packets_code" ON "dummy_packets" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_dummy_packets_sensitivity_level" ON "dummy_packets" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_dummy_packets_total_length" ON "dummy_packets" USING btree("total_length");
--> statement-breakpoint
CREATE TABLE "app_firewalls" (
    "active" BOOLEAN,
    "app_id" TEXT,
    "firewall" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_app_firewalls_tombstone" ON "app_firewalls" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_app_firewalls_status" ON "app_firewalls" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_app_firewalls_previous_status" ON "app_firewalls" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_app_firewalls_version" ON "app_firewalls" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_app_firewalls_created_date" ON "app_firewalls" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_app_firewalls_updated_date" ON "app_firewalls" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_app_firewalls_organization_id" ON "app_firewalls" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_app_firewalls_created_by" ON "app_firewalls" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_app_firewalls_updated_by" ON "app_firewalls" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_app_firewalls_deleted_by" ON "app_firewalls" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_app_firewalls_requested_by" ON "app_firewalls" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_app_firewalls_tags" ON "app_firewalls" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_app_firewalls_categories" ON "app_firewalls" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_app_firewalls_code" ON "app_firewalls" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_app_firewalls_sensitivity_level" ON "app_firewalls" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE TABLE "device_group_settings" (
    "name" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_device_group_settings_tombstone" ON "device_group_settings" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_device_group_settings_status" ON "device_group_settings" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_device_group_settings_previous_status" ON "device_group_settings" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_device_group_settings_version" ON "device_group_settings" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_device_group_settings_created_date" ON "device_group_settings" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_device_group_settings_updated_date" ON "device_group_settings" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_device_group_settings_organization_id" ON "device_group_settings" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_device_group_settings_created_by" ON "device_group_settings" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_device_group_settings_updated_by" ON "device_group_settings" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_device_group_settings_deleted_by" ON "device_group_settings" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_device_group_settings_requested_by" ON "device_group_settings" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_device_group_settings_tags" ON "device_group_settings" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_device_group_settings_categories" ON "device_group_settings" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_device_group_settings_code" ON "device_group_settings" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_device_group_settings_sensitivity_level" ON "device_group_settings" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE TABLE "device_tunnels" (
    "device_id" TEXT,
    "tunnel_type" TEXT,
    "service_id" TEXT,
    "tunnel_status" TEXT,
    "last_access_time" TEXT,
    "last_access_date" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_device_tunnels_tombstone" ON "device_tunnels" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_device_tunnels_status" ON "device_tunnels" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_device_tunnels_previous_status" ON "device_tunnels" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_device_tunnels_version" ON "device_tunnels" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_device_tunnels_created_date" ON "device_tunnels" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_device_tunnels_updated_date" ON "device_tunnels" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_device_tunnels_organization_id" ON "device_tunnels" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_device_tunnels_created_by" ON "device_tunnels" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_device_tunnels_updated_by" ON "device_tunnels" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_device_tunnels_deleted_by" ON "device_tunnels" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_device_tunnels_requested_by" ON "device_tunnels" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_device_tunnels_tags" ON "device_tunnels" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_device_tunnels_categories" ON "device_tunnels" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_device_tunnels_code" ON "device_tunnels" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_device_tunnels_sensitivity_level" ON "device_tunnels" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE TABLE "device_services" (
    "device_id" TEXT,
    "address" TEXT,
    "port" INTEGER,
    "protocol" TEXT,
    "program" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_device_services_tombstone" ON "device_services" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_device_services_status" ON "device_services" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_device_services_previous_status" ON "device_services" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_device_services_version" ON "device_services" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_device_services_created_date" ON "device_services" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_device_services_updated_date" ON "device_services" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_device_services_organization_id" ON "device_services" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_device_services_created_by" ON "device_services" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_device_services_updated_by" ON "device_services" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_device_services_deleted_by" ON "device_services" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_device_services_requested_by" ON "device_services" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_device_services_tags" ON "device_services" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_device_services_categories" ON "device_services" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_device_services_code" ON "device_services" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_device_services_sensitivity_level" ON "device_services" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE TABLE "device_groups" (
    "device_id" TEXT,
    "device_group_setting_id" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_device_groups_tombstone" ON "device_groups" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_device_groups_status" ON "device_groups" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_device_groups_previous_status" ON "device_groups" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_device_groups_version" ON "device_groups" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_device_groups_created_date" ON "device_groups" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_device_groups_updated_date" ON "device_groups" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_device_groups_organization_id" ON "device_groups" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_device_groups_created_by" ON "device_groups" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_device_groups_updated_by" ON "device_groups" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_device_groups_deleted_by" ON "device_groups" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_device_groups_requested_by" ON "device_groups" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_device_groups_tags" ON "device_groups" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_device_groups_categories" ON "device_groups" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_device_groups_code" ON "device_groups" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_device_groups_sensitivity_level" ON "device_groups" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE TABLE "device_heartbeats" (
    "device_id" TEXT,
    "hypertable_timestamp" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_device_heartbeats_tombstone" ON "device_heartbeats" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_device_heartbeats_status" ON "device_heartbeats" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_device_heartbeats_previous_status" ON "device_heartbeats" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_device_heartbeats_version" ON "device_heartbeats" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_device_heartbeats_created_date" ON "device_heartbeats" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_device_heartbeats_updated_date" ON "device_heartbeats" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_device_heartbeats_organization_id" ON "device_heartbeats" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_device_heartbeats_created_by" ON "device_heartbeats" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_device_heartbeats_updated_by" ON "device_heartbeats" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_device_heartbeats_deleted_by" ON "device_heartbeats" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_device_heartbeats_requested_by" ON "device_heartbeats" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_device_heartbeats_tags" ON "device_heartbeats" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_device_heartbeats_categories" ON "device_heartbeats" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_device_heartbeats_code" ON "device_heartbeats" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_device_heartbeats_sensitivity_level" ON "device_heartbeats" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE TABLE "temp_device_instances" (
    "device_id" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_temp_device_instances_tombstone" ON "temp_device_instances" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_instances_status" ON "temp_device_instances" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_instances_previous_status" ON "temp_device_instances" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_instances_version" ON "temp_device_instances" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_instances_created_date" ON "temp_device_instances" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_instances_updated_date" ON "temp_device_instances" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_instances_organization_id" ON "temp_device_instances" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_instances_created_by" ON "temp_device_instances" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_instances_updated_by" ON "temp_device_instances" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_instances_deleted_by" ON "temp_device_instances" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_instances_requested_by" ON "temp_device_instances" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_instances_tags" ON "temp_device_instances" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_instances_categories" ON "temp_device_instances" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_instances_code" ON "temp_device_instances" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_instances_sensitivity_level" ON "temp_device_instances" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE TABLE "resolutions" (
    "resolution_type" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_resolutions_tombstone" ON "resolutions" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_resolutions_status" ON "resolutions" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_resolutions_previous_status" ON "resolutions" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_resolutions_version" ON "resolutions" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_resolutions_created_date" ON "resolutions" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_resolutions_updated_date" ON "resolutions" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_resolutions_organization_id" ON "resolutions" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_resolutions_created_by" ON "resolutions" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_resolutions_updated_by" ON "resolutions" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_resolutions_deleted_by" ON "resolutions" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_resolutions_requested_by" ON "resolutions" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_resolutions_tags" ON "resolutions" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_resolutions_categories" ON "resolutions" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_resolutions_code" ON "resolutions" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_resolutions_sensitivity_level" ON "resolutions" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE TABLE "temp_device_interfaces" (
    "device_configuration_id" TEXT,
    "name" TEXT,
    "device" TEXT,
    "description" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_temp_device_interfaces_tombstone" ON "temp_device_interfaces" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_interfaces_status" ON "temp_device_interfaces" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_interfaces_previous_status" ON "temp_device_interfaces" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_interfaces_version" ON "temp_device_interfaces" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_interfaces_created_date" ON "temp_device_interfaces" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_interfaces_updated_date" ON "temp_device_interfaces" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_interfaces_organization_id" ON "temp_device_interfaces" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_interfaces_created_by" ON "temp_device_interfaces" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_interfaces_updated_by" ON "temp_device_interfaces" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_interfaces_deleted_by" ON "temp_device_interfaces" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_interfaces_requested_by" ON "temp_device_interfaces" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_interfaces_tags" ON "temp_device_interfaces" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_interfaces_categories" ON "temp_device_interfaces" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_interfaces_code" ON "temp_device_interfaces" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_interfaces_sensitivity_level" ON "temp_device_interfaces" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE TABLE "temp_appguard_logs" (
    "level" TEXT,
    "message" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_temp_appguard_logs_tombstone" ON "temp_appguard_logs" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_temp_appguard_logs_status" ON "temp_appguard_logs" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_temp_appguard_logs_previous_status" ON "temp_appguard_logs" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_temp_appguard_logs_version" ON "temp_appguard_logs" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_temp_appguard_logs_created_date" ON "temp_appguard_logs" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_temp_appguard_logs_updated_date" ON "temp_appguard_logs" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_temp_appguard_logs_organization_id" ON "temp_appguard_logs" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_temp_appguard_logs_created_by" ON "temp_appguard_logs" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_appguard_logs_updated_by" ON "temp_appguard_logs" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_appguard_logs_deleted_by" ON "temp_appguard_logs" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_appguard_logs_requested_by" ON "temp_appguard_logs" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_appguard_logs_tags" ON "temp_appguard_logs" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_temp_appguard_logs_categories" ON "temp_appguard_logs" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_temp_appguard_logs_code" ON "temp_appguard_logs" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_temp_appguard_logs_sensitivity_level" ON "temp_appguard_logs" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE TABLE "device_filter_rules" (
    "device_configuration_id" TEXT,
    "disabled" BOOLEAN,
    "policy" TEXT,
    "protocol" TEXT,
    "ipprotocol" TEXT,
    "source_inversed" BOOLEAN,
    "source_port_value" TEXT,
    "source_port_operator" TEXT,
    "source_ip_value" TEXT,
    "source_ip_operator" TEXT,
    "source_ip_version" INTEGER,
    "source_type" TEXT,
    "destination_inversed" BOOLEAN,
    "destination_port_value" TEXT,
    "destination_port_operator" TEXT,
    "destination_ip_value" TEXT,
    "destination_ip_operator" TEXT,
    "destination_ip_version" INTEGER,
    "destination_type" TEXT,
    "device_rule_status" TEXT,
    "description" TEXT,
    "interface" TEXT,
    "order" INTEGER,
    "associated_rule_id" TEXT,
    "table" TEXT,
    "chain" TEXT,
    "family" TEXT,
    "floating" BOOLEAN,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_device_filter_rules_tombstone" ON "device_filter_rules" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_device_filter_rules_status" ON "device_filter_rules" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_device_filter_rules_previous_status" ON "device_filter_rules" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_device_filter_rules_version" ON "device_filter_rules" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_device_filter_rules_created_date" ON "device_filter_rules" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_device_filter_rules_updated_date" ON "device_filter_rules" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_device_filter_rules_organization_id" ON "device_filter_rules" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_device_filter_rules_created_by" ON "device_filter_rules" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_device_filter_rules_updated_by" ON "device_filter_rules" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_device_filter_rules_deleted_by" ON "device_filter_rules" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_device_filter_rules_requested_by" ON "device_filter_rules" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_device_filter_rules_tags" ON "device_filter_rules" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_device_filter_rules_categories" ON "device_filter_rules" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_device_filter_rules_code" ON "device_filter_rules" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_device_filter_rules_sensitivity_level" ON "device_filter_rules" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE TABLE "aliases" (
    "device_configuration_id" TEXT,
    "type" TEXT,
    "name" TEXT,
    "description" TEXT,
    "alias_status" TEXT,
    "table" TEXT,
    "family" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_aliases_tombstone" ON "aliases" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_aliases_status" ON "aliases" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_aliases_previous_status" ON "aliases" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_aliases_version" ON "aliases" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_aliases_created_date" ON "aliases" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_aliases_updated_date" ON "aliases" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_aliases_organization_id" ON "aliases" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_aliases_created_by" ON "aliases" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_aliases_updated_by" ON "aliases" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_aliases_deleted_by" ON "aliases" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_aliases_requested_by" ON "aliases" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_aliases_tags" ON "aliases" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_aliases_categories" ON "aliases" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_aliases_code" ON "aliases" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_aliases_sensitivity_level" ON "aliases" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE TABLE "tcp_connections" (
    "source" TEXT,
    "sport" INTEGER,
    "dest" TEXT,
    "dport" INTEGER,
    "proto" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_tcp_connections_tombstone" ON "tcp_connections" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_tcp_connections_status" ON "tcp_connections" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_tcp_connections_previous_status" ON "tcp_connections" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_tcp_connections_version" ON "tcp_connections" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_tcp_connections_created_date" ON "tcp_connections" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_tcp_connections_updated_date" ON "tcp_connections" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_tcp_connections_organization_id" ON "tcp_connections" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_tcp_connections_created_by" ON "tcp_connections" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_tcp_connections_updated_by" ON "tcp_connections" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_tcp_connections_deleted_by" ON "tcp_connections" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_tcp_connections_requested_by" ON "tcp_connections" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_tcp_connections_tags" ON "tcp_connections" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_tcp_connections_categories" ON "tcp_connections" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_tcp_connections_code" ON "tcp_connections" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_tcp_connections_sensitivity_level" ON "tcp_connections" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE TABLE "temp_device_nat_rules" (
    "device_configuration_id" TEXT,
    "disabled" BOOLEAN,
    "protocol" TEXT,
    "ipprotocol" TEXT,
    "source_inversed" BOOLEAN,
    "source_port_value" TEXT,
    "source_port_operator" TEXT,
    "source_ip_value" TEXT,
    "source_ip_operator" TEXT,
    "source_ip_version" INTEGER,
    "source_type" TEXT,
    "destination_inversed" BOOLEAN,
    "destination_port_value" TEXT,
    "destination_port_operator" TEXT,
    "destination_ip_value" TEXT,
    "destination_ip_operator" TEXT,
    "destination_ip_version" INTEGER,
    "destination_type" TEXT,
    "device_rule_status" TEXT,
    "description" TEXT,
    "interface" TEXT,
    "redirect_ip" TEXT,
    "redirect_port" INTEGER,
    "order" INTEGER,
    "associated_rule_id" TEXT,
    "table" TEXT,
    "chain" TEXT,
    "family" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_temp_device_nat_rules_tombstone" ON "temp_device_nat_rules" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_nat_rules_status" ON "temp_device_nat_rules" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_nat_rules_previous_status" ON "temp_device_nat_rules" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_nat_rules_version" ON "temp_device_nat_rules" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_nat_rules_created_date" ON "temp_device_nat_rules" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_nat_rules_updated_date" ON "temp_device_nat_rules" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_nat_rules_organization_id" ON "temp_device_nat_rules" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_nat_rules_created_by" ON "temp_device_nat_rules" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_nat_rules_updated_by" ON "temp_device_nat_rules" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_nat_rules_deleted_by" ON "temp_device_nat_rules" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_nat_rules_requested_by" ON "temp_device_nat_rules" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_nat_rules_tags" ON "temp_device_nat_rules" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_nat_rules_categories" ON "temp_device_nat_rules" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_nat_rules_code" ON "temp_device_nat_rules" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_nat_rules_sensitivity_level" ON "temp_device_nat_rules" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE TABLE "device_instances" (
    "device_id" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_device_instances_tombstone" ON "device_instances" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_device_instances_status" ON "device_instances" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_device_instances_previous_status" ON "device_instances" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_device_instances_version" ON "device_instances" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_device_instances_created_date" ON "device_instances" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_device_instances_updated_date" ON "device_instances" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_device_instances_organization_id" ON "device_instances" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_device_instances_created_by" ON "device_instances" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_device_instances_updated_by" ON "device_instances" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_device_instances_deleted_by" ON "device_instances" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_device_instances_requested_by" ON "device_instances" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_device_instances_tags" ON "device_instances" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_device_instances_categories" ON "device_instances" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_device_instances_code" ON "device_instances" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_device_instances_sensitivity_level" ON "device_instances" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE TABLE "temp_device_services" (
    "device_id" TEXT,
    "address" TEXT,
    "port" INTEGER,
    "protocol" TEXT,
    "program" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_temp_device_services_tombstone" ON "temp_device_services" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_services_status" ON "temp_device_services" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_services_previous_status" ON "temp_device_services" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_services_version" ON "temp_device_services" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_services_created_date" ON "temp_device_services" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_services_updated_date" ON "temp_device_services" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_services_organization_id" ON "temp_device_services" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_services_created_by" ON "temp_device_services" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_services_updated_by" ON "temp_device_services" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_services_deleted_by" ON "temp_device_services" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_services_requested_by" ON "temp_device_services" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_services_tags" ON "temp_device_services" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_services_categories" ON "temp_device_services" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_services_code" ON "temp_device_services" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_services_sensitivity_level" ON "temp_device_services" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE TABLE "temp_connections" (
    "hypertable_timestamp" TEXT,
    "interface_name" TEXT,
    "total_packet" INTEGER,
    "total_byte" INTEGER,
    "device_id" TEXT,
    "protocol" TEXT,
    "source_ip" TEXT,
    "destination_ip" TEXT,
    "source_port" INTEGER,
    "destination_port" INTEGER,
    "remote_ip" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_temp_connections_tombstone" ON "temp_connections" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_temp_connections_status" ON "temp_connections" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_temp_connections_previous_status" ON "temp_connections" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_temp_connections_version" ON "temp_connections" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_temp_connections_created_date" ON "temp_connections" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_temp_connections_updated_date" ON "temp_connections" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_temp_connections_organization_id" ON "temp_connections" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_temp_connections_created_by" ON "temp_connections" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_connections_updated_by" ON "temp_connections" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_connections_deleted_by" ON "temp_connections" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_connections_requested_by" ON "temp_connections" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_connections_tags" ON "temp_connections" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_temp_connections_categories" ON "temp_connections" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_temp_connections_code" ON "temp_connections" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_temp_connections_sensitivity_level" ON "temp_connections" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE TABLE "http_requests" (
    "fw_policy" TEXT,
    "fw_reasons" TEXT,
    "ip" TEXT,
    "original_url" TEXT,
    "user_agent" TEXT,
    "headers" TEXT,
    "method" TEXT,
    "body" TEXT,
    "query" TEXT,
    "cookies" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_http_requests_tombstone" ON "http_requests" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_http_requests_status" ON "http_requests" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_http_requests_previous_status" ON "http_requests" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_http_requests_version" ON "http_requests" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_http_requests_created_date" ON "http_requests" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_http_requests_updated_date" ON "http_requests" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_http_requests_organization_id" ON "http_requests" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_http_requests_created_by" ON "http_requests" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_http_requests_updated_by" ON "http_requests" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_http_requests_deleted_by" ON "http_requests" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_http_requests_requested_by" ON "http_requests" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_http_requests_tags" ON "http_requests" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_http_requests_categories" ON "http_requests" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_http_requests_code" ON "http_requests" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_http_requests_sensitivity_level" ON "http_requests" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE TABLE "temp_port_aliases" (
    "alias_id" TEXT,
    "lower_port" INTEGER,
    "upper_port" INTEGER,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_temp_port_aliases_tombstone" ON "temp_port_aliases" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_temp_port_aliases_status" ON "temp_port_aliases" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_temp_port_aliases_previous_status" ON "temp_port_aliases" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_temp_port_aliases_version" ON "temp_port_aliases" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_temp_port_aliases_created_date" ON "temp_port_aliases" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_temp_port_aliases_updated_date" ON "temp_port_aliases" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_temp_port_aliases_organization_id" ON "temp_port_aliases" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_temp_port_aliases_created_by" ON "temp_port_aliases" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_port_aliases_updated_by" ON "temp_port_aliases" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_port_aliases_deleted_by" ON "temp_port_aliases" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_port_aliases_requested_by" ON "temp_port_aliases" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_port_aliases_tags" ON "temp_port_aliases" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_temp_port_aliases_categories" ON "temp_port_aliases" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_temp_port_aliases_code" ON "temp_port_aliases" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_temp_port_aliases_sensitivity_level" ON "temp_port_aliases" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE TABLE "versions" (
    "name" TEXT,
    "latest_version" TEXT,
    "minimum_version" TEXT,
    "update_type" TEXT,
    "release_notes" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_versions_tombstone" ON "versions" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_versions_status" ON "versions" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_versions_previous_status" ON "versions" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_versions_version" ON "versions" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_versions_created_date" ON "versions" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_versions_updated_date" ON "versions" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_versions_organization_id" ON "versions" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_versions_created_by" ON "versions" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_versions_updated_by" ON "versions" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_versions_deleted_by" ON "versions" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_versions_requested_by" ON "versions" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_versions_tags" ON "versions" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_versions_categories" ON "versions" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_versions_code" ON "versions" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_versions_sensitivity_level" ON "versions" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE TABLE "temp_device_tunnels" (
    "device_id" TEXT,
    "tunnel_type" TEXT,
    "service_id" TEXT,
    "tunnel_status" TEXT,
    "last_access_time" TEXT,
    "last_access_date" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_temp_device_tunnels_tombstone" ON "temp_device_tunnels" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_tunnels_status" ON "temp_device_tunnels" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_tunnels_previous_status" ON "temp_device_tunnels" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_tunnels_version" ON "temp_device_tunnels" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_tunnels_created_date" ON "temp_device_tunnels" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_tunnels_updated_date" ON "temp_device_tunnels" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_tunnels_organization_id" ON "temp_device_tunnels" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_tunnels_created_by" ON "temp_device_tunnels" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_tunnels_updated_by" ON "temp_device_tunnels" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_tunnels_deleted_by" ON "temp_device_tunnels" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_tunnels_requested_by" ON "temp_device_tunnels" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_tunnels_tags" ON "temp_device_tunnels" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_tunnels_categories" ON "temp_device_tunnels" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_tunnels_code" ON "temp_device_tunnels" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_tunnels_sensitivity_level" ON "temp_device_tunnels" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE TABLE "temp_device_filter_rules" (
    "device_configuration_id" TEXT,
    "disabled" BOOLEAN,
    "policy" TEXT,
    "protocol" TEXT,
    "ipprotocol" TEXT,
    "source_inversed" BOOLEAN,
    "source_port_value" TEXT,
    "source_port_operator" TEXT,
    "source_ip_value" TEXT,
    "source_ip_operator" TEXT,
    "source_ip_version" INTEGER,
    "source_type" TEXT,
    "destination_inversed" BOOLEAN,
    "destination_port_value" TEXT,
    "destination_port_operator" TEXT,
    "destination_ip_value" TEXT,
    "destination_ip_operator" TEXT,
    "destination_ip_version" INTEGER,
    "destination_type" TEXT,
    "device_rule_status" TEXT,
    "description" TEXT,
    "interface" TEXT,
    "order" INTEGER,
    "associated_rule_id" TEXT,
    "table" TEXT,
    "chain" TEXT,
    "family" TEXT,
    "floating" BOOLEAN,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_temp_device_filter_rules_tombstone" ON "temp_device_filter_rules" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_filter_rules_status" ON "temp_device_filter_rules" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_filter_rules_previous_status" ON "temp_device_filter_rules" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_filter_rules_version" ON "temp_device_filter_rules" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_filter_rules_created_date" ON "temp_device_filter_rules" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_filter_rules_updated_date" ON "temp_device_filter_rules" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_filter_rules_organization_id" ON "temp_device_filter_rules" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_filter_rules_created_by" ON "temp_device_filter_rules" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_filter_rules_updated_by" ON "temp_device_filter_rules" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_filter_rules_deleted_by" ON "temp_device_filter_rules" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_filter_rules_requested_by" ON "temp_device_filter_rules" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_filter_rules_tags" ON "temp_device_filter_rules" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_filter_rules_categories" ON "temp_device_filter_rules" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_filter_rules_code" ON "temp_device_filter_rules" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_filter_rules_sensitivity_level" ON "temp_device_filter_rules" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE TABLE "port_aliases" (
    "alias_id" TEXT,
    "lower_port" INTEGER,
    "upper_port" INTEGER,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_port_aliases_tombstone" ON "port_aliases" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_port_aliases_status" ON "port_aliases" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_port_aliases_previous_status" ON "port_aliases" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_port_aliases_version" ON "port_aliases" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_port_aliases_created_date" ON "port_aliases" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_port_aliases_updated_date" ON "port_aliases" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_port_aliases_organization_id" ON "port_aliases" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_port_aliases_created_by" ON "port_aliases" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_port_aliases_updated_by" ON "port_aliases" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_port_aliases_deleted_by" ON "port_aliases" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_port_aliases_requested_by" ON "port_aliases" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_port_aliases_tags" ON "port_aliases" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_port_aliases_categories" ON "port_aliases" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_port_aliases_code" ON "port_aliases" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_port_aliases_sensitivity_level" ON "port_aliases" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE TABLE "ip_infos" (
    "ip" TEXT,
    "country" TEXT,
    "asn" TEXT,
    "org" TEXT,
    "continent_code" TEXT,
    "city" TEXT,
    "region" TEXT,
    "postal" TEXT,
    "timezone" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_ip_infos_tombstone" ON "ip_infos" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_ip_infos_status" ON "ip_infos" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_ip_infos_previous_status" ON "ip_infos" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_ip_infos_version" ON "ip_infos" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_ip_infos_created_date" ON "ip_infos" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_ip_infos_updated_date" ON "ip_infos" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_ip_infos_organization_id" ON "ip_infos" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_ip_infos_created_by" ON "ip_infos" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_ip_infos_updated_by" ON "ip_infos" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_ip_infos_deleted_by" ON "ip_infos" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_ip_infos_requested_by" ON "ip_infos" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_ip_infos_tags" ON "ip_infos" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_ip_infos_categories" ON "ip_infos" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_ip_infos_code" ON "ip_infos" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_ip_infos_sensitivity_level" ON "ip_infos" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_ip_infos_ip" ON "ip_infos" USING btree("ip");
--> statement-breakpoint
CREATE INDEX "idx_ip_infos_country" ON "ip_infos" USING btree("country");
--> statement-breakpoint
CREATE TABLE "device_interfaces" (
    "device_configuration_id" TEXT,
    "name" TEXT,
    "device" TEXT,
    "description" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_device_interfaces_tombstone" ON "device_interfaces" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_device_interfaces_status" ON "device_interfaces" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_device_interfaces_previous_status" ON "device_interfaces" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_device_interfaces_version" ON "device_interfaces" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_device_interfaces_created_date" ON "device_interfaces" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_device_interfaces_updated_date" ON "device_interfaces" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_device_interfaces_organization_id" ON "device_interfaces" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_device_interfaces_created_by" ON "device_interfaces" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_device_interfaces_updated_by" ON "device_interfaces" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_device_interfaces_deleted_by" ON "device_interfaces" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_device_interfaces_requested_by" ON "device_interfaces" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_device_interfaces_tags" ON "device_interfaces" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_device_interfaces_categories" ON "device_interfaces" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_device_interfaces_code" ON "device_interfaces" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_device_interfaces_sensitivity_level" ON "device_interfaces" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE TABLE "temp_wallguard_logs" (
    "level" TEXT,
    "message" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_temp_wallguard_logs_tombstone" ON "temp_wallguard_logs" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_temp_wallguard_logs_status" ON "temp_wallguard_logs" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_temp_wallguard_logs_previous_status" ON "temp_wallguard_logs" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_temp_wallguard_logs_version" ON "temp_wallguard_logs" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_temp_wallguard_logs_created_date" ON "temp_wallguard_logs" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_temp_wallguard_logs_updated_date" ON "temp_wallguard_logs" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_temp_wallguard_logs_organization_id" ON "temp_wallguard_logs" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_temp_wallguard_logs_created_by" ON "temp_wallguard_logs" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_wallguard_logs_updated_by" ON "temp_wallguard_logs" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_wallguard_logs_deleted_by" ON "temp_wallguard_logs" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_wallguard_logs_requested_by" ON "temp_wallguard_logs" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_wallguard_logs_tags" ON "temp_wallguard_logs" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_temp_wallguard_logs_categories" ON "temp_wallguard_logs" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_temp_wallguard_logs_code" ON "temp_wallguard_logs" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_temp_wallguard_logs_sensitivity_level" ON "temp_wallguard_logs" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE TABLE "notifications" (
    "title" TEXT,
    "description" TEXT,
    "event_timestamp" TEXT,
    "link" TEXT,
    "icon" TEXT,
    "source" TEXT,
    "is_pinned" BOOLEAN,
    "recipient_id" TEXT,
    "actions" JSONB,
    "unread" TEXT,
    "low" TEXT,
    "priority_level" INTEGER,
    "expiry_date" TEXT,
    "metadata" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_notifications_tombstone" ON "notifications" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_notifications_status" ON "notifications" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_notifications_previous_status" ON "notifications" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_notifications_version" ON "notifications" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_notifications_created_date" ON "notifications" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_notifications_updated_date" ON "notifications" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_notifications_organization_id" ON "notifications" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_notifications_created_by" ON "notifications" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_notifications_updated_by" ON "notifications" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_notifications_deleted_by" ON "notifications" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_notifications_requested_by" ON "notifications" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_notifications_tags" ON "notifications" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_notifications_categories" ON "notifications" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_notifications_code" ON "notifications" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_notifications_sensitivity_level" ON "notifications" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_notifications_title" ON "notifications" USING btree("title");
--> statement-breakpoint
CREATE INDEX "idx_notifications_description" ON "notifications" USING btree("description");
--> statement-breakpoint
CREATE INDEX "idx_notifications_link" ON "notifications" USING btree("link");
--> statement-breakpoint
CREATE INDEX "idx_notifications_icon" ON "notifications" USING btree("icon");
--> statement-breakpoint
CREATE INDEX "idx_notifications_source" ON "notifications" USING btree("source");
--> statement-breakpoint
CREATE INDEX "idx_notifications_is_pinned" ON "notifications" USING btree("is_pinned");
--> statement-breakpoint
CREATE INDEX "idx_notifications_recipient_id" ON "notifications" USING btree("recipient_id");
--> statement-breakpoint
CREATE INDEX "idx_notifications_priority_level" ON "notifications" USING btree("priority_level");
--> statement-breakpoint
CREATE INDEX "idx_notifications_expiry_date" ON "notifications" USING btree("expiry_date");
--> statement-breakpoint
CREATE TABLE "device_nat_rules" (
    "device_configuration_id" TEXT,
    "disabled" BOOLEAN,
    "protocol" TEXT,
    "ipprotocol" TEXT,
    "source_inversed" BOOLEAN,
    "source_port_value" TEXT,
    "source_port_operator" TEXT,
    "source_ip_value" TEXT,
    "source_ip_operator" TEXT,
    "source_ip_version" INTEGER,
    "source_type" TEXT,
    "destination_inversed" BOOLEAN,
    "destination_port_value" TEXT,
    "destination_port_operator" TEXT,
    "destination_ip_value" TEXT,
    "destination_ip_operator" TEXT,
    "destination_ip_version" INTEGER,
    "destination_type" TEXT,
    "device_rule_status" TEXT,
    "description" TEXT,
    "interface" TEXT,
    "redirect_ip" TEXT,
    "redirect_port" INTEGER,
    "order" INTEGER,
    "associated_rule_id" TEXT,
    "table" TEXT,
    "chain" TEXT,
    "family" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_device_nat_rules_tombstone" ON "device_nat_rules" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_device_nat_rules_status" ON "device_nat_rules" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_device_nat_rules_previous_status" ON "device_nat_rules" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_device_nat_rules_version" ON "device_nat_rules" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_device_nat_rules_created_date" ON "device_nat_rules" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_device_nat_rules_updated_date" ON "device_nat_rules" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_device_nat_rules_organization_id" ON "device_nat_rules" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_device_nat_rules_created_by" ON "device_nat_rules" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_device_nat_rules_updated_by" ON "device_nat_rules" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_device_nat_rules_deleted_by" ON "device_nat_rules" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_device_nat_rules_requested_by" ON "device_nat_rules" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_device_nat_rules_tags" ON "device_nat_rules" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_device_nat_rules_categories" ON "device_nat_rules" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_device_nat_rules_code" ON "device_nat_rules" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_device_nat_rules_sensitivity_level" ON "device_nat_rules" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE TABLE "temp_ip_aliases" (
    "alias_id" TEXT,
    "ip" TEXT,
    "prefix" INTEGER,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_temp_ip_aliases_tombstone" ON "temp_ip_aliases" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_temp_ip_aliases_status" ON "temp_ip_aliases" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_temp_ip_aliases_previous_status" ON "temp_ip_aliases" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_temp_ip_aliases_version" ON "temp_ip_aliases" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_temp_ip_aliases_created_date" ON "temp_ip_aliases" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_temp_ip_aliases_updated_date" ON "temp_ip_aliases" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_temp_ip_aliases_organization_id" ON "temp_ip_aliases" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_temp_ip_aliases_created_by" ON "temp_ip_aliases" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_ip_aliases_updated_by" ON "temp_ip_aliases" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_ip_aliases_deleted_by" ON "temp_ip_aliases" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_ip_aliases_requested_by" ON "temp_ip_aliases" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_ip_aliases_tags" ON "temp_ip_aliases" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_temp_ip_aliases_categories" ON "temp_ip_aliases" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_temp_ip_aliases_code" ON "temp_ip_aliases" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_temp_ip_aliases_sensitivity_level" ON "temp_ip_aliases" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE TABLE "appguard_configs" (
    "active" BOOLEAN,
    "log_request" BOOLEAN,
    "log_response" BOOLEAN,
    "retention_sec" INTEGER,
    "ip_info_cache_size" INTEGER,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_appguard_configs_tombstone" ON "appguard_configs" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_appguard_configs_status" ON "appguard_configs" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_appguard_configs_previous_status" ON "appguard_configs" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_appguard_configs_version" ON "appguard_configs" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_appguard_configs_created_date" ON "appguard_configs" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_appguard_configs_updated_date" ON "appguard_configs" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_appguard_configs_organization_id" ON "appguard_configs" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_appguard_configs_created_by" ON "appguard_configs" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_appguard_configs_updated_by" ON "appguard_configs" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_appguard_configs_deleted_by" ON "appguard_configs" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_appguard_configs_requested_by" ON "appguard_configs" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_appguard_configs_tags" ON "appguard_configs" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_appguard_configs_categories" ON "appguard_configs" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_appguard_configs_code" ON "appguard_configs" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_appguard_configs_sensitivity_level" ON "appguard_configs" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE TABLE "device_interface_addresses" (
    "device_interface_id" TEXT,
    "address" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_device_interface_addresses_tombstone" ON "device_interface_addresses" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_device_interface_addresses_status" ON "device_interface_addresses" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_device_interface_addresses_previous_status" ON "device_interface_addresses" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_device_interface_addresses_version" ON "device_interface_addresses" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_device_interface_addresses_created_date" ON "device_interface_addresses" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_device_interface_addresses_updated_date" ON "device_interface_addresses" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_device_interface_addresses_organization_id" ON "device_interface_addresses" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_device_interface_addresses_created_by" ON "device_interface_addresses" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_device_interface_addresses_updated_by" ON "device_interface_addresses" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_device_interface_addresses_deleted_by" ON "device_interface_addresses" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_device_interface_addresses_requested_by" ON "device_interface_addresses" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_device_interface_addresses_tags" ON "device_interface_addresses" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_device_interface_addresses_categories" ON "device_interface_addresses" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_device_interface_addresses_code" ON "device_interface_addresses" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_device_interface_addresses_sensitivity_level" ON "device_interface_addresses" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE TABLE "smtp_requests" (
    "fw_policy" TEXT,
    "fw_reasons" TEXT,
    "ip" TEXT,
    "user_agent" TEXT,
    "headers" TEXT,
    "body" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_smtp_requests_tombstone" ON "smtp_requests" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_smtp_requests_status" ON "smtp_requests" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_smtp_requests_previous_status" ON "smtp_requests" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_smtp_requests_version" ON "smtp_requests" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_smtp_requests_created_date" ON "smtp_requests" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_smtp_requests_updated_date" ON "smtp_requests" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_smtp_requests_organization_id" ON "smtp_requests" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_smtp_requests_created_by" ON "smtp_requests" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_smtp_requests_updated_by" ON "smtp_requests" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_smtp_requests_deleted_by" ON "smtp_requests" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_smtp_requests_requested_by" ON "smtp_requests" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_smtp_requests_tags" ON "smtp_requests" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_smtp_requests_categories" ON "smtp_requests" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_smtp_requests_code" ON "smtp_requests" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_smtp_requests_sensitivity_level" ON "smtp_requests" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE TABLE "invitations" (
    "account_id" TEXT,
    "expiration_date" TEXT,
    "expiration_time" TEXT,
    "account_organization_id" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_invitations_tombstone" ON "invitations" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_invitations_status" ON "invitations" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_invitations_previous_status" ON "invitations" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_invitations_version" ON "invitations" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_invitations_created_date" ON "invitations" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_invitations_updated_date" ON "invitations" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_invitations_organization_id" ON "invitations" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_invitations_created_by" ON "invitations" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_invitations_updated_by" ON "invitations" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_invitations_deleted_by" ON "invitations" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_invitations_requested_by" ON "invitations" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_invitations_tags" ON "invitations" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_invitations_categories" ON "invitations" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_invitations_code" ON "invitations" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_invitations_sensitivity_level" ON "invitations" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_invitations_account_id" ON "invitations" USING btree("account_id");
--> statement-breakpoint
CREATE INDEX "idx_invitations_expiration_date" ON "invitations" USING btree("expiration_date");
--> statement-breakpoint
CREATE TABLE "setup_instructions" (
    "device_category" TEXT,
    "device_type" TEXT,
    "markdown" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_setup_instructions_tombstone" ON "setup_instructions" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_setup_instructions_status" ON "setup_instructions" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_setup_instructions_previous_status" ON "setup_instructions" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_setup_instructions_version" ON "setup_instructions" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_setup_instructions_created_date" ON "setup_instructions" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_setup_instructions_updated_date" ON "setup_instructions" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_setup_instructions_organization_id" ON "setup_instructions" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_setup_instructions_created_by" ON "setup_instructions" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_setup_instructions_updated_by" ON "setup_instructions" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_setup_instructions_deleted_by" ON "setup_instructions" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_setup_instructions_requested_by" ON "setup_instructions" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_setup_instructions_tags" ON "setup_instructions" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_setup_instructions_categories" ON "setup_instructions" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_setup_instructions_code" ON "setup_instructions" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_setup_instructions_sensitivity_level" ON "setup_instructions" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE TABLE "installation_codes" (
    "device_id" TEXT,
    "device_code" TEXT,
    "redeemed" BOOLEAN,
    "auto_authorization" BOOLEAN,
    "token" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_installation_codes_tombstone" ON "installation_codes" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_installation_codes_status" ON "installation_codes" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_installation_codes_previous_status" ON "installation_codes" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_installation_codes_version" ON "installation_codes" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_installation_codes_created_date" ON "installation_codes" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_installation_codes_updated_date" ON "installation_codes" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_installation_codes_organization_id" ON "installation_codes" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_installation_codes_created_by" ON "installation_codes" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_installation_codes_updated_by" ON "installation_codes" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_installation_codes_deleted_by" ON "installation_codes" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_installation_codes_requested_by" ON "installation_codes" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_installation_codes_tags" ON "installation_codes" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_installation_codes_categories" ON "installation_codes" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_installation_codes_code" ON "installation_codes" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_installation_codes_sensitivity_level" ON "installation_codes" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE TABLE "temp_device_remote_access_sessions" (
    "device_id" TEXT,
    "remote_access_type" TEXT,
    "remote_access_session" TEXT,
    "remote_access_status" TEXT,
    "instance_id" TEXT,
    "remote_access_category" TEXT,
    "remote_access_local_addr" TEXT,
    "remote_access_local_port" INTEGER,
    "remote_access_local_protocol" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_temp_device_remote_access_sessions_tombstone" ON "temp_device_remote_access_sessions" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_remote_access_sessions_status" ON "temp_device_remote_access_sessions" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_remote_access_sessions_previous_status" ON "temp_device_remote_access_sessions" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_remote_access_sessions_version" ON "temp_device_remote_access_sessions" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_remote_access_sessions_created_date" ON "temp_device_remote_access_sessions" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_remote_access_sessions_updated_date" ON "temp_device_remote_access_sessions" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_remote_access_sessions_organization_id" ON "temp_device_remote_access_sessions" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_remote_access_sessions_created_by" ON "temp_device_remote_access_sessions" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_remote_access_sessions_updated_by" ON "temp_device_remote_access_sessions" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_remote_access_sessions_deleted_by" ON "temp_device_remote_access_sessions" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_remote_access_sessions_requested_by" ON "temp_device_remote_access_sessions" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_remote_access_sessions_tags" ON "temp_device_remote_access_sessions" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_remote_access_sessions_categories" ON "temp_device_remote_access_sessions" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_remote_access_sessions_code" ON "temp_device_remote_access_sessions" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_temp_device_remote_access_sessions_sensitivity_level" ON "temp_device_remote_access_sessions" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE TABLE "temp_packets" (
    "hypertable_timestamp" TEXT,
    "interface_name" TEXT,
    "total_length" INTEGER,
    "device_id" TEXT,
    "source_mac" TEXT,
    "destination_mac" TEXT,
    "ether_type" TEXT,
    "protocol" TEXT,
    "source_ip" TEXT,
    "destination_ip" TEXT,
    "source_port" INTEGER,
    "destination_port" INTEGER,
    "remote_ip" TEXT,
    "tcp_header_length" INTEGER,
    "tcp_sequence_number" BIGINT,
    "tcp_acknowledgment_number" BIGINT,
    "tcp_data_offset" INTEGER,
    "tcp_flags" INTEGER,
    "tcp_window_size" INTEGER,
    "tcp_urgent_pointer" INTEGER,
    "icmp_type" INTEGER,
    "icmp_code" INTEGER,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_temp_packets_tombstone" ON "temp_packets" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_temp_packets_status" ON "temp_packets" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_temp_packets_previous_status" ON "temp_packets" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_temp_packets_version" ON "temp_packets" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_temp_packets_created_date" ON "temp_packets" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_temp_packets_updated_date" ON "temp_packets" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_temp_packets_organization_id" ON "temp_packets" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_temp_packets_created_by" ON "temp_packets" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_packets_updated_by" ON "temp_packets" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_packets_deleted_by" ON "temp_packets" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_packets_requested_by" ON "temp_packets" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_packets_tags" ON "temp_packets" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_temp_packets_categories" ON "temp_packets" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_temp_packets_code" ON "temp_packets" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_temp_packets_sensitivity_level" ON "temp_packets" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE TABLE "wallguard_logs" (
    "level" TEXT,
    "message" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_wallguard_logs_tombstone" ON "wallguard_logs" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_wallguard_logs_status" ON "wallguard_logs" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_wallguard_logs_previous_status" ON "wallguard_logs" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_wallguard_logs_version" ON "wallguard_logs" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_wallguard_logs_created_date" ON "wallguard_logs" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_wallguard_logs_updated_date" ON "wallguard_logs" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_wallguard_logs_organization_id" ON "wallguard_logs" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_wallguard_logs_created_by" ON "wallguard_logs" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_wallguard_logs_updated_by" ON "wallguard_logs" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_wallguard_logs_deleted_by" ON "wallguard_logs" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_wallguard_logs_requested_by" ON "wallguard_logs" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_wallguard_logs_tags" ON "wallguard_logs" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_wallguard_logs_categories" ON "wallguard_logs" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_wallguard_logs_code" ON "wallguard_logs" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_wallguard_logs_sensitivity_level" ON "wallguard_logs" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE TABLE "communication_templates" (
    "name" TEXT,
    "communication_template_status" TEXT,
    "event" TEXT,
    "content" TEXT,
    "subject" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_communication_templates_tombstone" ON "communication_templates" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_communication_templates_status" ON "communication_templates" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_communication_templates_previous_status" ON "communication_templates" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_communication_templates_version" ON "communication_templates" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_communication_templates_created_date" ON "communication_templates" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_communication_templates_updated_date" ON "communication_templates" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_communication_templates_organization_id" ON "communication_templates" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_communication_templates_created_by" ON "communication_templates" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_communication_templates_updated_by" ON "communication_templates" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_communication_templates_deleted_by" ON "communication_templates" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_communication_templates_requested_by" ON "communication_templates" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_communication_templates_tags" ON "communication_templates" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_communication_templates_categories" ON "communication_templates" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_communication_templates_code" ON "communication_templates" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_communication_templates_sensitivity_level" ON "communication_templates" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_communication_templates_name" ON "communication_templates" USING btree("name");
--> statement-breakpoint
CREATE INDEX "idx_communication_templates_communication_template_status" ON "communication_templates" USING btree("communication_template_status");
--> statement-breakpoint
CREATE INDEX "idx_communication_templates_event" ON "communication_templates" USING btree("event");
--> statement-breakpoint
CREATE INDEX "idx_communication_templates_content" ON "communication_templates" USING btree("content");
--> statement-breakpoint
CREATE INDEX "idx_communication_templates_subject" ON "communication_templates" USING btree("subject");
--> statement-breakpoint
CREATE TABLE "temp_aliases" (
    "device_configuration_id" TEXT,
    "type" TEXT,
    "name" TEXT,
    "description" TEXT,
    "alias_status" TEXT,
    "table" TEXT,
    "family" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_temp_aliases_tombstone" ON "temp_aliases" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_temp_aliases_status" ON "temp_aliases" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_temp_aliases_previous_status" ON "temp_aliases" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_temp_aliases_version" ON "temp_aliases" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_temp_aliases_created_date" ON "temp_aliases" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_temp_aliases_updated_date" ON "temp_aliases" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_temp_aliases_organization_id" ON "temp_aliases" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_temp_aliases_created_by" ON "temp_aliases" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_aliases_updated_by" ON "temp_aliases" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_aliases_deleted_by" ON "temp_aliases" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_aliases_requested_by" ON "temp_aliases" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_temp_aliases_tags" ON "temp_aliases" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_temp_aliases_categories" ON "temp_aliases" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_temp_aliases_code" ON "temp_aliases" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_temp_aliases_sensitivity_level" ON "temp_aliases" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE TABLE "locations" (
    "location_name" TEXT,
    "address_id" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_locations_tombstone" ON "locations" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_locations_status" ON "locations" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_locations_previous_status" ON "locations" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_locations_version" ON "locations" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_locations_created_date" ON "locations" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_locations_updated_date" ON "locations" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_locations_organization_id" ON "locations" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_locations_created_by" ON "locations" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_locations_updated_by" ON "locations" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_locations_deleted_by" ON "locations" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_locations_requested_by" ON "locations" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_locations_tags" ON "locations" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_locations_categories" ON "locations" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_locations_code" ON "locations" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_locations_sensitivity_level" ON "locations" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_locations_address_id" ON "locations" USING btree("address_id");
--> statement-breakpoint
CREATE INDEX "idx_locations_location_name" ON "locations" USING btree("location_name");
--> statement-breakpoint
CREATE TABLE "device_remote_access_sessions" (
    "device_id" TEXT,
    "remote_access_type" TEXT,
    "remote_access_session" TEXT,
    "remote_access_status" TEXT,
    "instance_id" TEXT,
    "remote_access_category" TEXT,
    "remote_access_local_addr" TEXT,
    "remote_access_local_port" INTEGER,
    "remote_access_local_protocol" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_device_remote_access_sessions_tombstone" ON "device_remote_access_sessions" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_device_remote_access_sessions_status" ON "device_remote_access_sessions" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_device_remote_access_sessions_previous_status" ON "device_remote_access_sessions" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_device_remote_access_sessions_version" ON "device_remote_access_sessions" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_device_remote_access_sessions_created_date" ON "device_remote_access_sessions" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_device_remote_access_sessions_updated_date" ON "device_remote_access_sessions" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_device_remote_access_sessions_organization_id" ON "device_remote_access_sessions" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_device_remote_access_sessions_created_by" ON "device_remote_access_sessions" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_device_remote_access_sessions_updated_by" ON "device_remote_access_sessions" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_device_remote_access_sessions_deleted_by" ON "device_remote_access_sessions" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_device_remote_access_sessions_requested_by" ON "device_remote_access_sessions" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_device_remote_access_sessions_tags" ON "device_remote_access_sessions" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_device_remote_access_sessions_categories" ON "device_remote_access_sessions" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_device_remote_access_sessions_code" ON "device_remote_access_sessions" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_device_remote_access_sessions_sensitivity_level" ON "device_remote_access_sessions" USING btree("sensitivity_level");
--> statement-breakpoint
ALTER TABLE "contacts" ADD COLUMN "address_id" TEXT;
--> statement-breakpoint
ALTER TABLE "samples" ADD COLUMN "tags" TEXT[];
--> statement-breakpoint
ALTER TABLE "samples" ADD COLUMN "test_obj" JSONB;
--> statement-breakpoint
ALTER TABLE "devices" ADD COLUMN "is_traffic_monitoring_enabled" BOOLEAN;
--> statement-breakpoint
ALTER TABLE "devices" ADD COLUMN "is_config_monitoring_enabled" BOOLEAN;
--> statement-breakpoint
ALTER TABLE "devices" ADD COLUMN "is_telemetry_monitoring_enabled" BOOLEAN;
--> statement-breakpoint
ALTER TABLE "devices" ADD COLUMN "is_device_authorized" BOOLEAN;
--> statement-breakpoint
ALTER TABLE "devices" ADD COLUMN "device_uuid" TEXT;
--> statement-breakpoint
ALTER TABLE "devices" ADD COLUMN "device_name" TEXT;
--> statement-breakpoint
ALTER TABLE "devices" ADD COLUMN "device_category" TEXT;
--> statement-breakpoint
ALTER TABLE "devices" ADD COLUMN "device_type" TEXT;
--> statement-breakpoint
ALTER TABLE "devices" ADD COLUMN "device_os" TEXT;
--> statement-breakpoint
ALTER TABLE "devices" ADD COLUMN "is_device_online" BOOLEAN;
--> statement-breakpoint
ALTER TABLE "contact_phone_numbers" ADD COLUMN "raw_phone_number" TEXT;
--> statement-breakpoint
ALTER TABLE "contact_phone_numbers" ADD COLUMN "iso_code" TEXT;
--> statement-breakpoint
ALTER TABLE "contact_phone_numbers" ADD COLUMN "country_code" TEXT;
--> statement-breakpoint
ALTER TABLE "contact_phone_numbers" ADD COLUMN "is_primary" BOOLEAN;
--> statement-breakpoint
ALTER TABLE "organization_contacts" ADD COLUMN "contact_organization_id" TEXT;
--> statement-breakpoint
ALTER TABLE "organization_contacts" ADD COLUMN "is_primary" BOOLEAN;
--> statement-breakpoint
ALTER TABLE "files" ADD COLUMN "versionId" TEXT;
--> statement-breakpoint
ALTER TABLE "files" ADD COLUMN "presignedURL" TEXT;
--> statement-breakpoint
ALTER TABLE "files" ADD COLUMN "presignedURLExpires" INTEGER;
--> statement-breakpoint
ALTER TABLE "devices" DROP COLUMN "model";
--> statement-breakpoint
ALTER TABLE "devices" DROP COLUMN "instance_name";
--> statement-breakpoint
ALTER TABLE "devices" DROP COLUMN "is_connection_established";
--> statement-breakpoint
ALTER TABLE "devices" DROP COLUMN "system_id";
--> statement-breakpoint
ALTER TABLE "devices" DROP COLUMN "last_heartbeat";
--> statement-breakpoint
ALTER TABLE "devices" DROP COLUMN "is_monitoring_enabled";
--> statement-breakpoint
ALTER TABLE "devices" DROP COLUMN "is_remote_access_enabled";
--> statement-breakpoint
ALTER TABLE "devices" DROP COLUMN "ip_address";
--> statement-breakpoint
ALTER TABLE "devices" DROP COLUMN "device_status";
--> statement-breakpoint
ALTER TABLE "devices" DROP COLUMN "device_gui_protocol";
--> statement-breakpoint
ALTER TABLE "contact_phone_numbers" DROP COLUMN "phone_number_raw";
--> statement-breakpoint
ALTER TABLE "files" DROP COLUMN "version_id";
--> statement-breakpoint
ALTER TABLE "files" DROP COLUMN "presigned_url";
--> statement-breakpoint
ALTER TABLE "files" DROP COLUMN "presigned_url_expire";
--> statement-breakpoint
CREATE INDEX "idx_contacts_first_name" ON "contacts" USING btree("first_name");
--> statement-breakpoint
CREATE INDEX "idx_contacts_last_name" ON "contacts" USING btree("last_name");
--> statement-breakpoint
CREATE INDEX "idx_contacts_account_id" ON "contacts" USING btree("account_id");
--> statement-breakpoint
CREATE INDEX "idx_contacts_address_id" ON "contacts" USING btree("address_id");
--> statement-breakpoint
CREATE INDEX "idx_contacts_date_of_birth" ON "contacts" USING btree("date_of_birth");
--> statement-breakpoint
CREATE INDEX "idx_contacts_middle_name" ON "contacts" USING btree("middle_name");
--> statement-breakpoint
CREATE INDEX "idx_samples_tags" ON "samples" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_devices_sensitivity_level" ON "devices" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_organization_contacts_contact_id" ON "organization_contacts" USING btree("contact_id");
--> statement-breakpoint
CREATE INDEX "idx_organization_contacts_is_primary" ON "organization_contacts" USING btree("is_primary");
--> statement-breakpoint
CREATE INDEX "idx_organization_contacts_contact_organization_id" ON "organization_contacts" USING btree("contact_organization_id");
--> statement-breakpoint
ALTER TABLE "ip_aliases" ADD CONSTRAINT "fk_ip_aliases_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "ip_aliases" ADD CONSTRAINT "fk_ip_aliases_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "ip_aliases" ADD CONSTRAINT "fk_ip_aliases_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "ip_aliases" ADD CONSTRAINT "fk_ip_aliases_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "ip_aliases" ADD CONSTRAINT "fk_ip_aliases_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "ip_aliases" ADD CONSTRAINT "fk_ip_aliases_alias_id" FOREIGN KEY ("alias_id") REFERENCES "public"."aliases"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "smtp_responses" ADD CONSTRAINT "fk_smtp_responses_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "smtp_responses" ADD CONSTRAINT "fk_smtp_responses_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "smtp_responses" ADD CONSTRAINT "fk_smtp_responses_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "smtp_responses" ADD CONSTRAINT "fk_smtp_responses_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "smtp_responses" ADD CONSTRAINT "fk_smtp_responses_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "appguard_logs" ADD CONSTRAINT "fk_appguard_logs_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "appguard_logs" ADD CONSTRAINT "fk_appguard_logs_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "appguard_logs" ADD CONSTRAINT "fk_appguard_logs_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "appguard_logs" ADD CONSTRAINT "fk_appguard_logs_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "appguard_logs" ADD CONSTRAINT "fk_appguard_logs_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "packets" ADD CONSTRAINT "fk_packets_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "packets" ADD CONSTRAINT "fk_packets_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "packets" ADD CONSTRAINT "fk_packets_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "packets" ADD CONSTRAINT "fk_packets_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "packets" ADD CONSTRAINT "fk_packets_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "packets" ADD CONSTRAINT "fk_packets_device_id" FOREIGN KEY ("device_id") REFERENCES "public"."devices"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_device_interface_addresses" ADD CONSTRAINT "fk_temp_device_interface_addresses_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_device_interface_addresses" ADD CONSTRAINT "fk_temp_device_interface_addresses_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_device_interface_addresses" ADD CONSTRAINT "fk_temp_device_interface_addresses_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_device_interface_addresses" ADD CONSTRAINT "fk_temp_device_interface_addresses_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_device_interface_addresses" ADD CONSTRAINT "fk_temp_device_interface_addresses_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_device_interface_addresses" ADD CONSTRAINT "fk_temp_device_interface_addresses_device_interface_id" FOREIGN KEY ("device_interface_id") REFERENCES "public"."device_interfaces"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "contacts" ADD CONSTRAINT "fk_contacts_address_id" FOREIGN KEY ("address_id") REFERENCES "public"."addresses"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "http_responses" ADD CONSTRAINT "fk_http_responses_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "http_responses" ADD CONSTRAINT "fk_http_responses_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "http_responses" ADD CONSTRAINT "fk_http_responses_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "http_responses" ADD CONSTRAINT "fk_http_responses_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "http_responses" ADD CONSTRAINT "fk_http_responses_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "system_resources" ADD CONSTRAINT "fk_system_resources_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "system_resources" ADD CONSTRAINT "fk_system_resources_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "system_resources" ADD CONSTRAINT "fk_system_resources_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "system_resources" ADD CONSTRAINT "fk_system_resources_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "system_resources" ADD CONSTRAINT "fk_system_resources_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "system_resources" ADD CONSTRAINT "fk_system_resources_device_id" FOREIGN KEY ("device_id") REFERENCES "public"."devices"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "organization_contact_user_roles" ADD CONSTRAINT "fk_organization_contact_user_roles_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "organization_contact_user_roles" ADD CONSTRAINT "fk_organization_contact_user_roles_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "organization_contact_user_roles" ADD CONSTRAINT "fk_organization_contact_user_roles_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "organization_contact_user_roles" ADD CONSTRAINT "fk_organization_contact_user_roles_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "organization_contact_user_roles" ADD CONSTRAINT "fk_organization_contact_user_roles_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "organization_contact_user_roles" ADD CONSTRAINT "fk_organization_contact_user_roles_organization_contact_id" FOREIGN KEY ("organization_contact_id") REFERENCES "public"."organization_contacts"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "organization_contact_user_roles" ADD CONSTRAINT "fk_organization_contact_user_roles_user_role_id" FOREIGN KEY ("user_role_id") REFERENCES "public"."user_roles"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_system_resources" ADD CONSTRAINT "fk_temp_system_resources_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_system_resources" ADD CONSTRAINT "fk_temp_system_resources_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_system_resources" ADD CONSTRAINT "fk_temp_system_resources_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_system_resources" ADD CONSTRAINT "fk_temp_system_resources_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_system_resources" ADD CONSTRAINT "fk_temp_system_resources_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_system_resources" ADD CONSTRAINT "fk_temp_system_resources_device_id" FOREIGN KEY ("device_id") REFERENCES "public"."devices"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_configurations" ADD CONSTRAINT "fk_device_configurations_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_configurations" ADD CONSTRAINT "fk_device_configurations_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_configurations" ADD CONSTRAINT "fk_device_configurations_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_configurations" ADD CONSTRAINT "fk_device_configurations_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_configurations" ADD CONSTRAINT "fk_device_configurations_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_configurations" ADD CONSTRAINT "fk_device_configurations_device_id" FOREIGN KEY ("device_id") REFERENCES "public"."devices"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "grid_filters" ADD CONSTRAINT "fk_grid_filters_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "grid_filters" ADD CONSTRAINT "fk_grid_filters_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "grid_filters" ADD CONSTRAINT "fk_grid_filters_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "grid_filters" ADD CONSTRAINT "fk_grid_filters_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "grid_filters" ADD CONSTRAINT "fk_grid_filters_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "grid_filters" ADD CONSTRAINT "fk_grid_filters_contact_id" FOREIGN KEY ("contact_id") REFERENCES "public"."contacts"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "connections" ADD CONSTRAINT "fk_connections_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "connections" ADD CONSTRAINT "fk_connections_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "connections" ADD CONSTRAINT "fk_connections_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "connections" ADD CONSTRAINT "fk_connections_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "connections" ADD CONSTRAINT "fk_connections_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "connections" ADD CONSTRAINT "fk_connections_device_id" FOREIGN KEY ("device_id") REFERENCES "public"."devices"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "dummy_packets" ADD CONSTRAINT "fk_dummy_packets_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "dummy_packets" ADD CONSTRAINT "fk_dummy_packets_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "dummy_packets" ADD CONSTRAINT "fk_dummy_packets_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "dummy_packets" ADD CONSTRAINT "fk_dummy_packets_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "dummy_packets" ADD CONSTRAINT "fk_dummy_packets_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "dummy_packets" ADD CONSTRAINT "fk_dummy_packets_device_id" FOREIGN KEY ("device_id") REFERENCES "public"."devices"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "app_firewalls" ADD CONSTRAINT "fk_app_firewalls_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "app_firewalls" ADD CONSTRAINT "fk_app_firewalls_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "app_firewalls" ADD CONSTRAINT "fk_app_firewalls_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "app_firewalls" ADD CONSTRAINT "fk_app_firewalls_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "app_firewalls" ADD CONSTRAINT "fk_app_firewalls_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_group_settings" ADD CONSTRAINT "fk_device_group_settings_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_group_settings" ADD CONSTRAINT "fk_device_group_settings_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_group_settings" ADD CONSTRAINT "fk_device_group_settings_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_group_settings" ADD CONSTRAINT "fk_device_group_settings_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_group_settings" ADD CONSTRAINT "fk_device_group_settings_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_tunnels" ADD CONSTRAINT "fk_device_tunnels_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_tunnels" ADD CONSTRAINT "fk_device_tunnels_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_tunnels" ADD CONSTRAINT "fk_device_tunnels_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_tunnels" ADD CONSTRAINT "fk_device_tunnels_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_tunnels" ADD CONSTRAINT "fk_device_tunnels_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_services" ADD CONSTRAINT "fk_device_services_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_services" ADD CONSTRAINT "fk_device_services_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_services" ADD CONSTRAINT "fk_device_services_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_services" ADD CONSTRAINT "fk_device_services_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_services" ADD CONSTRAINT "fk_device_services_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_services" ADD CONSTRAINT "fk_device_services_device_id" FOREIGN KEY ("device_id") REFERENCES "public"."devices"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_groups" ADD CONSTRAINT "fk_device_groups_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_groups" ADD CONSTRAINT "fk_device_groups_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_groups" ADD CONSTRAINT "fk_device_groups_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_groups" ADD CONSTRAINT "fk_device_groups_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_groups" ADD CONSTRAINT "fk_device_groups_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_groups" ADD CONSTRAINT "fk_device_groups_device_id" FOREIGN KEY ("device_id") REFERENCES "public"."devices"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_groups" ADD CONSTRAINT "fk_device_groups_device_group_setting_id" FOREIGN KEY ("device_group_setting_id") REFERENCES "public"."device_group_settings"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_heartbeats" ADD CONSTRAINT "fk_device_heartbeats_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_heartbeats" ADD CONSTRAINT "fk_device_heartbeats_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_heartbeats" ADD CONSTRAINT "fk_device_heartbeats_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_heartbeats" ADD CONSTRAINT "fk_device_heartbeats_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_heartbeats" ADD CONSTRAINT "fk_device_heartbeats_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_heartbeats" ADD CONSTRAINT "fk_device_heartbeats_device_id" FOREIGN KEY ("device_id") REFERENCES "public"."devices"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_device_instances" ADD CONSTRAINT "fk_temp_device_instances_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_device_instances" ADD CONSTRAINT "fk_temp_device_instances_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_device_instances" ADD CONSTRAINT "fk_temp_device_instances_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_device_instances" ADD CONSTRAINT "fk_temp_device_instances_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_device_instances" ADD CONSTRAINT "fk_temp_device_instances_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_device_instances" ADD CONSTRAINT "fk_temp_device_instances_device_id" FOREIGN KEY ("device_id") REFERENCES "public"."devices"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "resolutions" ADD CONSTRAINT "fk_resolutions_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "resolutions" ADD CONSTRAINT "fk_resolutions_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "resolutions" ADD CONSTRAINT "fk_resolutions_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "resolutions" ADD CONSTRAINT "fk_resolutions_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "resolutions" ADD CONSTRAINT "fk_resolutions_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_device_interfaces" ADD CONSTRAINT "fk_temp_device_interfaces_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_device_interfaces" ADD CONSTRAINT "fk_temp_device_interfaces_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_device_interfaces" ADD CONSTRAINT "fk_temp_device_interfaces_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_device_interfaces" ADD CONSTRAINT "fk_temp_device_interfaces_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_device_interfaces" ADD CONSTRAINT "fk_temp_device_interfaces_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_device_interfaces" ADD CONSTRAINT "fk_temp_device_interfaces_device_configuration_id" FOREIGN KEY ("device_configuration_id") REFERENCES "public"."device_configurations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_appguard_logs" ADD CONSTRAINT "fk_temp_appguard_logs_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_appguard_logs" ADD CONSTRAINT "fk_temp_appguard_logs_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_appguard_logs" ADD CONSTRAINT "fk_temp_appguard_logs_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_appguard_logs" ADD CONSTRAINT "fk_temp_appguard_logs_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_appguard_logs" ADD CONSTRAINT "fk_temp_appguard_logs_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_filter_rules" ADD CONSTRAINT "fk_device_filter_rules_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_filter_rules" ADD CONSTRAINT "fk_device_filter_rules_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_filter_rules" ADD CONSTRAINT "fk_device_filter_rules_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_filter_rules" ADD CONSTRAINT "fk_device_filter_rules_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_filter_rules" ADD CONSTRAINT "fk_device_filter_rules_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_filter_rules" ADD CONSTRAINT "fk_device_filter_rules_device_configuration_id" FOREIGN KEY ("device_configuration_id") REFERENCES "public"."device_configurations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "aliases" ADD CONSTRAINT "fk_aliases_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "aliases" ADD CONSTRAINT "fk_aliases_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "aliases" ADD CONSTRAINT "fk_aliases_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "aliases" ADD CONSTRAINT "fk_aliases_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "aliases" ADD CONSTRAINT "fk_aliases_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "aliases" ADD CONSTRAINT "fk_aliases_device_configuration_id" FOREIGN KEY ("device_configuration_id") REFERENCES "public"."device_configurations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "tcp_connections" ADD CONSTRAINT "fk_tcp_connections_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "tcp_connections" ADD CONSTRAINT "fk_tcp_connections_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "tcp_connections" ADD CONSTRAINT "fk_tcp_connections_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "tcp_connections" ADD CONSTRAINT "fk_tcp_connections_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "tcp_connections" ADD CONSTRAINT "fk_tcp_connections_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_device_nat_rules" ADD CONSTRAINT "fk_temp_device_nat_rules_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_device_nat_rules" ADD CONSTRAINT "fk_temp_device_nat_rules_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_device_nat_rules" ADD CONSTRAINT "fk_temp_device_nat_rules_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_device_nat_rules" ADD CONSTRAINT "fk_temp_device_nat_rules_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_device_nat_rules" ADD CONSTRAINT "fk_temp_device_nat_rules_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_device_nat_rules" ADD CONSTRAINT "fk_temp_device_nat_rules_device_configuration_id" FOREIGN KEY ("device_configuration_id") REFERENCES "public"."device_configurations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_instances" ADD CONSTRAINT "fk_device_instances_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_instances" ADD CONSTRAINT "fk_device_instances_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_instances" ADD CONSTRAINT "fk_device_instances_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_instances" ADD CONSTRAINT "fk_device_instances_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_instances" ADD CONSTRAINT "fk_device_instances_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_instances" ADD CONSTRAINT "fk_device_instances_device_id" FOREIGN KEY ("device_id") REFERENCES "public"."devices"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_device_services" ADD CONSTRAINT "fk_temp_device_services_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_device_services" ADD CONSTRAINT "fk_temp_device_services_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_device_services" ADD CONSTRAINT "fk_temp_device_services_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_device_services" ADD CONSTRAINT "fk_temp_device_services_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_device_services" ADD CONSTRAINT "fk_temp_device_services_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_device_services" ADD CONSTRAINT "fk_temp_device_services_device_id" FOREIGN KEY ("device_id") REFERENCES "public"."devices"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_connections" ADD CONSTRAINT "fk_temp_connections_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_connections" ADD CONSTRAINT "fk_temp_connections_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_connections" ADD CONSTRAINT "fk_temp_connections_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_connections" ADD CONSTRAINT "fk_temp_connections_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_connections" ADD CONSTRAINT "fk_temp_connections_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_connections" ADD CONSTRAINT "fk_temp_connections_device_id" FOREIGN KEY ("device_id") REFERENCES "public"."devices"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "http_requests" ADD CONSTRAINT "fk_http_requests_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "http_requests" ADD CONSTRAINT "fk_http_requests_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "http_requests" ADD CONSTRAINT "fk_http_requests_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "http_requests" ADD CONSTRAINT "fk_http_requests_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "http_requests" ADD CONSTRAINT "fk_http_requests_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_port_aliases" ADD CONSTRAINT "fk_temp_port_aliases_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_port_aliases" ADD CONSTRAINT "fk_temp_port_aliases_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_port_aliases" ADD CONSTRAINT "fk_temp_port_aliases_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_port_aliases" ADD CONSTRAINT "fk_temp_port_aliases_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_port_aliases" ADD CONSTRAINT "fk_temp_port_aliases_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_port_aliases" ADD CONSTRAINT "fk_temp_port_aliases_alias_id" FOREIGN KEY ("alias_id") REFERENCES "public"."aliases"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "versions" ADD CONSTRAINT "fk_versions_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "versions" ADD CONSTRAINT "fk_versions_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "versions" ADD CONSTRAINT "fk_versions_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "versions" ADD CONSTRAINT "fk_versions_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "versions" ADD CONSTRAINT "fk_versions_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_device_tunnels" ADD CONSTRAINT "fk_temp_device_tunnels_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_device_tunnels" ADD CONSTRAINT "fk_temp_device_tunnels_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_device_tunnels" ADD CONSTRAINT "fk_temp_device_tunnels_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_device_tunnels" ADD CONSTRAINT "fk_temp_device_tunnels_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_device_tunnels" ADD CONSTRAINT "fk_temp_device_tunnels_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_device_filter_rules" ADD CONSTRAINT "fk_temp_device_filter_rules_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_device_filter_rules" ADD CONSTRAINT "fk_temp_device_filter_rules_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_device_filter_rules" ADD CONSTRAINT "fk_temp_device_filter_rules_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_device_filter_rules" ADD CONSTRAINT "fk_temp_device_filter_rules_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_device_filter_rules" ADD CONSTRAINT "fk_temp_device_filter_rules_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_device_filter_rules" ADD CONSTRAINT "fk_temp_device_filter_rules_device_configuration_id" FOREIGN KEY ("device_configuration_id") REFERENCES "public"."device_configurations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "port_aliases" ADD CONSTRAINT "fk_port_aliases_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "port_aliases" ADD CONSTRAINT "fk_port_aliases_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "port_aliases" ADD CONSTRAINT "fk_port_aliases_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "port_aliases" ADD CONSTRAINT "fk_port_aliases_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "port_aliases" ADD CONSTRAINT "fk_port_aliases_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "port_aliases" ADD CONSTRAINT "fk_port_aliases_alias_id" FOREIGN KEY ("alias_id") REFERENCES "public"."aliases"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "organization_contacts" ADD CONSTRAINT "fk_organization_contacts_contact_organization_id" FOREIGN KEY ("contact_organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "ip_infos" ADD CONSTRAINT "fk_ip_infos_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "ip_infos" ADD CONSTRAINT "fk_ip_infos_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "ip_infos" ADD CONSTRAINT "fk_ip_infos_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "ip_infos" ADD CONSTRAINT "fk_ip_infos_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "ip_infos" ADD CONSTRAINT "fk_ip_infos_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_interfaces" ADD CONSTRAINT "fk_device_interfaces_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_interfaces" ADD CONSTRAINT "fk_device_interfaces_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_interfaces" ADD CONSTRAINT "fk_device_interfaces_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_interfaces" ADD CONSTRAINT "fk_device_interfaces_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_interfaces" ADD CONSTRAINT "fk_device_interfaces_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_interfaces" ADD CONSTRAINT "fk_device_interfaces_device_configuration_id" FOREIGN KEY ("device_configuration_id") REFERENCES "public"."device_configurations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_wallguard_logs" ADD CONSTRAINT "fk_temp_wallguard_logs_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_wallguard_logs" ADD CONSTRAINT "fk_temp_wallguard_logs_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_wallguard_logs" ADD CONSTRAINT "fk_temp_wallguard_logs_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_wallguard_logs" ADD CONSTRAINT "fk_temp_wallguard_logs_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_wallguard_logs" ADD CONSTRAINT "fk_temp_wallguard_logs_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "notifications" ADD CONSTRAINT "fk_notifications_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "notifications" ADD CONSTRAINT "fk_notifications_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "notifications" ADD CONSTRAINT "fk_notifications_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "notifications" ADD CONSTRAINT "fk_notifications_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "notifications" ADD CONSTRAINT "fk_notifications_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "notifications" ADD CONSTRAINT "fk_notifications_recipient_id" FOREIGN KEY ("recipient_id") REFERENCES "public"."contacts"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_nat_rules" ADD CONSTRAINT "fk_device_nat_rules_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_nat_rules" ADD CONSTRAINT "fk_device_nat_rules_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_nat_rules" ADD CONSTRAINT "fk_device_nat_rules_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_nat_rules" ADD CONSTRAINT "fk_device_nat_rules_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_nat_rules" ADD CONSTRAINT "fk_device_nat_rules_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_nat_rules" ADD CONSTRAINT "fk_device_nat_rules_device_configuration_id" FOREIGN KEY ("device_configuration_id") REFERENCES "public"."device_configurations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_ip_aliases" ADD CONSTRAINT "fk_temp_ip_aliases_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_ip_aliases" ADD CONSTRAINT "fk_temp_ip_aliases_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_ip_aliases" ADD CONSTRAINT "fk_temp_ip_aliases_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_ip_aliases" ADD CONSTRAINT "fk_temp_ip_aliases_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_ip_aliases" ADD CONSTRAINT "fk_temp_ip_aliases_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_ip_aliases" ADD CONSTRAINT "fk_temp_ip_aliases_alias_id" FOREIGN KEY ("alias_id") REFERENCES "public"."aliases"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "appguard_configs" ADD CONSTRAINT "fk_appguard_configs_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "appguard_configs" ADD CONSTRAINT "fk_appguard_configs_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "appguard_configs" ADD CONSTRAINT "fk_appguard_configs_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "appguard_configs" ADD CONSTRAINT "fk_appguard_configs_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "appguard_configs" ADD CONSTRAINT "fk_appguard_configs_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_interface_addresses" ADD CONSTRAINT "fk_device_interface_addresses_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_interface_addresses" ADD CONSTRAINT "fk_device_interface_addresses_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_interface_addresses" ADD CONSTRAINT "fk_device_interface_addresses_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_interface_addresses" ADD CONSTRAINT "fk_device_interface_addresses_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_interface_addresses" ADD CONSTRAINT "fk_device_interface_addresses_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_interface_addresses" ADD CONSTRAINT "fk_device_interface_addresses_device_interface_id" FOREIGN KEY ("device_interface_id") REFERENCES "public"."device_interfaces"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "smtp_requests" ADD CONSTRAINT "fk_smtp_requests_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "smtp_requests" ADD CONSTRAINT "fk_smtp_requests_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "smtp_requests" ADD CONSTRAINT "fk_smtp_requests_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "smtp_requests" ADD CONSTRAINT "fk_smtp_requests_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "smtp_requests" ADD CONSTRAINT "fk_smtp_requests_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "invitations" ADD CONSTRAINT "fk_invitations_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "invitations" ADD CONSTRAINT "fk_invitations_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "invitations" ADD CONSTRAINT "fk_invitations_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "invitations" ADD CONSTRAINT "fk_invitations_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "invitations" ADD CONSTRAINT "fk_invitations_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "invitations" ADD CONSTRAINT "fk_invitations_account_id" FOREIGN KEY ("account_id") REFERENCES "public"."organization_accounts"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "invitations" ADD CONSTRAINT "fk_invitations_account_organization_id" FOREIGN KEY ("account_organization_id") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "setup_instructions" ADD CONSTRAINT "fk_setup_instructions_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "setup_instructions" ADD CONSTRAINT "fk_setup_instructions_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "setup_instructions" ADD CONSTRAINT "fk_setup_instructions_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "setup_instructions" ADD CONSTRAINT "fk_setup_instructions_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "setup_instructions" ADD CONSTRAINT "fk_setup_instructions_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "installation_codes" ADD CONSTRAINT "fk_installation_codes_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "installation_codes" ADD CONSTRAINT "fk_installation_codes_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "installation_codes" ADD CONSTRAINT "fk_installation_codes_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "installation_codes" ADD CONSTRAINT "fk_installation_codes_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "installation_codes" ADD CONSTRAINT "fk_installation_codes_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "installation_codes" ADD CONSTRAINT "fk_installation_codes_device_id" FOREIGN KEY ("device_id") REFERENCES "public"."devices"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_device_remote_access_sessions" ADD CONSTRAINT "fk_temp_device_remote_access_sessions_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_device_remote_access_sessions" ADD CONSTRAINT "fk_temp_device_remote_access_sessions_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_device_remote_access_sessions" ADD CONSTRAINT "fk_temp_device_remote_access_sessions_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_device_remote_access_sessions" ADD CONSTRAINT "fk_temp_device_remote_access_sessions_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_device_remote_access_sessions" ADD CONSTRAINT "fk_temp_device_remote_access_sessions_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_device_remote_access_sessions" ADD CONSTRAINT "fk_temp_device_remote_access_sessions_device_id" FOREIGN KEY ("device_id") REFERENCES "public"."devices"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_packets" ADD CONSTRAINT "fk_temp_packets_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_packets" ADD CONSTRAINT "fk_temp_packets_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_packets" ADD CONSTRAINT "fk_temp_packets_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_packets" ADD CONSTRAINT "fk_temp_packets_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_packets" ADD CONSTRAINT "fk_temp_packets_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_packets" ADD CONSTRAINT "fk_temp_packets_device_id" FOREIGN KEY ("device_id") REFERENCES "public"."devices"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "wallguard_logs" ADD CONSTRAINT "fk_wallguard_logs_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "wallguard_logs" ADD CONSTRAINT "fk_wallguard_logs_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "wallguard_logs" ADD CONSTRAINT "fk_wallguard_logs_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "wallguard_logs" ADD CONSTRAINT "fk_wallguard_logs_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "wallguard_logs" ADD CONSTRAINT "fk_wallguard_logs_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "communication_templates" ADD CONSTRAINT "fk_communication_templates_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "communication_templates" ADD CONSTRAINT "fk_communication_templates_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "communication_templates" ADD CONSTRAINT "fk_communication_templates_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "communication_templates" ADD CONSTRAINT "fk_communication_templates_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "communication_templates" ADD CONSTRAINT "fk_communication_templates_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_aliases" ADD CONSTRAINT "fk_temp_aliases_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_aliases" ADD CONSTRAINT "fk_temp_aliases_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_aliases" ADD CONSTRAINT "fk_temp_aliases_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_aliases" ADD CONSTRAINT "fk_temp_aliases_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_aliases" ADD CONSTRAINT "fk_temp_aliases_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "temp_aliases" ADD CONSTRAINT "fk_temp_aliases_device_configuration_id" FOREIGN KEY ("device_configuration_id") REFERENCES "public"."device_configurations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "locations" ADD CONSTRAINT "fk_locations_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "locations" ADD CONSTRAINT "fk_locations_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "locations" ADD CONSTRAINT "fk_locations_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "locations" ADD CONSTRAINT "fk_locations_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "locations" ADD CONSTRAINT "fk_locations_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "locations" ADD CONSTRAINT "fk_locations_address_id" FOREIGN KEY ("address_id") REFERENCES "public"."addresses"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_remote_access_sessions" ADD CONSTRAINT "fk_device_remote_access_sessions_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_remote_access_sessions" ADD CONSTRAINT "fk_device_remote_access_sessions_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_remote_access_sessions" ADD CONSTRAINT "fk_device_remote_access_sessions_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_remote_access_sessions" ADD CONSTRAINT "fk_device_remote_access_sessions_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_remote_access_sessions" ADD CONSTRAINT "fk_device_remote_access_sessions_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "device_remote_access_sessions" ADD CONSTRAINT "fk_device_remote_access_sessions_device_id" FOREIGN KEY ("device_id") REFERENCES "public"."devices"("id") ON DELETE no action ON UPDATE no action;
