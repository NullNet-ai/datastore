ALTER TABLE `config_applications` ADD `version` integer DEFAULT 1;--> statement-breakpoint
CREATE INDEX `id_idx` ON `config_applications` (`id`);--> statement-breakpoint
CREATE INDEX `tombstone_idx` ON `config_applications` (`tombstone`);--> statement-breakpoint
CREATE INDEX `status_idx` ON `config_applications` (`status`);--> statement-breakpoint
CREATE INDEX `version_idx` ON `config_applications` (`version`);--> statement-breakpoint
CREATE INDEX `created_date_idx` ON `config_applications` (`created_date`);--> statement-breakpoint
CREATE INDEX `created_time_idx` ON `config_applications` (`created_time`);--> statement-breakpoint
CREATE INDEX `updated_date_idx` ON `config_applications` (`updated_date`);--> statement-breakpoint
CREATE INDEX `updated_time_idx` ON `config_applications` (`updated_time`);