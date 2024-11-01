CREATE TABLE `config_applications.ts` (
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
CREATE INDEX `id_idx` ON `config_applications.ts` (`id`);--> statement-breakpoint
CREATE INDEX `tombstone_idx` ON `config_applications.ts` (`tombstone`);--> statement-breakpoint
CREATE INDEX `status_idx` ON `config_applications.ts` (`status`);--> statement-breakpoint
CREATE INDEX `version_idx` ON `config_applications.ts` (`version`);--> statement-breakpoint
CREATE INDEX `created_date_idx` ON `config_applications.ts` (`created_date`);--> statement-breakpoint
CREATE INDEX `created_time_idx` ON `config_applications.ts` (`created_time`);--> statement-breakpoint
CREATE INDEX `updated_date_idx` ON `config_applications.ts` (`updated_date`);--> statement-breakpoint
CREATE INDEX `updated_time_idx` ON `config_applications.ts` (`updated_time`);--> statement-breakpoint
CREATE TABLE `config_sync.ts` (
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
CREATE INDEX `id_idx` ON `config_sync.ts` (`id`);--> statement-breakpoint
CREATE INDEX `tombstone_idx` ON `config_sync.ts` (`tombstone`);--> statement-breakpoint
CREATE INDEX `status_idx` ON `config_sync.ts` (`status`);--> statement-breakpoint
CREATE INDEX `version_idx` ON `config_sync.ts` (`version`);--> statement-breakpoint
CREATE INDEX `created_date_idx` ON `config_sync.ts` (`created_date`);--> statement-breakpoint
CREATE INDEX `created_time_idx` ON `config_sync.ts` (`created_time`);--> statement-breakpoint
CREATE INDEX `updated_date_idx` ON `config_sync.ts` (`updated_date`);--> statement-breakpoint
CREATE INDEX `updated_time_idx` ON `config_sync.ts` (`updated_time`);--> statement-breakpoint
CREATE TABLE `contact_emails.ts` (
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
	FOREIGN KEY (`contact_id`) REFERENCES `contacts.ts`(`id`) ON UPDATE no action ON DELETE no action
);
--> statement-breakpoint
CREATE INDEX `id_idx` ON `contact_emails.ts` (`id`);--> statement-breakpoint
CREATE INDEX `tombstone_idx` ON `contact_emails.ts` (`tombstone`);--> statement-breakpoint
CREATE INDEX `status_idx` ON `contact_emails.ts` (`status`);--> statement-breakpoint
CREATE INDEX `version_idx` ON `contact_emails.ts` (`version`);--> statement-breakpoint
CREATE INDEX `created_date_idx` ON `contact_emails.ts` (`created_date`);--> statement-breakpoint
CREATE INDEX `created_time_idx` ON `contact_emails.ts` (`created_time`);--> statement-breakpoint
CREATE INDEX `updated_date_idx` ON `contact_emails.ts` (`updated_date`);--> statement-breakpoint
CREATE INDEX `updated_time_idx` ON `contact_emails.ts` (`updated_time`);--> statement-breakpoint
CREATE TABLE `contact_phone_numbers.ts` (
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
	FOREIGN KEY (`contact_id`) REFERENCES `contacts.ts`(`id`) ON UPDATE no action ON DELETE no action
);
--> statement-breakpoint
CREATE INDEX `id_idx` ON `contact_phone_numbers.ts` (`id`);--> statement-breakpoint
CREATE INDEX `tombstone_idx` ON `contact_phone_numbers.ts` (`tombstone`);--> statement-breakpoint
CREATE INDEX `status_idx` ON `contact_phone_numbers.ts` (`status`);--> statement-breakpoint
CREATE INDEX `version_idx` ON `contact_phone_numbers.ts` (`version`);--> statement-breakpoint
CREATE INDEX `created_date_idx` ON `contact_phone_numbers.ts` (`created_date`);--> statement-breakpoint
CREATE INDEX `created_time_idx` ON `contact_phone_numbers.ts` (`created_time`);--> statement-breakpoint
CREATE INDEX `updated_date_idx` ON `contact_phone_numbers.ts` (`updated_date`);--> statement-breakpoint
CREATE INDEX `updated_time_idx` ON `contact_phone_numbers.ts` (`updated_time`);--> statement-breakpoint
CREATE TABLE `contacts.ts` (
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
CREATE INDEX `id_idx` ON `contacts.ts` (`id`);--> statement-breakpoint
CREATE INDEX `tombstone_idx` ON `contacts.ts` (`tombstone`);--> statement-breakpoint
CREATE INDEX `status_idx` ON `contacts.ts` (`status`);--> statement-breakpoint
CREATE INDEX `version_idx` ON `contacts.ts` (`version`);--> statement-breakpoint
CREATE INDEX `created_date_idx` ON `contacts.ts` (`created_date`);--> statement-breakpoint
CREATE INDEX `created_time_idx` ON `contacts.ts` (`created_time`);--> statement-breakpoint
CREATE INDEX `updated_date_idx` ON `contacts.ts` (`updated_date`);--> statement-breakpoint
CREATE INDEX `updated_time_idx` ON `contacts.ts` (`updated_time`);--> statement-breakpoint
CREATE TABLE `organization_domains.ts` (
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
	FOREIGN KEY (`organization_id`) REFERENCES `organizations.ts`(`id`) ON UPDATE no action ON DELETE no action
);
--> statement-breakpoint
CREATE UNIQUE INDEX `organization_domains.ts_domain_name_unique` ON `organization_domains.ts` (`domain_name`);--> statement-breakpoint
CREATE INDEX `id_idx` ON `organization_domains.ts` (`id`);--> statement-breakpoint
CREATE INDEX `tombstone_idx` ON `organization_domains.ts` (`tombstone`);--> statement-breakpoint
CREATE INDEX `status_idx` ON `organization_domains.ts` (`status`);--> statement-breakpoint
CREATE INDEX `version_idx` ON `organization_domains.ts` (`version`);--> statement-breakpoint
CREATE INDEX `created_date_idx` ON `organization_domains.ts` (`created_date`);--> statement-breakpoint
CREATE INDEX `created_time_idx` ON `organization_domains.ts` (`created_time`);--> statement-breakpoint
CREATE INDEX `updated_date_idx` ON `organization_domains.ts` (`updated_date`);--> statement-breakpoint
CREATE INDEX `updated_time_idx` ON `organization_domains.ts` (`updated_time`);--> statement-breakpoint
CREATE TABLE `organization_contact_accounts.ts` (
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
	FOREIGN KEY (`organization_contact_id`) REFERENCES `organization_contacts.ts`(`id`) ON UPDATE no action ON DELETE no action,
	FOREIGN KEY (`organization_id`) REFERENCES `organizations.ts`(`id`) ON UPDATE no action ON DELETE no action
);
--> statement-breakpoint
CREATE INDEX `id_idx` ON `organization_contact_accounts.ts` (`id`);--> statement-breakpoint
CREATE INDEX `tombstone_idx` ON `organization_contact_accounts.ts` (`tombstone`);--> statement-breakpoint
CREATE INDEX `status_idx` ON `organization_contact_accounts.ts` (`status`);--> statement-breakpoint
CREATE INDEX `version_idx` ON `organization_contact_accounts.ts` (`version`);--> statement-breakpoint
CREATE INDEX `created_date_idx` ON `organization_contact_accounts.ts` (`created_date`);--> statement-breakpoint
CREATE INDEX `created_time_idx` ON `organization_contact_accounts.ts` (`created_time`);--> statement-breakpoint
CREATE INDEX `updated_date_idx` ON `organization_contact_accounts.ts` (`updated_date`);--> statement-breakpoint
CREATE INDEX `updated_time_idx` ON `organization_contact_accounts.ts` (`updated_time`);--> statement-breakpoint
CREATE TABLE `organization_contacts.ts` (
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
	FOREIGN KEY (`contact_id`) REFERENCES `contacts.ts`(`id`) ON UPDATE no action ON DELETE no action,
	FOREIGN KEY (`organization_id`) REFERENCES `organizations.ts`(`id`) ON UPDATE no action ON DELETE no action
);
--> statement-breakpoint
CREATE INDEX `id_idx` ON `organization_contacts.ts` (`id`);--> statement-breakpoint
CREATE INDEX `tombstone_idx` ON `organization_contacts.ts` (`tombstone`);--> statement-breakpoint
CREATE INDEX `status_idx` ON `organization_contacts.ts` (`status`);--> statement-breakpoint
CREATE INDEX `version_idx` ON `organization_contacts.ts` (`version`);--> statement-breakpoint
CREATE INDEX `created_date_idx` ON `organization_contacts.ts` (`created_date`);--> statement-breakpoint
CREATE INDEX `created_time_idx` ON `organization_contacts.ts` (`created_time`);--> statement-breakpoint
CREATE INDEX `updated_date_idx` ON `organization_contacts.ts` (`updated_date`);--> statement-breakpoint
CREATE INDEX `updated_time_idx` ON `organization_contacts.ts` (`updated_time`);--> statement-breakpoint
CREATE TABLE `organization_files.ts` (
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
	FOREIGN KEY (`organizaion_id`) REFERENCES `organizations.ts`(`id`) ON UPDATE no action ON DELETE no action,
	FOREIGN KEY (`organization_contact_id`) REFERENCES `organization_contacts.ts`(`id`) ON UPDATE no action ON DELETE no action
);
--> statement-breakpoint
CREATE INDEX `id_idx` ON `organization_files.ts` (`id`);--> statement-breakpoint
CREATE INDEX `tombstone_idx` ON `organization_files.ts` (`tombstone`);--> statement-breakpoint
CREATE INDEX `status_idx` ON `organization_files.ts` (`status`);--> statement-breakpoint
CREATE INDEX `version_idx` ON `organization_files.ts` (`version`);--> statement-breakpoint
CREATE INDEX `created_date_idx` ON `organization_files.ts` (`created_date`);--> statement-breakpoint
CREATE INDEX `created_time_idx` ON `organization_files.ts` (`created_time`);--> statement-breakpoint
CREATE INDEX `updated_date_idx` ON `organization_files.ts` (`updated_date`);--> statement-breakpoint
CREATE INDEX `updated_time_idx` ON `organization_files.ts` (`updated_time`);--> statement-breakpoint
CREATE TABLE `organizations.ts` (
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
	FOREIGN KEY (`parent_organization_id`) REFERENCES `organizations.ts`(`id`) ON UPDATE no action ON DELETE no action
);
--> statement-breakpoint
CREATE INDEX `id_idx` ON `organizations.ts` (`id`);--> statement-breakpoint
CREATE INDEX `tombstone_idx` ON `organizations.ts` (`tombstone`);--> statement-breakpoint
CREATE INDEX `status_idx` ON `organizations.ts` (`status`);--> statement-breakpoint
CREATE INDEX `version_idx` ON `organizations.ts` (`version`);--> statement-breakpoint
CREATE INDEX `created_date_idx` ON `organizations.ts` (`created_date`);--> statement-breakpoint
CREATE INDEX `created_time_idx` ON `organizations.ts` (`created_time`);--> statement-breakpoint
CREATE INDEX `updated_date_idx` ON `organizations.ts` (`updated_date`);--> statement-breakpoint
CREATE INDEX `updated_time_idx` ON `organizations.ts` (`updated_time`);--> statement-breakpoint
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
