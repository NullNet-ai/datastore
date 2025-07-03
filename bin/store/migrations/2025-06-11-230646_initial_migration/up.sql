
--> statement-breakpoint
CREATE TABLE "allowed_fields" (
	"id" text PRIMARY KEY NOT NULL,
	"categories" text[] DEFAULT ARRAY[]::TEXT[],
	"code" text,
	"tombstone" integer DEFAULT 0,
	"status" text DEFAULT 'Active',
	"previous_status" text,
	"version" integer DEFAULT 1,
	"created_date" text,
	"created_time" text,
	"updated_date" text,
	"updated_time" text,
	"organization_id" text DEFAULT (null),
	"created_by" text DEFAULT (null),
	"updated_by" text DEFAULT (null),
	"deleted_by" text DEFAULT (null),
	"requested_by" text DEFAULT (null),
	"timestamp" text,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"label" text,
	"name" text,
	"type" text,
	"class_type_id" text,
	"is_optional" boolean DEFAULT false,
	"is_primary_key" boolean DEFAULT false,
	"reference_to" text,
	"data_type" text,
	"default_value" text
);
--> statement-breakpoint
CREATE TABLE "class_types" (
	"id" text PRIMARY KEY NOT NULL,
	"categories" text[] DEFAULT ARRAY[]::TEXT[],
	"code" text,
	"tombstone" integer DEFAULT 0,
	"status" text DEFAULT 'Active',
	"previous_status" text,
	"version" integer DEFAULT 1,
	"created_date" text,
	"created_time" text,
	"updated_date" text,
	"updated_time" text,
	"organization_id" text DEFAULT (null),
	"created_by" text DEFAULT (null),
	"updated_by" text DEFAULT (null),
	"deleted_by" text DEFAULT (null),
	"requested_by" text DEFAULT (null),
	"timestamp" text,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"type" text,
	"company" text,
	"entity" text,
	"is_list" boolean DEFAULT false,
	"is_with_version" boolean DEFAULT false,
	"schema_version" text
);
--> statement-breakpoint
CREATE TABLE "config_applications" (
	"id" text PRIMARY KEY NOT NULL,
	"categories" text[] DEFAULT ARRAY[]::TEXT[],
	"code" text,
	"tombstone" integer DEFAULT 0,
	"status" text DEFAULT 'Active',
	"previous_status" text,
	"version" integer DEFAULT 1,
	"created_date" text,
	"created_time" text,
	"updated_date" text,
	"updated_time" text,
	"organization_id" text DEFAULT (null),
	"created_by" text DEFAULT (null),
	"updated_by" text DEFAULT (null),
	"deleted_by" text DEFAULT (null),
	"requested_by" text DEFAULT (null),
	"timestamp" text,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"type" text,
	"value" text
);
--> statement-breakpoint
CREATE TABLE "counters" (
	"entity" text PRIMARY KEY NOT NULL,
	"default_code" integer DEFAULT 0,
	"prefix" text DEFAULT 'CTR',
	"counter" integer DEFAULT 0,
	"digits_number" integer DEFAULT 0
);
--> statement-breakpoint

--> statement-breakpoint
CREATE TABLE "crdt_merkles" (
	"group_id" text PRIMARY KEY NOT NULL,
	"timestamp" text NOT NULL,
	"merkle" text NOT NULL,
	CONSTRAINT "crdt_merkles_group_id_unique" UNIQUE("group_id")
);
--> statement-breakpoint
CREATE TABLE "crdt_messages" (
	"database" text,
	"dataset" text NOT NULL,
	"group_id" text NOT NULL,
	"timestamp" text NOT NULL,
	"row" text NOT NULL,
	"column" text NOT NULL,
	"client_id" text NOT NULL,
	"value" text NOT NULL,
	"operation" text,
	"hypertable_timestamp" text,
	CONSTRAINT "crdt_messages_timestamp_group_id_row_column_pk" PRIMARY KEY("timestamp","group_id","row","column")
);
--> statement-breakpoint
CREATE TABLE "organization_files" (
	"id" text PRIMARY KEY NOT NULL,
	"categories" text[] DEFAULT ARRAY[]::TEXT[],
	"code" text,
	"tombstone" integer DEFAULT 0,
	"status" text DEFAULT 'Active',
	"previous_status" text,
	"version" integer DEFAULT 1,
	"created_date" text,
	"created_time" text,
	"updated_date" text,
	"updated_time" text,
	"organization_id" text DEFAULT (null),
	"created_by" text DEFAULT (null),
	"updated_by" text DEFAULT (null),
	"deleted_by" text DEFAULT (null),
	"requested_by" text DEFAULT (null),
	"timestamp" text,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"organizaion_id" text,
	"organization_contact_id" text,
	"url" text,
	"name" text,
	"mime_type" text,
	"size" text,
	"type" text
);

--> statement-breakpoint
CREATE TABLE "queue_items" (
	"id" text PRIMARY KEY NOT NULL,
	"order" integer NOT NULL,
	"queue_id" text NOT NULL,
	"value" text NOT NULL
);
--> statement-breakpoint
CREATE TABLE "queues" (
	"id" text PRIMARY KEY NOT NULL,
	"name" text NOT NULL,
	"count" integer NOT NULL,
	"size" integer NOT NULL
);
--> statement-breakpoint
CREATE TABLE "sync_endpoints" (
	"id" text PRIMARY KEY NOT NULL,
	"name" text,
	"url" text,
	"group_id" text,
	"username" text,
	"password" text,
	"status" text
);
--> statement-breakpoint
CREATE TABLE "transactions" (
	"id" text PRIMARY KEY NOT NULL,
	"timestamp" text NOT NULL,
	"status" text DEFAULT 'Active' NOT NULL,
	"expiry" bigint
);
--> statement-breakpoint
CREATE TABLE "app_firewalls" (
	"id" uuid,
	"categories" text[] DEFAULT ARRAY[]::TEXT[],
	"code" text,
	"tombstone" integer DEFAULT 0,
	"status" text DEFAULT 'Active',
	"previous_status" text,
	"version" integer DEFAULT 1,
	"created_date" text,
	"created_time" text,
	"updated_date" text,
	"updated_time" text,
	"organization_id" text DEFAULT (null),
	"created_by" text DEFAULT (null),
	"updated_by" text DEFAULT (null),
	"deleted_by" text DEFAULT (null),
	"requested_by" text DEFAULT (null),
	"timestamp" text,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"app_id" text,
	"firewall" text,
	CONSTRAINT "app_firewalls_id_timestamp_app_id_pk" PRIMARY KEY("id","timestamp","app_id"),
	CONSTRAINT "app_firewalls_app_id_unique" UNIQUE("app_id")
);
--> statement-breakpoint
CREATE TABLE "appguard_logs" (
	"id" text,
	"categories" text[] DEFAULT ARRAY[]::TEXT[],
	"code" text,
	"tombstone" integer DEFAULT 0,
	"status" text DEFAULT 'Active',
	"previous_status" text,
	"version" integer DEFAULT 1,
	"created_date" text,
	"created_time" text,
	"updated_date" text,
	"updated_time" text,
	"organization_id" text DEFAULT (null),
	"created_by" text DEFAULT (null),
	"updated_by" text DEFAULT (null),
	"deleted_by" text DEFAULT (null),
	"requested_by" text DEFAULT (null),
	"timestamp" timestamp with time zone,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"level" text,
	"message" text,
	CONSTRAINT "appguard_logs_id_timestamp_pk" PRIMARY KEY("id","timestamp")
);
--> statement-breakpoint
CREATE TABLE "communication_templates" (
	"id" text PRIMARY KEY NOT NULL,
	"categories" text[] DEFAULT ARRAY[]::TEXT[],
	"code" text,
	"tombstone" integer DEFAULT 0,
	"status" text DEFAULT 'Active',
	"previous_status" text,
	"version" integer DEFAULT 1,
	"created_date" text,
	"created_time" text,
	"updated_date" text,
	"updated_time" text,
	"organization_id" text DEFAULT (null),
	"created_by" text DEFAULT (null),
	"updated_by" text DEFAULT (null),
	"deleted_by" text DEFAULT (null),
	"requested_by" text DEFAULT (null),
	"timestamp" text,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"name" text,
	"communication_template_status" text,
	"event" text,
	"content" text,
	"subject" text
);
--> statement-breakpoint
CREATE TABLE "connections" (
	"id" text,
	"categories" text[] DEFAULT ARRAY[]::TEXT[],
	"code" text,
	"tombstone" integer DEFAULT 0,
	"status" text DEFAULT 'Active',
	"previous_status" text,
	"version" integer DEFAULT 1,
	"created_date" text,
	"created_time" text,
	"updated_date" text,
	"updated_time" text,
	"organization_id" text DEFAULT (null),
	"created_by" text DEFAULT (null),
	"updated_by" text DEFAULT (null),
	"deleted_by" text DEFAULT (null),
	"requested_by" text DEFAULT (null),
	"timestamp" timestamp with time zone,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"interface_name" text,
	"total_packet" integer,
	"total_byte" integer,
	"device_id" text,
	"protocol" text,
	"source_ip" "inet",
	"destination_ip" "inet",
	"remote_ip" "inet",
	"source_port" integer,
	"destination_port" integer,
	"hypertable_timestamp" text,
	CONSTRAINT "connections_id_timestamp_pk" PRIMARY KEY("id","timestamp")
);

--> statement-breakpoint
CREATE TABLE "dead_letter_queue" (
	"id" text,
	"record_id" text,
	"created_date" timestamp DEFAULT now(),
	"table" text,
	"prefix" text,
	"error" text,
	CONSTRAINT "dead_letter_queue_id_pk" PRIMARY KEY("id")
);
--> statement-breakpoint
CREATE TABLE "device_aliases" (
	"id" text,
	"categories" text[] DEFAULT ARRAY[]::TEXT[],
	"code" text,
	"tombstone" integer DEFAULT 0,
	"status" text DEFAULT 'Active',
	"previous_status" text,
	"version" integer DEFAULT 1,
	"created_date" text,
	"created_time" text,
	"updated_date" text,
	"updated_time" text,
	"organization_id" text DEFAULT (null),
	"created_by" text DEFAULT (null),
	"updated_by" text DEFAULT (null),
	"deleted_by" text DEFAULT (null),
	"requested_by" text DEFAULT (null),
	"timestamp" text,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"device_configuration_id" text,
	"type" text,
	"name" text,
	"value" text,
	"description" text,
	"device_alias_status" text,
	CONSTRAINT "device_aliases_id_pk" PRIMARY KEY("id")
);
--> statement-breakpoint
CREATE TABLE "device_configurations" (
	"id" text,
	"categories" text[] DEFAULT ARRAY[]::TEXT[],
	"code" text,
	"tombstone" integer DEFAULT 0,
	"status" text DEFAULT 'Active',
	"previous_status" text,
	"version" integer DEFAULT 1,
	"created_date" text,
	"created_time" text,
	"updated_date" text,
	"updated_time" text,
	"organization_id" text DEFAULT (null),
	"created_by" text DEFAULT (null),
	"updated_by" text DEFAULT (null),
	"deleted_by" text DEFAULT (null),
	"requested_by" text DEFAULT (null),
	"timestamp" text,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"device_id" text,
	"digest" text,
	"hostname" text,
	"raw_content" text,
	"config_version" integer,
	CONSTRAINT "device_configurations_id_pk" PRIMARY KEY("id")
);
--> statement-breakpoint
CREATE TABLE "device_group_settings" (
	"id" text PRIMARY KEY NOT NULL,
	"categories" text[] DEFAULT ARRAY[]::TEXT[],
	"code" text,
	"tombstone" integer DEFAULT 0,
	"status" text DEFAULT 'Active',
	"previous_status" text,
	"version" integer DEFAULT 1,
	"created_date" text,
	"created_time" text,
	"updated_date" text,
	"updated_time" text,
	"organization_id" text DEFAULT (null),
	"created_by" text DEFAULT (null),
	"updated_by" text DEFAULT (null),
	"deleted_by" text DEFAULT (null),
	"requested_by" text DEFAULT (null),
	"timestamp" text,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"name" text
);
--> statement-breakpoint
CREATE TABLE "device_groups" (
	"id" text PRIMARY KEY NOT NULL,
	"categories" text[] DEFAULT ARRAY[]::TEXT[],
	"code" text,
	"tombstone" integer DEFAULT 0,
	"status" text DEFAULT 'Active',
	"previous_status" text,
	"version" integer DEFAULT 1,
	"created_date" text,
	"created_time" text,
	"updated_date" text,
	"updated_time" text,
	"organization_id" text DEFAULT (null),
	"created_by" text DEFAULT (null),
	"updated_by" text DEFAULT (null),
	"deleted_by" text DEFAULT (null),
	"requested_by" text DEFAULT (null),
	"timestamp" text,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"device_id" text,
	"device_group_setting_id" text
);
--> statement-breakpoint
CREATE TABLE "device_heartbeats" (
	"id" text,
	"categories" text[] DEFAULT ARRAY[]::TEXT[],
	"code" text,
	"tombstone" integer DEFAULT 0,
	"status" text DEFAULT 'Active',
	"previous_status" text,
	"version" integer DEFAULT 1,
	"created_date" text,
	"created_time" text,
	"updated_date" text,
	"updated_time" text,
	"organization_id" text DEFAULT (null),
	"created_by" text DEFAULT (null),
	"updated_by" text DEFAULT (null),
	"deleted_by" text DEFAULT (null),
	"requested_by" text DEFAULT (null),
	"timestamp" timestamp with time zone,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"device_id" text,
	"hypertable_timestamp" text,
	CONSTRAINT "device_heartbeats_id_timestamp_pk" PRIMARY KEY("id","timestamp")
);
--> statement-breakpoint
CREATE TABLE "device_interface_addresses" (
	"id" text,
	"categories" text[] DEFAULT ARRAY[]::TEXT[],
	"code" text,
	"tombstone" integer DEFAULT 0,
	"status" text DEFAULT 'Active',
	"previous_status" text,
	"version" integer,
	"created_date" text,
	"created_time" text,
	"updated_date" text,
	"updated_time" text,
	"organization_id" text DEFAULT (null),
	"created_by" text DEFAULT (null),
	"updated_by" text DEFAULT (null),
	"deleted_by" text DEFAULT (null),
	"requested_by" text DEFAULT (null),
	"timestamp" text,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"device_interface_id" text,
	"address" "inet",
	CONSTRAINT "device_interface_addresses_id_pk" PRIMARY KEY("id")
);
--> statement-breakpoint
CREATE TABLE "device_interfaces" (
	"id" text,
	"categories" text[] DEFAULT ARRAY[]::TEXT[],
	"code" text,
	"tombstone" integer DEFAULT 0,
	"status" text DEFAULT 'Active',
	"previous_status" text,
	"version" integer DEFAULT 1,
	"created_date" text,
	"created_time" text,
	"updated_date" text,
	"updated_time" text,
	"organization_id" text DEFAULT (null),
	"created_by" text DEFAULT (null),
	"updated_by" text DEFAULT (null),
	"deleted_by" text DEFAULT (null),
	"requested_by" text DEFAULT (null),
	"timestamp" text,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"device_configuration_id" text,
	"name" text,
	"device" text,
	"address" "inet",
	CONSTRAINT "device_interfaces_id_pk" PRIMARY KEY("id")
);
--> statement-breakpoint
CREATE TABLE "device_remote_access_sessions" (
	"id" text,
	"categories" text[] DEFAULT ARRAY[]::TEXT[],
	"code" text,
	"tombstone" integer DEFAULT 0,
	"status" text DEFAULT 'Active',
	"previous_status" text,
	"version" integer DEFAULT 1,
	"created_date" text,
	"created_time" text,
	"updated_date" text,
	"updated_time" text,
	"organization_id" text DEFAULT (null),
	"created_by" text DEFAULT (null),
	"updated_by" text DEFAULT (null),
	"deleted_by" text DEFAULT (null),
	"requested_by" text DEFAULT (null),
	"timestamp" timestamp with time zone DEFAULT now(),
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"device_id" text,
	"remote_access_type" text,
	"remote_access_session" text,
	"remote_access_status" text,
	"remote_access_category" text,
	CONSTRAINT "device_remote_access_sessions_id_pk" PRIMARY KEY("id")
);
--> statement-breakpoint
CREATE TABLE "device_rules" (
	"id" text,
	"categories" text[] DEFAULT ARRAY[]::TEXT[],
	"code" text,
	"tombstone" integer DEFAULT 0,
	"status" text DEFAULT 'Active',
	"previous_status" text,
	"version" integer DEFAULT 1,
	"created_date" text,
	"created_time" text,
	"updated_date" text,
	"updated_time" text,
	"organization_id" text DEFAULT (null),
	"created_by" text DEFAULT (null),
	"updated_by" text DEFAULT (null),
	"deleted_by" text DEFAULT (null),
	"requested_by" text DEFAULT (null),
	"timestamp" text,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"device_configuration_id" text,
	"disabled" boolean,
	"type" text,
	"policy" text,
	"protocol" text,
	"source_port" text,
	"source_addr" text,
	"source_type" text,
	"destination_port" text,
	"destination_addr" text,
	"description" text,
	"device_rule_status" text,
	"interface" text,
	"order" integer,
	"destination_inversed" boolean,
	"destination_type" text,
	"source_inversed" boolean,
	CONSTRAINT "device_rules_id_pk" PRIMARY KEY("id")
);
--> statement-breakpoint
CREATE TABLE "device_ssh_keys" (
	"id" text,
	"categories" text[] DEFAULT ARRAY[]::TEXT[],
	"code" text,
	"tombstone" integer DEFAULT 0,
	"status" text DEFAULT 'Active',
	"previous_status" text,
	"version" integer DEFAULT 1,
	"created_date" text,
	"created_time" text,
	"updated_date" text,
	"updated_time" text,
	"organization_id" text DEFAULT (null),
	"created_by" text DEFAULT (null),
	"updated_by" text DEFAULT (null),
	"deleted_by" text DEFAULT (null),
	"requested_by" text DEFAULT (null),
	"timestamp" text,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"public_key" text,
	"private_key" text,
	"passphrase" text,
	"device_id" text,
	CONSTRAINT "device_ssh_keys_id_timestamp_pk" PRIMARY KEY("id","timestamp")
);
--> statement-breakpoint
CREATE TABLE "dummy_packets" (
	"id" text,
	"categories" text[] DEFAULT ARRAY[]::TEXT[],
	"code" text,
	"tombstone" integer DEFAULT 0,
	"status" text DEFAULT 'Active',
	"previous_status" text,
	"version" integer DEFAULT 1,
	"created_date" text,
	"created_time" text,
	"updated_date" text,
	"updated_time" text,
	"organization_id" text DEFAULT (null),
	"created_by" text DEFAULT (null),
	"updated_by" text DEFAULT (null),
	"deleted_by" text DEFAULT (null),
	"requested_by" text DEFAULT (null),
	"timestamp" timestamp with time zone,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"interface_name" text,
	"total_length" integer,
	"device_id" text,
	"ether_type" text,
	"protocol" text,
	"source_ip" "inet",
	"destination_ip" "inet",
	"remote_ip" "inet",
	"source_port" integer,
	"destination_port" integer,
	"hypertable_timestamp" text,
	"source_mac" text,
	"destination_mac" text,
	"tcp_header_length" integer,
	"tcp_sequence_number" bigint,
	"tcp_acknowledgment_number" bigint,
	"tcp_data_offset" integer,
	"tcp_flags" integer,
	"tcp_window_size" integer,
	"tcp_urgent_pointer" integer,
	"icmp_type" integer,
	"icmp_code" integer,
	CONSTRAINT "dummy_packets_id_timestamp_pk" PRIMARY KEY("id","timestamp")
);
--> statement-breakpoint
CREATE TABLE "files" (
	"id" text PRIMARY KEY NOT NULL,
	"categories" text[] DEFAULT ARRAY[]::TEXT[],
	"code" text,
	"tombstone" integer DEFAULT 0,
	"status" text DEFAULT 'Active',
	"previous_status" text,
	"version" integer DEFAULT 1,
	"created_date" text,
	"created_time" text,
	"updated_date" text,
	"updated_time" text,
	"organization_id" text DEFAULT (null),
	"created_by" text DEFAULT (null),
	"updated_by" text DEFAULT (null),
	"deleted_by" text DEFAULT (null),
	"requested_by" text DEFAULT (null),
	"timestamp" text,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"fieldname" text,
	"originalname" text,
	"encoding" text,
	"mimetype" text,
	"destination" text,
	"filename" text,
	"path" text,
	"size" integer,
	"uploaded_by" text,
	"downloaded_by" text,
	"etag" text,
	"versionId" text,
	"download_path" text,
	"presignedURL" text,
	"presignedURLExpires" integer
);
--> statement-breakpoint
CREATE TABLE "grid_filters" (
	"id" text PRIMARY KEY NOT NULL,
	"categories" text[] DEFAULT ARRAY[]::TEXT[],
	"code" text,
	"tombstone" integer DEFAULT 0,
	"status" text DEFAULT 'Active',
	"previous_status" text,
	"version" integer DEFAULT 1,
	"created_date" text,
	"created_time" text,
	"updated_date" text,
	"updated_time" text,
	"organization_id" text DEFAULT (null),
	"created_by" text DEFAULT (null),
	"updated_by" text DEFAULT (null),
	"deleted_by" text DEFAULT (null),
	"requested_by" text DEFAULT (null),
	"timestamp" text,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"name" text,
	"grid_id" text,
	"link" text DEFAULT '',
	"is_current" boolean DEFAULT false,
	"is_default" boolean DEFAULT false,
	"contact_id" text,
	"account_organization_id" text,
	"entity" text,
	"columns" jsonb DEFAULT '[]'::jsonb,
	"groups" jsonb DEFAULT '[]'::jsonb,
	"sorts" jsonb DEFAULT '[]'::jsonb,
	"default_sorts" jsonb DEFAULT '[]'::jsonb,
	"advance_filters" jsonb DEFAULT '[]'::jsonb,
	"group_advance_filters" jsonb DEFAULT '[]'::jsonb,
	"filter_groups" jsonb DEFAULT '[]'::jsonb
);
--> statement-breakpoint
CREATE TABLE "http_requests" (
	"id" text,
	"categories" text[] DEFAULT ARRAY[]::TEXT[],
	"code" text,
	"tombstone" integer DEFAULT 0,
	"status" text DEFAULT 'Active',
	"previous_status" text,
	"version" integer DEFAULT 1,
	"created_date" text,
	"created_time" text,
	"updated_date" text,
	"updated_time" text,
	"organization_id" text DEFAULT (null),
	"created_by" text DEFAULT (null),
	"updated_by" text DEFAULT (null),
	"deleted_by" text DEFAULT (null),
	"requested_by" text DEFAULT (null),
	"timestamp" timestamp with time zone,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"fw_policy" text,
	"fw_reasons" text,
	"ip" "inet",
	"original_url" text,
	"user_agent" text,
	"headers" text,
	"method" text,
	"body" text,
	"query" text,
	"cookies" text,
	CONSTRAINT "http_requests_id_timestamp_pk" PRIMARY KEY("id","timestamp")
);
--> statement-breakpoint
CREATE TABLE "http_responses" (
	"id" text,
	"categories" text[] DEFAULT ARRAY[]::TEXT[],
	"code" text,
	"tombstone" integer DEFAULT 0,
	"status" text DEFAULT 'Active',
	"previous_status" text,
	"version" integer DEFAULT 1,
	"created_date" text,
	"created_time" text,
	"updated_date" text,
	"updated_time" text,
	"organization_id" text DEFAULT (null),
	"created_by" text DEFAULT (null),
	"updated_by" text DEFAULT (null),
	"deleted_by" text DEFAULT (null),
	"requested_by" text DEFAULT (null),
	"timestamp" timestamp with time zone,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"fw_policy" text,
	"fw_reasons" text,
	"ip" "inet",
	"response_code" bigint,
	"headers" text,
	"time" bigint,
	"size" bigint,
	CONSTRAINT "http_responses_id_timestamp_pk" PRIMARY KEY("id","timestamp")
);
--> statement-breakpoint
CREATE TABLE "invitations" (
	"id" text PRIMARY KEY NOT NULL,
	"categories" text[] DEFAULT ARRAY[]::TEXT[],
	"code" text,
	"tombstone" integer DEFAULT 0,
	"status" text DEFAULT 'Active',
	"previous_status" text,
	"version" integer DEFAULT 1,
	"created_date" text,
	"created_time" text,
	"updated_date" text,
	"updated_time" text,
	"organization_id" text DEFAULT (null),
	"created_by" text DEFAULT (null),
	"updated_by" text DEFAULT (null),
	"deleted_by" text DEFAULT (null),
	"requested_by" text DEFAULT (null),
	"timestamp" text,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"account_id" text,
	"expiration_date" text,
	"expiration_time" text,
	"account_organization_id" text
);
--> statement-breakpoint
CREATE TABLE "ip_blacklist" (
	"id" text,
	"categories" text[] DEFAULT ARRAY[]::TEXT[],
	"code" text,
	"tombstone" integer DEFAULT 0,
	"status" text DEFAULT 'Active',
	"previous_status" text,
	"version" integer DEFAULT 1,
	"created_date" text,
	"created_time" text,
	"updated_date" text,
	"updated_time" text,
	"organization_id" text DEFAULT (null),
	"created_by" text DEFAULT (null),
	"updated_by" text DEFAULT (null),
	"deleted_by" text DEFAULT (null),
	"requested_by" text DEFAULT (null),
	"timestamp" text,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"ip" "inet",
	CONSTRAINT "ip_blacklist_id_pk" PRIMARY KEY("id")
);
--> statement-breakpoint
CREATE TABLE "ip_infos" (
	"id" text,
	"categories" text[] DEFAULT ARRAY[]::TEXT[],
	"code" text,
	"tombstone" integer DEFAULT 0,
	"status" text DEFAULT 'Active',
	"previous_status" text,
	"version" integer DEFAULT 1,
	"created_date" text,
	"created_time" text,
	"updated_date" text,
	"updated_time" text,
	"organization_id" text DEFAULT (null),
	"created_by" text DEFAULT (null),
	"updated_by" text DEFAULT (null),
	"deleted_by" text DEFAULT (null),
	"requested_by" text DEFAULT (null),
	"timestamp" timestamp with time zone,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"ip" text,
	"country" text,
	"asn" text,
	"org" text,
	"continent_code" text,
	"city" text,
	"region" text,
	"postal" text,
	"timezone" text,
	"blacklist" boolean,
	CONSTRAINT "ip_infos_id_timestamp_ip_pk" PRIMARY KEY("id","timestamp","ip"),
	CONSTRAINT "ip_infos_ip_unique" UNIQUE("ip")
);
--> statement-breakpoint
CREATE TABLE "locations" (
	"id" text PRIMARY KEY NOT NULL,
	"categories" text[] DEFAULT ARRAY[]::TEXT[],
	"code" text,
	"tombstone" integer DEFAULT 0,
	"status" text DEFAULT 'Active',
	"previous_status" text,
	"version" integer DEFAULT 1,
	"created_date" text,
	"created_time" text,
	"updated_date" text,
	"updated_time" text,
	"organization_id" text DEFAULT (null),
	"created_by" text DEFAULT (null),
	"updated_by" text DEFAULT (null),
	"deleted_by" text DEFAULT (null),
	"requested_by" text DEFAULT (null),
	"timestamp" text,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"location_name" text,
	"address_id" text
);
--> statement-breakpoint
CREATE TABLE "notifications" (
	"id" text PRIMARY KEY NOT NULL,
	"categories" text[] DEFAULT ARRAY[]::TEXT[],
	"code" text,
	"tombstone" integer DEFAULT 0,
	"status" text DEFAULT 'Active',
	"previous_status" text,
	"version" integer DEFAULT 1,
	"created_date" text,
	"created_time" text,
	"updated_date" text,
	"updated_time" text,
	"organization_id" text DEFAULT (null),
	"created_by" text DEFAULT (null),
	"updated_by" text DEFAULT (null),
	"deleted_by" text DEFAULT (null),
	"requested_by" text DEFAULT (null),
	"timestamp" text,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"title" text,
	"description" text,
	"event_timestamp" text,
	"link" text DEFAULT '',
	"icon" text DEFAULT '',
	"source" text,
	"is_pinned" boolean DEFAULT false,
	"recipient_id" text,
	"actions" jsonb DEFAULT '[]'::jsonb,
	"notification_status" text DEFAULT 'unread',
	"priority_label" text DEFAULT 'low',
	"priority_level" integer DEFAULT 0,
	"expiry_date" text DEFAULT '',
	"metadata" text
);
--> statement-breakpoint
CREATE TABLE "organization_contact_user_roles" (
	"id" text PRIMARY KEY NOT NULL,
	"categories" text[] DEFAULT ARRAY[]::TEXT[],
	"code" text,
	"tombstone" integer DEFAULT 0,
	"status" text DEFAULT 'Active',
	"previous_status" text,
	"version" integer DEFAULT 1,
	"created_date" text,
	"created_time" text,
	"updated_date" text,
	"updated_time" text,
	"organization_id" text DEFAULT (null),
	"created_by" text DEFAULT (null),
	"updated_by" text DEFAULT (null),
	"deleted_by" text DEFAULT (null),
	"requested_by" text DEFAULT (null),
	"timestamp" text,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"organization_contact_id" text,
	"user_role_id" text
);

--> statement-breakpoint
CREATE TABLE "packets" (
	"id" text,
	"categories" text[] DEFAULT ARRAY[]::TEXT[],
	"code" text,
	"tombstone" integer DEFAULT 0,
	"status" text DEFAULT 'Active',
	"previous_status" text,
	"version" integer DEFAULT 1,
	"created_date" text,
	"created_time" text,
	"updated_date" text,
	"updated_time" text,
	"organization_id" text DEFAULT (null),
	"created_by" text DEFAULT (null),
	"updated_by" text DEFAULT (null),
	"deleted_by" text DEFAULT (null),
	"requested_by" text DEFAULT (null),
	"timestamp" timestamp with time zone,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"interface_name" text,
	"total_length" integer,
	"device_id" text,
	"ether_type" text,
	"protocol" text,
	"source_ip" "inet",
	"destination_ip" "inet",
	"remote_ip" "inet",
	"source_port" integer,
	"destination_port" integer,
	"hypertable_timestamp" text,
	"source_mac" text,
	"destination_mac" text,
	"tcp_header_length" integer,
	"tcp_sequence_number" bigint,
	"tcp_acknowledgment_number" bigint,
	"tcp_data_offset" integer,
	"tcp_flags" integer,
	"tcp_window_size" integer,
	"tcp_urgent_pointer" integer,
	"icmp_type" integer,
	"icmp_code" integer,
	CONSTRAINT "packets_id_timestamp_pk" PRIMARY KEY("id","timestamp")
);
--> statement-breakpoint
CREATE TABLE "postgres_channels" (
	"id" text PRIMARY KEY NOT NULL,
	"categories" text[] DEFAULT ARRAY[]::TEXT[],
	"code" text,
	"tombstone" integer DEFAULT 0,
	"status" text DEFAULT 'Active',
	"previous_status" text,
	"version" integer DEFAULT 1,
	"created_date" text,
	"created_time" text,
	"updated_date" text,
	"updated_time" text,
	"organization_id" text DEFAULT (null),
	"created_by" text DEFAULT (null),
	"updated_by" text DEFAULT (null),
	"deleted_by" text DEFAULT (null),
	"requested_by" text DEFAULT (null),
	"timestamp" timestamp with time zone,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"channel_name" text,
	"function" text,
	CONSTRAINT "postgres_channels_channel_name_unique" UNIQUE("channel_name")
);
--> statement-breakpoint
CREATE TABLE "resolutions" (
	"id" text PRIMARY KEY NOT NULL,
	"categories" text[] DEFAULT ARRAY[]::TEXT[],
	"code" text,
	"tombstone" integer DEFAULT 0,
	"status" text DEFAULT 'Active',
	"previous_status" text,
	"version" integer DEFAULT 1,
	"created_date" text,
	"created_time" text,
	"updated_date" text,
	"updated_time" text,
	"organization_id" text DEFAULT (null),
	"created_by" text DEFAULT (null),
	"updated_by" text DEFAULT (null),
	"deleted_by" text DEFAULT (null),
	"requested_by" text DEFAULT (null),
	"timestamp" text,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"resolution_type" text
);
--> statement-breakpoint
CREATE TABLE "smtp_requests" (
	"id" text,
	"categories" text[] DEFAULT ARRAY[]::TEXT[],
	"code" text,
	"tombstone" integer DEFAULT 0,
	"status" text DEFAULT 'Active',
	"previous_status" text,
	"version" integer DEFAULT 1,
	"created_date" text,
	"created_time" text,
	"updated_date" text,
	"updated_time" text,
	"organization_id" text DEFAULT (null),
	"created_by" text DEFAULT (null),
	"updated_by" text DEFAULT (null),
	"deleted_by" text DEFAULT (null),
	"requested_by" text DEFAULT (null),
	"timestamp" timestamp with time zone,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"fw_policy" text,
	"fw_reasons" text,
	"ip" "inet",
	"user_agent" text,
	"headers" text,
	"body" text,
	CONSTRAINT "smtp_requests_id_timestamp_pk" PRIMARY KEY("id","timestamp")
);
--> statement-breakpoint
CREATE TABLE "smtp_responses" (
	"id" text,
	"categories" text[] DEFAULT ARRAY[]::TEXT[],
	"code" text,
	"tombstone" integer DEFAULT 0,
	"status" text DEFAULT 'Active',
	"previous_status" text,
	"version" integer DEFAULT 1,
	"created_date" text,
	"created_time" text,
	"updated_date" text,
	"updated_time" text,
	"organization_id" text DEFAULT (null),
	"created_by" text DEFAULT (null),
	"updated_by" text DEFAULT (null),
	"deleted_by" text DEFAULT (null),
	"requested_by" text DEFAULT (null),
	"timestamp" timestamp with time zone,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"fw_policy" text,
	"fw_reasons" text,
	"ip" "inet",
	"response_code" bigint,
	"time" bigint,
	CONSTRAINT "smtp_responses_id_timestamp_pk" PRIMARY KEY("id","timestamp")
);
--> statement-breakpoint
CREATE TABLE "system_resources" (
	"id" text,
	"categories" text[] DEFAULT ARRAY[]::TEXT[],
	"code" text,
	"tombstone" integer DEFAULT 0,
	"status" text DEFAULT 'Active',
	"previous_status" text,
	"version" integer DEFAULT 1,
	"created_date" text,
	"created_time" text,
	"updated_date" text,
	"updated_time" text,
	"organization_id" text DEFAULT (null),
	"created_by" text DEFAULT (null),
	"updated_by" text DEFAULT (null),
	"deleted_by" text DEFAULT (null),
	"requested_by" text DEFAULT (null),
	"timestamp" timestamp with time zone,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"num_cpus" integer,
	"global_cpu_usage" double precision,
	"cpu_usages" text,
	"total_memory" bigint,
	"used_memory" bigint,
	"total_disk_space" bigint,
	"available_disk_space" bigint,
	"read_bytes" bigint,
	"written_bytes" bigint,
	"temperatures" text,
	CONSTRAINT "system_resources_id_timestamp_pk" PRIMARY KEY("id","timestamp")
);
--> statement-breakpoint
CREATE TABLE "tcp_connections" (
	"id" text,
	"categories" text[] DEFAULT ARRAY[]::TEXT[],
	"code" text,
	"tombstone" integer DEFAULT 0,
	"status" text DEFAULT 'Active',
	"previous_status" text,
	"version" integer DEFAULT 1,
	"created_date" text,
	"created_time" text,
	"updated_date" text,
	"updated_time" text,
	"organization_id" text DEFAULT (null),
	"created_by" text DEFAULT (null),
	"updated_by" text DEFAULT (null),
	"deleted_by" text DEFAULT (null),
	"requested_by" text DEFAULT (null),
	"timestamp" timestamp with time zone,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"source" "inet",
	"sport" integer,
	"dest" "inet",
	"dport" integer,
	"proto" text,
	CONSTRAINT "tcp_connections_id_timestamp_pk" PRIMARY KEY("id","timestamp")
);
--> statement-breakpoint
CREATE TABLE "temp_appguard_logs" (
	"id" text,
	"categories" text[] DEFAULT ARRAY[]::TEXT[],
	"code" text,
	"tombstone" integer DEFAULT 0,
	"status" text DEFAULT 'Active',
	"previous_status" text,
	"version" integer DEFAULT 1,
	"created_date" text,
	"created_time" text,
	"updated_date" text,
	"updated_time" text,
	"organization_id" text DEFAULT (null),
	"created_by" text DEFAULT (null),
	"updated_by" text DEFAULT (null),
	"deleted_by" text DEFAULT (null),
	"requested_by" text DEFAULT (null),
	"timestamp" timestamp with time zone,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"level" text,
	"message" text,
	CONSTRAINT "temp_appguard_logs_id_timestamp_pk" PRIMARY KEY("id","timestamp")
);
--> statement-breakpoint
CREATE TABLE "temp_connections" (
	"id" text,
	"categories" text[] DEFAULT ARRAY[]::TEXT[],
	"code" text,
	"tombstone" integer DEFAULT 0,
	"status" text DEFAULT 'Active',
	"previous_status" text,
	"version" integer DEFAULT 1,
	"created_date" text,
	"created_time" text,
	"updated_date" text,
	"updated_time" text,
	"organization_id" text DEFAULT (null),
	"created_by" text DEFAULT (null),
	"updated_by" text DEFAULT (null),
	"deleted_by" text DEFAULT (null),
	"requested_by" text DEFAULT (null),
	"timestamp" timestamp with time zone,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"hypertable_timestamp" text,
	"interface_name" text,
	"total_packet" integer,
	"total_byte" integer,
	"device_id" text,
	"protocol" text,
	"source_ip" "inet",
	"destination_ip" "inet",
	"source_port" integer,
	"destination_port" integer,
	"remote_ip" text,
	CONSTRAINT "temp_connections_id_timestamp_pk" PRIMARY KEY("id","timestamp")
);
--> statement-breakpoint
CREATE TABLE "temp_device_aliases" (
	"id" text,
	"categories" text[] DEFAULT ARRAY[]::TEXT[],
	"code" text,
	"tombstone" integer DEFAULT 0,
	"status" text DEFAULT 'Active',
	"previous_status" text,
	"version" integer DEFAULT 1,
	"created_date" text,
	"created_time" text,
	"updated_date" text,
	"updated_time" text,
	"organization_id" text DEFAULT (null),
	"created_by" text DEFAULT (null),
	"updated_by" text DEFAULT (null),
	"deleted_by" text DEFAULT (null),
	"requested_by" text DEFAULT (null),
	"timestamp" text,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"device_configuration_id" text,
	"type" text,
	"name" text,
	"value" text,
	"description" text,
	"device_alias_status" text,
	CONSTRAINT "temp_device_aliases_id_pk" PRIMARY KEY("id")
);
--> statement-breakpoint
CREATE TABLE "temp_device_interface_addresses" (
	"id" text,
	"categories" text[] DEFAULT ARRAY[]::TEXT[],
	"code" text,
	"tombstone" integer DEFAULT 0,
	"status" text DEFAULT 'Active',
	"previous_status" text,
	"version" integer,
	"created_date" text,
	"created_time" text,
	"updated_date" text,
	"updated_time" text,
	"organization_id" text DEFAULT (null),
	"created_by" text DEFAULT (null),
	"updated_by" text DEFAULT (null),
	"deleted_by" text DEFAULT (null),
	"requested_by" text DEFAULT (null),
	"timestamp" text,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"device_interface_id" text,
	"address" "inet",
	CONSTRAINT "temp_device_interface_addresses_id_pk" PRIMARY KEY("id")
);
--> statement-breakpoint
CREATE TABLE "temp_device_interfaces" (
	"id" text,
	"categories" text[] DEFAULT ARRAY[]::TEXT[],
	"code" text,
	"tombstone" integer DEFAULT 0,
	"status" text DEFAULT 'Active',
	"previous_status" text,
	"version" integer DEFAULT 1,
	"created_date" text,
	"created_time" text,
	"updated_date" text,
	"updated_time" text,
	"organization_id" text DEFAULT (null),
	"created_by" text DEFAULT (null),
	"updated_by" text DEFAULT (null),
	"deleted_by" text DEFAULT (null),
	"requested_by" text DEFAULT (null),
	"timestamp" text,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"device_configuration_id" text,
	"name" text,
	"device" text,
	"address" "inet",
	CONSTRAINT "temp_device_interfaces_id_pk" PRIMARY KEY("id")
);
--> statement-breakpoint
CREATE TABLE "temp_device_remote_access_sessions" (
	"id" text,
	"categories" text[] DEFAULT ARRAY[]::TEXT[],
	"code" text,
	"tombstone" integer DEFAULT 0,
	"status" text DEFAULT 'Active',
	"previous_status" text,
	"version" integer DEFAULT 1,
	"created_date" text,
	"created_time" text,
	"updated_date" text,
	"updated_time" text,
	"organization_id" text DEFAULT (null),
	"created_by" text DEFAULT (null),
	"updated_by" text DEFAULT (null),
	"deleted_by" text DEFAULT (null),
	"requested_by" text DEFAULT (null),
	"timestamp" timestamp with time zone DEFAULT now(),
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"device_id" text,
	"remote_access_type" text,
	"remote_access_session" text,
	"remote_access_status" text,
	"remote_access_category" text,
	CONSTRAINT "temp_device_remote_access_sessions_id_pk" PRIMARY KEY("id")
);
--> statement-breakpoint
CREATE TABLE "temp_device_rules" (
	"id" text,
	"categories" text[] DEFAULT ARRAY[]::TEXT[],
	"code" text,
	"tombstone" integer DEFAULT 0,
	"status" text DEFAULT 'Active',
	"previous_status" text,
	"version" integer DEFAULT 1,
	"created_date" text,
	"created_time" text,
	"updated_date" text,
	"updated_time" text,
	"organization_id" text DEFAULT (null),
	"created_by" text DEFAULT (null),
	"updated_by" text DEFAULT (null),
	"deleted_by" text DEFAULT (null),
	"requested_by" text DEFAULT (null),
	"timestamp" text,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"device_configuration_id" text,
	"disabled" boolean,
	"type" text,
	"policy" text,
	"protocol" text,
	"source_port" text,
	"source_addr" text,
	"source_type" text,
	"source_inversed" boolean,
	"destination_port" text,
	"destination_addr" text,
	"destination_type" text,
	"destination_inversed" boolean,
	"description" text,
	"device_rule_status" text,
	"interface" text,
	"order" integer,
	CONSTRAINT "temp_device_rules_id_pk" PRIMARY KEY("id")
);
--> statement-breakpoint
CREATE TABLE "temp_ip_blacklist" (
	"id" text,
	"categories" text[] DEFAULT ARRAY[]::TEXT[],
	"code" text,
	"tombstone" integer DEFAULT 0,
	"status" text DEFAULT 'Active',
	"previous_status" text,
	"version" integer DEFAULT 1,
	"created_date" text,
	"created_time" text,
	"updated_date" text,
	"updated_time" text,
	"organization_id" text DEFAULT (null),
	"created_by" text DEFAULT (null),
	"updated_by" text DEFAULT (null),
	"deleted_by" text DEFAULT (null),
	"requested_by" text DEFAULT (null),
	"timestamp" text,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"ip" "inet",
	CONSTRAINT "temp_ip_blacklist_id_pk" PRIMARY KEY("id")
);
--> statement-breakpoint
CREATE TABLE "temp_packets" (
	"id" text,
	"categories" text[] DEFAULT ARRAY[]::TEXT[],
	"code" text,
	"tombstone" integer DEFAULT 0,
	"status" text DEFAULT 'Active',
	"previous_status" text,
	"version" integer DEFAULT 1,
	"created_date" text,
	"created_time" text,
	"updated_date" text,
	"updated_time" text,
	"organization_id" text DEFAULT (null),
	"created_by" text DEFAULT (null),
	"updated_by" text DEFAULT (null),
	"deleted_by" text DEFAULT (null),
	"requested_by" text DEFAULT (null),
	"timestamp" timestamp with time zone,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"hypertable_timestamp" text,
	"interface_name" text,
	"total_length" integer,
	"device_id" text,
	"source_mac" text,
	"destination_mac" text,
	"ether_type" text,
	"protocol" text,
	"source_ip" "inet",
	"destination_ip" "inet",
	"source_port" integer,
	"destination_port" integer,
	"remote_ip" text,
	"tcp_header_length" integer,
	"tcp_sequence_number" bigint,
	"tcp_acknowledgment_number" bigint,
	"tcp_data_offset" integer,
	"tcp_flags" integer,
	"tcp_window_size" integer,
	"tcp_urgent_pointer" integer,
	"icmp_type" integer,
	"icmp_code" integer,
	CONSTRAINT "temp_packets_id_timestamp_pk" PRIMARY KEY("id","timestamp")
);
--> statement-breakpoint
CREATE TABLE "temp_system_resources" (
	"id" text,
	"categories" text[] DEFAULT ARRAY[]::TEXT[],
	"code" text,
	"tombstone" integer DEFAULT 0,
	"status" text DEFAULT 'Active',
	"previous_status" text,
	"version" integer DEFAULT 1,
	"created_date" text,
	"created_time" text,
	"updated_date" text,
	"updated_time" text,
	"organization_id" text DEFAULT (null),
	"created_by" text DEFAULT (null),
	"updated_by" text DEFAULT (null),
	"deleted_by" text DEFAULT (null),
	"requested_by" text DEFAULT (null),
	"timestamp" timestamp with time zone,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"num_cpus" integer,
	"global_cpu_usage" double precision,
	"cpu_usages" text,
	"total_memory" bigint,
	"used_memory" bigint,
	"total_disk_space" bigint,
	"available_disk_space" bigint,
	"read_bytes" bigint,
	"written_bytes" bigint,
	"temperatures" text,
	CONSTRAINT "temp_system_resources_id_timestamp_pk" PRIMARY KEY("id","timestamp")
);
--> statement-breakpoint
CREATE TABLE "temp_wallguard_logs" (
	"id" text,
	"categories" text[] DEFAULT ARRAY[]::TEXT[],
	"code" text,
	"tombstone" integer DEFAULT 0,
	"status" text DEFAULT 'Active',
	"previous_status" text,
	"version" integer DEFAULT 1,
	"created_date" text,
	"created_time" text,
	"updated_date" text,
	"updated_time" text,
	"organization_id" text DEFAULT (null),
	"created_by" text DEFAULT (null),
	"updated_by" text DEFAULT (null),
	"deleted_by" text DEFAULT (null),
	"requested_by" text DEFAULT (null),
	"timestamp" timestamp with time zone,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"level" text,
	"message" text,
	CONSTRAINT "temp_wallguard_logs_id_timestamp_pk" PRIMARY KEY("id","timestamp")
);

--> statement-breakpoint
CREATE TABLE "wallguard_logs" (
	"id" text,
	"categories" text[] DEFAULT ARRAY[]::TEXT[],
	"code" text,
	"tombstone" integer DEFAULT 0,
	"status" text DEFAULT 'Active',
	"previous_status" text,
	"version" integer DEFAULT 1,
	"created_date" text,
	"created_time" text,
	"updated_date" text,
	"updated_time" text,
	"organization_id" text DEFAULT (null),
	"created_by" text DEFAULT (null),
	"updated_by" text DEFAULT (null),
	"deleted_by" text DEFAULT (null),
	"requested_by" text DEFAULT (null),
	"timestamp" timestamp with time zone,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"level" text,
	"message" text,
	CONSTRAINT "wallguard_logs_id_timestamp_pk" PRIMARY KEY("id","timestamp")
);
--> statement-breakpoint
ALTER TABLE "allowed_fields" ADD CONSTRAINT "allowed_fields_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "allowed_fields" ADD CONSTRAINT "allowed_fields_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "allowed_fields" ADD CONSTRAINT "allowed_fields_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "allowed_fields" ADD CONSTRAINT "allowed_fields_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "allowed_fields" ADD CONSTRAINT "allowed_fields_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "allowed_fields" ADD CONSTRAINT "allowed_fields_class_type_id_class_types_id_fk" FOREIGN KEY ("class_type_id") REFERENCES "public"."class_types"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "class_types" ADD CONSTRAINT "class_types_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "class_types" ADD CONSTRAINT "class_types_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "class_types" ADD CONSTRAINT "class_types_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "class_types" ADD CONSTRAINT "class_types_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "class_types" ADD CONSTRAINT "class_types_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "app_firewalls" ADD CONSTRAINT "app_firewalls_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "app_firewalls" ADD CONSTRAINT "app_firewalls_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "app_firewalls" ADD CONSTRAINT "app_firewalls_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "app_firewalls" ADD CONSTRAINT "app_firewalls_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "app_firewalls" ADD CONSTRAINT "app_firewalls_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "appguard_logs" ADD CONSTRAINT "appguard_logs_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "appguard_logs" ADD CONSTRAINT "appguard_logs_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "appguard_logs" ADD CONSTRAINT "appguard_logs_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "appguard_logs" ADD CONSTRAINT "appguard_logs_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "appguard_logs" ADD CONSTRAINT "appguard_logs_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "communication_templates" ADD CONSTRAINT "communication_templates_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "communication_templates" ADD CONSTRAINT "communication_templates_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "communication_templates" ADD CONSTRAINT "communication_templates_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "communication_templates" ADD CONSTRAINT "communication_templates_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "communication_templates" ADD CONSTRAINT "communication_templates_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "connections" ADD CONSTRAINT "connections_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "connections" ADD CONSTRAINT "connections_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "connections" ADD CONSTRAINT "connections_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "connections" ADD CONSTRAINT "connections_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "connections" ADD CONSTRAINT "connections_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "connections" ADD CONSTRAINT "connections_device_id_devices_id_fk" FOREIGN KEY ("device_id") REFERENCES "public"."devices"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_aliases" ADD CONSTRAINT "device_aliases_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_aliases" ADD CONSTRAINT "device_aliases_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_aliases" ADD CONSTRAINT "device_aliases_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_aliases" ADD CONSTRAINT "device_aliases_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_aliases" ADD CONSTRAINT "device_aliases_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_aliases" ADD CONSTRAINT "device_aliases_device_configuration_id_device_configurations_id_fk" FOREIGN KEY ("device_configuration_id") REFERENCES "public"."device_configurations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_configurations" ADD CONSTRAINT "device_configurations_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_configurations" ADD CONSTRAINT "device_configurations_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_configurations" ADD CONSTRAINT "device_configurations_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_configurations" ADD CONSTRAINT "device_configurations_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_configurations" ADD CONSTRAINT "device_configurations_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_configurations" ADD CONSTRAINT "device_configurations_device_id_devices_id_fk" FOREIGN KEY ("device_id") REFERENCES "public"."devices"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_group_settings" ADD CONSTRAINT "device_group_settings_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_group_settings" ADD CONSTRAINT "device_group_settings_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_group_settings" ADD CONSTRAINT "device_group_settings_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_group_settings" ADD CONSTRAINT "device_group_settings_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_group_settings" ADD CONSTRAINT "device_group_settings_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_groups" ADD CONSTRAINT "device_groups_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_groups" ADD CONSTRAINT "device_groups_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_groups" ADD CONSTRAINT "device_groups_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_groups" ADD CONSTRAINT "device_groups_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_groups" ADD CONSTRAINT "device_groups_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_groups" ADD CONSTRAINT "device_groups_device_id_devices_id_fk" FOREIGN KEY ("device_id") REFERENCES "public"."devices"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_groups" ADD CONSTRAINT "device_groups_device_group_setting_id_device_group_settings_id_fk" FOREIGN KEY ("device_group_setting_id") REFERENCES "public"."device_group_settings"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_heartbeats" ADD CONSTRAINT "device_heartbeats_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_heartbeats" ADD CONSTRAINT "device_heartbeats_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_heartbeats" ADD CONSTRAINT "device_heartbeats_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_heartbeats" ADD CONSTRAINT "device_heartbeats_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_heartbeats" ADD CONSTRAINT "device_heartbeats_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_heartbeats" ADD CONSTRAINT "device_heartbeats_device_id_devices_id_fk" FOREIGN KEY ("device_id") REFERENCES "public"."devices"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_interface_addresses" ADD CONSTRAINT "device_interface_addresses_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_interface_addresses" ADD CONSTRAINT "device_interface_addresses_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_interface_addresses" ADD CONSTRAINT "device_interface_addresses_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_interface_addresses" ADD CONSTRAINT "device_interface_addresses_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_interface_addresses" ADD CONSTRAINT "device_interface_addresses_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_interface_addresses" ADD CONSTRAINT "device_interface_addresses_device_interface_id_device_interfaces_id_fk" FOREIGN KEY ("device_interface_id") REFERENCES "public"."device_interfaces"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_interfaces" ADD CONSTRAINT "device_interfaces_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_interfaces" ADD CONSTRAINT "device_interfaces_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_interfaces" ADD CONSTRAINT "device_interfaces_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_interfaces" ADD CONSTRAINT "device_interfaces_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_interfaces" ADD CONSTRAINT "device_interfaces_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_interfaces" ADD CONSTRAINT "device_interfaces_device_configuration_id_device_configurations_id_fk" FOREIGN KEY ("device_configuration_id") REFERENCES "public"."device_configurations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_remote_access_sessions" ADD CONSTRAINT "device_remote_access_sessions_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_remote_access_sessions" ADD CONSTRAINT "device_remote_access_sessions_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_remote_access_sessions" ADD CONSTRAINT "device_remote_access_sessions_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_remote_access_sessions" ADD CONSTRAINT "device_remote_access_sessions_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_remote_access_sessions" ADD CONSTRAINT "device_remote_access_sessions_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_remote_access_sessions" ADD CONSTRAINT "device_remote_access_sessions_device_id_devices_id_fk" FOREIGN KEY ("device_id") REFERENCES "public"."devices"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_rules" ADD CONSTRAINT "device_rules_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_rules" ADD CONSTRAINT "device_rules_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_rules" ADD CONSTRAINT "device_rules_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_rules" ADD CONSTRAINT "device_rules_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_rules" ADD CONSTRAINT "device_rules_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_rules" ADD CONSTRAINT "device_rules_device_configuration_id_device_configurations_id_fk" FOREIGN KEY ("device_configuration_id") REFERENCES "public"."device_configurations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_ssh_keys" ADD CONSTRAINT "device_ssh_keys_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_ssh_keys" ADD CONSTRAINT "device_ssh_keys_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_ssh_keys" ADD CONSTRAINT "device_ssh_keys_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_ssh_keys" ADD CONSTRAINT "device_ssh_keys_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_ssh_keys" ADD CONSTRAINT "device_ssh_keys_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "device_ssh_keys" ADD CONSTRAINT "device_ssh_keys_device_id_devices_id_fk" FOREIGN KEY ("device_id") REFERENCES "public"."devices"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "dummy_packets" ADD CONSTRAINT "dummy_packets_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "dummy_packets" ADD CONSTRAINT "dummy_packets_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "dummy_packets" ADD CONSTRAINT "dummy_packets_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "dummy_packets" ADD CONSTRAINT "dummy_packets_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "dummy_packets" ADD CONSTRAINT "dummy_packets_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "dummy_packets" ADD CONSTRAINT "dummy_packets_device_id_devices_id_fk" FOREIGN KEY ("device_id") REFERENCES "public"."devices"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "files" ADD CONSTRAINT "files_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "files" ADD CONSTRAINT "files_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "files" ADD CONSTRAINT "files_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "files" ADD CONSTRAINT "files_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "files" ADD CONSTRAINT "files_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "grid_filters" ADD CONSTRAINT "grid_filters_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "grid_filters" ADD CONSTRAINT "grid_filters_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "grid_filters" ADD CONSTRAINT "grid_filters_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "grid_filters" ADD CONSTRAINT "grid_filters_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "grid_filters" ADD CONSTRAINT "grid_filters_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "grid_filters" ADD CONSTRAINT "grid_filters_contact_id_contacts_id_fk" FOREIGN KEY ("contact_id") REFERENCES "public"."contacts"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "http_requests" ADD CONSTRAINT "http_requests_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "http_requests" ADD CONSTRAINT "http_requests_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "http_requests" ADD CONSTRAINT "http_requests_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "http_requests" ADD CONSTRAINT "http_requests_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "http_requests" ADD CONSTRAINT "http_requests_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "http_responses" ADD CONSTRAINT "http_responses_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "http_responses" ADD CONSTRAINT "http_responses_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "http_responses" ADD CONSTRAINT "http_responses_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "http_responses" ADD CONSTRAINT "http_responses_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "http_responses" ADD CONSTRAINT "http_responses_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "invitations" ADD CONSTRAINT "invitations_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "invitations" ADD CONSTRAINT "invitations_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "invitations" ADD CONSTRAINT "invitations_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "invitations" ADD CONSTRAINT "invitations_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "invitations" ADD CONSTRAINT "invitations_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "invitations" ADD CONSTRAINT "invitations_account_id_organization_accounts_id_fk" FOREIGN KEY ("account_id") REFERENCES "public"."organization_accounts"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "invitations" ADD CONSTRAINT "invitations_account_organization_id_account_organizations_id_fk" FOREIGN KEY ("account_organization_id") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "ip_blacklist" ADD CONSTRAINT "ip_blacklist_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "ip_blacklist" ADD CONSTRAINT "ip_blacklist_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "ip_blacklist" ADD CONSTRAINT "ip_blacklist_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "ip_blacklist" ADD CONSTRAINT "ip_blacklist_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "ip_blacklist" ADD CONSTRAINT "ip_blacklist_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "ip_infos" ADD CONSTRAINT "ip_infos_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "ip_infos" ADD CONSTRAINT "ip_infos_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "ip_infos" ADD CONSTRAINT "ip_infos_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "ip_infos" ADD CONSTRAINT "ip_infos_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "ip_infos" ADD CONSTRAINT "ip_infos_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "locations" ADD CONSTRAINT "locations_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "locations" ADD CONSTRAINT "locations_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "locations" ADD CONSTRAINT "locations_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "locations" ADD CONSTRAINT "locations_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "locations" ADD CONSTRAINT "locations_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "locations" ADD CONSTRAINT "locations_address_id_addresses_id_fk" FOREIGN KEY ("address_id") REFERENCES "public"."addresses"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "notifications" ADD CONSTRAINT "notifications_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "notifications" ADD CONSTRAINT "notifications_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "notifications" ADD CONSTRAINT "notifications_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "notifications" ADD CONSTRAINT "notifications_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "notifications" ADD CONSTRAINT "notifications_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "notifications" ADD CONSTRAINT "notifications_recipient_id_contacts_id_fk" FOREIGN KEY ("recipient_id") REFERENCES "public"."contacts"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "organization_contact_user_roles" ADD CONSTRAINT "organization_contact_user_roles_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "organization_contact_user_roles" ADD CONSTRAINT "organization_contact_user_roles_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "organization_contact_user_roles" ADD CONSTRAINT "organization_contact_user_roles_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "organization_contact_user_roles" ADD CONSTRAINT "organization_contact_user_roles_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "organization_contact_user_roles" ADD CONSTRAINT "organization_contact_user_roles_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "organization_contact_user_roles" ADD CONSTRAINT "organization_contact_user_roles_organization_contact_id_organization_contacts_id_fk" FOREIGN KEY ("organization_contact_id") REFERENCES "public"."organization_contacts"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "organization_contact_user_roles" ADD CONSTRAINT "organization_contact_user_roles_user_role_id_user_roles_id_fk" FOREIGN KEY ("user_role_id") REFERENCES "public"."user_roles"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "packets" ADD CONSTRAINT "packets_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "packets" ADD CONSTRAINT "packets_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "packets" ADD CONSTRAINT "packets_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "packets" ADD CONSTRAINT "packets_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "packets" ADD CONSTRAINT "packets_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "packets" ADD CONSTRAINT "packets_device_id_devices_id_fk" FOREIGN KEY ("device_id") REFERENCES "public"."devices"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "postgres_channels" ADD CONSTRAINT "postgres_channels_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "postgres_channels" ADD CONSTRAINT "postgres_channels_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "postgres_channels" ADD CONSTRAINT "postgres_channels_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "postgres_channels" ADD CONSTRAINT "postgres_channels_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "postgres_channels" ADD CONSTRAINT "postgres_channels_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "resolutions" ADD CONSTRAINT "resolutions_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "resolutions" ADD CONSTRAINT "resolutions_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "resolutions" ADD CONSTRAINT "resolutions_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "resolutions" ADD CONSTRAINT "resolutions_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "resolutions" ADD CONSTRAINT "resolutions_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "smtp_requests" ADD CONSTRAINT "smtp_requests_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "smtp_requests" ADD CONSTRAINT "smtp_requests_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "smtp_requests" ADD CONSTRAINT "smtp_requests_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "smtp_requests" ADD CONSTRAINT "smtp_requests_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "smtp_requests" ADD CONSTRAINT "smtp_requests_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "smtp_responses" ADD CONSTRAINT "smtp_responses_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "smtp_responses" ADD CONSTRAINT "smtp_responses_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "smtp_responses" ADD CONSTRAINT "smtp_responses_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "smtp_responses" ADD CONSTRAINT "smtp_responses_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "smtp_responses" ADD CONSTRAINT "smtp_responses_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "system_resources" ADD CONSTRAINT "system_resources_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "system_resources" ADD CONSTRAINT "system_resources_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "system_resources" ADD CONSTRAINT "system_resources_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "system_resources" ADD CONSTRAINT "system_resources_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "system_resources" ADD CONSTRAINT "system_resources_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "tcp_connections" ADD CONSTRAINT "tcp_connections_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "tcp_connections" ADD CONSTRAINT "tcp_connections_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "tcp_connections" ADD CONSTRAINT "tcp_connections_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "tcp_connections" ADD CONSTRAINT "tcp_connections_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "tcp_connections" ADD CONSTRAINT "tcp_connections_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_appguard_logs" ADD CONSTRAINT "temp_appguard_logs_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_appguard_logs" ADD CONSTRAINT "temp_appguard_logs_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_appguard_logs" ADD CONSTRAINT "temp_appguard_logs_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_appguard_logs" ADD CONSTRAINT "temp_appguard_logs_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_appguard_logs" ADD CONSTRAINT "temp_appguard_logs_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_connections" ADD CONSTRAINT "temp_connections_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_connections" ADD CONSTRAINT "temp_connections_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_connections" ADD CONSTRAINT "temp_connections_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_connections" ADD CONSTRAINT "temp_connections_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_connections" ADD CONSTRAINT "temp_connections_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_connections" ADD CONSTRAINT "temp_connections_device_id_devices_id_fk" FOREIGN KEY ("device_id") REFERENCES "public"."devices"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_device_aliases" ADD CONSTRAINT "temp_device_aliases_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_device_aliases" ADD CONSTRAINT "temp_device_aliases_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_device_aliases" ADD CONSTRAINT "temp_device_aliases_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_device_aliases" ADD CONSTRAINT "temp_device_aliases_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_device_aliases" ADD CONSTRAINT "temp_device_aliases_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_device_aliases" ADD CONSTRAINT "temp_device_aliases_device_configuration_id_device_configurations_id_fk" FOREIGN KEY ("device_configuration_id") REFERENCES "public"."device_configurations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_device_interface_addresses" ADD CONSTRAINT "temp_device_interface_addresses_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_device_interface_addresses" ADD CONSTRAINT "temp_device_interface_addresses_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_device_interface_addresses" ADD CONSTRAINT "temp_device_interface_addresses_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_device_interface_addresses" ADD CONSTRAINT "temp_device_interface_addresses_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_device_interface_addresses" ADD CONSTRAINT "temp_device_interface_addresses_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_device_interface_addresses" ADD CONSTRAINT "temp_device_interface_addresses_device_interface_id_device_interfaces_id_fk" FOREIGN KEY ("device_interface_id") REFERENCES "public"."device_interfaces"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_device_interfaces" ADD CONSTRAINT "temp_device_interfaces_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_device_interfaces" ADD CONSTRAINT "temp_device_interfaces_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_device_interfaces" ADD CONSTRAINT "temp_device_interfaces_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_device_interfaces" ADD CONSTRAINT "temp_device_interfaces_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_device_interfaces" ADD CONSTRAINT "temp_device_interfaces_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_device_interfaces" ADD CONSTRAINT "temp_device_interfaces_device_configuration_id_device_configurations_id_fk" FOREIGN KEY ("device_configuration_id") REFERENCES "public"."device_configurations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_device_remote_access_sessions" ADD CONSTRAINT "temp_device_remote_access_sessions_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_device_remote_access_sessions" ADD CONSTRAINT "temp_device_remote_access_sessions_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_device_remote_access_sessions" ADD CONSTRAINT "temp_device_remote_access_sessions_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_device_remote_access_sessions" ADD CONSTRAINT "temp_device_remote_access_sessions_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_device_remote_access_sessions" ADD CONSTRAINT "temp_device_remote_access_sessions_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_device_remote_access_sessions" ADD CONSTRAINT "temp_device_remote_access_sessions_device_id_devices_id_fk" FOREIGN KEY ("device_id") REFERENCES "public"."devices"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_device_rules" ADD CONSTRAINT "temp_device_rules_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_device_rules" ADD CONSTRAINT "temp_device_rules_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_device_rules" ADD CONSTRAINT "temp_device_rules_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_device_rules" ADD CONSTRAINT "temp_device_rules_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_device_rules" ADD CONSTRAINT "temp_device_rules_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_device_rules" ADD CONSTRAINT "temp_device_rules_device_configuration_id_device_configurations_id_fk" FOREIGN KEY ("device_configuration_id") REFERENCES "public"."device_configurations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_ip_blacklist" ADD CONSTRAINT "temp_ip_blacklist_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_ip_blacklist" ADD CONSTRAINT "temp_ip_blacklist_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_ip_blacklist" ADD CONSTRAINT "temp_ip_blacklist_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_ip_blacklist" ADD CONSTRAINT "temp_ip_blacklist_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_ip_blacklist" ADD CONSTRAINT "temp_ip_blacklist_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_packets" ADD CONSTRAINT "temp_packets_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_packets" ADD CONSTRAINT "temp_packets_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_packets" ADD CONSTRAINT "temp_packets_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_packets" ADD CONSTRAINT "temp_packets_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_packets" ADD CONSTRAINT "temp_packets_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_packets" ADD CONSTRAINT "temp_packets_device_id_devices_id_fk" FOREIGN KEY ("device_id") REFERENCES "public"."devices"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_system_resources" ADD CONSTRAINT "temp_system_resources_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_system_resources" ADD CONSTRAINT "temp_system_resources_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_system_resources" ADD CONSTRAINT "temp_system_resources_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_system_resources" ADD CONSTRAINT "temp_system_resources_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_system_resources" ADD CONSTRAINT "temp_system_resources_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_wallguard_logs" ADD CONSTRAINT "temp_wallguard_logs_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_wallguard_logs" ADD CONSTRAINT "temp_wallguard_logs_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_wallguard_logs" ADD CONSTRAINT "temp_wallguard_logs_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_wallguard_logs" ADD CONSTRAINT "temp_wallguard_logs_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "temp_wallguard_logs" ADD CONSTRAINT "temp_wallguard_logs_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "wallguard_logs" ADD CONSTRAINT "wallguard_logs_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "wallguard_logs" ADD CONSTRAINT "wallguard_logs_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "wallguard_logs" ADD CONSTRAINT "wallguard_logs_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "wallguard_logs" ADD CONSTRAINT "wallguard_logs_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "wallguard_logs" ADD CONSTRAINT "wallguard_logs_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
CREATE INDEX "allowed_fields_id_idx" ON "allowed_fields" USING btree ("id");--> statement-breakpoint
CREATE INDEX "allowed_fields_categories_idx" ON "allowed_fields" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "allowed_fields_code_idx" ON "allowed_fields" USING btree ("code");--> statement-breakpoint
CREATE INDEX "allowed_fields_tombstone_idx" ON "allowed_fields" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "allowed_fields_status_idx" ON "allowed_fields" USING btree ("status");--> statement-breakpoint
CREATE INDEX "allowed_fields_previous_status_idx" ON "allowed_fields" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "allowed_fields_version_idx" ON "allowed_fields" USING btree ("version");--> statement-breakpoint
CREATE INDEX "allowed_fields_created_date_idx" ON "allowed_fields" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "allowed_fields_updated_date_idx" ON "allowed_fields" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "allowed_fields_organization_id_idx" ON "allowed_fields" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "allowed_fields_created_by_idx" ON "allowed_fields" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "allowed_fields_updated_by_idx" ON "allowed_fields" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "allowed_fields_deleted_by_idx" ON "allowed_fields" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "allowed_fields_requested_by_idx" ON "allowed_fields" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "allowed_fields_tags_idx" ON "allowed_fields" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "allowed_fields_image_url_idx" ON "allowed_fields" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "class_types_id_idx" ON "class_types" USING btree ("id");--> statement-breakpoint
CREATE INDEX "class_types_categories_idx" ON "class_types" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "class_types_code_idx" ON "class_types" USING btree ("code");--> statement-breakpoint
CREATE INDEX "class_types_tombstone_idx" ON "class_types" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "class_types_status_idx" ON "class_types" USING btree ("status");--> statement-breakpoint
CREATE INDEX "class_types_previous_status_idx" ON "class_types" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "class_types_version_idx" ON "class_types" USING btree ("version");--> statement-breakpoint
CREATE INDEX "class_types_created_date_idx" ON "class_types" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "class_types_updated_date_idx" ON "class_types" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "class_types_organization_id_idx" ON "class_types" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "class_types_created_by_idx" ON "class_types" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "class_types_updated_by_idx" ON "class_types" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "class_types_deleted_by_idx" ON "class_types" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "class_types_requested_by_idx" ON "class_types" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "class_types_tags_idx" ON "class_types" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "class_types_image_url_idx" ON "class_types" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "organization_files_id_idx" ON "organization_files" USING btree ("id");--> statement-breakpoint
CREATE INDEX "organization_files_categories_idx" ON "organization_files" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "organization_files_code_idx" ON "organization_files" USING btree ("code");--> statement-breakpoint
CREATE INDEX "organization_files_tombstone_idx" ON "organization_files" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "organization_files_status_idx" ON "organization_files" USING btree ("status");--> statement-breakpoint
CREATE INDEX "organization_files_previous_status_idx" ON "organization_files" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "organization_files_version_idx" ON "organization_files" USING btree ("version");--> statement-breakpoint
CREATE INDEX "organization_files_created_date_idx" ON "organization_files" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "organization_files_updated_date_idx" ON "organization_files" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "organization_files_organization_id_idx" ON "organization_files" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "organization_files_created_by_idx" ON "organization_files" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "organization_files_updated_by_idx" ON "organization_files" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "organization_files_deleted_by_idx" ON "organization_files" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "organization_files_requested_by_idx" ON "organization_files" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "organization_files_tags_idx" ON "organization_files" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "organization_files_image_url_idx" ON "organization_files" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "app_firewalls_id_idx" ON "app_firewalls" USING btree ("id");--> statement-breakpoint
CREATE INDEX "app_firewalls_categories_idx" ON "app_firewalls" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "app_firewalls_code_idx" ON "app_firewalls" USING btree ("code");--> statement-breakpoint
CREATE INDEX "app_firewalls_tombstone_idx" ON "app_firewalls" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "app_firewalls_status_idx" ON "app_firewalls" USING btree ("status");--> statement-breakpoint
CREATE INDEX "app_firewalls_previous_status_idx" ON "app_firewalls" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "app_firewalls_version_idx" ON "app_firewalls" USING btree ("version");--> statement-breakpoint
CREATE INDEX "app_firewalls_created_date_idx" ON "app_firewalls" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "app_firewalls_updated_date_idx" ON "app_firewalls" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "app_firewalls_organization_id_idx" ON "app_firewalls" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "app_firewalls_created_by_idx" ON "app_firewalls" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "app_firewalls_updated_by_idx" ON "app_firewalls" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "app_firewalls_deleted_by_idx" ON "app_firewalls" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "app_firewalls_requested_by_idx" ON "app_firewalls" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "app_firewalls_tags_idx" ON "app_firewalls" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "app_firewalls_image_url_idx" ON "app_firewalls" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "appguard_logs_id_idx" ON "appguard_logs" USING btree ("id");--> statement-breakpoint
CREATE INDEX "appguard_logs_categories_idx" ON "appguard_logs" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "appguard_logs_code_idx" ON "appguard_logs" USING btree ("code");--> statement-breakpoint
CREATE INDEX "appguard_logs_tombstone_idx" ON "appguard_logs" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "appguard_logs_status_idx" ON "appguard_logs" USING btree ("status");--> statement-breakpoint
CREATE INDEX "appguard_logs_previous_status_idx" ON "appguard_logs" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "appguard_logs_version_idx" ON "appguard_logs" USING btree ("version");--> statement-breakpoint
CREATE INDEX "appguard_logs_created_date_idx" ON "appguard_logs" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "appguard_logs_updated_date_idx" ON "appguard_logs" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "appguard_logs_organization_id_idx" ON "appguard_logs" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "appguard_logs_created_by_idx" ON "appguard_logs" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "appguard_logs_updated_by_idx" ON "appguard_logs" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "appguard_logs_deleted_by_idx" ON "appguard_logs" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "appguard_logs_requested_by_idx" ON "appguard_logs" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "appguard_logs_tags_idx" ON "appguard_logs" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "appguard_logs_image_url_idx" ON "appguard_logs" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "communication_templates_id_idx" ON "communication_templates" USING btree ("id");--> statement-breakpoint
CREATE INDEX "communication_templates_categories_idx" ON "communication_templates" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "communication_templates_code_idx" ON "communication_templates" USING btree ("code");--> statement-breakpoint
CREATE INDEX "communication_templates_tombstone_idx" ON "communication_templates" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "communication_templates_status_idx" ON "communication_templates" USING btree ("status");--> statement-breakpoint
CREATE INDEX "communication_templates_previous_status_idx" ON "communication_templates" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "communication_templates_version_idx" ON "communication_templates" USING btree ("version");--> statement-breakpoint
CREATE INDEX "communication_templates_created_date_idx" ON "communication_templates" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "communication_templates_updated_date_idx" ON "communication_templates" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "communication_templates_organization_id_idx" ON "communication_templates" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "communication_templates_created_by_idx" ON "communication_templates" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "communication_templates_updated_by_idx" ON "communication_templates" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "communication_templates_deleted_by_idx" ON "communication_templates" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "communication_templates_requested_by_idx" ON "communication_templates" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "communication_templates_tags_idx" ON "communication_templates" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "communication_templates_image_url_idx" ON "communication_templates" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "communication_templates_name_idx" ON "communication_templates" USING btree ("name");--> statement-breakpoint
CREATE INDEX "communication_templates_communication_template_status_idx" ON "communication_templates" USING btree ("communication_template_status");--> statement-breakpoint
CREATE INDEX "communication_templates_event_idx" ON "communication_templates" USING btree ("event");--> statement-breakpoint
CREATE INDEX "communication_templates_content_idx" ON "communication_templates" USING btree ("content");--> statement-breakpoint
CREATE INDEX "communication_templates_subject_idx" ON "communication_templates" USING btree ("subject");--> statement-breakpoint
CREATE INDEX "connections_id_idx" ON "connections" USING btree ("id");--> statement-breakpoint
CREATE INDEX "connections_categories_idx" ON "connections" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "connections_code_idx" ON "connections" USING btree ("code");--> statement-breakpoint
CREATE INDEX "connections_tombstone_idx" ON "connections" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "connections_status_idx" ON "connections" USING btree ("status");--> statement-breakpoint
CREATE INDEX "connections_previous_status_idx" ON "connections" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "connections_version_idx" ON "connections" USING btree ("version");--> statement-breakpoint
CREATE INDEX "connections_created_date_idx" ON "connections" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "connections_updated_date_idx" ON "connections" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "connections_organization_id_idx" ON "connections" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "connections_created_by_idx" ON "connections" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "connections_updated_by_idx" ON "connections" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "connections_deleted_by_idx" ON "connections" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "connections_requested_by_idx" ON "connections" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "connections_tags_idx" ON "connections" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "connections_image_url_idx" ON "connections" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "connections_timestamp_idx" ON "connections" USING btree ("timestamp");--> statement-breakpoint
CREATE INDEX "connections_interface_name_idx" ON "connections" USING btree ("interface_name");--> statement-breakpoint
CREATE INDEX "connections_total_packet_idx" ON "connections" USING btree ("total_packet");--> statement-breakpoint
CREATE INDEX "connections_total_byte_idx" ON "connections" USING btree ("total_byte");--> statement-breakpoint
CREATE INDEX "connections_device_id_idx" ON "connections" USING btree ("device_id");--> statement-breakpoint
CREATE INDEX "connections_protocol_idx" ON "connections" USING btree ("protocol");--> statement-breakpoint
CREATE INDEX "connections_source_ip_idx" ON "connections" USING btree ("source_ip");--> statement-breakpoint
CREATE INDEX "connections_destination_ip_idx" ON "connections" USING btree ("destination_ip");--> statement-breakpoint
CREATE INDEX "connections_remote_ip_idx" ON "connections" USING btree ("remote_ip");--> statement-breakpoint
CREATE INDEX "connections_source_port_idx" ON "connections" USING btree ("source_port");--> statement-breakpoint
CREATE INDEX "connections_destination_port_idx" ON "connections" USING btree ("destination_port");--> statement-breakpoint
CREATE INDEX "device_aliases_id_idx" ON "device_aliases" USING btree ("id");--> statement-breakpoint
CREATE INDEX "device_aliases_categories_idx" ON "device_aliases" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "device_aliases_code_idx" ON "device_aliases" USING btree ("code");--> statement-breakpoint
CREATE INDEX "device_aliases_tombstone_idx" ON "device_aliases" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "device_aliases_status_idx" ON "device_aliases" USING btree ("status");--> statement-breakpoint
CREATE INDEX "device_aliases_previous_status_idx" ON "device_aliases" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "device_aliases_version_idx" ON "device_aliases" USING btree ("version");--> statement-breakpoint
CREATE INDEX "device_aliases_created_date_idx" ON "device_aliases" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "device_aliases_updated_date_idx" ON "device_aliases" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "device_aliases_organization_id_idx" ON "device_aliases" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "device_aliases_created_by_idx" ON "device_aliases" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "device_aliases_updated_by_idx" ON "device_aliases" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "device_aliases_deleted_by_idx" ON "device_aliases" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "device_aliases_requested_by_idx" ON "device_aliases" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "device_aliases_tags_idx" ON "device_aliases" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "device_aliases_image_url_idx" ON "device_aliases" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "device_aliases_device_configuration_id_idx" ON "device_aliases" USING btree ("device_configuration_id");--> statement-breakpoint
CREATE INDEX "device_aliases_type_idx" ON "device_aliases" USING btree ("type");--> statement-breakpoint
CREATE INDEX "device_aliases_name_idx" ON "device_aliases" USING btree ("name");--> statement-breakpoint
CREATE INDEX "device_aliases_value_idx" ON "device_aliases" USING btree ("value");--> statement-breakpoint
CREATE INDEX "device_aliases_description_idx" ON "device_aliases" USING btree ("description");--> statement-breakpoint
CREATE INDEX "device_aliases_device_alias_status_idx" ON "device_aliases" USING btree ("device_alias_status");--> statement-breakpoint
CREATE INDEX "device_configurations_id_idx" ON "device_configurations" USING btree ("id");--> statement-breakpoint
CREATE INDEX "device_configurations_categories_idx" ON "device_configurations" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "device_configurations_code_idx" ON "device_configurations" USING btree ("code");--> statement-breakpoint
CREATE INDEX "device_configurations_tombstone_idx" ON "device_configurations" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "device_configurations_status_idx" ON "device_configurations" USING btree ("status");--> statement-breakpoint
CREATE INDEX "device_configurations_previous_status_idx" ON "device_configurations" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "device_configurations_version_idx" ON "device_configurations" USING btree ("version");--> statement-breakpoint
CREATE INDEX "device_configurations_created_date_idx" ON "device_configurations" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "device_configurations_updated_date_idx" ON "device_configurations" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "device_configurations_organization_id_idx" ON "device_configurations" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "device_configurations_created_by_idx" ON "device_configurations" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "device_configurations_updated_by_idx" ON "device_configurations" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "device_configurations_deleted_by_idx" ON "device_configurations" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "device_configurations_requested_by_idx" ON "device_configurations" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "device_configurations_tags_idx" ON "device_configurations" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "device_configurations_image_url_idx" ON "device_configurations" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "device_group_settings_id_idx" ON "device_group_settings" USING btree ("id");--> statement-breakpoint
CREATE INDEX "device_group_settings_categories_idx" ON "device_group_settings" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "device_group_settings_code_idx" ON "device_group_settings" USING btree ("code");--> statement-breakpoint
CREATE INDEX "device_group_settings_tombstone_idx" ON "device_group_settings" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "device_group_settings_status_idx" ON "device_group_settings" USING btree ("status");--> statement-breakpoint
CREATE INDEX "device_group_settings_previous_status_idx" ON "device_group_settings" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "device_group_settings_version_idx" ON "device_group_settings" USING btree ("version");--> statement-breakpoint
CREATE INDEX "device_group_settings_created_date_idx" ON "device_group_settings" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "device_group_settings_updated_date_idx" ON "device_group_settings" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "device_group_settings_organization_id_idx" ON "device_group_settings" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "device_group_settings_created_by_idx" ON "device_group_settings" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "device_group_settings_updated_by_idx" ON "device_group_settings" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "device_group_settings_deleted_by_idx" ON "device_group_settings" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "device_group_settings_requested_by_idx" ON "device_group_settings" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "device_group_settings_tags_idx" ON "device_group_settings" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "device_group_settings_image_url_idx" ON "device_group_settings" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "device_groups_id_idx" ON "device_groups" USING btree ("id");--> statement-breakpoint
CREATE INDEX "device_groups_categories_idx" ON "device_groups" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "device_groups_code_idx" ON "device_groups" USING btree ("code");--> statement-breakpoint
CREATE INDEX "device_groups_tombstone_idx" ON "device_groups" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "device_groups_status_idx" ON "device_groups" USING btree ("status");--> statement-breakpoint
CREATE INDEX "device_groups_previous_status_idx" ON "device_groups" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "device_groups_version_idx" ON "device_groups" USING btree ("version");--> statement-breakpoint
CREATE INDEX "device_groups_created_date_idx" ON "device_groups" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "device_groups_updated_date_idx" ON "device_groups" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "device_groups_organization_id_idx" ON "device_groups" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "device_groups_created_by_idx" ON "device_groups" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "device_groups_updated_by_idx" ON "device_groups" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "device_groups_deleted_by_idx" ON "device_groups" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "device_groups_requested_by_idx" ON "device_groups" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "device_groups_tags_idx" ON "device_groups" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "device_groups_image_url_idx" ON "device_groups" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "device_heartbeats_id_idx" ON "device_heartbeats" USING btree ("id");--> statement-breakpoint
CREATE INDEX "device_heartbeats_categories_idx" ON "device_heartbeats" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "device_heartbeats_code_idx" ON "device_heartbeats" USING btree ("code");--> statement-breakpoint
CREATE INDEX "device_heartbeats_tombstone_idx" ON "device_heartbeats" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "device_heartbeats_status_idx" ON "device_heartbeats" USING btree ("status");--> statement-breakpoint
CREATE INDEX "device_heartbeats_previous_status_idx" ON "device_heartbeats" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "device_heartbeats_version_idx" ON "device_heartbeats" USING btree ("version");--> statement-breakpoint
CREATE INDEX "device_heartbeats_created_date_idx" ON "device_heartbeats" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "device_heartbeats_updated_date_idx" ON "device_heartbeats" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "device_heartbeats_organization_id_idx" ON "device_heartbeats" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "device_heartbeats_created_by_idx" ON "device_heartbeats" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "device_heartbeats_updated_by_idx" ON "device_heartbeats" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "device_heartbeats_deleted_by_idx" ON "device_heartbeats" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "device_heartbeats_requested_by_idx" ON "device_heartbeats" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "device_heartbeats_tags_idx" ON "device_heartbeats" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "device_heartbeats_image_url_idx" ON "device_heartbeats" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "device_interface_addresses_id_idx" ON "device_interface_addresses" USING btree ("id");--> statement-breakpoint
CREATE INDEX "device_interface_addresses_categories_idx" ON "device_interface_addresses" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "device_interface_addresses_code_idx" ON "device_interface_addresses" USING btree ("code");--> statement-breakpoint
CREATE INDEX "device_interface_addresses_tombstone_idx" ON "device_interface_addresses" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "device_interface_addresses_status_idx" ON "device_interface_addresses" USING btree ("status");--> statement-breakpoint
CREATE INDEX "device_interface_addresses_previous_status_idx" ON "device_interface_addresses" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "device_interface_addresses_version_idx" ON "device_interface_addresses" USING btree ("version");--> statement-breakpoint
CREATE INDEX "device_interface_addresses_created_date_idx" ON "device_interface_addresses" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "device_interface_addresses_updated_date_idx" ON "device_interface_addresses" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "device_interface_addresses_organization_id_idx" ON "device_interface_addresses" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "device_interface_addresses_created_by_idx" ON "device_interface_addresses" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "device_interface_addresses_updated_by_idx" ON "device_interface_addresses" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "device_interface_addresses_deleted_by_idx" ON "device_interface_addresses" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "device_interface_addresses_requested_by_idx" ON "device_interface_addresses" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "device_interface_addresses_tags_idx" ON "device_interface_addresses" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "device_interface_addresses_image_url_idx" ON "device_interface_addresses" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "device_interface_addresses_device_interface_id_idx" ON "device_interface_addresses" USING btree ("device_interface_id");--> statement-breakpoint
CREATE INDEX "device_interface_addresses_address_idx" ON "device_interface_addresses" USING btree ("address");--> statement-breakpoint
CREATE INDEX "device_interfaces_id_idx" ON "device_interfaces" USING btree ("id");--> statement-breakpoint
CREATE INDEX "device_interfaces_categories_idx" ON "device_interfaces" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "device_interfaces_code_idx" ON "device_interfaces" USING btree ("code");--> statement-breakpoint
CREATE INDEX "device_interfaces_tombstone_idx" ON "device_interfaces" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "device_interfaces_status_idx" ON "device_interfaces" USING btree ("status");--> statement-breakpoint
CREATE INDEX "device_interfaces_previous_status_idx" ON "device_interfaces" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "device_interfaces_version_idx" ON "device_interfaces" USING btree ("version");--> statement-breakpoint
CREATE INDEX "device_interfaces_created_date_idx" ON "device_interfaces" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "device_interfaces_updated_date_idx" ON "device_interfaces" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "device_interfaces_organization_id_idx" ON "device_interfaces" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "device_interfaces_created_by_idx" ON "device_interfaces" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "device_interfaces_updated_by_idx" ON "device_interfaces" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "device_interfaces_deleted_by_idx" ON "device_interfaces" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "device_interfaces_requested_by_idx" ON "device_interfaces" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "device_interfaces_tags_idx" ON "device_interfaces" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "device_interfaces_image_url_idx" ON "device_interfaces" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "device_interfaces_device_configuration_id_idx" ON "device_interfaces" USING btree ("device_configuration_id");--> statement-breakpoint
CREATE INDEX "device_interfaces_name_idx" ON "device_interfaces" USING btree ("name");--> statement-breakpoint
CREATE INDEX "device_interfaces_device_idx" ON "device_interfaces" USING btree ("device");--> statement-breakpoint
CREATE INDEX "device_remote_access_sessions_id_idx" ON "device_remote_access_sessions" USING btree ("id");--> statement-breakpoint
CREATE INDEX "device_remote_access_sessions_categories_idx" ON "device_remote_access_sessions" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "device_remote_access_sessions_code_idx" ON "device_remote_access_sessions" USING btree ("code");--> statement-breakpoint
CREATE INDEX "device_remote_access_sessions_tombstone_idx" ON "device_remote_access_sessions" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "device_remote_access_sessions_status_idx" ON "device_remote_access_sessions" USING btree ("status");--> statement-breakpoint
CREATE INDEX "device_remote_access_sessions_previous_status_idx" ON "device_remote_access_sessions" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "device_remote_access_sessions_version_idx" ON "device_remote_access_sessions" USING btree ("version");--> statement-breakpoint
CREATE INDEX "device_remote_access_sessions_created_date_idx" ON "device_remote_access_sessions" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "device_remote_access_sessions_updated_date_idx" ON "device_remote_access_sessions" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "device_remote_access_sessions_organization_id_idx" ON "device_remote_access_sessions" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "device_remote_access_sessions_created_by_idx" ON "device_remote_access_sessions" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "device_remote_access_sessions_updated_by_idx" ON "device_remote_access_sessions" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "device_remote_access_sessions_deleted_by_idx" ON "device_remote_access_sessions" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "device_remote_access_sessions_requested_by_idx" ON "device_remote_access_sessions" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "device_remote_access_sessions_tags_idx" ON "device_remote_access_sessions" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "device_remote_access_sessions_image_url_idx" ON "device_remote_access_sessions" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "device_rules_id_idx" ON "device_rules" USING btree ("id");--> statement-breakpoint
CREATE INDEX "device_rules_categories_idx" ON "device_rules" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "device_rules_code_idx" ON "device_rules" USING btree ("code");--> statement-breakpoint
CREATE INDEX "device_rules_tombstone_idx" ON "device_rules" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "device_rules_status_idx" ON "device_rules" USING btree ("status");--> statement-breakpoint
CREATE INDEX "device_rules_previous_status_idx" ON "device_rules" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "device_rules_version_idx" ON "device_rules" USING btree ("version");--> statement-breakpoint
CREATE INDEX "device_rules_created_date_idx" ON "device_rules" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "device_rules_updated_date_idx" ON "device_rules" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "device_rules_organization_id_idx" ON "device_rules" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "device_rules_created_by_idx" ON "device_rules" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "device_rules_updated_by_idx" ON "device_rules" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "device_rules_deleted_by_idx" ON "device_rules" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "device_rules_requested_by_idx" ON "device_rules" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "device_rules_tags_idx" ON "device_rules" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "device_rules_image_url_idx" ON "device_rules" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "device_rules_device_configuration_id_idx" ON "device_rules" USING btree ("device_configuration_id");--> statement-breakpoint
CREATE INDEX "device_rules_disabled_idx" ON "device_rules" USING btree ("disabled");--> statement-breakpoint
CREATE INDEX "device_rules_type_idx" ON "device_rules" USING btree ("type");--> statement-breakpoint
CREATE INDEX "device_rules_policy_idx" ON "device_rules" USING btree ("policy");--> statement-breakpoint
CREATE INDEX "device_rules_protocol_idx" ON "device_rules" USING btree ("protocol");--> statement-breakpoint
CREATE INDEX "device_rules_source_port_idx" ON "device_rules" USING btree ("source_port");--> statement-breakpoint
CREATE INDEX "device_rules_source_addr_idx" ON "device_rules" USING btree ("source_addr");--> statement-breakpoint
CREATE INDEX "device_rules_source_type_idx" ON "device_rules" USING btree ("source_type");--> statement-breakpoint
CREATE INDEX "device_rules_destination_port_idx" ON "device_rules" USING btree ("destination_port");--> statement-breakpoint
CREATE INDEX "device_rules_destination_addr_idx" ON "device_rules" USING btree ("destination_addr");--> statement-breakpoint
CREATE INDEX "device_rules_description_idx" ON "device_rules" USING btree ("description");--> statement-breakpoint
CREATE INDEX "device_rules_device_rule_status_idx" ON "device_rules" USING btree ("device_rule_status");--> statement-breakpoint
CREATE INDEX "device_rules_interface_idx" ON "device_rules" USING btree ("interface");--> statement-breakpoint
CREATE INDEX "device_rules_order_idx" ON "device_rules" USING btree ("order");--> statement-breakpoint
CREATE INDEX "device_ssh_keys_id_idx" ON "device_ssh_keys" USING btree ("id");--> statement-breakpoint
CREATE INDEX "device_ssh_keys_categories_idx" ON "device_ssh_keys" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "device_ssh_keys_code_idx" ON "device_ssh_keys" USING btree ("code");--> statement-breakpoint
CREATE INDEX "device_ssh_keys_tombstone_idx" ON "device_ssh_keys" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "device_ssh_keys_status_idx" ON "device_ssh_keys" USING btree ("status");--> statement-breakpoint
CREATE INDEX "device_ssh_keys_previous_status_idx" ON "device_ssh_keys" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "device_ssh_keys_version_idx" ON "device_ssh_keys" USING btree ("version");--> statement-breakpoint
CREATE INDEX "device_ssh_keys_created_date_idx" ON "device_ssh_keys" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "device_ssh_keys_updated_date_idx" ON "device_ssh_keys" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "device_ssh_keys_organization_id_idx" ON "device_ssh_keys" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "device_ssh_keys_created_by_idx" ON "device_ssh_keys" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "device_ssh_keys_updated_by_idx" ON "device_ssh_keys" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "device_ssh_keys_deleted_by_idx" ON "device_ssh_keys" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "device_ssh_keys_requested_by_idx" ON "device_ssh_keys" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "device_ssh_keys_tags_idx" ON "device_ssh_keys" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "device_ssh_keys_image_url_idx" ON "device_ssh_keys" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "device_ssh_keys_public_key_idx" ON "device_ssh_keys" USING btree ("public_key");--> statement-breakpoint
CREATE INDEX "device_ssh_keys_private_key_idx" ON "device_ssh_keys" USING btree ("private_key");--> statement-breakpoint
CREATE INDEX "device_ssh_keys_passphrase_idx" ON "device_ssh_keys" USING btree ("passphrase");--> statement-breakpoint
CREATE INDEX "device_ssh_keys_device_id_idx" ON "device_ssh_keys" USING btree ("device_id");--> statement-breakpoint
CREATE INDEX "dummy_packets_id_idx" ON "dummy_packets" USING btree ("id");--> statement-breakpoint
CREATE INDEX "dummy_packets_categories_idx" ON "dummy_packets" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "dummy_packets_code_idx" ON "dummy_packets" USING btree ("code");--> statement-breakpoint
CREATE INDEX "dummy_packets_tombstone_idx" ON "dummy_packets" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "dummy_packets_status_idx" ON "dummy_packets" USING btree ("status");--> statement-breakpoint
CREATE INDEX "dummy_packets_previous_status_idx" ON "dummy_packets" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "dummy_packets_version_idx" ON "dummy_packets" USING btree ("version");--> statement-breakpoint
CREATE INDEX "dummy_packets_created_date_idx" ON "dummy_packets" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "dummy_packets_updated_date_idx" ON "dummy_packets" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "dummy_packets_organization_id_idx" ON "dummy_packets" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "dummy_packets_created_by_idx" ON "dummy_packets" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "dummy_packets_updated_by_idx" ON "dummy_packets" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "dummy_packets_deleted_by_idx" ON "dummy_packets" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "dummy_packets_requested_by_idx" ON "dummy_packets" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "dummy_packets_tags_idx" ON "dummy_packets" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "dummy_packets_image_url_idx" ON "dummy_packets" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "dummy_packets_timestamp_idx" ON "dummy_packets" USING btree ("timestamp");--> statement-breakpoint
CREATE INDEX "dummy_packets_interface_name_idx" ON "dummy_packets" USING btree ("interface_name");--> statement-breakpoint
CREATE INDEX "dummy_packets_total_length_idx" ON "dummy_packets" USING btree ("total_length");--> statement-breakpoint
CREATE INDEX "dummy_packets_device_id_idx" ON "dummy_packets" USING btree ("device_id");--> statement-breakpoint
CREATE INDEX "dummy_packets_ether_type_idx" ON "dummy_packets" USING btree ("ether_type");--> statement-breakpoint
CREATE INDEX "dummy_packets_protocol_idx" ON "dummy_packets" USING btree ("protocol");--> statement-breakpoint
CREATE INDEX "dummy_packets_source_ip_idx" ON "dummy_packets" USING btree ("source_ip");--> statement-breakpoint
CREATE INDEX "dummy_packets_destination_ip_idx" ON "dummy_packets" USING btree ("destination_ip");--> statement-breakpoint
CREATE INDEX "dummy_packets_remote_ip_idx" ON "dummy_packets" USING btree ("remote_ip");--> statement-breakpoint
CREATE INDEX "dummy_packets_source_port_idx" ON "dummy_packets" USING btree ("source_port");--> statement-breakpoint
CREATE INDEX "dummy_packets_destination_port_idx" ON "dummy_packets" USING btree ("destination_port");--> statement-breakpoint
CREATE INDEX "files_id_idx" ON "files" USING btree ("id");--> statement-breakpoint
CREATE INDEX "files_categories_idx" ON "files" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "files_code_idx" ON "files" USING btree ("code");--> statement-breakpoint
CREATE INDEX "files_tombstone_idx" ON "files" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "files_status_idx" ON "files" USING btree ("status");--> statement-breakpoint
CREATE INDEX "files_previous_status_idx" ON "files" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "files_version_idx" ON "files" USING btree ("version");--> statement-breakpoint
CREATE INDEX "files_created_date_idx" ON "files" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "files_updated_date_idx" ON "files" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "files_organization_id_idx" ON "files" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "files_created_by_idx" ON "files" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "files_updated_by_idx" ON "files" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "files_deleted_by_idx" ON "files" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "files_requested_by_idx" ON "files" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "files_tags_idx" ON "files" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "files_image_url_idx" ON "files" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "grid_filters_id_idx" ON "grid_filters" USING btree ("id");--> statement-breakpoint
CREATE INDEX "grid_filters_categories_idx" ON "grid_filters" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "grid_filters_code_idx" ON "grid_filters" USING btree ("code");--> statement-breakpoint
CREATE INDEX "grid_filters_tombstone_idx" ON "grid_filters" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "grid_filters_status_idx" ON "grid_filters" USING btree ("status");--> statement-breakpoint
CREATE INDEX "grid_filters_previous_status_idx" ON "grid_filters" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "grid_filters_version_idx" ON "grid_filters" USING btree ("version");--> statement-breakpoint
CREATE INDEX "grid_filters_created_date_idx" ON "grid_filters" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "grid_filters_updated_date_idx" ON "grid_filters" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "grid_filters_organization_id_idx" ON "grid_filters" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "grid_filters_created_by_idx" ON "grid_filters" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "grid_filters_updated_by_idx" ON "grid_filters" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "grid_filters_deleted_by_idx" ON "grid_filters" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "grid_filters_requested_by_idx" ON "grid_filters" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "grid_filters_tags_idx" ON "grid_filters" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "grid_filters_image_url_idx" ON "grid_filters" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "grid_filters_name_idx" ON "grid_filters" USING btree ("name");--> statement-breakpoint
CREATE INDEX "grid_filters_grid_id_idx" ON "grid_filters" USING btree ("grid_id");--> statement-breakpoint
CREATE INDEX "grid_filters_link_idx" ON "grid_filters" USING btree ("link");--> statement-breakpoint
CREATE INDEX "grid_filters_is_current_idx" ON "grid_filters" USING btree ("is_current");--> statement-breakpoint
CREATE INDEX "grid_filters_is_default_idx" ON "grid_filters" USING btree ("is_default");--> statement-breakpoint
CREATE INDEX "grid_filters_contact_id_idx" ON "grid_filters" USING btree ("contact_id");--> statement-breakpoint
CREATE INDEX "grid_filters_entity_idx" ON "grid_filters" USING btree ("entity");--> statement-breakpoint
CREATE INDEX "http_requests_id_idx" ON "http_requests" USING btree ("id");--> statement-breakpoint
CREATE INDEX "http_requests_categories_idx" ON "http_requests" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "http_requests_code_idx" ON "http_requests" USING btree ("code");--> statement-breakpoint
CREATE INDEX "http_requests_tombstone_idx" ON "http_requests" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "http_requests_status_idx" ON "http_requests" USING btree ("status");--> statement-breakpoint
CREATE INDEX "http_requests_previous_status_idx" ON "http_requests" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "http_requests_version_idx" ON "http_requests" USING btree ("version");--> statement-breakpoint
CREATE INDEX "http_requests_created_date_idx" ON "http_requests" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "http_requests_updated_date_idx" ON "http_requests" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "http_requests_organization_id_idx" ON "http_requests" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "http_requests_created_by_idx" ON "http_requests" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "http_requests_updated_by_idx" ON "http_requests" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "http_requests_deleted_by_idx" ON "http_requests" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "http_requests_requested_by_idx" ON "http_requests" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "http_requests_tags_idx" ON "http_requests" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "http_requests_image_url_idx" ON "http_requests" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "http_responses_id_idx" ON "http_responses" USING btree ("id");--> statement-breakpoint
CREATE INDEX "http_responses_categories_idx" ON "http_responses" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "http_responses_code_idx" ON "http_responses" USING btree ("code");--> statement-breakpoint
CREATE INDEX "http_responses_tombstone_idx" ON "http_responses" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "http_responses_status_idx" ON "http_responses" USING btree ("status");--> statement-breakpoint
CREATE INDEX "http_responses_previous_status_idx" ON "http_responses" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "http_responses_version_idx" ON "http_responses" USING btree ("version");--> statement-breakpoint
CREATE INDEX "http_responses_created_date_idx" ON "http_responses" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "http_responses_updated_date_idx" ON "http_responses" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "http_responses_organization_id_idx" ON "http_responses" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "http_responses_created_by_idx" ON "http_responses" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "http_responses_updated_by_idx" ON "http_responses" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "http_responses_deleted_by_idx" ON "http_responses" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "http_responses_requested_by_idx" ON "http_responses" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "http_responses_tags_idx" ON "http_responses" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "http_responses_image_url_idx" ON "http_responses" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "invitations_id_idx" ON "invitations" USING btree ("id");--> statement-breakpoint
CREATE INDEX "invitations_categories_idx" ON "invitations" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "invitations_code_idx" ON "invitations" USING btree ("code");--> statement-breakpoint
CREATE INDEX "invitations_tombstone_idx" ON "invitations" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "invitations_status_idx" ON "invitations" USING btree ("status");--> statement-breakpoint
CREATE INDEX "invitations_previous_status_idx" ON "invitations" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "invitations_version_idx" ON "invitations" USING btree ("version");--> statement-breakpoint
CREATE INDEX "invitations_created_date_idx" ON "invitations" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "invitations_updated_date_idx" ON "invitations" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "invitations_organization_id_idx" ON "invitations" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "invitations_created_by_idx" ON "invitations" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "invitations_updated_by_idx" ON "invitations" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "invitations_deleted_by_idx" ON "invitations" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "invitations_requested_by_idx" ON "invitations" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "invitations_tags_idx" ON "invitations" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "invitations_image_url_idx" ON "invitations" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "invitations_account_id_idx" ON "invitations" USING btree ("account_id");--> statement-breakpoint
CREATE INDEX "invitations_expiration_date_idx" ON "invitations" USING btree ("expiration_date");--> statement-breakpoint
CREATE INDEX "ip_blacklist_id_idx" ON "ip_blacklist" USING btree ("id");--> statement-breakpoint
CREATE INDEX "ip_blacklist_categories_idx" ON "ip_blacklist" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "ip_blacklist_code_idx" ON "ip_blacklist" USING btree ("code");--> statement-breakpoint
CREATE INDEX "ip_blacklist_tombstone_idx" ON "ip_blacklist" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "ip_blacklist_status_idx" ON "ip_blacklist" USING btree ("status");--> statement-breakpoint
CREATE INDEX "ip_blacklist_previous_status_idx" ON "ip_blacklist" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "ip_blacklist_version_idx" ON "ip_blacklist" USING btree ("version");--> statement-breakpoint
CREATE INDEX "ip_blacklist_created_date_idx" ON "ip_blacklist" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "ip_blacklist_updated_date_idx" ON "ip_blacklist" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "ip_blacklist_organization_id_idx" ON "ip_blacklist" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "ip_blacklist_created_by_idx" ON "ip_blacklist" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "ip_blacklist_updated_by_idx" ON "ip_blacklist" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "ip_blacklist_deleted_by_idx" ON "ip_blacklist" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "ip_blacklist_requested_by_idx" ON "ip_blacklist" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "ip_blacklist_tags_idx" ON "ip_blacklist" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "ip_blacklist_image_url_idx" ON "ip_blacklist" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "ip_infos_id_idx" ON "ip_infos" USING btree ("id");--> statement-breakpoint
CREATE INDEX "ip_infos_categories_idx" ON "ip_infos" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "ip_infos_code_idx" ON "ip_infos" USING btree ("code");--> statement-breakpoint
CREATE INDEX "ip_infos_tombstone_idx" ON "ip_infos" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "ip_infos_status_idx" ON "ip_infos" USING btree ("status");--> statement-breakpoint
CREATE INDEX "ip_infos_previous_status_idx" ON "ip_infos" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "ip_infos_version_idx" ON "ip_infos" USING btree ("version");--> statement-breakpoint
CREATE INDEX "ip_infos_created_date_idx" ON "ip_infos" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "ip_infos_updated_date_idx" ON "ip_infos" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "ip_infos_organization_id_idx" ON "ip_infos" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "ip_infos_created_by_idx" ON "ip_infos" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "ip_infos_updated_by_idx" ON "ip_infos" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "ip_infos_deleted_by_idx" ON "ip_infos" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "ip_infos_requested_by_idx" ON "ip_infos" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "ip_infos_tags_idx" ON "ip_infos" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "ip_infos_image_url_idx" ON "ip_infos" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "locations_id_idx" ON "locations" USING btree ("id");--> statement-breakpoint
CREATE INDEX "locations_categories_idx" ON "locations" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "locations_code_idx" ON "locations" USING btree ("code");--> statement-breakpoint
CREATE INDEX "locations_tombstone_idx" ON "locations" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "locations_status_idx" ON "locations" USING btree ("status");--> statement-breakpoint
CREATE INDEX "locations_previous_status_idx" ON "locations" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "locations_version_idx" ON "locations" USING btree ("version");--> statement-breakpoint
CREATE INDEX "locations_created_date_idx" ON "locations" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "locations_updated_date_idx" ON "locations" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "locations_organization_id_idx" ON "locations" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "locations_created_by_idx" ON "locations" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "locations_updated_by_idx" ON "locations" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "locations_deleted_by_idx" ON "locations" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "locations_requested_by_idx" ON "locations" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "locations_tags_idx" ON "locations" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "locations_image_url_idx" ON "locations" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "locations_location_name_idx" ON "locations" USING btree ("location_name");--> statement-breakpoint
CREATE INDEX "locations_address_id_idx" ON "locations" USING btree ("address_id");--> statement-breakpoint
CREATE INDEX "notifications_id_idx" ON "notifications" USING btree ("id");--> statement-breakpoint
CREATE INDEX "notifications_categories_idx" ON "notifications" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "notifications_code_idx" ON "notifications" USING btree ("code");--> statement-breakpoint
CREATE INDEX "notifications_tombstone_idx" ON "notifications" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "notifications_status_idx" ON "notifications" USING btree ("status");--> statement-breakpoint
CREATE INDEX "notifications_previous_status_idx" ON "notifications" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "notifications_version_idx" ON "notifications" USING btree ("version");--> statement-breakpoint
CREATE INDEX "notifications_created_date_idx" ON "notifications" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "notifications_updated_date_idx" ON "notifications" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "notifications_organization_id_idx" ON "notifications" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "notifications_created_by_idx" ON "notifications" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "notifications_updated_by_idx" ON "notifications" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "notifications_deleted_by_idx" ON "notifications" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "notifications_requested_by_idx" ON "notifications" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "notifications_tags_idx" ON "notifications" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "notifications_image_url_idx" ON "notifications" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "notifications_title_idx" ON "notifications" USING btree ("title");--> statement-breakpoint
CREATE INDEX "notifications_description_idx" ON "notifications" USING btree ("description");--> statement-breakpoint
CREATE INDEX "notifications_link_idx" ON "notifications" USING btree ("link");--> statement-breakpoint
CREATE INDEX "notifications_icon_idx" ON "notifications" USING btree ("icon");--> statement-breakpoint
CREATE INDEX "notifications_source_idx" ON "notifications" USING btree ("source");--> statement-breakpoint
CREATE INDEX "notifications_is_pinned_idx" ON "notifications" USING btree ("is_pinned");--> statement-breakpoint
CREATE INDEX "notifications_recipient_id_idx" ON "notifications" USING btree ("recipient_id");--> statement-breakpoint
CREATE INDEX "notifications_notification_status_idx" ON "notifications" USING btree ("notification_status");--> statement-breakpoint
CREATE INDEX "notifications_priority_label_idx" ON "notifications" USING btree ("priority_label");--> statement-breakpoint
CREATE INDEX "notifications_priority_level_idx" ON "notifications" USING btree ("priority_level");--> statement-breakpoint
CREATE INDEX "notifications_expiry_date_idx" ON "notifications" USING btree ("expiry_date");--> statement-breakpoint
CREATE INDEX "packets_id_idx" ON "packets" USING btree ("id");--> statement-breakpoint
CREATE INDEX "packets_categories_idx" ON "packets" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "packets_code_idx" ON "packets" USING btree ("code");--> statement-breakpoint
CREATE INDEX "packets_tombstone_idx" ON "packets" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "packets_status_idx" ON "packets" USING btree ("status");--> statement-breakpoint
CREATE INDEX "packets_previous_status_idx" ON "packets" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "packets_version_idx" ON "packets" USING btree ("version");--> statement-breakpoint
CREATE INDEX "packets_created_date_idx" ON "packets" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "packets_updated_date_idx" ON "packets" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "packets_organization_id_idx" ON "packets" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "packets_created_by_idx" ON "packets" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "packets_updated_by_idx" ON "packets" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "packets_deleted_by_idx" ON "packets" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "packets_requested_by_idx" ON "packets" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "packets_tags_idx" ON "packets" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "packets_image_url_idx" ON "packets" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "packets_timestamp_idx" ON "packets" USING btree ("timestamp");--> statement-breakpoint
CREATE INDEX "packets_interface_name_idx" ON "packets" USING btree ("interface_name");--> statement-breakpoint
CREATE INDEX "packets_total_length_idx" ON "packets" USING btree ("total_length");--> statement-breakpoint
CREATE INDEX "packets_device_id_idx" ON "packets" USING btree ("device_id");--> statement-breakpoint
CREATE INDEX "packets_ether_type_idx" ON "packets" USING btree ("ether_type");--> statement-breakpoint
CREATE INDEX "packets_protocol_idx" ON "packets" USING btree ("protocol");--> statement-breakpoint
CREATE INDEX "packets_source_ip_idx" ON "packets" USING btree ("source_ip");--> statement-breakpoint
CREATE INDEX "packets_destination_ip_idx" ON "packets" USING btree ("destination_ip");--> statement-breakpoint
CREATE INDEX "packets_remote_ip_idx" ON "packets" USING btree ("remote_ip");--> statement-breakpoint
CREATE INDEX "packets_source_port_idx" ON "packets" USING btree ("source_port");--> statement-breakpoint
CREATE INDEX "packets_destination_port_idx" ON "packets" USING btree ("destination_port");--> statement-breakpoint
CREATE INDEX "postgres_channels_id_idx" ON "postgres_channels" USING btree ("id");--> statement-breakpoint
CREATE INDEX "postgres_channels_categories_idx" ON "postgres_channels" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "postgres_channels_code_idx" ON "postgres_channels" USING btree ("code");--> statement-breakpoint
CREATE INDEX "postgres_channels_tombstone_idx" ON "postgres_channels" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "postgres_channels_status_idx" ON "postgres_channels" USING btree ("status");--> statement-breakpoint
CREATE INDEX "postgres_channels_previous_status_idx" ON "postgres_channels" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "postgres_channels_version_idx" ON "postgres_channels" USING btree ("version");--> statement-breakpoint
CREATE INDEX "postgres_channels_created_date_idx" ON "postgres_channels" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "postgres_channels_updated_date_idx" ON "postgres_channels" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "postgres_channels_organization_id_idx" ON "postgres_channels" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "postgres_channels_created_by_idx" ON "postgres_channels" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "postgres_channels_updated_by_idx" ON "postgres_channels" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "postgres_channels_deleted_by_idx" ON "postgres_channels" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "postgres_channels_requested_by_idx" ON "postgres_channels" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "postgres_channels_tags_idx" ON "postgres_channels" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "postgres_channels_image_url_idx" ON "postgres_channels" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "resolutions_id_idx" ON "resolutions" USING btree ("id");--> statement-breakpoint
CREATE INDEX "resolutions_categories_idx" ON "resolutions" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "resolutions_code_idx" ON "resolutions" USING btree ("code");--> statement-breakpoint
CREATE INDEX "resolutions_tombstone_idx" ON "resolutions" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "resolutions_status_idx" ON "resolutions" USING btree ("status");--> statement-breakpoint
CREATE INDEX "resolutions_previous_status_idx" ON "resolutions" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "resolutions_version_idx" ON "resolutions" USING btree ("version");--> statement-breakpoint
CREATE INDEX "resolutions_created_date_idx" ON "resolutions" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "resolutions_updated_date_idx" ON "resolutions" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "resolutions_organization_id_idx" ON "resolutions" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "resolutions_created_by_idx" ON "resolutions" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "resolutions_updated_by_idx" ON "resolutions" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "resolutions_deleted_by_idx" ON "resolutions" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "resolutions_requested_by_idx" ON "resolutions" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "resolutions_tags_idx" ON "resolutions" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "resolutions_image_url_idx" ON "resolutions" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "smtp_requests_id_idx" ON "smtp_requests" USING btree ("id");--> statement-breakpoint
CREATE INDEX "smtp_requests_categories_idx" ON "smtp_requests" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "smtp_requests_code_idx" ON "smtp_requests" USING btree ("code");--> statement-breakpoint
CREATE INDEX "smtp_requests_tombstone_idx" ON "smtp_requests" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "smtp_requests_status_idx" ON "smtp_requests" USING btree ("status");--> statement-breakpoint
CREATE INDEX "smtp_requests_previous_status_idx" ON "smtp_requests" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "smtp_requests_version_idx" ON "smtp_requests" USING btree ("version");--> statement-breakpoint
CREATE INDEX "smtp_requests_created_date_idx" ON "smtp_requests" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "smtp_requests_updated_date_idx" ON "smtp_requests" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "smtp_requests_organization_id_idx" ON "smtp_requests" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "smtp_requests_created_by_idx" ON "smtp_requests" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "smtp_requests_updated_by_idx" ON "smtp_requests" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "smtp_requests_deleted_by_idx" ON "smtp_requests" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "smtp_requests_requested_by_idx" ON "smtp_requests" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "smtp_requests_tags_idx" ON "smtp_requests" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "smtp_requests_image_url_idx" ON "smtp_requests" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "smtp_responses_id_idx" ON "smtp_responses" USING btree ("id");--> statement-breakpoint
CREATE INDEX "smtp_responses_categories_idx" ON "smtp_responses" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "smtp_responses_code_idx" ON "smtp_responses" USING btree ("code");--> statement-breakpoint
CREATE INDEX "smtp_responses_tombstone_idx" ON "smtp_responses" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "smtp_responses_status_idx" ON "smtp_responses" USING btree ("status");--> statement-breakpoint
CREATE INDEX "smtp_responses_previous_status_idx" ON "smtp_responses" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "smtp_responses_version_idx" ON "smtp_responses" USING btree ("version");--> statement-breakpoint
CREATE INDEX "smtp_responses_created_date_idx" ON "smtp_responses" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "smtp_responses_updated_date_idx" ON "smtp_responses" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "smtp_responses_organization_id_idx" ON "smtp_responses" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "smtp_responses_created_by_idx" ON "smtp_responses" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "smtp_responses_updated_by_idx" ON "smtp_responses" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "smtp_responses_deleted_by_idx" ON "smtp_responses" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "smtp_responses_requested_by_idx" ON "smtp_responses" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "smtp_responses_tags_idx" ON "smtp_responses" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "smtp_responses_image_url_idx" ON "smtp_responses" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "system_resources_id_idx" ON "system_resources" USING btree ("id");--> statement-breakpoint
CREATE INDEX "system_resources_categories_idx" ON "system_resources" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "system_resources_code_idx" ON "system_resources" USING btree ("code");--> statement-breakpoint
CREATE INDEX "system_resources_tombstone_idx" ON "system_resources" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "system_resources_status_idx" ON "system_resources" USING btree ("status");--> statement-breakpoint
CREATE INDEX "system_resources_previous_status_idx" ON "system_resources" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "system_resources_version_idx" ON "system_resources" USING btree ("version");--> statement-breakpoint
CREATE INDEX "system_resources_created_date_idx" ON "system_resources" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "system_resources_updated_date_idx" ON "system_resources" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "system_resources_organization_id_idx" ON "system_resources" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "system_resources_created_by_idx" ON "system_resources" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "system_resources_updated_by_idx" ON "system_resources" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "system_resources_deleted_by_idx" ON "system_resources" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "system_resources_requested_by_idx" ON "system_resources" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "system_resources_tags_idx" ON "system_resources" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "system_resources_image_url_idx" ON "system_resources" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "tcp_connections_id_idx" ON "tcp_connections" USING btree ("id");--> statement-breakpoint
CREATE INDEX "tcp_connections_categories_idx" ON "tcp_connections" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "tcp_connections_code_idx" ON "tcp_connections" USING btree ("code");--> statement-breakpoint
CREATE INDEX "tcp_connections_tombstone_idx" ON "tcp_connections" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "tcp_connections_status_idx" ON "tcp_connections" USING btree ("status");--> statement-breakpoint
CREATE INDEX "tcp_connections_previous_status_idx" ON "tcp_connections" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "tcp_connections_version_idx" ON "tcp_connections" USING btree ("version");--> statement-breakpoint
CREATE INDEX "tcp_connections_created_date_idx" ON "tcp_connections" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "tcp_connections_updated_date_idx" ON "tcp_connections" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "tcp_connections_organization_id_idx" ON "tcp_connections" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "tcp_connections_created_by_idx" ON "tcp_connections" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "tcp_connections_updated_by_idx" ON "tcp_connections" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "tcp_connections_deleted_by_idx" ON "tcp_connections" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "tcp_connections_requested_by_idx" ON "tcp_connections" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "tcp_connections_tags_idx" ON "tcp_connections" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "tcp_connections_image_url_idx" ON "tcp_connections" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "temp_appguard_logs_id_idx" ON "temp_appguard_logs" USING btree ("id");--> statement-breakpoint
CREATE INDEX "temp_appguard_logs_categories_idx" ON "temp_appguard_logs" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "temp_appguard_logs_code_idx" ON "temp_appguard_logs" USING btree ("code");--> statement-breakpoint
CREATE INDEX "temp_appguard_logs_tombstone_idx" ON "temp_appguard_logs" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "temp_appguard_logs_status_idx" ON "temp_appguard_logs" USING btree ("status");--> statement-breakpoint
CREATE INDEX "temp_appguard_logs_previous_status_idx" ON "temp_appguard_logs" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "temp_appguard_logs_version_idx" ON "temp_appguard_logs" USING btree ("version");--> statement-breakpoint
CREATE INDEX "temp_appguard_logs_created_date_idx" ON "temp_appguard_logs" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "temp_appguard_logs_updated_date_idx" ON "temp_appguard_logs" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "temp_appguard_logs_organization_id_idx" ON "temp_appguard_logs" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "temp_appguard_logs_created_by_idx" ON "temp_appguard_logs" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "temp_appguard_logs_updated_by_idx" ON "temp_appguard_logs" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "temp_appguard_logs_deleted_by_idx" ON "temp_appguard_logs" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "temp_appguard_logs_requested_by_idx" ON "temp_appguard_logs" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "temp_appguard_logs_tags_idx" ON "temp_appguard_logs" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "temp_appguard_logs_image_url_idx" ON "temp_appguard_logs" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "temp_connections_id_idx" ON "temp_connections" USING btree ("id");--> statement-breakpoint
CREATE INDEX "temp_connections_categories_idx" ON "temp_connections" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "temp_connections_code_idx" ON "temp_connections" USING btree ("code");--> statement-breakpoint
CREATE INDEX "temp_connections_tombstone_idx" ON "temp_connections" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "temp_connections_status_idx" ON "temp_connections" USING btree ("status");--> statement-breakpoint
CREATE INDEX "temp_connections_previous_status_idx" ON "temp_connections" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "temp_connections_version_idx" ON "temp_connections" USING btree ("version");--> statement-breakpoint
CREATE INDEX "temp_connections_created_date_idx" ON "temp_connections" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "temp_connections_updated_date_idx" ON "temp_connections" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "temp_connections_organization_id_idx" ON "temp_connections" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "temp_connections_created_by_idx" ON "temp_connections" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "temp_connections_updated_by_idx" ON "temp_connections" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "temp_connections_deleted_by_idx" ON "temp_connections" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "temp_connections_requested_by_idx" ON "temp_connections" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "temp_connections_tags_idx" ON "temp_connections" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "temp_connections_image_url_idx" ON "temp_connections" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "temp_device_aliases_id_idx" ON "temp_device_aliases" USING btree ("id");--> statement-breakpoint
CREATE INDEX "temp_device_aliases_categories_idx" ON "temp_device_aliases" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "temp_device_aliases_code_idx" ON "temp_device_aliases" USING btree ("code");--> statement-breakpoint
CREATE INDEX "temp_device_aliases_tombstone_idx" ON "temp_device_aliases" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "temp_device_aliases_status_idx" ON "temp_device_aliases" USING btree ("status");--> statement-breakpoint
CREATE INDEX "temp_device_aliases_previous_status_idx" ON "temp_device_aliases" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "temp_device_aliases_version_idx" ON "temp_device_aliases" USING btree ("version");--> statement-breakpoint
CREATE INDEX "temp_device_aliases_created_date_idx" ON "temp_device_aliases" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "temp_device_aliases_updated_date_idx" ON "temp_device_aliases" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "temp_device_aliases_organization_id_idx" ON "temp_device_aliases" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "temp_device_aliases_created_by_idx" ON "temp_device_aliases" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "temp_device_aliases_updated_by_idx" ON "temp_device_aliases" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "temp_device_aliases_deleted_by_idx" ON "temp_device_aliases" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "temp_device_aliases_requested_by_idx" ON "temp_device_aliases" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "temp_device_aliases_tags_idx" ON "temp_device_aliases" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "temp_device_aliases_image_url_idx" ON "temp_device_aliases" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "temp_device_interface_addresses_id_idx" ON "temp_device_interface_addresses" USING btree ("id");--> statement-breakpoint
CREATE INDEX "temp_device_interface_addresses_categories_idx" ON "temp_device_interface_addresses" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "temp_device_interface_addresses_code_idx" ON "temp_device_interface_addresses" USING btree ("code");--> statement-breakpoint
CREATE INDEX "temp_device_interface_addresses_tombstone_idx" ON "temp_device_interface_addresses" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "temp_device_interface_addresses_status_idx" ON "temp_device_interface_addresses" USING btree ("status");--> statement-breakpoint
CREATE INDEX "temp_device_interface_addresses_previous_status_idx" ON "temp_device_interface_addresses" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "temp_device_interface_addresses_version_idx" ON "temp_device_interface_addresses" USING btree ("version");--> statement-breakpoint
CREATE INDEX "temp_device_interface_addresses_created_date_idx" ON "temp_device_interface_addresses" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "temp_device_interface_addresses_updated_date_idx" ON "temp_device_interface_addresses" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "temp_device_interface_addresses_organization_id_idx" ON "temp_device_interface_addresses" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "temp_device_interface_addresses_created_by_idx" ON "temp_device_interface_addresses" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "temp_device_interface_addresses_updated_by_idx" ON "temp_device_interface_addresses" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "temp_device_interface_addresses_deleted_by_idx" ON "temp_device_interface_addresses" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "temp_device_interface_addresses_requested_by_idx" ON "temp_device_interface_addresses" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "temp_device_interface_addresses_tags_idx" ON "temp_device_interface_addresses" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "temp_device_interface_addresses_image_url_idx" ON "temp_device_interface_addresses" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "temp_device_interfaces_id_idx" ON "temp_device_interfaces" USING btree ("id");--> statement-breakpoint
CREATE INDEX "temp_device_interfaces_categories_idx" ON "temp_device_interfaces" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "temp_device_interfaces_code_idx" ON "temp_device_interfaces" USING btree ("code");--> statement-breakpoint
CREATE INDEX "temp_device_interfaces_tombstone_idx" ON "temp_device_interfaces" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "temp_device_interfaces_status_idx" ON "temp_device_interfaces" USING btree ("status");--> statement-breakpoint
CREATE INDEX "temp_device_interfaces_previous_status_idx" ON "temp_device_interfaces" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "temp_device_interfaces_version_idx" ON "temp_device_interfaces" USING btree ("version");--> statement-breakpoint
CREATE INDEX "temp_device_interfaces_created_date_idx" ON "temp_device_interfaces" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "temp_device_interfaces_updated_date_idx" ON "temp_device_interfaces" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "temp_device_interfaces_organization_id_idx" ON "temp_device_interfaces" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "temp_device_interfaces_created_by_idx" ON "temp_device_interfaces" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "temp_device_interfaces_updated_by_idx" ON "temp_device_interfaces" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "temp_device_interfaces_deleted_by_idx" ON "temp_device_interfaces" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "temp_device_interfaces_requested_by_idx" ON "temp_device_interfaces" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "temp_device_interfaces_tags_idx" ON "temp_device_interfaces" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "temp_device_interfaces_image_url_idx" ON "temp_device_interfaces" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "temp_device_remote_access_sessions_id_idx" ON "temp_device_remote_access_sessions" USING btree ("id");--> statement-breakpoint
CREATE INDEX "temp_device_remote_access_sessions_categories_idx" ON "temp_device_remote_access_sessions" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "temp_device_remote_access_sessions_code_idx" ON "temp_device_remote_access_sessions" USING btree ("code");--> statement-breakpoint
CREATE INDEX "temp_device_remote_access_sessions_tombstone_idx" ON "temp_device_remote_access_sessions" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "temp_device_remote_access_sessions_status_idx" ON "temp_device_remote_access_sessions" USING btree ("status");--> statement-breakpoint
CREATE INDEX "temp_device_remote_access_sessions_previous_status_idx" ON "temp_device_remote_access_sessions" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "temp_device_remote_access_sessions_version_idx" ON "temp_device_remote_access_sessions" USING btree ("version");--> statement-breakpoint
CREATE INDEX "temp_device_remote_access_sessions_created_date_idx" ON "temp_device_remote_access_sessions" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "temp_device_remote_access_sessions_updated_date_idx" ON "temp_device_remote_access_sessions" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "temp_device_remote_access_sessions_organization_id_idx" ON "temp_device_remote_access_sessions" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "temp_device_remote_access_sessions_created_by_idx" ON "temp_device_remote_access_sessions" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "temp_device_remote_access_sessions_updated_by_idx" ON "temp_device_remote_access_sessions" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "temp_device_remote_access_sessions_deleted_by_idx" ON "temp_device_remote_access_sessions" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "temp_device_remote_access_sessions_requested_by_idx" ON "temp_device_remote_access_sessions" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "temp_device_remote_access_sessions_tags_idx" ON "temp_device_remote_access_sessions" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "temp_device_remote_access_sessions_image_url_idx" ON "temp_device_remote_access_sessions" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "temp_device_rules_id_idx" ON "temp_device_rules" USING btree ("id");--> statement-breakpoint
CREATE INDEX "temp_device_rules_categories_idx" ON "temp_device_rules" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "temp_device_rules_code_idx" ON "temp_device_rules" USING btree ("code");--> statement-breakpoint
CREATE INDEX "temp_device_rules_tombstone_idx" ON "temp_device_rules" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "temp_device_rules_status_idx" ON "temp_device_rules" USING btree ("status");--> statement-breakpoint
CREATE INDEX "temp_device_rules_previous_status_idx" ON "temp_device_rules" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "temp_device_rules_version_idx" ON "temp_device_rules" USING btree ("version");--> statement-breakpoint
CREATE INDEX "temp_device_rules_created_date_idx" ON "temp_device_rules" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "temp_device_rules_updated_date_idx" ON "temp_device_rules" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "temp_device_rules_organization_id_idx" ON "temp_device_rules" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "temp_device_rules_created_by_idx" ON "temp_device_rules" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "temp_device_rules_updated_by_idx" ON "temp_device_rules" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "temp_device_rules_deleted_by_idx" ON "temp_device_rules" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "temp_device_rules_requested_by_idx" ON "temp_device_rules" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "temp_device_rules_tags_idx" ON "temp_device_rules" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "temp_device_rules_image_url_idx" ON "temp_device_rules" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "temp_ip_blacklist_id_idx" ON "temp_ip_blacklist" USING btree ("id");--> statement-breakpoint
CREATE INDEX "temp_ip_blacklist_categories_idx" ON "temp_ip_blacklist" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "temp_ip_blacklist_code_idx" ON "temp_ip_blacklist" USING btree ("code");--> statement-breakpoint
CREATE INDEX "temp_ip_blacklist_tombstone_idx" ON "temp_ip_blacklist" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "temp_ip_blacklist_status_idx" ON "temp_ip_blacklist" USING btree ("status");--> statement-breakpoint
CREATE INDEX "temp_ip_blacklist_previous_status_idx" ON "temp_ip_blacklist" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "temp_ip_blacklist_version_idx" ON "temp_ip_blacklist" USING btree ("version");--> statement-breakpoint
CREATE INDEX "temp_ip_blacklist_created_date_idx" ON "temp_ip_blacklist" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "temp_ip_blacklist_updated_date_idx" ON "temp_ip_blacklist" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "temp_ip_blacklist_organization_id_idx" ON "temp_ip_blacklist" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "temp_ip_blacklist_created_by_idx" ON "temp_ip_blacklist" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "temp_ip_blacklist_updated_by_idx" ON "temp_ip_blacklist" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "temp_ip_blacklist_deleted_by_idx" ON "temp_ip_blacklist" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "temp_ip_blacklist_requested_by_idx" ON "temp_ip_blacklist" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "temp_ip_blacklist_tags_idx" ON "temp_ip_blacklist" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "temp_ip_blacklist_image_url_idx" ON "temp_ip_blacklist" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "temp_packets_id_idx" ON "temp_packets" USING btree ("id");--> statement-breakpoint
CREATE INDEX "temp_packets_categories_idx" ON "temp_packets" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "temp_packets_code_idx" ON "temp_packets" USING btree ("code");--> statement-breakpoint
CREATE INDEX "temp_packets_tombstone_idx" ON "temp_packets" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "temp_packets_status_idx" ON "temp_packets" USING btree ("status");--> statement-breakpoint
CREATE INDEX "temp_packets_previous_status_idx" ON "temp_packets" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "temp_packets_version_idx" ON "temp_packets" USING btree ("version");--> statement-breakpoint
CREATE INDEX "temp_packets_created_date_idx" ON "temp_packets" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "temp_packets_updated_date_idx" ON "temp_packets" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "temp_packets_organization_id_idx" ON "temp_packets" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "temp_packets_created_by_idx" ON "temp_packets" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "temp_packets_updated_by_idx" ON "temp_packets" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "temp_packets_deleted_by_idx" ON "temp_packets" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "temp_packets_requested_by_idx" ON "temp_packets" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "temp_packets_tags_idx" ON "temp_packets" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "temp_packets_image_url_idx" ON "temp_packets" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "temp_system_resources_id_idx" ON "temp_system_resources" USING btree ("id");--> statement-breakpoint
CREATE INDEX "temp_system_resources_categories_idx" ON "temp_system_resources" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "temp_system_resources_code_idx" ON "temp_system_resources" USING btree ("code");--> statement-breakpoint
CREATE INDEX "temp_system_resources_tombstone_idx" ON "temp_system_resources" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "temp_system_resources_status_idx" ON "temp_system_resources" USING btree ("status");--> statement-breakpoint
CREATE INDEX "temp_system_resources_previous_status_idx" ON "temp_system_resources" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "temp_system_resources_version_idx" ON "temp_system_resources" USING btree ("version");--> statement-breakpoint
CREATE INDEX "temp_system_resources_created_date_idx" ON "temp_system_resources" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "temp_system_resources_updated_date_idx" ON "temp_system_resources" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "temp_system_resources_organization_id_idx" ON "temp_system_resources" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "temp_system_resources_created_by_idx" ON "temp_system_resources" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "temp_system_resources_updated_by_idx" ON "temp_system_resources" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "temp_system_resources_deleted_by_idx" ON "temp_system_resources" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "temp_system_resources_requested_by_idx" ON "temp_system_resources" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "temp_system_resources_tags_idx" ON "temp_system_resources" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "temp_system_resources_image_url_idx" ON "temp_system_resources" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "temp_wallguard_logs_id_idx" ON "temp_wallguard_logs" USING btree ("id");--> statement-breakpoint
CREATE INDEX "temp_wallguard_logs_categories_idx" ON "temp_wallguard_logs" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "temp_wallguard_logs_code_idx" ON "temp_wallguard_logs" USING btree ("code");--> statement-breakpoint
CREATE INDEX "temp_wallguard_logs_tombstone_idx" ON "temp_wallguard_logs" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "temp_wallguard_logs_status_idx" ON "temp_wallguard_logs" USING btree ("status");--> statement-breakpoint
CREATE INDEX "temp_wallguard_logs_previous_status_idx" ON "temp_wallguard_logs" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "temp_wallguard_logs_version_idx" ON "temp_wallguard_logs" USING btree ("version");--> statement-breakpoint
CREATE INDEX "temp_wallguard_logs_created_date_idx" ON "temp_wallguard_logs" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "temp_wallguard_logs_updated_date_idx" ON "temp_wallguard_logs" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "temp_wallguard_logs_organization_id_idx" ON "temp_wallguard_logs" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "temp_wallguard_logs_created_by_idx" ON "temp_wallguard_logs" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "temp_wallguard_logs_updated_by_idx" ON "temp_wallguard_logs" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "temp_wallguard_logs_deleted_by_idx" ON "temp_wallguard_logs" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "temp_wallguard_logs_requested_by_idx" ON "temp_wallguard_logs" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "temp_wallguard_logs_tags_idx" ON "temp_wallguard_logs" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "temp_wallguard_logs_image_url_idx" ON "temp_wallguard_logs" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "wallguard_logs_id_idx" ON "wallguard_logs" USING btree ("id");--> statement-breakpoint
CREATE INDEX "wallguard_logs_categories_idx" ON "wallguard_logs" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "wallguard_logs_code_idx" ON "wallguard_logs" USING btree ("code");--> statement-breakpoint
CREATE INDEX "wallguard_logs_tombstone_idx" ON "wallguard_logs" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "wallguard_logs_status_idx" ON "wallguard_logs" USING btree ("status");--> statement-breakpoint
CREATE INDEX "wallguard_logs_previous_status_idx" ON "wallguard_logs" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "wallguard_logs_version_idx" ON "wallguard_logs" USING btree ("version");--> statement-breakpoint
CREATE INDEX "wallguard_logs_created_date_idx" ON "wallguard_logs" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "wallguard_logs_updated_date_idx" ON "wallguard_logs" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "wallguard_logs_organization_id_idx" ON "wallguard_logs" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "wallguard_logs_created_by_idx" ON "wallguard_logs" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "wallguard_logs_updated_by_idx" ON "wallguard_logs" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "wallguard_logs_deleted_by_idx" ON "wallguard_logs" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "wallguard_logs_requested_by_idx" ON "wallguard_logs" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "wallguard_logs_tags_idx" ON "wallguard_logs" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "wallguard_logs_image_url_idx" ON "wallguard_logs" USING btree ("image_url");
