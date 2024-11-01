CREATE INDEX `id_idx` ON `config_sync` (`id`);--> statement-breakpoint
CREATE INDEX `tombstone_idx` ON `config_sync` (`tombstone`);--> statement-breakpoint
CREATE INDEX `status_idx` ON `config_sync` (`status`);--> statement-breakpoint
CREATE INDEX `version_idx` ON `config_sync` (`version`);--> statement-breakpoint
CREATE INDEX `created_date_idx` ON `config_sync` (`created_date`);--> statement-breakpoint
CREATE INDEX `created_time_idx` ON `config_sync` (`created_time`);--> statement-breakpoint
CREATE INDEX `updated_date_idx` ON `config_sync` (`updated_date`);--> statement-breakpoint
CREATE INDEX `updated_time_idx` ON `config_sync` (`updated_time`);