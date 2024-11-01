CREATE TABLE `config_applications` (
	`type` text,
	`value` text,
	`id` text PRIMARY KEY NOT NULL,
	`tombstone` integer DEFAULT 0,
	`status` text DEFAULT 'Active',
	`version` integer DEFAULT 1,
	`created_date` text,
	`created_time` text,
	`updated_date` text,
	`updated_time` text
);
--> statement-breakpoint
CREATE TABLE `config_sync` (
	`type` text,
	`value` text,
	`id` text PRIMARY KEY NOT NULL,
	`tombstone` integer DEFAULT 0,
	`status` text DEFAULT 'Active',
	`version` integer DEFAULT 1,
	`created_date` text,
	`created_time` text,
	`updated_date` text,
	`updated_time` text
);
--> statement-breakpoint
CREATE TABLE `contact_emails` (
	`categories` text DEFAULT (json_array()),
	`contact_id` text,
	`email` text,
	`id` text PRIMARY KEY NOT NULL,
	`tombstone` integer DEFAULT 0,
	`status` text DEFAULT 'Active',
	`version` integer DEFAULT 1,
	`created_date` text,
	`created_time` text,
	`updated_date` text,
	`updated_time` text,
	FOREIGN KEY (`contact_id`) REFERENCES `contacts`(`id`) ON UPDATE no action ON DELETE no action
);
--> statement-breakpoint
CREATE TABLE `contact_phone_numbers` (
	`categories` text DEFAULT (json_array()),
	`contact_id` text,
	`phone_number_raw` text,
	`id` text PRIMARY KEY NOT NULL,
	`tombstone` integer DEFAULT 0,
	`status` text DEFAULT 'Active',
	`version` integer DEFAULT 1,
	`created_date` text,
	`created_time` text,
	`updated_date` text,
	`updated_time` text,
	FOREIGN KEY (`contact_id`) REFERENCES `contacts`(`id`) ON UPDATE no action ON DELETE no action
);
--> statement-breakpoint
CREATE TABLE `contacts` (
	`categories` text DEFAULT (json_array()),
	`first_name` text,
	`middle_name` text,
	`last_name` text,
	`date_of_birth` text,
	`id` text PRIMARY KEY NOT NULL,
	`tombstone` integer DEFAULT 0,
	`status` text DEFAULT 'Active',
	`version` integer DEFAULT 1,
	`created_date` text,
	`created_time` text,
	`updated_date` text,
	`updated_time` text
);
--> statement-breakpoint
CREATE TABLE `organization_domains` (
	`categories` text DEFAULT (json_array()),
	`domain_name` text,
	`organization_id` text,
	`id` text PRIMARY KEY NOT NULL,
	`tombstone` integer DEFAULT 0,
	`status` text DEFAULT 'Active',
	`version` integer DEFAULT 1,
	`created_date` text,
	`created_time` text,
	`updated_date` text,
	`updated_time` text,
	FOREIGN KEY (`organization_id`) REFERENCES `organizations`(`id`) ON UPDATE no action ON DELETE no action
);
--> statement-breakpoint
CREATE UNIQUE INDEX `organization_domains_domain_name_unique` ON `organization_domains` (`domain_name`);--> statement-breakpoint
CREATE TABLE `organization_contact_accounts` (
	`categories` text DEFAULT (json_array()),
	`organization_contact_id` text,
	`organization_id` text,
	`email` text,
	`password` text,
	`id` text PRIMARY KEY NOT NULL,
	`tombstone` integer DEFAULT 0,
	`status` text DEFAULT 'Active',
	`version` integer DEFAULT 1,
	`created_date` text,
	`created_time` text,
	`updated_date` text,
	`updated_time` text,
	FOREIGN KEY (`organization_contact_id`) REFERENCES `organization_contacts`(`id`) ON UPDATE no action ON DELETE no action,
	FOREIGN KEY (`organization_id`) REFERENCES `organizations`(`id`) ON UPDATE no action ON DELETE no action
);
--> statement-breakpoint
CREATE TABLE `organization_contacts` (
	`categories` text DEFAULT (json_array()),
	`contact_id` text,
	`organization_id` text,
	`id` text PRIMARY KEY NOT NULL,
	`tombstone` integer DEFAULT 0,
	`status` text DEFAULT 'Active',
	`version` integer DEFAULT 1,
	`created_date` text,
	`created_time` text,
	`updated_date` text,
	`updated_time` text,
	FOREIGN KEY (`contact_id`) REFERENCES `contacts`(`id`) ON UPDATE no action ON DELETE no action,
	FOREIGN KEY (`organization_id`) REFERENCES `organizations`(`id`) ON UPDATE no action ON DELETE no action
);
--> statement-breakpoint
CREATE TABLE `organization_files` (
	`categories` text DEFAULT (json_array()),
	`organizaion_id` text,
	`organization_contact_id` text,
	`url` text,
	`name` text,
	`mime_type` text,
	`size` text,
	`type` text,
	`id` text PRIMARY KEY NOT NULL,
	`tombstone` integer DEFAULT 0,
	`status` text DEFAULT 'Active',
	`version` integer DEFAULT 1,
	`created_date` text,
	`created_time` text,
	`updated_date` text,
	`updated_time` text,
	FOREIGN KEY (`organizaion_id`) REFERENCES `organizations`(`id`) ON UPDATE no action ON DELETE no action,
	FOREIGN KEY (`organization_contact_id`) REFERENCES `organization_contacts`(`id`) ON UPDATE no action ON DELETE no action
);
--> statement-breakpoint
CREATE TABLE `organizations` (
	`categories` text DEFAULT (json_array()),
	`parent_organization_id` text DEFAULT (null),
	`name` text,
	`id` text PRIMARY KEY NOT NULL,
	`tombstone` integer DEFAULT 0,
	`status` text DEFAULT 'Active',
	`version` integer DEFAULT 1,
	`created_date` text,
	`created_time` text,
	`updated_date` text,
	`updated_time` text,
	FOREIGN KEY (`parent_organization_id`) REFERENCES `organizations`(`id`) ON UPDATE no action ON DELETE no action
);
--> statement-breakpoint
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
