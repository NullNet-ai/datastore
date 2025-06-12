-- Your SQL goes here
CREATE TABLE "entities" (
	"id" text PRIMARY KEY NOT NULL,
	"name" text NOT NULL,
	"organization_id" text,
	"version" serial NOT NULL,
	"created_by" text,
	"updated_by" text,
	"deleted_by" text,
	"timestamp" text,
	"tombstone" integer DEFAULT 0
);
--> statement-breakpoint
CREATE TABLE "entity_fields" (
	"id" text PRIMARY KEY NOT NULL,
	"entity_id" text NOT NULL,
	"field_id" text NOT NULL,
	"version" integer DEFAULT 1,
	"created_by" text,
	"updated_by" text,
	"deleted_by" text,
	"timestamp" text,
	"tombstone" integer DEFAULT 0
);
--> statement-breakpoint
CREATE TABLE "permissions" (
	"id" text PRIMARY KEY NOT NULL,
	"record_id" text,
	"record_entity" text,
	"read" boolean DEFAULT false,
	"write" boolean DEFAULT false,
	"encrypt" boolean DEFAULT false,
	"decrypt" boolean DEFAULT false,
	"required" boolean DEFAULT false,
	"sensitive" boolean DEFAULT false,
	"archive" boolean DEFAULT false,
	"delete" boolean DEFAULT false,
	"version" serial NOT NULL,
	"created_by" text,
	"updated_by" text,
	"deleted_by" text,
	"timestamp" text,
	"tombstone" integer DEFAULT 0
);
--> statement-breakpoint
CREATE TABLE "encryption_keys" (
	"id" text PRIMARY KEY NOT NULL,
	"organization_id" text NOT NULL,
	"entity" text NOT NULL,
	"created_by" text,
	"timestamp" text,
	"tombstone" integer DEFAULT 0
);
--> statement-breakpoint
CREATE TABLE "sessions" (
	"sid" text PRIMARY KEY NOT NULL,
	"sess" jsonb NOT NULL,
	"expire" timestamp NOT NULL
);


CREATE TABLE "data_permissions" (
	"id" text PRIMARY KEY NOT NULL,
	"entity_field_id" text NOT NULL,
	"permission_id" text NOT NULL,
	"inherited_permission_id" text,
	"account_organization_id" text NOT NULL,
	"version" serial NOT NULL,
	"created_by" text,
	"updated_by" text,
	"deleted_by" text,
	"timestamp" timestamp with time zone DEFAULT now() NOT NULL,
	"tombstone" integer DEFAULT 0
);


CREATE TABLE "role_permissions" (
	"id" text PRIMARY KEY NOT NULL,
	"role_name" text NOT NULL,
	"permission_id" text NOT NULL,
	"version" serial NOT NULL,
	"created_by" text,
	"updated_by" text,
	"deleted_by" text,
	"timestamp" timestamp with time zone DEFAULT now() NOT NULL,
	"tombstone" integer DEFAULT 0,
	CONSTRAINT "role_permissions_role_name_unique" UNIQUE("role_name")
);


ALTER TABLE "entity_fields" ADD CONSTRAINT "entity_fields_entity_id_entities_id_fk" FOREIGN KEY ("entity_id") REFERENCES "public"."entities"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "entity_fields" ADD CONSTRAINT "entity_fields_field_id_fields_id_fk" FOREIGN KEY ("field_id") REFERENCES "public"."fields"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "data_permissions" ADD CONSTRAINT "data_permissions_entity_field_id_entity_fields_id_fk" FOREIGN KEY ("entity_field_id") REFERENCES "public"."entity_fields"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "data_permissions" ADD CONSTRAINT "data_permissions_permission_id_permissions_id_fk" FOREIGN KEY ("permission_id") REFERENCES "public"."permissions"("id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "data_permissions" ADD CONSTRAINT "data_permissions_inherited_permission_id_permissions_id_fk" FOREIGN KEY ("inherited_permission_id") REFERENCES "public"."permissions"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "data_permissions" ADD CONSTRAINT "data_permissions_account_organization_id_account_organizations_id_fk" FOREIGN KEY ("account_organization_id") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "role_permissions" ADD CONSTRAINT "role_permissions_permission_id_permissions_id_fk" FOREIGN KEY ("permission_id") REFERENCES "public"."permissions"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint

CREATE INDEX "entities_id_idx" ON "entities" USING btree ("id");--> statement-breakpoint
CREATE INDEX "entities_name_idx" ON "entities" USING btree ("name");--> statement-breakpoint
CREATE INDEX "entities_organization_id_idx" ON "entities" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "entities_version_idx" ON "entities" USING btree ("version");--> statement-breakpoint
CREATE INDEX "entities_created_by_idx" ON "entities" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "entities_updated_by_idx" ON "entities" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "entities_deleted_by_idx" ON "entities" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "entities_tombstone_idx" ON "entities" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "entity_fields_id_idx" ON "entity_fields" USING btree ("id");--> statement-breakpoint
CREATE INDEX "entity_fields_entity_id_idx" ON "entity_fields" USING btree ("entity_id");--> statement-breakpoint
CREATE INDEX "entity_fields_field_id_idx" ON "entity_fields" USING btree ("field_id");--> statement-breakpoint
CREATE INDEX "entity_fields_field_version_idx" ON "entity_fields" USING btree ("version");--> statement-breakpoint
CREATE INDEX "entity_fields_schema_version_idx" ON "entity_fields" USING btree ("version");--> statement-breakpoint
CREATE INDEX "entity_fields_created_by_idx" ON "entity_fields" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "entity_fields_updated_by_idx" ON "entity_fields" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "entity_fields_deleted_by_idx" ON "entity_fields" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "entity_fields_tombstone_idx" ON "entity_fields" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "permissions_id_idx" ON "permissions" USING btree ("id");--> statement-breakpoint
CREATE INDEX "permissions_record_id_idx" ON "permissions" USING btree ("record_id");--> statement-breakpoint
CREATE INDEX "permissions_record_entity_idx" ON "permissions" USING btree ("record_entity");--> statement-breakpoint
CREATE INDEX "permissions_read_idx" ON "permissions" USING btree ("read");--> statement-breakpoint
CREATE INDEX "permissions_write_idx" ON "permissions" USING btree ("write");--> statement-breakpoint
CREATE INDEX "permissions_encrypt_idx" ON "permissions" USING btree ("encrypt");--> statement-breakpoint
CREATE INDEX "permissions_decrypt_idx" ON "permissions" USING btree ("decrypt");--> statement-breakpoint
CREATE INDEX "permissions_required_idx" ON "permissions" USING btree ("required");--> statement-breakpoint
CREATE INDEX "permissions_sensitive_idx" ON "permissions" USING btree ("sensitive");--> statement-breakpoint
CREATE INDEX "permissions_archive_idx" ON "permissions" USING btree ("archive");--> statement-breakpoint
CREATE INDEX "permissions_delete_idx" ON "permissions" USING btree ("delete");--> statement-breakpoint
CREATE INDEX "permissions_version_idx" ON "permissions" USING btree ("version");--> statement-breakpoint
CREATE INDEX "permissions_created_by_idx" ON "permissions" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "permissions_updated_by_idx" ON "permissions" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "permissions_deleted_by_idx" ON "permissions" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "permissions_tombstone_idx" ON "permissions" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "encryption_keys_id_idx" ON "encryption_keys" USING btree ("id");--> statement-breakpoint
CREATE INDEX "encryption_keys_organization_id_idx" ON "encryption_keys" USING btree ("organization_id");--> statement-breakpoint
CREATE INDEX "encryption_keys_entity_idx" ON "encryption_keys" USING btree ("entity");--> statement-breakpoint
CREATE INDEX "encryption_keys_created_by_idx" ON "encryption_keys" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "encryption_keys_tombstone_idx" ON "encryption_keys" USING btree ("tombstone");--> statement-breakpoint
CREATE INDEX "sessions_sid_idx" ON "sessions" USING btree ("sid");--> statement-breakpoint
CREATE INDEX "data_permissions_id_idx" ON "data_permissions" USING btree ("id");--> statement-breakpoint
CREATE INDEX "data_permissions_entity_field_id_idx" ON "data_permissions" USING btree ("entity_field_id");--> statement-breakpoint
CREATE INDEX "data_permissions_permission_id_idx" ON "data_permissions" USING btree ("permission_id");--> statement-breakpoint
CREATE INDEX "data_permissions_inherited_permission_id_idx" ON "data_permissions" USING btree ("inherited_permission_id");--> statement-breakpoint
CREATE INDEX "data_permissions_account_organization_id_idx" ON "data_permissions" USING btree ("account_organization_id");--> statement-breakpoint
CREATE INDEX "data_permissions_version_idx" ON "data_permissions" USING btree ("version");--> statement-breakpoint
CREATE INDEX "data_permissions_created_by_idx" ON "data_permissions" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "data_permissions_updated_by_idx" ON "data_permissions" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "data_permissions_deleted_by_idx" ON "data_permissions" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "data_permissions_tombstone_idx" ON "data_permissions" USING btree ("tombstone");
CREATE INDEX "role_permissions_id_idx" ON "role_permissions" USING btree ("id");--> statement-breakpoint
CREATE INDEX "role_permissions_role_name_idx" ON "role_permissions" USING btree ("role_name");--> statement-breakpoint
CREATE INDEX "role_permissions_permission_id_idx" ON "role_permissions" USING btree ("permission_id");--> statement-breakpoint
CREATE INDEX "role_permissions_version_idx" ON "role_permissions" USING btree ("version");--> statement-breakpoint
CREATE INDEX "role_permissions_created_by_idx" ON "role_permissions" USING btree ("created_by");--> statement-breakpoint
CREATE INDEX "role_permissions_updated_by_idx" ON "role_permissions" USING btree ("updated_by");--> statement-breakpoint
CREATE INDEX "role_permissions_deleted_by_idx" ON "role_permissions" USING btree ("deleted_by");--> statement-breakpoint
CREATE INDEX "role_permissions_tombstone_idx" ON "role_permissions" USING btree ("tombstone");
