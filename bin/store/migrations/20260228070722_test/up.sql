-- Your SQL goes here

CREATE INDEX "idx_classroom_course_stories_episodes_tombstone" ON "classroom_course_stories_episodes" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_stories_episodes_status" ON "classroom_course_stories_episodes" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_stories_episodes_previous_status" ON "classroom_course_stories_episodes" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_stories_episodes_version" ON "classroom_course_stories_episodes" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_stories_episodes_created_date" ON "classroom_course_stories_episodes" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_stories_episodes_updated_date" ON "classroom_course_stories_episodes" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_stories_episodes_organization_id" ON "classroom_course_stories_episodes" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_stories_episodes_created_by" ON "classroom_course_stories_episodes" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_stories_episodes_updated_by" ON "classroom_course_stories_episodes" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_stories_episodes_deleted_by" ON "classroom_course_stories_episodes" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_stories_episodes_requested_by" ON "classroom_course_stories_episodes" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_stories_episodes_tags" ON "classroom_course_stories_episodes" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_stories_episodes_categories" ON "classroom_course_stories_episodes" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_stories_episodes_code" ON "classroom_course_stories_episodes" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_stories_episodes_sensitivity_level" ON "classroom_course_stories_episodes" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_stories_episodes_classroom_id" ON "classroom_course_stories_episodes" USING btree("classroom_id");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_stories_episodes_course_id" ON "classroom_course_stories_episodes" USING btree("course_id");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_stories_episodes_story_id" ON "classroom_course_stories_episodes" USING btree("story_id");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_stories_episodes_episode_id" ON "classroom_course_stories_episodes" USING btree("episode_id");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_stories_episodes_start_date" ON "classroom_course_stories_episodes" USING btree("start_date");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_stories_episodes_order" ON "classroom_course_stories_episodes" USING btree("order");
--> statement-breakpoint
ALTER TABLE "classroom_course_stories_episodes" ADD CONSTRAINT "fk_classroom_course_stories_episodes_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classroom_course_stories_episodes" ADD CONSTRAINT "fk_classroom_course_stories_episodes_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classroom_course_stories_episodes" ADD CONSTRAINT "fk_classroom_course_stories_episodes_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classroom_course_stories_episodes" ADD CONSTRAINT "fk_classroom_course_stories_episodes_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classroom_course_stories_episodes" ADD CONSTRAINT "fk_classroom_course_stories_episodes_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classroom_course_stories_episodes" ADD CONSTRAINT "fk_classroom_course_stories_episodes_classroom_id" FOREIGN KEY ("classroom_id") REFERENCES "public"."classrooms"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classroom_course_stories_episodes" ADD CONSTRAINT "fk_classroom_course_stories_episodes_course_id" FOREIGN KEY ("course_id") REFERENCES "public"."courses"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classroom_course_stories_episodes" ADD CONSTRAINT "fk_classroom_course_stories_episodes_story_id" FOREIGN KEY ("story_id") REFERENCES "public"."stories"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classroom_course_stories_episodes" ADD CONSTRAINT "fk_classroom_course_stories_episodes_episode_id" FOREIGN KEY ("episode_id") REFERENCES "public"."episodes"("id") ON DELETE no action ON UPDATE no action;
