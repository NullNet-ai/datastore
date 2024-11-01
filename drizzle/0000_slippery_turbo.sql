CREATE TABLE `config_applications` (
	`id` text PRIMARY KEY NOT NULL,
	`tombstone` integer DEFAULT 0,
	`status` text DEFAULT 'Active',
	`version` integer DEFAULT 1,
	`created_date` text,
	`created_time` text,
	`updated_date` text,
	`updated_time` text,
	`type` text,
	`value` text
);
--> statement-breakpoint
CREATE INDEX `config_applications_id_idx` ON `config_applications` (`id`);--> statement-breakpoint
CREATE INDEX `config_applications_tombstone_idx` ON `config_applications` (`tombstone`);--> statement-breakpoint
CREATE INDEX `config_applications_status_idx` ON `config_applications` (`status`);--> statement-breakpoint
CREATE INDEX `config_applications_version_idx` ON `config_applications` (`version`);--> statement-breakpoint
CREATE INDEX `config_applications_created_date_idx` ON `config_applications` (`created_date`);--> statement-breakpoint
CREATE INDEX `config_applications_updated_date_idx` ON `config_applications` (`updated_date`);--> statement-breakpoint
CREATE TABLE `config_sync` (
	`id` text PRIMARY KEY NOT NULL,
	`tombstone` integer DEFAULT 0,
	`status` text DEFAULT 'Active',
	`version` integer DEFAULT 1,
	`created_date` text,
	`created_time` text,
	`updated_date` text,
	`updated_time` text,
	`type` text,
	`value` text
);
--> statement-breakpoint
CREATE INDEX `config_sync_id_idx` ON `config_sync` (`id`);--> statement-breakpoint
CREATE INDEX `config_sync_tombstone_idx` ON `config_sync` (`tombstone`);--> statement-breakpoint
CREATE INDEX `config_sync_status_idx` ON `config_sync` (`status`);--> statement-breakpoint
CREATE INDEX `config_sync_version_idx` ON `config_sync` (`version`);--> statement-breakpoint
CREATE INDEX `config_sync_created_date_idx` ON `config_sync` (`created_date`);--> statement-breakpoint
CREATE INDEX `config_sync_updated_date_idx` ON `config_sync` (`updated_date`);--> statement-breakpoint
CREATE TABLE `contact_emails` (
	`id` text PRIMARY KEY NOT NULL,
	`tombstone` integer DEFAULT 0,
	`status` text DEFAULT 'Active',
	`version` integer DEFAULT 1,
	`created_date` text,
	`created_time` text,
	`updated_date` text,
	`updated_time` text,
	`categories` text DEFAULT (json_array()),
	`contact_id` text,
	`email` text,
	FOREIGN KEY (`contact_id`) REFERENCES `contacts`(`id`) ON UPDATE no action ON DELETE no action
);
--> statement-breakpoint
CREATE INDEX `contact_emails_id_idx` ON `contact_emails` (`id`);--> statement-breakpoint
CREATE INDEX `contact_emails_tombstone_idx` ON `contact_emails` (`tombstone`);--> statement-breakpoint
CREATE INDEX `contact_emails_status_idx` ON `contact_emails` (`status`);--> statement-breakpoint
CREATE INDEX `contact_emails_version_idx` ON `contact_emails` (`version`);--> statement-breakpoint
CREATE INDEX `contact_emails_created_date_idx` ON `contact_emails` (`created_date`);--> statement-breakpoint
CREATE INDEX `contact_emails_updated_date_idx` ON `contact_emails` (`updated_date`);--> statement-breakpoint
CREATE TABLE `contact_phone_numbers` (
	`id` text PRIMARY KEY NOT NULL,
	`tombstone` integer DEFAULT 0,
	`status` text DEFAULT 'Active',
	`version` integer DEFAULT 1,
	`created_date` text,
	`created_time` text,
	`updated_date` text,
	`updated_time` text,
	`categories` text DEFAULT (json_array()),
	`contact_id` text,
	`phone_number_raw` text,
	FOREIGN KEY (`contact_id`) REFERENCES `contacts`(`id`) ON UPDATE no action ON DELETE no action
);
--> statement-breakpoint
CREATE INDEX `contact_phone_numbers_id_idx` ON `contact_phone_numbers` (`id`);--> statement-breakpoint
CREATE INDEX `contact_phone_numbers_tombstone_idx` ON `contact_phone_numbers` (`tombstone`);--> statement-breakpoint
CREATE INDEX `contact_phone_numbers_status_idx` ON `contact_phone_numbers` (`status`);--> statement-breakpoint
CREATE INDEX `contact_phone_numbers_version_idx` ON `contact_phone_numbers` (`version`);--> statement-breakpoint
CREATE INDEX `contact_phone_numbers_created_date_idx` ON `contact_phone_numbers` (`created_date`);--> statement-breakpoint
CREATE INDEX `contact_phone_numbers_updated_date_idx` ON `contact_phone_numbers` (`updated_date`);--> statement-breakpoint
CREATE TABLE `contacts` (
	`id` text PRIMARY KEY NOT NULL,
	`tombstone` integer DEFAULT 0,
	`status` text DEFAULT 'Active',
	`version` integer DEFAULT 1,
	`created_date` text,
	`created_time` text,
	`updated_date` text,
	`updated_time` text,
	`categories` text DEFAULT (json_array()),
	`first_name` text,
	`middle_name` text,
	`last_name` text,
	`date_of_birth` text
);
--> statement-breakpoint
CREATE INDEX `contacts_id_idx` ON `contacts` (`id`);--> statement-breakpoint
CREATE INDEX `contacts_tombstone_idx` ON `contacts` (`tombstone`);--> statement-breakpoint
CREATE INDEX `contacts_status_idx` ON `contacts` (`status`);--> statement-breakpoint
CREATE INDEX `contacts_version_idx` ON `contacts` (`version`);--> statement-breakpoint
CREATE INDEX `contacts_created_date_idx` ON `contacts` (`created_date`);--> statement-breakpoint
CREATE INDEX `contacts_updated_date_idx` ON `contacts` (`updated_date`);--> statement-breakpoint
CREATE TABLE `organization_domains` (
	`id` text PRIMARY KEY NOT NULL,
	`tombstone` integer DEFAULT 0,
	`status` text DEFAULT 'Active',
	`version` integer DEFAULT 1,
	`created_date` text,
	`created_time` text,
	`updated_date` text,
	`updated_time` text,
	`categories` text DEFAULT (json_array()),
	`domain_name` text,
	`organization_id` text,
	FOREIGN KEY (`organization_id`) REFERENCES `organizations`(`id`) ON UPDATE no action ON DELETE no action
);
--> statement-breakpoint
CREATE UNIQUE INDEX `organization_domains_domain_name_unique` ON `organization_domains` (`domain_name`);--> statement-breakpoint
CREATE INDEX `organization_domains_id_idx` ON `organization_domains` (`id`);--> statement-breakpoint
CREATE INDEX `organization_domains_tombstone_idx` ON `organization_domains` (`tombstone`);--> statement-breakpoint
CREATE INDEX `organization_domains_status_idx` ON `organization_domains` (`status`);--> statement-breakpoint
CREATE INDEX `organization_domains_version_idx` ON `organization_domains` (`version`);--> statement-breakpoint
CREATE INDEX `organization_domains_created_date_idx` ON `organization_domains` (`created_date`);--> statement-breakpoint
CREATE INDEX `organization_domains_updated_date_idx` ON `organization_domains` (`updated_date`);--> statement-breakpoint
CREATE TABLE `organization_contact_accounts` (
	`id` text PRIMARY KEY NOT NULL,
	`tombstone` integer DEFAULT 0,
	`status` text DEFAULT 'Active',
	`version` integer DEFAULT 1,
	`created_date` text,
	`created_time` text,
	`updated_date` text,
	`updated_time` text,
	`categories` text DEFAULT (json_array()),
	`organization_contact_id` text,
	`organization_id` text,
	`email` text,
	`password` text,
	FOREIGN KEY (`organization_contact_id`) REFERENCES `organization_contacts`(`id`) ON UPDATE no action ON DELETE no action,
	FOREIGN KEY (`organization_id`) REFERENCES `organizations`(`id`) ON UPDATE no action ON DELETE no action
);
--> statement-breakpoint
CREATE INDEX `organization_contact_accounts_id_idx` ON `organization_contact_accounts` (`id`);--> statement-breakpoint
CREATE INDEX `organization_contact_accounts_tombstone_idx` ON `organization_contact_accounts` (`tombstone`);--> statement-breakpoint
CREATE INDEX `organization_contact_accounts_status_idx` ON `organization_contact_accounts` (`status`);--> statement-breakpoint
CREATE INDEX `organization_contact_accounts_version_idx` ON `organization_contact_accounts` (`version`);--> statement-breakpoint
CREATE INDEX `organization_contact_accounts_created_date_idx` ON `organization_contact_accounts` (`created_date`);--> statement-breakpoint
CREATE INDEX `organization_contact_accounts_updated_date_idx` ON `organization_contact_accounts` (`updated_date`);--> statement-breakpoint
CREATE TABLE `organization_contacts` (
	`id` text PRIMARY KEY NOT NULL,
	`tombstone` integer DEFAULT 0,
	`status` text DEFAULT 'Active',
	`version` integer DEFAULT 1,
	`created_date` text,
	`created_time` text,
	`updated_date` text,
	`updated_time` text,
	`categories` text DEFAULT (json_array()),
	`contact_id` text,
	`organization_id` text,
	FOREIGN KEY (`contact_id`) REFERENCES `contacts`(`id`) ON UPDATE no action ON DELETE no action,
	FOREIGN KEY (`organization_id`) REFERENCES `organizations`(`id`) ON UPDATE no action ON DELETE no action
);
--> statement-breakpoint
CREATE INDEX `organization_contacts_id_idx` ON `organization_contacts` (`id`);--> statement-breakpoint
CREATE INDEX `organization_contacts_tombstone_idx` ON `organization_contacts` (`tombstone`);--> statement-breakpoint
CREATE INDEX `organization_contacts_status_idx` ON `organization_contacts` (`status`);--> statement-breakpoint
CREATE INDEX `organization_contacts_version_idx` ON `organization_contacts` (`version`);--> statement-breakpoint
CREATE INDEX `organization_contacts_created_date_idx` ON `organization_contacts` (`created_date`);--> statement-breakpoint
CREATE INDEX `organization_contacts_updated_date_idx` ON `organization_contacts` (`updated_date`);--> statement-breakpoint
CREATE TABLE `organization_files` (
	`id` text PRIMARY KEY NOT NULL,
	`tombstone` integer DEFAULT 0,
	`status` text DEFAULT 'Active',
	`version` integer DEFAULT 1,
	`created_date` text,
	`created_time` text,
	`updated_date` text,
	`updated_time` text,
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
CREATE INDEX `organization_files_tombstone_idx` ON `organization_files` (`tombstone`);--> statement-breakpoint
CREATE INDEX `organization_files_status_idx` ON `organization_files` (`status`);--> statement-breakpoint
CREATE INDEX `organization_files_version_idx` ON `organization_files` (`version`);--> statement-breakpoint
CREATE INDEX `organization_files_created_date_idx` ON `organization_files` (`created_date`);--> statement-breakpoint
CREATE INDEX `organization_files_updated_date_idx` ON `organization_files` (`updated_date`);--> statement-breakpoint
CREATE TABLE `organizations` (
	`id` text PRIMARY KEY NOT NULL,
	`tombstone` integer DEFAULT 0,
	`status` text DEFAULT 'Active',
	`version` integer DEFAULT 1,
	`created_date` text,
	`created_time` text,
	`updated_date` text,
	`updated_time` text,
	`categories` text DEFAULT (json_array()),
	`parent_organization_id` text DEFAULT (null),
	`name` text,
	FOREIGN KEY (`parent_organization_id`) REFERENCES `organizations`(`id`) ON UPDATE no action ON DELETE no action
);
--> statement-breakpoint
CREATE INDEX `organizations_id_idx` ON `organizations` (`id`);--> statement-breakpoint
CREATE INDEX `organizations_tombstone_idx` ON `organizations` (`tombstone`);--> statement-breakpoint
CREATE INDEX `organizations_status_idx` ON `organizations` (`status`);--> statement-breakpoint
CREATE INDEX `organizations_version_idx` ON `organizations` (`version`);--> statement-breakpoint
CREATE INDEX `organizations_created_date_idx` ON `organizations` (`created_date`);--> statement-breakpoint
CREATE INDEX `organizations_updated_date_idx` ON `organizations` (`updated_date`);--> statement-breakpoint
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
CREATE TABLE `transactions` (
	`id` text PRIMARY KEY NOT NULL,
	`timestamp` text NOT NULL,
	`status` text DEFAULT 'Active' NOT NULL,
	`expiry` integer
);
--> statement-breakpoint
CREATE TABLE `queue_items` (
	`id` text PRIMARY KEY NOT NULL,
	`order` integer NOT NULL,
	`queue_id` text NOT NULL,
	`value` text NOT NULL
);
--> statement-breakpoint
CREATE TABLE `queue` (
	`id` text PRIMARY KEY NOT NULL,
	`name` text NOT NULL,
	`count` integer NOT NULL,
	`size` integer NOT NULL
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
