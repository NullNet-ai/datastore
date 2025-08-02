-- Your SQL goes here

CREATE INDEX "idx_files_filename" ON "files" USING btree(filename);
--> statement-breakpoint
CREATE INDEX "idx_files_etag" ON "files" USING btree(etag);
--> statement-breakpoint
CREATE INDEX "idx_files_mimetype" ON "files" USING btree(mimetype);
--> statement-breakpoint
CREATE INDEX "idx_files_tags" ON "files" USING btree(tags);
