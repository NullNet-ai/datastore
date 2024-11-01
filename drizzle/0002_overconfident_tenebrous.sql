DROP INDEX IF EXISTS `id_idx`;--> statement-breakpoint
DROP INDEX IF EXISTS `tombstone_idx`;--> statement-breakpoint
DROP INDEX IF EXISTS `status_idx`;--> statement-breakpoint
DROP INDEX IF EXISTS `version_idx`;--> statement-breakpoint
DROP INDEX IF EXISTS `created_date_idx`;--> statement-breakpoint
DROP INDEX IF EXISTS `created_time_idx`;--> statement-breakpoint
DROP INDEX IF EXISTS `updated_date_idx`;--> statement-breakpoint
DROP INDEX IF EXISTS `updated_time_idx`;--> statement-breakpoint
CREATE INDEX `config_sync_id_idx` ON `config_sync` (`id`);--> statement-breakpoint
CREATE INDEX `config_sync_tombstone_idx` ON `config_sync` (`tombstone`);--> statement-breakpoint
CREATE INDEX `config_sync_status_idx` ON `config_sync` (`status`);--> statement-breakpoint
CREATE INDEX `config_sync_version_idx` ON `config_sync` (`version`);--> statement-breakpoint
CREATE INDEX `config_sync_created_date_idx` ON `config_sync` (`created_date`);--> statement-breakpoint
CREATE INDEX `config_sync_created_time_idx` ON `config_sync` (`created_time`);--> statement-breakpoint
CREATE INDEX `config_sync_updated_date_idx` ON `config_sync` (`updated_date`);--> statement-breakpoint
CREATE INDEX `config_sync_updated_time_idx` ON `config_sync` (`updated_time`);