CREATE TABLE `contact_emails` (
	`id` text PRIMARY KEY NOT NULL,
	`code` text,
	`tombstone` integer DEFAULT 0,
	`status` text DEFAULT 'Active',
	`version` integer DEFAULT 1,
	`created_date` text,
	`created_time` text,
	`updated_date` text,
	`updated_time` text,
	`organization_id` text,
	`created_by` text,
	`updated_by` text,
	`deleted_by` text,
	`requested_by` text,
	`timestamp` text,
	`categories` text DEFAULT (json_array()),
	`contact_id` text,
	`email` text,
	FOREIGN KEY (`contact_id`) REFERENCES `contacts`(`id`) ON UPDATE no action ON DELETE no action
);
--> statement-breakpoint
CREATE INDEX `contact_emails_id_idx` ON `contact_emails` (`id`);--> statement-breakpoint
CREATE INDEX `contact_emails_code_idx` ON `contact_emails` (`code`);--> statement-breakpoint
CREATE INDEX `contact_emails_tombstone_idx` ON `contact_emails` (`tombstone`);--> statement-breakpoint
CREATE INDEX `contact_emails_status_idx` ON `contact_emails` (`status`);--> statement-breakpoint
CREATE INDEX `contact_emails_version_idx` ON `contact_emails` (`version`);--> statement-breakpoint
CREATE INDEX `contact_emails_created_date_idx` ON `contact_emails` (`created_date`);--> statement-breakpoint
CREATE INDEX `contact_emails_updated_date_idx` ON `contact_emails` (`updated_date`);--> statement-breakpoint
CREATE INDEX `contact_emails_organization_id_idx` ON `contact_emails` (`organization_id`);--> statement-breakpoint
CREATE INDEX `contact_emails_created_by_idx` ON `contact_emails` (`created_by`);--> statement-breakpoint
CREATE INDEX `contact_emails_updated_by_idx` ON `contact_emails` (`updated_by`);--> statement-breakpoint
CREATE INDEX `contact_emails_deleted_by_idx` ON `contact_emails` (`deleted_by`);--> statement-breakpoint
CREATE INDEX `contact_emails_requested_by_idx` ON `contact_emails` (`requested_by`);--> statement-breakpoint
CREATE TABLE `contact_phone_numbers` (
	`id` text PRIMARY KEY NOT NULL,
	`code` text,
	`tombstone` integer DEFAULT 0,
	`status` text DEFAULT 'Active',
	`version` integer DEFAULT 1,
	`created_date` text,
	`created_time` text,
	`updated_date` text,
	`updated_time` text,
	`organization_id` text,
	`created_by` text,
	`updated_by` text,
	`deleted_by` text,
	`requested_by` text,
	`timestamp` text,
	`categories` text DEFAULT (json_array()),
	`contact_id` text,
	`phone_number_raw` text,
	FOREIGN KEY (`contact_id`) REFERENCES `contacts`(`id`) ON UPDATE no action ON DELETE no action
);
--> statement-breakpoint
CREATE INDEX `contact_phone_numbers_id_idx` ON `contact_phone_numbers` (`id`);--> statement-breakpoint
CREATE INDEX `contact_phone_numbers_code_idx` ON `contact_phone_numbers` (`code`);--> statement-breakpoint
CREATE INDEX `contact_phone_numbers_tombstone_idx` ON `contact_phone_numbers` (`tombstone`);--> statement-breakpoint
CREATE INDEX `contact_phone_numbers_status_idx` ON `contact_phone_numbers` (`status`);--> statement-breakpoint
CREATE INDEX `contact_phone_numbers_version_idx` ON `contact_phone_numbers` (`version`);--> statement-breakpoint
CREATE INDEX `contact_phone_numbers_created_date_idx` ON `contact_phone_numbers` (`created_date`);--> statement-breakpoint
CREATE INDEX `contact_phone_numbers_updated_date_idx` ON `contact_phone_numbers` (`updated_date`);--> statement-breakpoint
CREATE INDEX `contact_phone_numbers_organization_id_idx` ON `contact_phone_numbers` (`organization_id`);--> statement-breakpoint
CREATE INDEX `contact_phone_numbers_created_by_idx` ON `contact_phone_numbers` (`created_by`);--> statement-breakpoint
CREATE INDEX `contact_phone_numbers_updated_by_idx` ON `contact_phone_numbers` (`updated_by`);--> statement-breakpoint
CREATE INDEX `contact_phone_numbers_deleted_by_idx` ON `contact_phone_numbers` (`deleted_by`);--> statement-breakpoint
CREATE INDEX `contact_phone_numbers_requested_by_idx` ON `contact_phone_numbers` (`requested_by`);--> statement-breakpoint
CREATE TABLE `contacts` (
	`id` text PRIMARY KEY NOT NULL,
	`code` text,
	`tombstone` integer DEFAULT 0,
	`status` text DEFAULT 'Active',
	`version` integer DEFAULT 1,
	`created_date` text,
	`created_time` text,
	`updated_date` text,
	`updated_time` text,
	`organization_id` text,
	`created_by` text,
	`updated_by` text,
	`deleted_by` text,
	`requested_by` text,
	`timestamp` text,
	`categories` text DEFAULT (json_array()),
	`first_name` text,
	`middle_name` text,
	`last_name` text,
	`date_of_birth` text
);
--> statement-breakpoint
CREATE INDEX `contacts_id_idx` ON `contacts` (`id`);--> statement-breakpoint
CREATE INDEX `contacts_code_idx` ON `contacts` (`code`);--> statement-breakpoint
CREATE INDEX `contacts_tombstone_idx` ON `contacts` (`tombstone`);--> statement-breakpoint
CREATE INDEX `contacts_status_idx` ON `contacts` (`status`);--> statement-breakpoint
CREATE INDEX `contacts_version_idx` ON `contacts` (`version`);--> statement-breakpoint
CREATE INDEX `contacts_created_date_idx` ON `contacts` (`created_date`);--> statement-breakpoint
CREATE INDEX `contacts_updated_date_idx` ON `contacts` (`updated_date`);--> statement-breakpoint
CREATE INDEX `contacts_organization_id_idx` ON `contacts` (`organization_id`);--> statement-breakpoint
CREATE INDEX `contacts_created_by_idx` ON `contacts` (`created_by`);--> statement-breakpoint
CREATE INDEX `contacts_updated_by_idx` ON `contacts` (`updated_by`);--> statement-breakpoint
CREATE INDEX `contacts_deleted_by_idx` ON `contacts` (`deleted_by`);--> statement-breakpoint
CREATE INDEX `contacts_requested_by_idx` ON `contacts` (`requested_by`);--> statement-breakpoint
CREATE TABLE `organization_contact_accounts` (
	`id` text PRIMARY KEY NOT NULL,
	`code` text,
	`tombstone` integer DEFAULT 0,
	`status` text DEFAULT 'Active',
	`version` integer DEFAULT 1,
	`created_date` text,
	`created_time` text,
	`updated_date` text,
	`updated_time` text,
	`organization_id` text,
	`created_by` text,
	`updated_by` text,
	`deleted_by` text,
	`requested_by` text,
	`timestamp` text,
	`categories` text DEFAULT (json_array()),
	`organization_contact_id` text,
	`email` text,
	`password` text,
	FOREIGN KEY (`organization_id`) REFERENCES `organizations`(`id`) ON UPDATE no action ON DELETE no action,
	FOREIGN KEY (`organization_contact_id`) REFERENCES `organization_contacts`(`id`) ON UPDATE no action ON DELETE no action
);
--> statement-breakpoint
CREATE INDEX `organization_contact_accounts_id_idx` ON `organization_contact_accounts` (`id`);--> statement-breakpoint
CREATE INDEX `organization_contact_accounts_code_idx` ON `organization_contact_accounts` (`code`);--> statement-breakpoint
CREATE INDEX `organization_contact_accounts_tombstone_idx` ON `organization_contact_accounts` (`tombstone`);--> statement-breakpoint
CREATE INDEX `organization_contact_accounts_status_idx` ON `organization_contact_accounts` (`status`);--> statement-breakpoint
CREATE INDEX `organization_contact_accounts_version_idx` ON `organization_contact_accounts` (`version`);--> statement-breakpoint
CREATE INDEX `organization_contact_accounts_created_date_idx` ON `organization_contact_accounts` (`created_date`);--> statement-breakpoint
CREATE INDEX `organization_contact_accounts_updated_date_idx` ON `organization_contact_accounts` (`updated_date`);--> statement-breakpoint
CREATE INDEX `organization_contact_accounts_organization_id_idx` ON `organization_contact_accounts` (`organization_id`);--> statement-breakpoint
CREATE INDEX `organization_contact_accounts_created_by_idx` ON `organization_contact_accounts` (`created_by`);--> statement-breakpoint
CREATE INDEX `organization_contact_accounts_updated_by_idx` ON `organization_contact_accounts` (`updated_by`);--> statement-breakpoint
CREATE INDEX `organization_contact_accounts_deleted_by_idx` ON `organization_contact_accounts` (`deleted_by`);--> statement-breakpoint
CREATE INDEX `organization_contact_accounts_requested_by_idx` ON `organization_contact_accounts` (`requested_by`);--> statement-breakpoint
CREATE TABLE `organization_contacts` (
	`id` text PRIMARY KEY NOT NULL,
	`code` text,
	`tombstone` integer DEFAULT 0,
	`status` text DEFAULT 'Active',
	`version` integer DEFAULT 1,
	`created_date` text,
	`created_time` text,
	`updated_date` text,
	`updated_time` text,
	`organization_id` text,
	`created_by` text,
	`updated_by` text,
	`deleted_by` text,
	`requested_by` text,
	`timestamp` text,
	`categories` text DEFAULT (json_array()),
	`contact_id` text,
	FOREIGN KEY (`organization_id`) REFERENCES `organizations`(`id`) ON UPDATE no action ON DELETE no action,
	FOREIGN KEY (`contact_id`) REFERENCES `contacts`(`id`) ON UPDATE no action ON DELETE no action
);
--> statement-breakpoint
CREATE INDEX `organization_contacts_id_idx` ON `organization_contacts` (`id`);--> statement-breakpoint
CREATE INDEX `organization_contacts_code_idx` ON `organization_contacts` (`code`);--> statement-breakpoint
CREATE INDEX `organization_contacts_tombstone_idx` ON `organization_contacts` (`tombstone`);--> statement-breakpoint
CREATE INDEX `organization_contacts_status_idx` ON `organization_contacts` (`status`);--> statement-breakpoint
CREATE INDEX `organization_contacts_version_idx` ON `organization_contacts` (`version`);--> statement-breakpoint
CREATE INDEX `organization_contacts_created_date_idx` ON `organization_contacts` (`created_date`);--> statement-breakpoint
CREATE INDEX `organization_contacts_updated_date_idx` ON `organization_contacts` (`updated_date`);--> statement-breakpoint
CREATE INDEX `organization_contacts_organization_id_idx` ON `organization_contacts` (`organization_id`);--> statement-breakpoint
CREATE INDEX `organization_contacts_created_by_idx` ON `organization_contacts` (`created_by`);--> statement-breakpoint
CREATE INDEX `organization_contacts_updated_by_idx` ON `organization_contacts` (`updated_by`);--> statement-breakpoint
CREATE INDEX `organization_contacts_deleted_by_idx` ON `organization_contacts` (`deleted_by`);--> statement-breakpoint
CREATE INDEX `organization_contacts_requested_by_idx` ON `organization_contacts` (`requested_by`);--> statement-breakpoint
CREATE TABLE `organization_domains` (
	`id` text PRIMARY KEY NOT NULL,
	`code` text,
	`tombstone` integer DEFAULT 0,
	`status` text DEFAULT 'Active',
	`version` integer DEFAULT 1,
	`created_date` text,
	`created_time` text,
	`updated_date` text,
	`updated_time` text,
	`organization_id` text,
	`created_by` text,
	`updated_by` text,
	`deleted_by` text,
	`requested_by` text,
	`timestamp` text,
	`categories` text DEFAULT (json_array()),
	`domain_name` text,
	FOREIGN KEY (`organization_id`) REFERENCES `organizations`(`id`) ON UPDATE no action ON DELETE no action
);
--> statement-breakpoint
CREATE UNIQUE INDEX `organization_domains_domain_name_unique` ON `organization_domains` (`domain_name`);--> statement-breakpoint
CREATE INDEX `organization_domains_id_idx` ON `organization_domains` (`id`);--> statement-breakpoint
CREATE INDEX `organization_domains_code_idx` ON `organization_domains` (`code`);--> statement-breakpoint
CREATE INDEX `organization_domains_tombstone_idx` ON `organization_domains` (`tombstone`);--> statement-breakpoint
CREATE INDEX `organization_domains_status_idx` ON `organization_domains` (`status`);--> statement-breakpoint
CREATE INDEX `organization_domains_version_idx` ON `organization_domains` (`version`);--> statement-breakpoint
CREATE INDEX `organization_domains_created_date_idx` ON `organization_domains` (`created_date`);--> statement-breakpoint
CREATE INDEX `organization_domains_updated_date_idx` ON `organization_domains` (`updated_date`);--> statement-breakpoint
CREATE INDEX `organization_domains_organization_id_idx` ON `organization_domains` (`organization_id`);--> statement-breakpoint
CREATE INDEX `organization_domains_created_by_idx` ON `organization_domains` (`created_by`);--> statement-breakpoint
CREATE INDEX `organization_domains_updated_by_idx` ON `organization_domains` (`updated_by`);--> statement-breakpoint
CREATE INDEX `organization_domains_deleted_by_idx` ON `organization_domains` (`deleted_by`);--> statement-breakpoint
CREATE INDEX `organization_domains_requested_by_idx` ON `organization_domains` (`requested_by`);--> statement-breakpoint
CREATE TABLE `organization_files` (
	`id` text PRIMARY KEY NOT NULL,
	`code` text,
	`tombstone` integer DEFAULT 0,
	`status` text DEFAULT 'Active',
	`version` integer DEFAULT 1,
	`created_date` text,
	`created_time` text,
	`updated_date` text,
	`updated_time` text,
	`organization_id` text,
	`created_by` text,
	`updated_by` text,
	`deleted_by` text,
	`requested_by` text,
	`timestamp` text,
	`categories` text DEFAULT (json_array()),
	`organizaion_id` text,
	`organization_contact_id` text,
	`url` text,
	`name` text,
	`mime_type` text,
	`size` text,
	`type` text,
	FOREIGN KEY (`organizaion_id`) REFERENCES `organizations`(`id`) ON UPDATE no action ON DELETE no action,
	FOREIGN KEY (`organization_contact_id`) REFERENCES `organization_contacts`(`id`) ON UPDATE no action ON DELETE no action
);
--> statement-breakpoint
CREATE INDEX `organization_files_id_idx` ON `organization_files` (`id`);--> statement-breakpoint
CREATE INDEX `organization_files_code_idx` ON `organization_files` (`code`);--> statement-breakpoint
CREATE INDEX `organization_files_tombstone_idx` ON `organization_files` (`tombstone`);--> statement-breakpoint
CREATE INDEX `organization_files_status_idx` ON `organization_files` (`status`);--> statement-breakpoint
CREATE INDEX `organization_files_version_idx` ON `organization_files` (`version`);--> statement-breakpoint
CREATE INDEX `organization_files_created_date_idx` ON `organization_files` (`created_date`);--> statement-breakpoint
CREATE INDEX `organization_files_updated_date_idx` ON `organization_files` (`updated_date`);--> statement-breakpoint
CREATE INDEX `organization_files_organization_id_idx` ON `organization_files` (`organization_id`);--> statement-breakpoint
CREATE INDEX `organization_files_created_by_idx` ON `organization_files` (`created_by`);--> statement-breakpoint
CREATE INDEX `organization_files_updated_by_idx` ON `organization_files` (`updated_by`);--> statement-breakpoint
CREATE INDEX `organization_files_deleted_by_idx` ON `organization_files` (`deleted_by`);--> statement-breakpoint
CREATE INDEX `organization_files_requested_by_idx` ON `organization_files` (`requested_by`);--> statement-breakpoint
CREATE TABLE `organizations` (
	`id` text PRIMARY KEY NOT NULL,
	`code` text,
	`tombstone` integer DEFAULT 0,
	`status` text DEFAULT 'Active',
	`version` integer DEFAULT 1,
	`created_date` text,
	`created_time` text,
	`updated_date` text,
	`updated_time` text,
	`organization_id` text,
	`created_by` text,
	`updated_by` text,
	`deleted_by` text,
	`requested_by` text,
	`timestamp` text,
	`categories` text DEFAULT (json_array()),
	`parent_organization_id` text DEFAULT (null),
	`name` text,
	FOREIGN KEY (`parent_organization_id`) REFERENCES `organizations`(`id`) ON UPDATE no action ON DELETE no action
);
--> statement-breakpoint
CREATE INDEX `organizations_id_idx` ON `organizations` (`id`);--> statement-breakpoint
CREATE INDEX `organizations_code_idx` ON `organizations` (`code`);--> statement-breakpoint
CREATE INDEX `organizations_tombstone_idx` ON `organizations` (`tombstone`);--> statement-breakpoint
CREATE INDEX `organizations_status_idx` ON `organizations` (`status`);--> statement-breakpoint
CREATE INDEX `organizations_version_idx` ON `organizations` (`version`);--> statement-breakpoint
CREATE INDEX `organizations_created_date_idx` ON `organizations` (`created_date`);--> statement-breakpoint
CREATE INDEX `organizations_updated_date_idx` ON `organizations` (`updated_date`);--> statement-breakpoint
CREATE INDEX `organizations_organization_id_idx` ON `organizations` (`organization_id`);--> statement-breakpoint
CREATE INDEX `organizations_created_by_idx` ON `organizations` (`created_by`);--> statement-breakpoint
CREATE INDEX `organizations_updated_by_idx` ON `organizations` (`updated_by`);--> statement-breakpoint
CREATE INDEX `organizations_deleted_by_idx` ON `organizations` (`deleted_by`);--> statement-breakpoint
CREATE INDEX `organizations_requested_by_idx` ON `organizations` (`requested_by`);--> statement-breakpoint
CREATE TABLE `config_applications` (
	`id` text PRIMARY KEY NOT NULL,
	`code` text,
	`tombstone` integer DEFAULT 0,
	`status` text DEFAULT 'Active',
	`version` integer DEFAULT 1,
	`created_date` text,
	`created_time` text,
	`updated_date` text,
	`updated_time` text,
	`organization_id` text,
	`created_by` text,
	`updated_by` text,
	`deleted_by` text,
	`requested_by` text,
	`timestamp` text,
	`type` text,
	`value` text
);
--> statement-breakpoint
CREATE INDEX `config_applications_id_idx` ON `config_applications` (`id`);--> statement-breakpoint
CREATE INDEX `config_applications_code_idx` ON `config_applications` (`code`);--> statement-breakpoint
CREATE INDEX `config_applications_tombstone_idx` ON `config_applications` (`tombstone`);--> statement-breakpoint
CREATE INDEX `config_applications_status_idx` ON `config_applications` (`status`);--> statement-breakpoint
CREATE INDEX `config_applications_version_idx` ON `config_applications` (`version`);--> statement-breakpoint
CREATE INDEX `config_applications_created_date_idx` ON `config_applications` (`created_date`);--> statement-breakpoint
CREATE INDEX `config_applications_updated_date_idx` ON `config_applications` (`updated_date`);--> statement-breakpoint
CREATE INDEX `config_applications_organization_id_idx` ON `config_applications` (`organization_id`);--> statement-breakpoint
CREATE INDEX `config_applications_created_by_idx` ON `config_applications` (`created_by`);--> statement-breakpoint
CREATE INDEX `config_applications_updated_by_idx` ON `config_applications` (`updated_by`);--> statement-breakpoint
CREATE INDEX `config_applications_deleted_by_idx` ON `config_applications` (`deleted_by`);--> statement-breakpoint
CREATE INDEX `config_applications_requested_by_idx` ON `config_applications` (`requested_by`);--> statement-breakpoint
CREATE TABLE `config_sync` (
	`id` text PRIMARY KEY NOT NULL,
	`code` text,
	`tombstone` integer DEFAULT 0,
	`status` text DEFAULT 'Active',
	`version` integer DEFAULT 1,
	`created_date` text,
	`created_time` text,
	`updated_date` text,
	`updated_time` text,
	`organization_id` text,
	`created_by` text,
	`updated_by` text,
	`deleted_by` text,
	`requested_by` text,
	`timestamp` text,
	`type` text,
	`value` text
);
--> statement-breakpoint
CREATE INDEX `config_sync_id_idx` ON `config_sync` (`id`);--> statement-breakpoint
CREATE INDEX `config_sync_code_idx` ON `config_sync` (`code`);--> statement-breakpoint
CREATE INDEX `config_sync_tombstone_idx` ON `config_sync` (`tombstone`);--> statement-breakpoint
CREATE INDEX `config_sync_status_idx` ON `config_sync` (`status`);--> statement-breakpoint
CREATE INDEX `config_sync_version_idx` ON `config_sync` (`version`);--> statement-breakpoint
CREATE INDEX `config_sync_created_date_idx` ON `config_sync` (`created_date`);--> statement-breakpoint
CREATE INDEX `config_sync_updated_date_idx` ON `config_sync` (`updated_date`);--> statement-breakpoint
CREATE INDEX `config_sync_organization_id_idx` ON `config_sync` (`organization_id`);--> statement-breakpoint
CREATE INDEX `config_sync_created_by_idx` ON `config_sync` (`created_by`);--> statement-breakpoint
CREATE INDEX `config_sync_updated_by_idx` ON `config_sync` (`updated_by`);--> statement-breakpoint
CREATE INDEX `config_sync_deleted_by_idx` ON `config_sync` (`deleted_by`);--> statement-breakpoint
CREATE INDEX `config_sync_requested_by_idx` ON `config_sync` (`requested_by`);--> statement-breakpoint
CREATE TABLE `class_types` (
	`id` text PRIMARY KEY NOT NULL,
	`code` text,
	`tombstone` integer DEFAULT 0,
	`status` text DEFAULT 'Active',
	`version` integer DEFAULT 1,
	`created_date` text,
	`created_time` text,
	`updated_date` text,
	`updated_time` text,
	`organization_id` text,
	`created_by` text,
	`updated_by` text,
	`deleted_by` text,
	`requested_by` text,
	`timestamp` text,
	`type` text,
	`company` text NOT NULL,
	`entity` text NOT NULL,
	`is_list` integer DEFAULT false,
	`is_with_version` integer DEFAULT false,
	`schema_version` text NOT NULL
);
--> statement-breakpoint
CREATE INDEX `class_types_id_idx` ON `class_types` (`id`);--> statement-breakpoint
CREATE INDEX `class_types_code_idx` ON `class_types` (`code`);--> statement-breakpoint
CREATE INDEX `class_types_tombstone_idx` ON `class_types` (`tombstone`);--> statement-breakpoint
CREATE INDEX `class_types_status_idx` ON `class_types` (`status`);--> statement-breakpoint
CREATE INDEX `class_types_version_idx` ON `class_types` (`version`);--> statement-breakpoint
CREATE INDEX `class_types_created_date_idx` ON `class_types` (`created_date`);--> statement-breakpoint
CREATE INDEX `class_types_updated_date_idx` ON `class_types` (`updated_date`);--> statement-breakpoint
CREATE INDEX `class_types_organization_id_idx` ON `class_types` (`organization_id`);--> statement-breakpoint
CREATE INDEX `class_types_created_by_idx` ON `class_types` (`created_by`);--> statement-breakpoint
CREATE INDEX `class_types_updated_by_idx` ON `class_types` (`updated_by`);--> statement-breakpoint
CREATE INDEX `class_types_deleted_by_idx` ON `class_types` (`deleted_by`);--> statement-breakpoint
CREATE INDEX `class_types_requested_by_idx` ON `class_types` (`requested_by`);--> statement-breakpoint
CREATE TABLE `fields` (
	`id` text PRIMARY KEY NOT NULL,
	`code` text,
	`tombstone` integer DEFAULT 0,
	`status` text DEFAULT 'Active',
	`version` integer DEFAULT 1,
	`created_date` text,
	`created_time` text,
	`updated_date` text,
	`updated_time` text,
	`organization_id` text,
	`created_by` text,
	`updated_by` text,
	`deleted_by` text,
	`requested_by` text,
	`timestamp` text,
	`label` text NOT NULL,
	`name` text NOT NULL,
	`type` text NOT NULL
);
--> statement-breakpoint
CREATE INDEX `fields_id_idx` ON `fields` (`id`);--> statement-breakpoint
CREATE INDEX `fields_code_idx` ON `fields` (`code`);--> statement-breakpoint
CREATE INDEX `fields_tombstone_idx` ON `fields` (`tombstone`);--> statement-breakpoint
CREATE INDEX `fields_status_idx` ON `fields` (`status`);--> statement-breakpoint
CREATE INDEX `fields_version_idx` ON `fields` (`version`);--> statement-breakpoint
CREATE INDEX `fields_created_date_idx` ON `fields` (`created_date`);--> statement-breakpoint
CREATE INDEX `fields_updated_date_idx` ON `fields` (`updated_date`);--> statement-breakpoint
CREATE INDEX `fields_organization_id_idx` ON `fields` (`organization_id`);--> statement-breakpoint
CREATE INDEX `fields_created_by_idx` ON `fields` (`created_by`);--> statement-breakpoint
CREATE INDEX `fields_updated_by_idx` ON `fields` (`updated_by`);--> statement-breakpoint
CREATE INDEX `fields_deleted_by_idx` ON `fields` (`deleted_by`);--> statement-breakpoint
CREATE INDEX `fields_requested_by_idx` ON `fields` (`requested_by`);--> statement-breakpoint
CREATE TABLE `allowed_fields` (
	`id` text PRIMARY KEY NOT NULL,
	`code` text,
	`tombstone` integer DEFAULT 0,
	`status` text DEFAULT 'Active',
	`version` integer DEFAULT 1,
	`created_date` text,
	`created_time` text,
	`updated_date` text,
	`updated_time` text,
	`organization_id` text,
	`created_by` text,
	`updated_by` text,
	`deleted_by` text,
	`requested_by` text,
	`timestamp` text,
	`label` text NOT NULL,
	`name` text NOT NULL,
	`type` text NOT NULL,
	`class_type_id` text NOT NULL,
	`is_optional` integer DEFAULT false,
	`is_primary_key` integer DEFAULT false,
	`reference_to` text,
	`data_type` text NOT NULL,
	`default_value` text NOT NULL,
	FOREIGN KEY (`class_type_id`) REFERENCES `class_types`(`id`) ON UPDATE no action ON DELETE no action
);
--> statement-breakpoint
CREATE INDEX `allowed_fields_id_idx` ON `allowed_fields` (`id`);--> statement-breakpoint
CREATE INDEX `allowed_fields_code_idx` ON `allowed_fields` (`code`);--> statement-breakpoint
CREATE INDEX `allowed_fields_tombstone_idx` ON `allowed_fields` (`tombstone`);--> statement-breakpoint
CREATE INDEX `allowed_fields_status_idx` ON `allowed_fields` (`status`);--> statement-breakpoint
CREATE INDEX `allowed_fields_version_idx` ON `allowed_fields` (`version`);--> statement-breakpoint
CREATE INDEX `allowed_fields_created_date_idx` ON `allowed_fields` (`created_date`);--> statement-breakpoint
CREATE INDEX `allowed_fields_updated_date_idx` ON `allowed_fields` (`updated_date`);--> statement-breakpoint
CREATE INDEX `allowed_fields_organization_id_idx` ON `allowed_fields` (`organization_id`);--> statement-breakpoint
CREATE INDEX `allowed_fields_created_by_idx` ON `allowed_fields` (`created_by`);--> statement-breakpoint
CREATE INDEX `allowed_fields_updated_by_idx` ON `allowed_fields` (`updated_by`);--> statement-breakpoint
CREATE INDEX `allowed_fields_deleted_by_idx` ON `allowed_fields` (`deleted_by`);--> statement-breakpoint
CREATE INDEX `allowed_fields_requested_by_idx` ON `allowed_fields` (`requested_by`);--> statement-breakpoint
CREATE TABLE `crdt_merkles` (
	`group_id` text PRIMARY KEY NOT NULL,
	`timestamp` text NOT NULL,
	`merkle` text NOT NULL
);
--> statement-breakpoint
CREATE UNIQUE INDEX `crdt_merkles_group_id_unique` ON `crdt_merkles` (`group_id`);--> statement-breakpoint
CREATE TABLE `crdt_messages` (
	`database` text,
	`dataset` text NOT NULL,
	`group_id` text NOT NULL,
	`timestamp` text NOT NULL,
	`row` text NOT NULL,
	`column` text NOT NULL,
	`client_id` text NOT NULL,
	`value` text NOT NULL,
	PRIMARY KEY(`timestamp`, `group_id`, `row`, `column`)
);
--> statement-breakpoint
CREATE TABLE `queue` (
	`id` text PRIMARY KEY NOT NULL,
	`name` text NOT NULL,
	`count` integer NOT NULL,
	`size` integer NOT NULL
);
--> statement-breakpoint
CREATE TABLE `queue_items` (
	`id` text PRIMARY KEY NOT NULL,
	`order` integer NOT NULL,
	`queue_id` text NOT NULL,
	`value` text NOT NULL
);
--> statement-breakpoint
CREATE TABLE `sync_endpoints` (
	`id` text PRIMARY KEY NOT NULL,
	`name` text,
	`url` text,
	`group_id` text,
	`username` text,
	`password` text,
	`status` text
);
--> statement-breakpoint
CREATE TABLE `transactions` (
	`id` text PRIMARY KEY NOT NULL,
	`timestamp` text NOT NULL,
	`status` text DEFAULT 'Active' NOT NULL,
	`expiry` integer
);
