
--> statement-breakpoint
CREATE TABLE "contact_emails" (
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
	"organization_id" text,
	"created_by" text,
	"updated_by" text,
	"deleted_by" text,
	"requested_by" text,
	"timestamp" timestamp with time zone DEFAULT now() NOT NULL,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"sensitivity_level" integer DEFAULT 1000,
	"contact_id" text,
	"email" text,
	"is_primary" boolean DEFAULT false
);
--> statement-breakpoint
CREATE TABLE "contact_phone_numbers" (
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
	"organization_id" text,
	"created_by" text,
	"updated_by" text,
	"deleted_by" text,
	"requested_by" text,
	"timestamp" timestamp with time zone DEFAULT now() NOT NULL,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"sensitivity_level" integer DEFAULT 1000,
	"contact_id" text,
	"phone_number_raw" text
);
--> statement-breakpoint
CREATE TABLE "contacts" (
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
	"organization_id" text,
	"created_by" text,
	"updated_by" text,
	"deleted_by" text,
	"requested_by" text,
	"timestamp" timestamp with time zone DEFAULT now() NOT NULL,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"sensitivity_level" integer DEFAULT 1000,
	"first_name" text,
	"middle_name" text,
	"last_name" text,
	"date_of_birth" text,
	"account_id" text
);
--> statement-breakpoint
CREATE TABLE "organization_accounts" (
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
	"organization_id" text,
	"created_by" text,
	"updated_by" text,
	"deleted_by" text,
	"requested_by" text,
	"timestamp" timestamp with time zone DEFAULT now() NOT NULL,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"sensitivity_level" integer DEFAULT 1000,
	"organization_contact_id" text,
	"account_id" text,
	"account_secret" text,
	"role_id" text,
	"contact_id" text,
	"device_id" text,
	"is_new_user" boolean DEFAULT false,
	"account_status" text,
	"external_contact_id" text
);
--> statement-breakpoint
CREATE TABLE "organization_contacts" (
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
	"organization_id" text,
	"created_by" text,
	"updated_by" text,
	"deleted_by" text,
	"requested_by" text,
	"timestamp" timestamp with time zone DEFAULT now() NOT NULL,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"sensitivity_level" integer DEFAULT 1000,
	"contact_id" text
);
--> statement-breakpoint
CREATE TABLE "organization_domains" (
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
	"organization_id" text,
	"created_by" text,
	"updated_by" text,
	"deleted_by" text,
	"requested_by" text,
	"timestamp" timestamp with time zone DEFAULT now() NOT NULL,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"sensitivity_level" integer DEFAULT 1000,
	"domain_name" text,
	CONSTRAINT "organization_domains_domain_name_unique" UNIQUE("domain_name")
);

--> statement-breakpoint
CREATE TABLE "organizations" (
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
	"organization_id" text,
	"created_by" text,
	"updated_by" text,
	"deleted_by" text,
	"requested_by" text,
	"timestamp" timestamp with time zone DEFAULT now() NOT NULL,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"sensitivity_level" integer DEFAULT 1000,
	"parent_organization_id" text,
	"name" text,
	"organization_level" integer DEFAULT 0,
	"root_organization_id" text,
	"path_level" jsonb DEFAULT '[]'
);
--> statement-breakpoint
CREATE TABLE "external_contacts" (
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
	"organization_id" text,
	"created_by" text,
	"updated_by" text,
	"deleted_by" text,
	"requested_by" text,
	"timestamp" timestamp with time zone DEFAULT now() NOT NULL,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"sensitivity_level" integer DEFAULT 1000,
	"first_name" text,
	"last_name" text,
	"email" text
);
--> statement-breakpoint
CREATE TABLE "account_organizations" (
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
	"organization_id" text,
	"created_by" text,
	"updated_by" text,
	"deleted_by" text,
	"requested_by" text,
	"timestamp" timestamp with time zone DEFAULT now() NOT NULL,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"sensitivity_level" integer DEFAULT 1000,
	"contact_id" text,
	"email" text,
	"account_id" text,
	"role_id" text,
	"account_organization_status" text,
	"is_invited" boolean DEFAULT false,
	"device_id" text
);
--> statement-breakpoint
CREATE TABLE "account_profiles" (
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
	"organization_id" text,
	"created_by" text,
	"updated_by" text,
	"deleted_by" text,
	"requested_by" text,
	"timestamp" timestamp with time zone DEFAULT now() NOT NULL,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"sensitivity_level" integer DEFAULT 1000,
	"first_name" text,
	"last_name" text,
	"email" text,
	"account_id" text
);
--> statement-breakpoint
CREATE TABLE "accounts" (
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
	"organization_id" text,
	"created_by" text,
	"updated_by" text,
	"deleted_by" text,
	"requested_by" text,
	"timestamp" timestamp with time zone DEFAULT now() NOT NULL,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"sensitivity_level" integer DEFAULT 1000,
	"account_id" text,
	"account_secret" text,
	"account_status" text,
	"is_new_user" boolean DEFAULT false
);
--> statement-breakpoint
CREATE TABLE "addresses" (
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
	"organization_id" text,
	"created_by" text,
	"updated_by" text,
	"deleted_by" text,
	"requested_by" text,
	"timestamp" timestamp with time zone DEFAULT now() NOT NULL,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"sensitivity_level" integer DEFAULT 1000,
	"address" text,
	"address_line_one" text,
	"address_line_two" text,
	"latitude" real,
	"longitude" real,
	"place_id" text,
	"street_number" text,
	"street" text,
	"region" text,
	"region_code" text,
	"country_code" text,
	"postal_code" text,
	"country" text,
	"state" text,
	"city" text
);
--> statement-breakpoint
CREATE TABLE "user_roles" (
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
	"organization_id" text,
	"created_by" text,
	"updated_by" text,
	"deleted_by" text,
	"requested_by" text,
	"timestamp" timestamp with time zone DEFAULT now() NOT NULL,
	"tags" text[] DEFAULT ARRAY[]::TEXT[],
	"image_url" varchar(300),
	"sensitivity_level" integer DEFAULT 1000,
	"entity" text,
	"role" text,
	CONSTRAINT "user_roles_role_unique" UNIQUE("role")
);
--> statement-breakpoint
CREATE TABLE "entities" (
	"id" text PRIMARY KEY NOT NULL,
	"name" text NOT NULL,
	"organization_id" text,
	"created_by" text,
	"updated_by" text,
	"deleted_by" text,
	"timestamp" timestamp with time zone DEFAULT now() NOT NULL,
	"tombstone" integer DEFAULT 0,
	CONSTRAINT "entities_name_unique" UNIQUE("name")
);
--> statement-breakpoint
CREATE TABLE "fields" (
	"id" text PRIMARY KEY NOT NULL,
	"label" text,
	"name" text,
	"field_type" text,
	"constraints" jsonb DEFAULT '[]',
	"_default" text,
	"reference_to" text,
	"created_by" text,
	"updated_by" text,
	"deleted_by" text,
	"timestamp" timestamp with time zone DEFAULT now() NOT NULL,
	"tombstone" integer DEFAULT 0
);
--> statement-breakpoint
CREATE TABLE "entity_fields" (
	"id" text PRIMARY KEY NOT NULL,
	"entity_id" text NOT NULL,
	"field_id" text NOT NULL,
	"sensitivity_level" integer DEFAULT 1000,
	"is_encryptable" boolean DEFAULT false,
	"schema_version" integer DEFAULT 1,
	"created_by" text,
	"updated_by" text,
	"deleted_by" text,
	"timestamp" timestamp with time zone DEFAULT now() NOT NULL,
	"tombstone" integer DEFAULT 0
);

CREATE TABLE "system_config_fields" (
	"field_id" text PRIMARY KEY NOT NULL,
	"is_searchable" boolean DEFAULT false,
	"is_system_field" boolean DEFAULT false,
	"is_encryptable" boolean NOT NULL,
	"is_allowed_to_return" boolean DEFAULT true,
	"created_by" text,
	"updated_by" text,
	"deleted_by" text,
	"timestamp" timestamp with time zone DEFAULT now() NOT NULL,
	"tombstone" integer DEFAULT 0
);
CREATE TABLE "devices" (
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
	"model" text,
	"address_id" text,
	"instance_name" text,
	"is_connection_established" boolean DEFAULT false,
	"system_id" text,
	"device_version" text,
	"last_heartbeat" text,
	"is_monitoring_enabled" boolean DEFAULT true,
	"is_remote_access_enabled" boolean DEFAULT true,
	"ip_address" "inet",
	"device_status" text,
	"device_gui_protocol" text,
	"sensitivity_level" integer DEFAULT 1000
);
--> statement-breakpoint
CREATE TABLE "permissions" (
	"id" text PRIMARY KEY NOT NULL,
	"read" boolean DEFAULT true,
	"write" boolean DEFAULT false,
	"encrypt" boolean DEFAULT false,
	"decrypt" boolean DEFAULT false,
	"required" boolean DEFAULT false,
	"sensitive" boolean DEFAULT false,
	"archive" boolean DEFAULT false,
	"delete" boolean DEFAULT false,
	"created_by" text,
	"updated_by" text,
	"deleted_by" text,
	"timestamp" timestamp with time zone DEFAULT now() NOT NULL,
	"tombstone" integer DEFAULT 0
);
--> statement-breakpoint
CREATE TABLE "data_permissions" (
	"id" text PRIMARY KEY NOT NULL,
	"entity_field_id" text NOT NULL,
	"permission_id" text NOT NULL,
	"record_permission_id" text,
	"role_permission_id" text,
	"account_organization_id" text NOT NULL,
	"created_by" text,
	"updated_by" text,
	"deleted_by" text,
	"timestamp" timestamp with time zone DEFAULT now() NOT NULL,
	"tombstone" integer DEFAULT 0,
	"sensitivity_level" integer DEFAULT 1000
);
--> statement-breakpoint
CREATE TABLE "role_permissions" (
	"id" text PRIMARY KEY NOT NULL,
	"role_id" text NOT NULL,
	"permission_id" text NOT NULL,
	"created_by" text,
	"updated_by" text,
	"deleted_by" text,
	"timestamp" timestamp with time zone DEFAULT now() NOT NULL,
	"tombstone" integer DEFAULT 0
);
--> statement-breakpoint
CREATE TABLE "record_permissions" (
	"id" text PRIMARY KEY NOT NULL,
	"record_id" text NOT NULL,
	"record_entity" text NOT NULL,
	"permission_id" text NOT NULL,
	"created_by" text,
	"updated_by" text,
	"deleted_by" text,
	"timestamp" timestamp with time zone DEFAULT now() NOT NULL,
	"tombstone" integer DEFAULT 0
);
--> statement-breakpoint
CREATE TABLE "encryption_keys" (
	"id" text PRIMARY KEY NOT NULL,
	"organization_id" text NOT NULL,
	"entity" text NOT NULL,
	"created_by" text,
	"timestamp" timestamp with time zone DEFAULT now() NOT NULL,
	"tombstone" integer DEFAULT 0
);
--> statement-breakpoint
CREATE TABLE "sessions" (
	"sid" text PRIMARY KEY NOT NULL,
	"sess" jsonb NOT NULL,
	"expire" timestamp NOT NULL
);

CREATE TABLE "table_indexes" (
	"id" text PRIMARY KEY NOT NULL,
	"entity_id" text NOT NULL,
	"secondary_index" text,
	"compound_index" jsonb DEFAULT '[]',
	"created_by" text,
	"updated_by" text,
	"deleted_by" text,
	"timestamp" timestamp with time zone DEFAULT now() NOT NULL,
	"tombstone" integer DEFAULT 0
);

--> statement-breakpoint
CREATE TABLE "samples" (
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
	"organization_id" text,
	"created_by" text,
	"updated_by" text,
	"deleted_by" text,
	"requested_by" text,
	"timestamp" timestamp with time zone DEFAULT now() NOT NULL,
	"sensitivity_level" integer DEFAULT 1000,
	"name" text,
	"sample_text" text
);

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
	"sensitivity_level" integer DEFAULT 1000,
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
	"sensitivity_level" integer DEFAULT 1000,
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
	"sensitivity_level" integer DEFAULT 1000,
	"name" text,
	"communication_template_status" text,
	"event" text,
	"content" text,
	"subject" text
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




-- device_heartbeats table removed
--> statement-breakpoint



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

--> statement-breakpoint
-- locations table removed
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
	"sensitivity_level" integer DEFAULT 1000,
	"channel_name" text,
	"function" text,
	CONSTRAINT "postgres_channels_channel_name_unique" UNIQUE("channel_name")
);

CREATE TABLE stream_queue (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    last_accessed TIMESTAMPTZ
);
CREATE TABLE stream_queue_items (
    id TEXT PRIMARY KEY,
    queue_name TEXT NOT NULL REFERENCES stream_queue(name) ON DELETE CASCADE,
    content JSONB NOT NULL,
    timestamp TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_stream_queue_items_queue_name ON stream_queue_items(queue_name);
CREATE INDEX idx_stream_queue_items_timestamp ON stream_queue_items(timestamp);

CREATE INDEX idx_stream_queue_name ON stream_queue(name);
CREATE INDEX idx_stream_queue_created_at ON stream_queue(created_at);
CREATE INDEX idx_stream_queue_last_accessed ON stream_queue(last_accessed);


ALTER TABLE "table_indexes" ADD CONSTRAINT "table_indexes_entity_id_entities_id_fk" FOREIGN KEY ("entity_id") REFERENCES "public"."entities"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "devices" ADD CONSTRAINT "devices_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "devices" ADD CONSTRAINT "devices_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "devices" ADD CONSTRAINT "devices_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "devices" ADD CONSTRAINT "devices_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "devices" ADD CONSTRAINT "devices_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "devices" ADD CONSTRAINT "devices_address_id_addresses_id_fk" FOREIGN KEY ("address_id") REFERENCES "public"."addresses"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "contact_emails" ADD CONSTRAINT "contact_emails_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "contact_emails" ADD CONSTRAINT "contact_emails_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "contact_emails" ADD CONSTRAINT "contact_emails_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "contact_emails" ADD CONSTRAINT "contact_emails_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "contact_emails" ADD CONSTRAINT "contact_emails_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "contact_emails" ADD CONSTRAINT "contact_emails_contact_id_contacts_id_fk" FOREIGN KEY ("contact_id") REFERENCES "public"."contacts"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "contact_phone_numbers" ADD CONSTRAINT "contact_phone_numbers_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "contact_phone_numbers" ADD CONSTRAINT "contact_phone_numbers_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "contact_phone_numbers" ADD CONSTRAINT "contact_phone_numbers_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "contact_phone_numbers" ADD CONSTRAINT "contact_phone_numbers_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "contact_phone_numbers" ADD CONSTRAINT "contact_phone_numbers_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "contact_phone_numbers" ADD CONSTRAINT "contact_phone_numbers_contact_id_contacts_id_fk" FOREIGN KEY ("contact_id") REFERENCES "public"."contacts"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "contacts" ADD CONSTRAINT "contacts_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "contacts" ADD CONSTRAINT "contacts_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "contacts" ADD CONSTRAINT "contacts_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "contacts" ADD CONSTRAINT "contacts_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "contacts" ADD CONSTRAINT "contacts_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "contacts" ADD CONSTRAINT "contacts_account_id_accounts_id_fk" FOREIGN KEY ("account_id") REFERENCES "public"."accounts"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "organization_accounts" ADD CONSTRAINT "organization_accounts_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "organization_accounts" ADD CONSTRAINT "organization_accounts_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "organization_accounts" ADD CONSTRAINT "organization_accounts_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "organization_accounts" ADD CONSTRAINT "organization_accounts_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "organization_accounts" ADD CONSTRAINT "organization_accounts_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "organization_accounts" ADD CONSTRAINT "organization_accounts_organization_contact_id_organization_contacts_id_fk" FOREIGN KEY ("organization_contact_id") REFERENCES "public"."organization_contacts"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "organization_accounts" ADD CONSTRAINT "organization_accounts_contact_id_contacts_id_fk" FOREIGN KEY ("contact_id") REFERENCES "public"."contacts"("id") ON DELETE set null ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "organization_accounts" ADD CONSTRAINT "organization_accounts_device_id_devices_id_fk" FOREIGN KEY ("device_id") REFERENCES "public"."devices"("id") ON DELETE set null ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "organization_accounts" ADD CONSTRAINT "organization_accounts_external_contact_id_external_contacts_id_fk" FOREIGN KEY ("external_contact_id") REFERENCES "public"."external_contacts"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "organization_contacts" ADD CONSTRAINT "organization_contacts_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "organization_contacts" ADD CONSTRAINT "organization_contacts_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "organization_contacts" ADD CONSTRAINT "organization_contacts_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "organization_contacts" ADD CONSTRAINT "organization_contacts_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "organization_contacts" ADD CONSTRAINT "organization_contacts_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "organization_contacts" ADD CONSTRAINT "organization_contacts_contact_id_contacts_id_fk" FOREIGN KEY ("contact_id") REFERENCES "public"."contacts"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "organization_domains" ADD CONSTRAINT "organization_domains_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "organization_domains" ADD CONSTRAINT "organization_domains_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "organization_domains" ADD CONSTRAINT "organization_domains_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "organization_domains" ADD CONSTRAINT "organization_domains_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "organization_domains" ADD CONSTRAINT "organization_domains_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "organizations" ADD CONSTRAINT "organizations_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "organizations" ADD CONSTRAINT "organizations_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "organizations" ADD CONSTRAINT "organizations_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "organizations" ADD CONSTRAINT "organizations_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "organizations" ADD CONSTRAINT "organizations_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "organizations" ADD CONSTRAINT "organizations_parent_organization_id_organizations_id_fk" FOREIGN KEY ("parent_organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "organizations" ADD CONSTRAINT "organizations_root_organization_id_organizations_id_fk" FOREIGN KEY ("root_organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "external_contacts" ADD CONSTRAINT "external_contacts_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "external_contacts" ADD CONSTRAINT "external_contacts_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "external_contacts" ADD CONSTRAINT "external_contacts_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "external_contacts" ADD CONSTRAINT "external_contacts_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "external_contacts" ADD CONSTRAINT "external_contacts_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "account_organizations" ADD CONSTRAINT "account_organizations_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "account_organizations" ADD CONSTRAINT "account_organizations_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "account_organizations" ADD CONSTRAINT "account_organizations_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "account_organizations" ADD CONSTRAINT "account_organizations_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "account_organizations" ADD CONSTRAINT "account_organizations_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "account_organizations" ADD CONSTRAINT "account_organizations_contact_id_contacts_id_fk" FOREIGN KEY ("contact_id") REFERENCES "public"."contacts"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "account_organizations" ADD CONSTRAINT "account_organizations_account_id_accounts_id_fk" FOREIGN KEY ("account_id") REFERENCES "public"."accounts"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "account_organizations" ADD CONSTRAINT "account_organizations_device_id_devices_id_fk" FOREIGN KEY ("device_id") REFERENCES "public"."devices"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "account_profiles" ADD CONSTRAINT "account_profiles_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "account_profiles" ADD CONSTRAINT "account_profiles_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "account_profiles" ADD CONSTRAINT "account_profiles_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "account_profiles" ADD CONSTRAINT "account_profiles_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "account_profiles" ADD CONSTRAINT "account_profiles_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "account_profiles" ADD CONSTRAINT "account_profiles_account_id_accounts_id_fk" FOREIGN KEY ("account_id") REFERENCES "public"."accounts"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "accounts" ADD CONSTRAINT "accounts_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "accounts" ADD CONSTRAINT "accounts_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "accounts" ADD CONSTRAINT "accounts_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "accounts" ADD CONSTRAINT "accounts_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "accounts" ADD CONSTRAINT "accounts_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "addresses" ADD CONSTRAINT "addresses_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "addresses" ADD CONSTRAINT "addresses_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "addresses" ADD CONSTRAINT "addresses_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "addresses" ADD CONSTRAINT "addresses_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "addresses" ADD CONSTRAINT "addresses_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "user_roles" ADD CONSTRAINT "user_roles_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "user_roles" ADD CONSTRAINT "user_roles_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "user_roles" ADD CONSTRAINT "user_roles_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "user_roles" ADD CONSTRAINT "user_roles_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "user_roles" ADD CONSTRAINT "user_roles_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "entity_fields" ADD CONSTRAINT "entity_fields_entity_id_entities_id_fk" FOREIGN KEY ("entity_id") REFERENCES "public"."entities"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "entity_fields" ADD CONSTRAINT "entity_fields_field_id_fields_id_fk" FOREIGN KEY ("field_id") REFERENCES "public"."fields"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "data_permissions" ADD CONSTRAINT "data_permissions_entity_field_id_entity_fields_id_fk" FOREIGN KEY ("entity_field_id") REFERENCES "public"."entity_fields"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "data_permissions" ADD CONSTRAINT "data_permissions_permission_id_permissions_id_fk" FOREIGN KEY ("permission_id") REFERENCES "public"."permissions"("id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "data_permissions" ADD CONSTRAINT "data_permissions_record_permission_id_record_permissions_id_fk" FOREIGN KEY ("record_permission_id") REFERENCES "public"."record_permissions"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "data_permissions" ADD CONSTRAINT "data_permissions_role_permission_id_role_permissions_id_fk" FOREIGN KEY ("role_permission_id") REFERENCES "public"."role_permissions"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "data_permissions" ADD CONSTRAINT "data_permissions_account_organization_id_account_organizations_id_fk" FOREIGN KEY ("account_organization_id") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "role_permissions" ADD CONSTRAINT "role_permissions_role_id_user_roles_id_fk" FOREIGN KEY ("role_id") REFERENCES "public"."user_roles"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "role_permissions" ADD CONSTRAINT "role_permissions_permission_id_permissions_id_fk" FOREIGN KEY ("permission_id") REFERENCES "public"."permissions"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "record_permissions" ADD CONSTRAINT "record_permissions_permission_id_permissions_id_fk" FOREIGN KEY ("permission_id") REFERENCES "public"."permissions"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "samples" ADD CONSTRAINT "samples_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "samples" ADD CONSTRAINT "samples_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "samples" ADD CONSTRAINT "samples_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "samples" ADD CONSTRAINT "samples_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "samples" ADD CONSTRAINT "samples_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "system_config_fields" ADD CONSTRAINT "system_config_fields_field_id_fields_id_fk" FOREIGN KEY ("field_id") REFERENCES "public"."fields"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint

CREATE INDEX "contact_emails_id_idx" ON "contact_emails" USING btree ("id");--> statement-breakpoint
CREATE INDEX "contact_emails_categories_idx" ON "contact_emails" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "contact_emails_code_idx" ON "contact_emails" USING btree ("code");--> statement-breakpoint
CREATE INDEX "contact_emails_tombstone_idx" ON "contact_emails" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "contact_emails_status_idx" ON "contact_emails" USING btree ("status");--> statement-breakpoint
CREATE INDEX "contact_emails_previous_status_idx" ON "contact_emails" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "contact_emails_version_idx" ON "contact_emails" USING btree ("version");--> statement-breakpoint
CREATE INDEX "contact_emails_created_date_idx" ON "contact_emails" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "contact_emails_updated_date_idx" ON "contact_emails" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "contact_emails_organization_id_idx" ON "contact_emails" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "contact_emails_created_by_idx" ON "contact_emails" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "contact_emails_updated_by_idx" ON "contact_emails" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "contact_emails_deleted_by_idx" ON "contact_emails" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "contact_emails_requested_by_idx" ON "contact_emails" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "contact_emails_tags_idx" ON "contact_emails" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "contact_emails_image_url_idx" ON "contact_emails" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "contact_emails_sensitivity_level_idx" ON "contact_emails" USING btree ("sensitivity_level");--> statement-breakpoint
CREATE INDEX "contact_phone_numbers_id_idx" ON "contact_phone_numbers" USING btree ("id");--> statement-breakpoint
CREATE INDEX "contact_phone_numbers_categories_idx" ON "contact_phone_numbers" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "contact_phone_numbers_code_idx" ON "contact_phone_numbers" USING btree ("code");--> statement-breakpoint
CREATE INDEX "contact_phone_numbers_tombstone_idx" ON "contact_phone_numbers" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "contact_phone_numbers_status_idx" ON "contact_phone_numbers" USING btree ("status");--> statement-breakpoint
CREATE INDEX "contact_phone_numbers_previous_status_idx" ON "contact_phone_numbers" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "contact_phone_numbers_version_idx" ON "contact_phone_numbers" USING btree ("version");--> statement-breakpoint
CREATE INDEX "contact_phone_numbers_created_date_idx" ON "contact_phone_numbers" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "contact_phone_numbers_updated_date_idx" ON "contact_phone_numbers" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "contact_phone_numbers_organization_id_idx" ON "contact_phone_numbers" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "contact_phone_numbers_created_by_idx" ON "contact_phone_numbers" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "contact_phone_numbers_updated_by_idx" ON "contact_phone_numbers" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "contact_phone_numbers_deleted_by_idx" ON "contact_phone_numbers" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "contact_phone_numbers_requested_by_idx" ON "contact_phone_numbers" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "contact_phone_numbers_tags_idx" ON "contact_phone_numbers" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "contact_phone_numbers_image_url_idx" ON "contact_phone_numbers" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "contact_phone_numbers_sensitivity_level_idx" ON "contact_phone_numbers" USING btree ("sensitivity_level");--> statement-breakpoint
CREATE INDEX "contacts_id_idx" ON "contacts" USING btree ("id");--> statement-breakpoint
CREATE INDEX "contacts_categories_idx" ON "contacts" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "contacts_code_idx" ON "contacts" USING btree ("code");--> statement-breakpoint
CREATE INDEX "contacts_tombstone_idx" ON "contacts" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "contacts_status_idx" ON "contacts" USING btree ("status");--> statement-breakpoint
CREATE INDEX "contacts_previous_status_idx" ON "contacts" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "contacts_version_idx" ON "contacts" USING btree ("version");--> statement-breakpoint
CREATE INDEX "contacts_created_date_idx" ON "contacts" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "contacts_updated_date_idx" ON "contacts" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "contacts_organization_id_idx" ON "contacts" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "contacts_created_by_idx" ON "contacts" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "contacts_updated_by_idx" ON "contacts" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "contacts_deleted_by_idx" ON "contacts" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "contacts_requested_by_idx" ON "contacts" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "contacts_tags_idx" ON "contacts" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "contacts_image_url_idx" ON "contacts" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "contacts_sensitivity_level_idx" ON "contacts" USING btree ("sensitivity_level");--> statement-breakpoint
CREATE INDEX "organization_accounts_id_idx" ON "organization_accounts" USING btree ("id");--> statement-breakpoint
CREATE INDEX "organization_accounts_categories_idx" ON "organization_accounts" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "organization_accounts_code_idx" ON "organization_accounts" USING btree ("code");--> statement-breakpoint
CREATE INDEX "organization_accounts_tombstone_idx" ON "organization_accounts" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "organization_accounts_status_idx" ON "organization_accounts" USING btree ("status");--> statement-breakpoint
CREATE INDEX "organization_accounts_previous_status_idx" ON "organization_accounts" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "organization_accounts_version_idx" ON "organization_accounts" USING btree ("version");--> statement-breakpoint
CREATE INDEX "organization_accounts_created_date_idx" ON "organization_accounts" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "organization_accounts_updated_date_idx" ON "organization_accounts" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "organization_accounts_organization_id_idx" ON "organization_accounts" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "organization_accounts_created_by_idx" ON "organization_accounts" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "organization_accounts_updated_by_idx" ON "organization_accounts" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "organization_accounts_deleted_by_idx" ON "organization_accounts" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "organization_accounts_requested_by_idx" ON "organization_accounts" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "organization_accounts_tags_idx" ON "organization_accounts" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "organization_accounts_image_url_idx" ON "organization_accounts" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "organization_accounts_sensitivity_level_idx" ON "organization_accounts" USING btree ("sensitivity_level");--> statement-breakpoint
CREATE INDEX "organization_contacts_id_idx" ON "organization_contacts" USING btree ("id");--> statement-breakpoint
CREATE INDEX "organization_contacts_categories_idx" ON "organization_contacts" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "organization_contacts_code_idx" ON "organization_contacts" USING btree ("code");--> statement-breakpoint
CREATE INDEX "organization_contacts_tombstone_idx" ON "organization_contacts" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "organization_contacts_status_idx" ON "organization_contacts" USING btree ("status");--> statement-breakpoint
CREATE INDEX "organization_contacts_previous_status_idx" ON "organization_contacts" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "organization_contacts_version_idx" ON "organization_contacts" USING btree ("version");--> statement-breakpoint
CREATE INDEX "organization_contacts_created_date_idx" ON "organization_contacts" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "organization_contacts_updated_date_idx" ON "organization_contacts" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "organization_contacts_organization_id_idx" ON "organization_contacts" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "organization_contacts_created_by_idx" ON "organization_contacts" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "organization_contacts_updated_by_idx" ON "organization_contacts" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "organization_contacts_deleted_by_idx" ON "organization_contacts" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "organization_contacts_requested_by_idx" ON "organization_contacts" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "organization_contacts_tags_idx" ON "organization_contacts" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "organization_contacts_image_url_idx" ON "organization_contacts" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "organization_contacts_sensitivity_level_idx" ON "organization_contacts" USING btree ("sensitivity_level");--> statement-breakpoint
CREATE INDEX "organization_domains_id_idx" ON "organization_domains" USING btree ("id");--> statement-breakpoint
CREATE INDEX "organization_domains_categories_idx" ON "organization_domains" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "organization_domains_code_idx" ON "organization_domains" USING btree ("code");--> statement-breakpoint
CREATE INDEX "organization_domains_tombstone_idx" ON "organization_domains" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "organization_domains_status_idx" ON "organization_domains" USING btree ("status");--> statement-breakpoint
CREATE INDEX "organization_domains_previous_status_idx" ON "organization_domains" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "organization_domains_version_idx" ON "organization_domains" USING btree ("version");--> statement-breakpoint
CREATE INDEX "organization_domains_created_date_idx" ON "organization_domains" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "organization_domains_updated_date_idx" ON "organization_domains" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "organization_domains_organization_id_idx" ON "organization_domains" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "organization_domains_created_by_idx" ON "organization_domains" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "organization_domains_updated_by_idx" ON "organization_domains" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "organization_domains_deleted_by_idx" ON "organization_domains" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "organization_domains_requested_by_idx" ON "organization_domains" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "organization_domains_tags_idx" ON "organization_domains" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "organization_domains_image_url_idx" ON "organization_domains" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "organization_domains_sensitivity_level_idx" ON "organization_domains" USING btree ("sensitivity_level");--> statement-breakpoint
CREATE INDEX "organizations_id_idx" ON "organizations" USING btree ("id");--> statement-breakpoint
CREATE INDEX "organizations_categories_idx" ON "organizations" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "organizations_code_idx" ON "organizations" USING btree ("code");--> statement-breakpoint
CREATE INDEX "organizations_tombstone_idx" ON "organizations" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "organizations_status_idx" ON "organizations" USING btree ("status");--> statement-breakpoint
CREATE INDEX "organizations_previous_status_idx" ON "organizations" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "organizations_version_idx" ON "organizations" USING btree ("version");--> statement-breakpoint
CREATE INDEX "organizations_created_date_idx" ON "organizations" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "organizations_updated_date_idx" ON "organizations" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "organizations_organization_id_idx" ON "organizations" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "organizations_created_by_idx" ON "organizations" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "organizations_updated_by_idx" ON "organizations" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "organizations_deleted_by_idx" ON "organizations" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "organizations_requested_by_idx" ON "organizations" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "organizations_tags_idx" ON "organizations" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "organizations_image_url_idx" ON "organizations" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "organizations_sensitivity_level_idx" ON "organizations" USING btree ("sensitivity_level");--> statement-breakpoint
CREATE INDEX "external_contacts_id_idx" ON "external_contacts" USING btree ("id");--> statement-breakpoint
CREATE INDEX "external_contacts_categories_idx" ON "external_contacts" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "external_contacts_code_idx" ON "external_contacts" USING btree ("code");--> statement-breakpoint
CREATE INDEX "external_contacts_tombstone_idx" ON "external_contacts" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "external_contacts_status_idx" ON "external_contacts" USING btree ("status");--> statement-breakpoint
CREATE INDEX "external_contacts_previous_status_idx" ON "external_contacts" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "external_contacts_version_idx" ON "external_contacts" USING btree ("version");--> statement-breakpoint
CREATE INDEX "external_contacts_created_date_idx" ON "external_contacts" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "external_contacts_updated_date_idx" ON "external_contacts" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "external_contacts_organization_id_idx" ON "external_contacts" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "external_contacts_created_by_idx" ON "external_contacts" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "external_contacts_updated_by_idx" ON "external_contacts" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "external_contacts_deleted_by_idx" ON "external_contacts" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "external_contacts_requested_by_idx" ON "external_contacts" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "external_contacts_tags_idx" ON "external_contacts" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "external_contacts_image_url_idx" ON "external_contacts" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "external_contacts_sensitivity_level_idx" ON "external_contacts" USING btree ("sensitivity_level");--> statement-breakpoint
CREATE INDEX "account_organizations_id_idx" ON "account_organizations" USING btree ("id");--> statement-breakpoint
CREATE INDEX "account_organizations_categories_idx" ON "account_organizations" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "account_organizations_code_idx" ON "account_organizations" USING btree ("code");--> statement-breakpoint
CREATE INDEX "account_organizations_tombstone_idx" ON "account_organizations" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "account_organizations_status_idx" ON "account_organizations" USING btree ("status");--> statement-breakpoint
CREATE INDEX "account_organizations_previous_status_idx" ON "account_organizations" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "account_organizations_version_idx" ON "account_organizations" USING btree ("version");--> statement-breakpoint
CREATE INDEX "account_organizations_created_date_idx" ON "account_organizations" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "account_organizations_updated_date_idx" ON "account_organizations" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "account_organizations_organization_id_idx" ON "account_organizations" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "account_organizations_created_by_idx" ON "account_organizations" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "account_organizations_updated_by_idx" ON "account_organizations" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "account_organizations_deleted_by_idx" ON "account_organizations" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "account_organizations_requested_by_idx" ON "account_organizations" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "account_organizations_tags_idx" ON "account_organizations" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "account_organizations_image_url_idx" ON "account_organizations" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "account_organizations_sensitivity_level_idx" ON "account_organizations" USING btree ("sensitivity_level");--> statement-breakpoint
CREATE INDEX "account_profiles_id_idx" ON "account_profiles" USING btree ("id");--> statement-breakpoint
CREATE INDEX "account_profiles_categories_idx" ON "account_profiles" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "account_profiles_code_idx" ON "account_profiles" USING btree ("code");--> statement-breakpoint
CREATE INDEX "account_profiles_tombstone_idx" ON "account_profiles" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "account_profiles_status_idx" ON "account_profiles" USING btree ("status");--> statement-breakpoint
CREATE INDEX "account_profiles_previous_status_idx" ON "account_profiles" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "account_profiles_version_idx" ON "account_profiles" USING btree ("version");--> statement-breakpoint
CREATE INDEX "account_profiles_created_date_idx" ON "account_profiles" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "account_profiles_updated_date_idx" ON "account_profiles" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "account_profiles_organization_id_idx" ON "account_profiles" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "account_profiles_created_by_idx" ON "account_profiles" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "account_profiles_updated_by_idx" ON "account_profiles" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "account_profiles_deleted_by_idx" ON "account_profiles" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "account_profiles_requested_by_idx" ON "account_profiles" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "account_profiles_tags_idx" ON "account_profiles" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "account_profiles_image_url_idx" ON "account_profiles" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "account_profiles_sensitivity_level_idx" ON "account_profiles" USING btree ("sensitivity_level");--> statement-breakpoint
CREATE INDEX "accounts_id_idx" ON "accounts" USING btree ("id");--> statement-breakpoint
CREATE INDEX "accounts_categories_idx" ON "accounts" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "accounts_code_idx" ON "accounts" USING btree ("code");--> statement-breakpoint
CREATE INDEX "accounts_tombstone_idx" ON "accounts" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "accounts_status_idx" ON "accounts" USING btree ("status");--> statement-breakpoint
CREATE INDEX "accounts_previous_status_idx" ON "accounts" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "accounts_version_idx" ON "accounts" USING btree ("version");--> statement-breakpoint
CREATE INDEX "accounts_created_date_idx" ON "accounts" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "accounts_updated_date_idx" ON "accounts" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "accounts_organization_id_idx" ON "accounts" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "accounts_created_by_idx" ON "accounts" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "accounts_updated_by_idx" ON "accounts" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "accounts_deleted_by_idx" ON "accounts" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "accounts_requested_by_idx" ON "accounts" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "accounts_tags_idx" ON "accounts" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "accounts_image_url_idx" ON "accounts" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "accounts_sensitivity_level_idx" ON "accounts" USING btree ("sensitivity_level");--> statement-breakpoint
CREATE INDEX "addresses_id_idx" ON "addresses" USING btree ("id");--> statement-breakpoint
CREATE INDEX "addresses_categories_idx" ON "addresses" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "addresses_code_idx" ON "addresses" USING btree ("code");--> statement-breakpoint
CREATE INDEX "addresses_tombstone_idx" ON "addresses" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "addresses_status_idx" ON "addresses" USING btree ("status");--> statement-breakpoint
CREATE INDEX "addresses_previous_status_idx" ON "addresses" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "addresses_version_idx" ON "addresses" USING btree ("version");--> statement-breakpoint
CREATE INDEX "addresses_created_date_idx" ON "addresses" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "addresses_updated_date_idx" ON "addresses" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "addresses_organization_id_idx" ON "addresses" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "addresses_created_by_idx" ON "addresses" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "addresses_updated_by_idx" ON "addresses" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "addresses_deleted_by_idx" ON "addresses" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "addresses_requested_by_idx" ON "addresses" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "addresses_tags_idx" ON "addresses" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "addresses_image_url_idx" ON "addresses" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "addresses_sensitivity_level_idx" ON "addresses" USING btree ("sensitivity_level");--> statement-breakpoint
CREATE INDEX "addresses_address_idx" ON "addresses" USING btree ("address");--> statement-breakpoint
CREATE INDEX "addresses_address_line_one_idx" ON "addresses" USING btree ("address_line_one");--> statement-breakpoint
CREATE INDEX "addresses_address_line_two_idx" ON "addresses" USING btree ("address_line_two");--> statement-breakpoint
CREATE INDEX "addresses_latitude_idx" ON "addresses" USING btree ("latitude");--> statement-breakpoint
CREATE INDEX "addresses_longitude_idx" ON "addresses" USING btree ("longitude");--> statement-breakpoint
CREATE INDEX "addresses_place_id_idx" ON "addresses" USING btree ("place_id");--> statement-breakpoint
CREATE INDEX "addresses_street_number_idx" ON "addresses" USING btree ("street_number");--> statement-breakpoint
CREATE INDEX "addresses_street_idx" ON "addresses" USING btree ("street");--> statement-breakpoint
CREATE INDEX "addresses_region_idx" ON "addresses" USING btree ("region");--> statement-breakpoint
CREATE INDEX "addresses_region_code_idx" ON "addresses" USING btree ("region_code");--> statement-breakpoint
CREATE INDEX "addresses_country_code_idx" ON "addresses" USING btree ("country_code");--> statement-breakpoint
CREATE INDEX "addresses_postal_code_idx" ON "addresses" USING btree ("postal_code");--> statement-breakpoint
CREATE INDEX "addresses_country_idx" ON "addresses" USING btree ("country");--> statement-breakpoint
CREATE INDEX "addresses_state_idx" ON "addresses" USING btree ("state");--> statement-breakpoint
CREATE INDEX "addresses_city_idx" ON "addresses" USING btree ("city");--> statement-breakpoint
CREATE INDEX "user_roles_id_idx" ON "user_roles" USING btree ("id");--> statement-breakpoint
CREATE INDEX "user_roles_categories_idx" ON "user_roles" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "user_roles_code_idx" ON "user_roles" USING btree ("code");--> statement-breakpoint
CREATE INDEX "user_roles_tombstone_idx" ON "user_roles" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "user_roles_status_idx" ON "user_roles" USING btree ("status");--> statement-breakpoint
CREATE INDEX "user_roles_previous_status_idx" ON "user_roles" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "user_roles_version_idx" ON "user_roles" USING btree ("version");--> statement-breakpoint
CREATE INDEX "user_roles_created_date_idx" ON "user_roles" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "user_roles_updated_date_idx" ON "user_roles" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "user_roles_organization_id_idx" ON "user_roles" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "user_roles_created_by_idx" ON "user_roles" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "user_roles_updated_by_idx" ON "user_roles" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "user_roles_deleted_by_idx" ON "user_roles" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "user_roles_requested_by_idx" ON "user_roles" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "user_roles_tags_idx" ON "user_roles" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "user_roles_image_url_idx" ON "user_roles" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "user_roles_sensitivity_level_idx" ON "user_roles" USING btree ("sensitivity_level");--> statement-breakpoint
CREATE INDEX "user_roles_entity_idx" ON "user_roles" USING btree ("entity");--> statement-breakpoint
CREATE INDEX "user_roles_role_idx" ON "user_roles" USING btree ("role");--> statement-breakpoint
CREATE INDEX "entities_id_idx" ON "entities" USING btree ("id");--> statement-breakpoint
CREATE INDEX "entities_name_idx" ON "entities" USING btree ("name");--> statement-breakpoint
CREATE INDEX "entities_organization_id_idx" ON "entities" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "entities_created_by_idx" ON "entities" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "entities_updated_by_idx" ON "entities" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "entities_deleted_by_idx" ON "entities" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "entities_tombstone_idx" ON "entities" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "fields_id_idx" ON "fields" USING btree ("id");--> statement-breakpoint
CREATE INDEX "fields_label_idx" ON "fields" USING btree ("label");--> statement-breakpoint
CREATE INDEX "fields_name_idx" ON "fields" USING btree ("name");--> statement-breakpoint
CREATE INDEX "fields_field_type_idx" ON "fields" USING btree ("field_type");--> statement-breakpoint
CREATE INDEX "fields_constraints_idx" ON "fields" USING btree ("constraints");--> statement-breakpoint
CREATE INDEX "fields__default_idx" ON "fields" USING btree ("_default");--> statement-breakpoint
CREATE INDEX "fields_reference_to_idx" ON "fields" USING btree ("reference_to");--> statement-breakpoint
CREATE INDEX "fields_created_by_idx" ON "fields" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "fields_updated_by_idx" ON "fields" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "fields_deleted_by_idx" ON "fields" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "fields_tombstone_idx" ON "fields" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "entity_fields_id_idx" ON "entity_fields" USING btree ("id");--> statement-breakpoint
CREATE INDEX "entity_fields_entity_id_idx" ON "entity_fields" USING btree ("entity_id");--> statement-breakpoint
CREATE INDEX "entity_fields_field_id_idx" ON "entity_fields" USING btree ("field_id");--> statement-breakpoint
CREATE INDEX "entity_fields_sensitivity_level_idx" ON "entity_fields" USING btree ("sensitivity_level");--> statement-breakpoint
CREATE INDEX "entity_fields_is_encryptable_idx" ON "entity_fields" USING btree ("is_encryptable");--> statement-breakpoint
CREATE INDEX "entity_fields_schema_version_idx" ON "entity_fields" USING btree ("schema_version");--> statement-breakpoint
CREATE INDEX "entity_fields_created_by_idx" ON "entity_fields" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "entity_fields_updated_by_idx" ON "entity_fields" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "entity_fields_deleted_by_idx" ON "entity_fields" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "entity_fields_tombstone_idx" ON "entity_fields" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "permissions_id_idx" ON "permissions" USING btree ("id");--> statement-breakpoint
CREATE INDEX "permissions_read_idx" ON "permissions" USING btree ("read");--> statement-breakpoint
CREATE INDEX "permissions_write_idx" ON "permissions" USING btree ("write");--> statement-breakpoint
CREATE INDEX "permissions_encrypt_idx" ON "permissions" USING btree ("encrypt");--> statement-breakpoint
CREATE INDEX "permissions_decrypt_idx" ON "permissions" USING btree ("decrypt");--> statement-breakpoint
CREATE INDEX "permissions_required_idx" ON "permissions" USING btree ("required");--> statement-breakpoint
CREATE INDEX "permissions_sensitive_idx" ON "permissions" USING btree ("sensitive");--> statement-breakpoint
CREATE INDEX "permissions_archive_idx" ON "permissions" USING btree ("archive");--> statement-breakpoint
CREATE INDEX "permissions_delete_idx" ON "permissions" USING btree ("delete");--> statement-breakpoint
CREATE INDEX "permissions_created_by_idx" ON "permissions" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "permissions_updated_by_idx" ON "permissions" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "permissions_deleted_by_idx" ON "permissions" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "permissions_tombstone_idx" ON "permissions" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "data_permissions_id_idx" ON "data_permissions" USING btree ("id");--> statement-breakpoint
CREATE INDEX "data_permissions_entity_field_id_idx" ON "data_permissions" USING btree ("entity_field_id");--> statement-breakpoint
CREATE INDEX "data_permissions_permission_id_idx" ON "data_permissions" USING btree ("permission_id");--> statement-breakpoint
CREATE INDEX "data_permissions_record_permission_id_idx" ON "data_permissions" USING btree ("record_permission_id");--> statement-breakpoint
CREATE INDEX "data_permissions_role_permission_id_idx" ON "data_permissions" USING btree ("role_permission_id");--> statement-breakpoint
CREATE INDEX "data_permissions_account_organization_id_idx" ON "data_permissions" USING btree ("account_organization_id");--> statement-breakpoint
CREATE INDEX "data_permissions_created_by_idx" ON "data_permissions" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "data_permissions_updated_by_idx" ON "data_permissions" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "data_permissions_deleted_by_idx" ON "data_permissions" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "data_permissions_tombstone_idx" ON "data_permissions" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "data_permissions_sensitivity_level_idx" ON "data_permissions" USING btree ("sensitivity_level");--> statement-breakpoint
CREATE INDEX "role_permissions_id_idx" ON "role_permissions" USING btree ("id");--> statement-breakpoint
CREATE INDEX "role_permissions_role_id_idx" ON "role_permissions" USING btree ("role_id");--> statement-breakpoint
CREATE INDEX "role_permissions_permission_id_idx" ON "role_permissions" USING btree ("permission_id");--> statement-breakpoint
CREATE INDEX "role_permissions_created_by_idx" ON "role_permissions" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "role_permissions_updated_by_idx" ON "role_permissions" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "role_permissions_deleted_by_idx" ON "role_permissions" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "role_permissions_tombstone_idx" ON "role_permissions" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "record_permissions_id_idx" ON "record_permissions" USING btree ("id");--> statement-breakpoint
CREATE INDEX "record_permissions_record_id_idx" ON "record_permissions" USING btree ("record_id");--> statement-breakpoint
CREATE INDEX "record_permissions_record_entity_idx" ON "record_permissions" USING btree ("record_entity");--> statement-breakpoint
CREATE INDEX "record_permissions_permission_id_idx" ON "record_permissions" USING btree ("permission_id");--> statement-breakpoint
CREATE INDEX "record_permissions_created_by_idx" ON "record_permissions" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "record_permissions_updated_by_idx" ON "record_permissions" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "record_permissions_deleted_by_idx" ON "record_permissions" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "record_permissions_tombstone_idx" ON "record_permissions" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "encryption_keys_id_idx" ON "encryption_keys" USING btree ("id");--> statement-breakpoint
CREATE INDEX "encryption_keys_organization_id_idx" ON "encryption_keys" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "encryption_keys_entity_idx" ON "encryption_keys" USING btree ("entity");--> statement-breakpoint
CREATE INDEX "encryption_keys_created_by_idx" ON "encryption_keys" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "encryption_keys_tombstone_idx" ON "encryption_keys" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "sessions_sid_idx" ON "sessions" USING btree ("sid");--> statement-breakpoint
CREATE INDEX "samples_id_idx" ON "samples" USING btree ("id");--> statement-breakpoint
CREATE INDEX "samples_categories_idx" ON "samples" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "samples_code_idx" ON "samples" USING btree ("code");--> statement-breakpoint
CREATE INDEX "samples_tombstone_idx" ON "samples" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "samples_status_idx" ON "samples" USING btree ("status");--> statement-breakpoint
CREATE INDEX "samples_previous_status_idx" ON "samples" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "samples_version_idx" ON "samples" USING btree ("version");--> statement-breakpoint
CREATE INDEX "samples_created_date_idx" ON "samples" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "samples_updated_date_idx" ON "samples" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "samples_organization_id_idx" ON "samples" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "samples_created_by_idx" ON "samples" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "samples_updated_by_idx" ON "samples" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "samples_deleted_by_idx" ON "samples" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "samples_requested_by_idx" ON "samples" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "samples_sensitivity_level_idx" ON "samples" USING btree ("sensitivity_level");--> statement-breakpoint
CREATE INDEX "devices_id_idx" ON "devices" USING btree ("id");--> statement-breakpoint
CREATE INDEX "devices_categories_idx" ON "devices" USING btree ("categories");--> statement-breakpoint
CREATE INDEX "devices_code_idx" ON "devices" USING btree ("code");--> statement-breakpoint
CREATE INDEX "devices_tombstone_idx" ON "devices" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "devices_status_idx" ON "devices" USING btree ("status");--> statement-breakpoint
CREATE INDEX "devices_previous_status_idx" ON "devices" USING btree ("previous_status");--> statement-breakpoint
CREATE INDEX "devices_version_idx" ON "devices" USING btree ("version");--> statement-breakpoint
CREATE INDEX "devices_created_date_idx" ON "devices" USING btree ("created_date");--> statement-breakpoint
CREATE INDEX "devices_updated_date_idx" ON "devices" USING btree ("updated_date");--> statement-breakpoint
CREATE INDEX "devices_organization_id_idx" ON "devices" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "devices_created_by_idx" ON "devices" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "devices_updated_by_idx" ON "devices" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "devices_deleted_by_idx" ON "devices" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "devices_requested_by_idx" ON "devices" USING btree ("requested_by");--> statement-breakpoint
CREATE INDEX "devices_tags_idx" ON "devices" USING btree ("tags");--> statement-breakpoint
CREATE INDEX "devices_image_url_idx" ON "devices" USING btree ("image_url");--> statement-breakpoint
CREATE INDEX "system_config_fields_field_id_idx" ON "system_config_fields" USING btree ("field_id");--> statement-breakpoint
CREATE INDEX "system_config_fields_is_searchable_idx" ON "system_config_fields" USING btree ("is_searchable");--> statement-breakpoint
CREATE INDEX "system_config_fields_is_system_field_idx" ON "system_config_fields" USING btree ("is_system_field");--> statement-breakpoint
CREATE INDEX "system_config_fields_is_encryptable_idx" ON "system_config_fields" USING btree ("is_encryptable");--> statement-breakpoint
CREATE INDEX "system_config_fields_is_allowed_to_return_idx" ON "system_config_fields" USING btree ("is_allowed_to_return");--> statement-breakpoint
CREATE INDEX "system_config_fields_created_by_idx" ON "system_config_fields" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "system_config_fields_updated_by_idx" ON "system_config_fields" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "system_config_fields_deleted_by_idx" ON "system_config_fields" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "system_config_fields_tombstone_idx" ON "system_config_fields" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "table_indexes_id_idx" ON "table_indexes" USING btree ("id");--> statement-breakpoint
CREATE INDEX "table_indexes_entity_id_idx" ON "table_indexes" USING btree ("entity_id");--> statement-breakpoint
CREATE INDEX "table_indexes_secondary_index_idx" ON "table_indexes" USING btree ("secondary_index");--> statement-breakpoint
CREATE INDEX "table_indexes_compound_index_idx" ON "table_indexes" USING btree ("compound_index");--> statement-breakpoint
CREATE INDEX "table_indexes_created_by_idx" ON "table_indexes" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "table_indexes_updated_by_idx" ON "table_indexes" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "table_indexes_deleted_by_idx" ON "table_indexes" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "table_indexes_tombstone_idx" ON "table_indexes" USING btree ("tombstone");--> statement-breakpoint

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


ALTER TABLE "communication_templates" ADD CONSTRAINT "communication_templates_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "communication_templates" ADD CONSTRAINT "communication_templates_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "communication_templates" ADD CONSTRAINT "communication_templates_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "communication_templates" ADD CONSTRAINT "communication_templates_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "communication_templates" ADD CONSTRAINT "communication_templates_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "grid_filters" ADD CONSTRAINT "grid_filters_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "grid_filters" ADD CONSTRAINT "grid_filters_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "grid_filters" ADD CONSTRAINT "grid_filters_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "grid_filters" ADD CONSTRAINT "grid_filters_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "grid_filters" ADD CONSTRAINT "grid_filters_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "grid_filters" ADD CONSTRAINT "grid_filters_contact_id_contacts_id_fk" FOREIGN KEY ("contact_id") REFERENCES "public"."contacts"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "invitations" ADD CONSTRAINT "invitations_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "invitations" ADD CONSTRAINT "invitations_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "invitations" ADD CONSTRAINT "invitations_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "invitations" ADD CONSTRAINT "invitations_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "invitations" ADD CONSTRAINT "invitations_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "invitations" ADD CONSTRAINT "invitations_account_id_organization_accounts_id_fk" FOREIGN KEY ("account_id") REFERENCES "public"."organization_accounts"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "invitations" ADD CONSTRAINT "invitations_account_organization_id_account_organizations_id_fk" FOREIGN KEY ("account_organization_id") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint

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

ALTER TABLE "postgres_channels" ADD CONSTRAINT "postgres_channels_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "postgres_channels" ADD CONSTRAINT "postgres_channels_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "postgres_channels" ADD CONSTRAINT "postgres_channels_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "postgres_channels" ADD CONSTRAINT "postgres_channels_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "postgres_channels" ADD CONSTRAINT "postgres_channels_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint

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
CREATE INDEX "allowed_fields_sensitivity_level_idx" ON "allowed_fields" USING btree ("sensitivity_level");--> statement-breakpoint
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
CREATE INDEX "class_types_sensitivity_level_idx" ON "class_types" USING btree ("sensitivity_level");--> statement-breakpoint
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
CREATE INDEX "postgres_channels_sensitivity_level_idx" ON "postgres_channels" USING btree ("sensitivity_level");--> statement-breakpoint
--> statement-breakpoint
-- CRDT Messages Performance Indexes
-- Individual indexes for each column
CREATE INDEX IF NOT EXISTS idx_crdt_messages_dataset ON crdt_messages("dataset");
--> statement-breakpoint
CREATE INDEX IF NOT EXISTS idx_crdt_messages_column ON crdt_messages("column");
--> statement-breakpoint
CREATE INDEX IF NOT EXISTS idx_crdt_messages_row ON crdt_messages("row");
--> statement-breakpoint
CREATE INDEX IF NOT EXISTS idx_crdt_messages_timestamp ON crdt_messages("timestamp");
--> statement-breakpoint
-- Composite index for the common query pattern (dataset, column, row)
-- This will significantly speed up the find_existing_messages function
CREATE INDEX IF NOT EXISTS idx_crdt_messages_dataset_column_row ON crdt_messages("dataset", "column", "row");
--> statement-breakpoint
-- Composite index including timestamp for ordering
CREATE INDEX IF NOT EXISTS idx_crdt_messages_dataset_column_row_timestamp ON crdt_messages("dataset", "column", "row", "timestamp" DESC);
--> statement-breakpoint
-- Optional: Index for timestamp-based queries (get_messages_since)
CREATE INDEX IF NOT EXISTS idx_crdt_messages_timestamp_desc ON crdt_messages("timestamp" DESC);



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



ALTER TABLE "devices" 
ADD COLUMN "sync_status" text DEFAULT 'in-process';
ALTER TABLE "devices" 
ADD COLUMN "is_batch" boolean DEFAULT false;

ALTER TABLE "postgres_channels" 
ADD COLUMN "sync_status" text DEFAULT 'in-process';
ALTER TABLE "postgres_channels" 
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