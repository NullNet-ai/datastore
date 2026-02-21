-- Your SQL goes here

CREATE TABLE "game_choice_reports" (
    "course_id" TEXT,
    "course_title" TEXT DEFAULT '',
    "story_id" TEXT,
    "story_title" TEXT DEFAULT '',
    "episode_number" INTEGER DEFAULT 0,
    "topic" TEXT DEFAULT '',
    "question" TEXT DEFAULT '',
    "students" INTEGER DEFAULT 0,
    "correct_choice" TEXT DEFAULT '',
    "avg_tries" INTEGER DEFAULT 0,
    "report_id" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "stories" (
    "name" TEXT,
    "course_id" TEXT,
    "order" INTEGER DEFAULT 0,
    "description" TEXT DEFAULT '',
    "story_identifier" TEXT DEFAULT '',
    "bundle_file_name" TEXT DEFAULT '',
    "allowed_grades" JSONB DEFAULT '[]'::jsonb,
    "birthdate_cutoff" TEXT DEFAULT '',
    "must_be_born_on_or_after_birthdate_cutoff" TEXT DEFAULT '',
    "must_be_born_before_birthdate_cutoff" TEXT DEFAULT '',
    "allowed_ages" JSONB DEFAULT '[]'::jsonb,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "sponsorships" (
    "name" TEXT,
    "sponsor_organization_id" TEXT,
    "sponsor_sub_organization_id" TEXT,
    "sponsor_website" TEXT,
    "sponsor_status" TEXT,
    "start_date" TEXT,
    "start_time" TEXT DEFAULT '00:00',
    "end_date" TEXT,
    "end_time" TEXT DEFAULT '00:00',
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "user_progress" (
    "user_id" TEXT,
    "total_episodes_started" INTEGER DEFAULT 0,
    "total_episodes_completed" INTEGER DEFAULT 0,
    "total_chapters_started" INTEGER DEFAULT 0,
    "total_chapters_completed" INTEGER DEFAULT 0,
    "total_questions_answered" INTEGER DEFAULT 0,
    "last_activity" TIMESTAMPTZ,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "teacher_students" (
    "teacher_id" TEXT,
    "student_id" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "course_completion_reports" (
    "contact_code" TEXT,
    "role" TEXT,
    "student_course_status" TEXT,
    "status_date" TEXT,
    "status_time" TEXT DEFAULT '00:00',
    "report_id" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "classroom_course_stories_episodes" (
    "classroom_id" TEXT,
    "course_id" TEXT,
    "story_id" TEXT,
    "episode_id" TEXT,
    "start_date" TEXT,
    "order" INTEGER DEFAULT 0,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "organization_contact_user_roles" (
    "organization_contact_id" TEXT,
    "user_role_id" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "grid_filters" (
    "name" TEXT,
    "grid_id" TEXT,
    "link" TEXT DEFAULT '',
    "is_current" BOOLEAN DEFAULT false,
    "is_default" BOOLEAN DEFAULT false,
    "contact_id" TEXT,
    "account_organization_id" TEXT,
    "entity" TEXT,
    "columns" JSONB DEFAULT '[]'::jsonb,
    "groups" JSONB DEFAULT '[]'::jsonb,
    "sorts" JSONB DEFAULT '[]'::jsonb,
    "default_sorts" JSONB DEFAULT '[]'::jsonb,
    "advance_filters" JSONB DEFAULT '[]'::jsonb,
    "group_advance_filters" JSONB DEFAULT '[]'::jsonb,
    "filter_groups" JSONB DEFAULT '[]'::jsonb,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "student_progress" (
    "student_id" TEXT,
    "classroom_id" TEXT,
    "course_id" TEXT,
    "story_id" TEXT,
    "total_episodes_started" INTEGER DEFAULT 0,
    "total_episodes_completed" INTEGER DEFAULT 0,
    "total_chapters_started" INTEGER DEFAULT 0,
    "total_chapters_completed" INTEGER DEFAULT 0,
    "total_questions_answered" INTEGER DEFAULT 0,
    "total_episodes_completed_in_string" TEXT DEFAULT '',
    "total_episodes" INTEGER DEFAULT 0,
    "last_activity" TIMESTAMPTZ,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "conversation_topics" (
    "title" TEXT,
    "topic_contact_categories" TEXT[],
    "topic_contact_roles" TEXT[],
    "age_start" INTEGER,
    "age_end" INTEGER,
    "topic_status" TEXT,
    "is_show_to_new_contacts_only" BOOLEAN,
    "order" INTEGER DEFAULT 0,
    "source" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "faq_questions" (
    "faq_id" TEXT,
    "category_id" TEXT,
    "question" TEXT,
    "order" INTEGER DEFAULT 0,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "smtp_contacts" (
    "smtp_payload_id" TEXT,
    "contact_id" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "game_stats" (
    "contact_id" TEXT,
    "event_id" TEXT,
    "event_type" TEXT,
    "course_id" TEXT,
    "story_id" TEXT,
    "episode_id" TEXT,
    "classroom_id" TEXT,
    "chapter_number" TEXT,
    "volume_level" TEXT,
    "question" TEXT,
    "choices" JSONB DEFAULT '[]'::jsonb,
    "selected_choice" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "related_contacts" (
    "contact_id" TEXT,
    "student_id" TEXT,
    "is_verified" BOOLEAN DEFAULT false,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "episode_events" (
    "user_id" TEXT,
    "course_id" TEXT,
    "story_id" TEXT,
    "episode_id" TEXT,
    "event_type" TEXT,
    "game_status" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "teacher_schools" (
    "school_id" TEXT,
    "teacher_id" TEXT,
    "district_id" TEXT,
    "department_id" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "transport_provider_credentials" (
    "transport_provider_id" TEXT,
    "host" TEXT,
    "port" INTEGER,
    "is_secure" BOOLEAN DEFAULT false,
    "username" TEXT,
    "password" TEXT,
    "api_key" TEXT,
    "additional_config" JSONB DEFAULT '{}'::jsonb,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "smtp_attachments" (
    "filename" TEXT,
    "content_type" TEXT,
    "content_disposition" TEXT,
    "checksum" TEXT,
    "size" INTEGER,
    "content" TEXT,
    "content_id" TEXT,
    "cid" TEXT,
    "related" BOOLEAN,
    "headers" JSONB DEFAULT '{}'::jsonb,
    "smtp_payload_id" TEXT,
    "file_id" TEXT,
    "type_field" TEXT,
    "part_id" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "student_session_logs" (
    "user_id" TEXT,
    "action" TEXT,
    "session_token" TEXT,
    "timestamp" TIMESTAMPTZ,
    "device_details" TEXT,
    "ip_address" TEXT,
    "route" TEXT,
    "reason" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "game_states" (
    "user_id" TEXT DEFAULT '',
    "course_id" TEXT DEFAULT '',
    "story_id" TEXT DEFAULT '',
    "episode_id" TEXT DEFAULT '',
    "chapter_number" TEXT DEFAULT '',
    "game_token" JSONB DEFAULT '{}'::jsonb,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "game_choices" (
    "question_id" TEXT,
    "choice_text" TEXT DEFAULT '',
    "is_correct" BOOLEAN DEFAULT false,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "classroom_students" (
    "classroom_id" TEXT,
    "student_id" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "classroom_course_stories" (
    "classroom_id" TEXT,
    "course_id" TEXT,
    "story_id" TEXT,
    "start_date" TEXT,
    "order" INTEGER DEFAULT 0,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "avatar_selections" (
    "user_id" TEXT,
    "gender" TEXT,
    "skin_color" TEXT,
    "hair_color" TEXT,
    "eye_color" TEXT,
    "hair_style" TEXT,
    "clothing" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "user_settings" (
    "user_id" TEXT,
    "sound" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "chapter_events" (
    "user_id" TEXT,
    "course_id" TEXT,
    "story_id" TEXT,
    "episode_id" TEXT,
    "chapter_number" INTEGER,
    "event_type" TEXT,
    "game_status" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "classroom_course_episodes" (
    "classroom_id" TEXT,
    "course_id" TEXT,
    "story_id" TEXT,
    "episode_id" TEXT,
    "start_date" TEXT,
    "order" INTEGER DEFAULT 0,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "faq_categories" (
    "faq_id" TEXT,
    "category" TEXT,
    "order" INTEGER DEFAULT 0,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "locations" (
    "location_name" TEXT,
    "address_id" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "conversations" (
    "conversation_id" TEXT,
    "conversation_replica_name" TEXT,
    "last_message_activity" TIMESTAMPTZ,
    "tavus_conversation_id" TEXT,
    "tavus_conversation_replica_name" TEXT,
    "tavus_conversation_start_date" TEXT,
    "tavus_conversation_end_date" TEXT,
    "tavus_conversation_date_start" TIMESTAMPTZ,
    "tavus_conversation_date_end" TIMESTAMPTZ,
    "hume_ai_job_id" TEXT,
    "hume_ai_sync_status" TEXT,
    "user_id" TEXT,
    "contact_id" TEXT,
    "conversation_topic_id" TEXT,
    "session_recording_url" TEXT,
    "method" TEXT,
    "perception" JSONB DEFAULT '[]'::jsonb,
    "duration" TEXT,
    "summary" TEXT,
    "transcript" JSONB DEFAULT '[]'::jsonb,
    "source" TEXT,
    "app_version" TEXT,
    "ip_address" TEXT,
    "is_skip_select_topic" BOOLEAN DEFAULT false,
    "cybertipline_report_report_annotations" TEXT,
    "topic_name" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "common_session_logs" (
    "user_id" TEXT,
    "action" TEXT,
    "session_token" TEXT,
    "timestamp" TEXT,
    "device_details" TEXT,
    "ip_address" TEXT,
    "route" TEXT,
    "reason" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "game_questions" (
    "course_id" TEXT,
    "course_title" TEXT DEFAULT '',
    "story_id" TEXT,
    "story_title" TEXT DEFAULT '',
    "episode_number" INTEGER DEFAULT 0,
    "question" TEXT DEFAULT '',
    "topic" TEXT DEFAULT '',
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "communication_templates" (
    "name" TEXT,
    "communication_template_status" TEXT,
    "event" TEXT,
    "content" TEXT,
    "subject" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "classroom_courses" (
    "classroom_id" TEXT,
    "course_id" TEXT,
    "start_date" TEXT,
    "order" INTEGER DEFAULT 0,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "student_stats" (
    "student_id" TEXT,
    "classroom_id" TEXT,
    "course_id" TEXT,
    "story_id" TEXT,
    "total_episodes_started" INTEGER DEFAULT 0,
    "total_episodes_completed" INTEGER DEFAULT 0,
    "total_chapters_started" INTEGER DEFAULT 0,
    "total_chapters_completed" INTEGER DEFAULT 0,
    "total_questions_answered" INTEGER DEFAULT 0,
    "total_episodes_completed_in_string" TEXT DEFAULT '',
    "total_episodes" INTEGER DEFAULT 0,
    "last_activity" TIMESTAMPTZ,
    "last_activity_date" TIMESTAMPTZ,
    "last_activity_time" TEXT DEFAULT '00:00',
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "common_sessions" (
    "user_id" TEXT,
    "session_token" TEXT,
    "route" TEXT,
    "session_status" TEXT,
    "activity_status" TEXT,
    "last_activity_at" TEXT,
    "expires_at" TEXT,
    "device_details" TEXT,
    "ip_address" TEXT,
    "portal_application_type" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "episodes" (
    "name" TEXT,
    "story_id" TEXT,
    "order" INTEGER DEFAULT 0,
    "course_id" TEXT,
    "description" TEXT DEFAULT '',
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "smtp_attachment_headers" (
    "header_key" TEXT,
    "header_value" TEXT,
    "smtp_attachment_id" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "courses" (
    "title" TEXT,
    "order" INTEGER DEFAULT 0,
    "description" TEXT,
    "is_show_assistant" BOOLEAN DEFAULT true,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "smtp_payloads" (
    "sender" TEXT,
    "recipients" JSONB DEFAULT '[]'::jsonb,
    "cc_recipients" JSONB DEFAULT '[]'::jsonb,
    "bcc_recipients" JSONB DEFAULT '[]'::jsonb,
    "subject" TEXT,
    "html" TEXT,
    "parent_id" TEXT,
    "attachment_ids" JSONB DEFAULT '[]'::jsonb,
    "thread_id" TEXT,
    "send_strategy" TEXT,
    "priority" TEXT,
    "transport_provider_id" TEXT,
    "source" TEXT,
    "method" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "smtp_response_links" (
    "smtp_payload_id" TEXT,
    "callback_url" TEXT,
    "expiry" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "smtp_transactions" (
    "smtp_payload_id" TEXT,
    "transaction_status" TEXT,
    "response_message" TEXT,
    "trigger_type" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "report_files" (
    "report_id" TEXT,
    "cybertipline_report_file_id" TEXT DEFAULT '',
    "evidence_url" TEXT DEFAULT '',
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "invitations" (
    "account_id" TEXT,
    "expiration_date" TEXT,
    "expiration_time" TEXT DEFAULT '00:00',
    "account_organization_id" TEXT,
    "related_contact_id" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "faq_answers" (
    "faq_id" TEXT,
    "category_id" TEXT,
    "question_id" TEXT,
    "answer" TEXT,
    "order" INTEGER DEFAULT 0,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "classrooms" (
    "name" TEXT,
    "description" TEXT,
    "grade" TEXT,
    "color" TEXT,
    "avatar" TEXT,
    "department_id" TEXT,
    "district_id" TEXT,
    "school_id" TEXT,
    "teacher_id" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "notifications" (
    "title" TEXT,
    "description" TEXT,
    "event_timestamp" TEXT,
    "link" TEXT DEFAULT '',
    "icon" TEXT DEFAULT '',
    "source" TEXT,
    "is_pinned" BOOLEAN DEFAULT false,
    "recipient_id" TEXT,
    "actions" JSONB DEFAULT '[]'::jsonb,
    "notification_status" TEXT DEFAULT 'unread',
    "priority_label" TEXT DEFAULT 'low',
    "priority_level" INTEGER DEFAULT 0,
    "expiry_date" TEXT DEFAULT '',
    "expiry_time" TEXT DEFAULT '00:00',
    "metadata" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "classroom_stats" (
    "classroom_id" TEXT,
    "story_id" TEXT,
    "student_id" TEXT,
    "course_id" TEXT,
    "number_of_students" TEXT DEFAULT '0',
    "student_completed" TEXT DEFAULT '0/0',
    "progress_status" TEXT DEFAULT 'Not Started',
    "progress_start_date" TIMESTAMPTZ,
    "progress_start_time" TEXT DEFAULT '00:00',
    "progress_start_date_string" TEXT DEFAULT '',
    "completed_date" TIMESTAMPTZ,
    "completed_time" TEXT DEFAULT '00:00',
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "onboarding_contacts" (
    "contact_id" TEXT,
    "is_done" BOOLEAN DEFAULT false,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "conversation_messages" (
    "conversation_id" TEXT,
    "content" TEXT,
    "role" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "incident_reports" (
    "user_id" TEXT,
    "report_status" TEXT,
    "source" TEXT,
    "app_version" TEXT,
    "conversation_id" TEXT,
    "cybertipline_report_id" TEXT,
    "cybertipline_report_case_number" TEXT,
    "cybertipline_report_type" TEXT,
    "cybertipline_report_status" TEXT,
    "cybertipline_api_response_status_code" TEXT,
    "additional_info" TEXT,
    "reporter_first_name" TEXT,
    "reporter_last_name" TEXT,
    "reporter_email" TEXT,
    "reporter_type" TEXT,
    "incident_date" TEXT,
    "incident_time" TEXT,
    "incident_timezone" TEXT,
    "incident_type" TEXT,
    "incident_escalate_to_high_priority" TEXT,
    "incident_location_type" TEXT,
    "incident_location_type_value" JSONB DEFAULT '{}'::jsonb,
    "reported_person_first_name" TEXT,
    "reported_person_last_name" TEXT,
    "reported_person_email" TEXT,
    "reported_person_screen_name" TEXT,
    "reported_person_profile_url" TEXT,
    "reported_person_address_street" TEXT,
    "reported_person_address_city" TEXT,
    "reported_person_address_zip_code" TEXT,
    "reported_person_address_state" TEXT,
    "reported_person_address_non_usa_state" TEXT,
    "reported_person_address_country" TEXT,
    "reported_person_phone_country_code" TEXT,
    "reported_person_phone_extension" TEXT,
    "ip_address" TEXT,
    "date_submission" TIMESTAMPTZ,
    "submission_date" TIMESTAMPTZ,
    "submission_time" TEXT DEFAULT '00:00',
    "evidence_urls" JSONB DEFAULT '{}'::jsonb,
    "reported_person_username" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "reports" (
    "report_type" TEXT[],
    "job_status" TEXT,
    "start_date" TEXT,
    "start_time" TEXT DEFAULT '00:00',
    "end_date" TEXT,
    "end_time" TEXT DEFAULT '00:00',
    "progress" TEXT DEFAULT '0%',
    "reason" TEXT DEFAULT '',
    "department_id" TEXT,
    "district_id" TEXT,
    "school_id" TEXT,
    "course_id" TEXT,
    "story_id" TEXT,
    "record_count" TEXT DEFAULT '-',
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "game_responses" (
    "user_id" TEXT,
    "course_id" TEXT,
    "story_id" TEXT,
    "episode_id" TEXT,
    "chapter_number" INTEGER,
    "question_text" TEXT,
    "selected_answer" TEXT,
    "game_status" TEXT,
    "options" JSONB DEFAULT '[]'::jsonb,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "emotion_predictions" (
    "conversation_id" TEXT,
    "model" TEXT,
    "prob" TEXT,
    "box" JSONB DEFAULT '{}'::jsonb,
    "emotions" JSONB DEFAULT '[]'::jsonb,
    "time" TEXT,
    "id" TEXT,
    "timestamp" TIMESTAMPTZ,
    "hypertable_timestamp" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT
);
--> statement-breakpoint
CREATE TABLE "game_review_question_reports" (
    "course_id" TEXT,
    "course_title" TEXT DEFAULT '',
    "story_id" TEXT,
    "story_title" TEXT DEFAULT '',
    "episode_number" INTEGER DEFAULT 0,
    "question" TEXT DEFAULT '',
    "students" INTEGER DEFAULT 0,
    "strongly_disagree" INTEGER DEFAULT 0,
    "disagree" INTEGER DEFAULT 0,
    "agree" INTEGER DEFAULT 0,
    "strongly_agree" INTEGER DEFAULT 0,
    "report_id" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "student_sessions" (
    "user_id" TEXT,
    "session_token" TEXT,
    "route" TEXT,
    "student_status" TEXT,
    "activity_status" TEXT,
    "last_activity_at" TIMESTAMPTZ,
    "expires_at" TIMESTAMPTZ,
    "device_details" TEXT,
    "ip_address" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "transport_providers" (
    "name" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "faqs" (
    "target_app" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "report_child_victims" (
    "report_id" TEXT,
    "first_name" TEXT DEFAULT '',
    "last_name" TEXT DEFAULT '',
    "birth_date" TEXT DEFAULT '',
    "email" TEXT DEFAULT '',
    "address_street" TEXT DEFAULT '',
    "address_city" TEXT DEFAULT '',
    "address_zip_code" TEXT DEFAULT '',
    "address_state" TEXT DEFAULT '',
    "address_non_usa_state" TEXT DEFAULT '',
    "address_country" TEXT DEFAULT '',
    "phone_country_code" TEXT DEFAULT '',
    "phone_extension" TEXT DEFAULT '',
    "screen_name" TEXT DEFAULT '',
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
ALTER TABLE "organization_accounts" ADD COLUMN "email" TEXT;
--> statement-breakpoint
ALTER TABLE "organization_accounts" ADD COLUMN "password" TEXT;
--> statement-breakpoint
ALTER TABLE "organization_accounts" ADD COLUMN "account_organization_id" TEXT;
--> statement-breakpoint
ALTER TABLE "contacts" ADD COLUMN "username" TEXT;
--> statement-breakpoint
ALTER TABLE "contacts" ADD COLUMN "address_id" TEXT;
--> statement-breakpoint
ALTER TABLE "contacts" ADD COLUMN "department_id" TEXT;
--> statement-breakpoint
ALTER TABLE "contacts" ADD COLUMN "district_id" TEXT;
--> statement-breakpoint
ALTER TABLE "contacts" ADD COLUMN "school_id" TEXT;
--> statement-breakpoint
ALTER TABLE "contacts" ADD COLUMN "grade_level" TEXT;
--> statement-breakpoint
ALTER TABLE "contacts" ADD COLUMN "school_year" TEXT;
--> statement-breakpoint
ALTER TABLE "contacts" ADD COLUMN "teacher_id" TEXT;
--> statement-breakpoint
ALTER TABLE "contacts" ADD COLUMN "last_assistant_toggle_option_value" BOOLEAN;
--> statement-breakpoint
ALTER TABLE "organization_contacts" ADD COLUMN "contact_organization_id" TEXT;
--> statement-breakpoint
ALTER TABLE "organization_contacts" ADD COLUMN "is_primary" BOOLEAN;
--> statement-breakpoint
ALTER TABLE "contact_phone_numbers" ADD COLUMN "raw_phone_number" TEXT;
--> statement-breakpoint
ALTER TABLE "contact_phone_numbers" ADD COLUMN "iso_code" TEXT;
--> statement-breakpoint
ALTER TABLE "contact_phone_numbers" ADD COLUMN "country_code" TEXT;
--> statement-breakpoint
ALTER TABLE "contact_phone_numbers" ADD COLUMN "is_primary" BOOLEAN;
--> statement-breakpoint
ALTER TABLE "organizations" ADD COLUMN "skyll_id" TEXT;
--> statement-breakpoint
ALTER TABLE "organizations" ADD COLUMN "department_id" TEXT;
--> statement-breakpoint
ALTER TABLE "organizations" ADD COLUMN "district_id" TEXT;
--> statement-breakpoint
ALTER TABLE "organizations" ADD COLUMN "school_id" TEXT;
--> statement-breakpoint
ALTER TABLE "organizations" ADD COLUMN "city" TEXT;
--> statement-breakpoint
ALTER TABLE "organizations" ADD COLUMN "county" TEXT;
--> statement-breakpoint
ALTER TABLE "organizations" ADD COLUMN "state" TEXT;
--> statement-breakpoint
ALTER TABLE "organizations" ADD COLUMN "school_identifier" TEXT;
--> statement-breakpoint
ALTER TABLE "organizations" ADD COLUMN "district_identifier" TEXT;
--> statement-breakpoint
ALTER TABLE "organizations" ADD COLUMN "superintendent_id" TEXT;
--> statement-breakpoint
ALTER TABLE "organizations" ADD COLUMN "principal_id" TEXT;
--> statement-breakpoint
ALTER TABLE "postgres_channels" ADD COLUMN "channel_timestamp" TIMESTAMPTZ;
--> statement-breakpoint
ALTER TABLE "contact_phone_numbers" DROP COLUMN "phone_number_raw";
--> statement-breakpoint
CREATE INDEX "idx_game_choice_reports_tombstone" ON "game_choice_reports" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_game_choice_reports_status" ON "game_choice_reports" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_game_choice_reports_previous_status" ON "game_choice_reports" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_game_choice_reports_version" ON "game_choice_reports" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_game_choice_reports_created_date" ON "game_choice_reports" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_game_choice_reports_updated_date" ON "game_choice_reports" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_game_choice_reports_organization_id" ON "game_choice_reports" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_game_choice_reports_created_by" ON "game_choice_reports" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_game_choice_reports_updated_by" ON "game_choice_reports" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_game_choice_reports_deleted_by" ON "game_choice_reports" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_game_choice_reports_requested_by" ON "game_choice_reports" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_game_choice_reports_tags" ON "game_choice_reports" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_game_choice_reports_categories" ON "game_choice_reports" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_game_choice_reports_code" ON "game_choice_reports" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_game_choice_reports_sensitivity_level" ON "game_choice_reports" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_game_choice_reports_course_id" ON "game_choice_reports" USING btree("course_id");
--> statement-breakpoint
CREATE INDEX "idx_game_choice_reports_course_title" ON "game_choice_reports" USING btree("course_title");
--> statement-breakpoint
CREATE INDEX "idx_game_choice_reports_story_id" ON "game_choice_reports" USING btree("story_id");
--> statement-breakpoint
CREATE INDEX "idx_game_choice_reports_story_title" ON "game_choice_reports" USING btree("story_title");
--> statement-breakpoint
CREATE INDEX "idx_game_choice_reports_episode_number" ON "game_choice_reports" USING btree("episode_number");
--> statement-breakpoint
CREATE INDEX "idx_game_choice_reports_topic" ON "game_choice_reports" USING btree("topic");
--> statement-breakpoint
CREATE INDEX "idx_game_choice_reports_question" ON "game_choice_reports" USING btree("question");
--> statement-breakpoint
CREATE INDEX "idx_game_choice_reports_students" ON "game_choice_reports" USING btree("students");
--> statement-breakpoint
CREATE INDEX "idx_game_choice_reports_correct_choice" ON "game_choice_reports" USING btree("correct_choice");
--> statement-breakpoint
CREATE INDEX "idx_game_choice_reports_avg_tries" ON "game_choice_reports" USING btree("avg_tries");
--> statement-breakpoint
CREATE INDEX "idx_game_choice_reports_report_id" ON "game_choice_reports" USING btree("report_id");
--> statement-breakpoint
CREATE INDEX "idx_stories_tombstone" ON "stories" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_stories_status" ON "stories" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_stories_previous_status" ON "stories" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_stories_version" ON "stories" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_stories_created_date" ON "stories" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_stories_updated_date" ON "stories" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_stories_organization_id" ON "stories" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_stories_created_by" ON "stories" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_stories_updated_by" ON "stories" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_stories_deleted_by" ON "stories" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_stories_requested_by" ON "stories" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_stories_tags" ON "stories" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_stories_categories" ON "stories" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_stories_code" ON "stories" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_stories_sensitivity_level" ON "stories" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_stories_name" ON "stories" USING btree("name");
--> statement-breakpoint
CREATE INDEX "idx_stories_course_id" ON "stories" USING btree("course_id");
--> statement-breakpoint
CREATE INDEX "idx_stories_order" ON "stories" USING btree("order");
--> statement-breakpoint
CREATE INDEX "idx_stories_description" ON "stories" USING btree("description");
--> statement-breakpoint
CREATE INDEX "idx_stories_story_identifier" ON "stories" USING btree("story_identifier");
--> statement-breakpoint
CREATE INDEX "idx_stories_bundle_file_name" ON "stories" USING btree("bundle_file_name");
--> statement-breakpoint
CREATE INDEX "idx_stories_allowed_grades" ON "stories" USING btree("allowed_grades");
--> statement-breakpoint
CREATE INDEX "idx_stories_birthdate_cutoff" ON "stories" USING btree("birthdate_cutoff");
--> statement-breakpoint
CREATE INDEX "idx_stories_allowed_ages" ON "stories" USING btree("allowed_ages");
--> statement-breakpoint
CREATE INDEX "idx_sponsorships_tombstone" ON "sponsorships" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_sponsorships_status" ON "sponsorships" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_sponsorships_previous_status" ON "sponsorships" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_sponsorships_version" ON "sponsorships" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_sponsorships_created_date" ON "sponsorships" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_sponsorships_updated_date" ON "sponsorships" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_sponsorships_organization_id" ON "sponsorships" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_sponsorships_created_by" ON "sponsorships" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_sponsorships_updated_by" ON "sponsorships" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_sponsorships_deleted_by" ON "sponsorships" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_sponsorships_requested_by" ON "sponsorships" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_sponsorships_tags" ON "sponsorships" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_sponsorships_categories" ON "sponsorships" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_sponsorships_code" ON "sponsorships" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_sponsorships_sensitivity_level" ON "sponsorships" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_sponsorships_name" ON "sponsorships" USING btree("name");
--> statement-breakpoint
CREATE INDEX "idx_sponsorships_sponsor_organization_id" ON "sponsorships" USING btree("sponsor_organization_id");
--> statement-breakpoint
CREATE INDEX "idx_sponsorships_sponsor_sub_organization_id" ON "sponsorships" USING btree("sponsor_sub_organization_id");
--> statement-breakpoint
CREATE INDEX "idx_sponsorships_sponsor_website" ON "sponsorships" USING btree("sponsor_website");
--> statement-breakpoint
CREATE INDEX "idx_sponsorships_sponsor_status" ON "sponsorships" USING btree("sponsor_status");
--> statement-breakpoint
CREATE INDEX "idx_sponsorships_start_date" ON "sponsorships" USING btree("start_date");
--> statement-breakpoint
CREATE INDEX "idx_sponsorships_start_time" ON "sponsorships" USING btree("start_time");
--> statement-breakpoint
CREATE INDEX "idx_sponsorships_end_date" ON "sponsorships" USING btree("end_date");
--> statement-breakpoint
CREATE INDEX "idx_sponsorships_end_time" ON "sponsorships" USING btree("end_time");
--> statement-breakpoint
CREATE INDEX "idx_user_progress_tombstone" ON "user_progress" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_user_progress_status" ON "user_progress" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_user_progress_previous_status" ON "user_progress" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_user_progress_version" ON "user_progress" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_user_progress_created_date" ON "user_progress" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_user_progress_updated_date" ON "user_progress" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_user_progress_organization_id" ON "user_progress" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_user_progress_created_by" ON "user_progress" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_user_progress_updated_by" ON "user_progress" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_user_progress_deleted_by" ON "user_progress" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_user_progress_requested_by" ON "user_progress" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_user_progress_tags" ON "user_progress" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_user_progress_categories" ON "user_progress" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_user_progress_code" ON "user_progress" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_user_progress_sensitivity_level" ON "user_progress" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_user_progress_user_id" ON "user_progress" USING btree("user_id");
--> statement-breakpoint
CREATE INDEX "idx_user_progress_total_episodes_started" ON "user_progress" USING btree("total_episodes_started");
--> statement-breakpoint
CREATE INDEX "idx_user_progress_total_episodes_completed" ON "user_progress" USING btree("total_episodes_completed");
--> statement-breakpoint
CREATE INDEX "idx_user_progress_total_chapters_started" ON "user_progress" USING btree("total_chapters_started");
--> statement-breakpoint
CREATE INDEX "idx_user_progress_total_chapters_completed" ON "user_progress" USING btree("total_chapters_completed");
--> statement-breakpoint
CREATE INDEX "idx_user_progress_total_questions_answered" ON "user_progress" USING btree("total_questions_answered");
--> statement-breakpoint
CREATE INDEX "idx_user_progress_last_activity" ON "user_progress" USING btree("last_activity");
--> statement-breakpoint
CREATE INDEX "idx_teacher_students_tombstone" ON "teacher_students" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_teacher_students_status" ON "teacher_students" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_teacher_students_previous_status" ON "teacher_students" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_teacher_students_version" ON "teacher_students" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_teacher_students_created_date" ON "teacher_students" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_teacher_students_updated_date" ON "teacher_students" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_teacher_students_organization_id" ON "teacher_students" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_teacher_students_created_by" ON "teacher_students" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_teacher_students_updated_by" ON "teacher_students" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_teacher_students_deleted_by" ON "teacher_students" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_teacher_students_requested_by" ON "teacher_students" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_teacher_students_tags" ON "teacher_students" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_teacher_students_categories" ON "teacher_students" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_teacher_students_code" ON "teacher_students" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_teacher_students_sensitivity_level" ON "teacher_students" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_teacher_students_teacher_id" ON "teacher_students" USING btree("teacher_id");
--> statement-breakpoint
CREATE INDEX "idx_teacher_students_student_id" ON "teacher_students" USING btree("student_id");
--> statement-breakpoint
CREATE INDEX "idx_course_completion_reports_tombstone" ON "course_completion_reports" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_course_completion_reports_status" ON "course_completion_reports" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_course_completion_reports_previous_status" ON "course_completion_reports" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_course_completion_reports_version" ON "course_completion_reports" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_course_completion_reports_created_date" ON "course_completion_reports" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_course_completion_reports_updated_date" ON "course_completion_reports" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_course_completion_reports_organization_id" ON "course_completion_reports" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_course_completion_reports_created_by" ON "course_completion_reports" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_course_completion_reports_updated_by" ON "course_completion_reports" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_course_completion_reports_deleted_by" ON "course_completion_reports" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_course_completion_reports_requested_by" ON "course_completion_reports" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_course_completion_reports_tags" ON "course_completion_reports" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_course_completion_reports_categories" ON "course_completion_reports" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_course_completion_reports_code" ON "course_completion_reports" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_course_completion_reports_sensitivity_level" ON "course_completion_reports" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_course_completion_reports_contact_code" ON "course_completion_reports" USING btree("contact_code");
--> statement-breakpoint
CREATE INDEX "idx_course_completion_reports_role" ON "course_completion_reports" USING btree("role");
--> statement-breakpoint
CREATE INDEX "idx_course_completion_reports_student_course_status" ON "course_completion_reports" USING btree("student_course_status");
--> statement-breakpoint
CREATE INDEX "idx_course_completion_reports_status_date" ON "course_completion_reports" USING btree("status_date");
--> statement-breakpoint
CREATE INDEX "idx_course_completion_reports_status_time" ON "course_completion_reports" USING btree("status_time");
--> statement-breakpoint
CREATE INDEX "idx_course_completion_reports_report_id" ON "course_completion_reports" USING btree("report_id");
--> statement-breakpoint
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
CREATE INDEX "idx_organization_contact_user_roles_tombstone" ON "organization_contact_user_roles" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_organization_contact_user_roles_status" ON "organization_contact_user_roles" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_organization_contact_user_roles_previous_status" ON "organization_contact_user_roles" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_organization_contact_user_roles_version" ON "organization_contact_user_roles" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_organization_contact_user_roles_created_date" ON "organization_contact_user_roles" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_organization_contact_user_roles_updated_date" ON "organization_contact_user_roles" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_organization_contact_user_roles_organization_id" ON "organization_contact_user_roles" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_organization_contact_user_roles_created_by" ON "organization_contact_user_roles" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_organization_contact_user_roles_updated_by" ON "organization_contact_user_roles" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_organization_contact_user_roles_deleted_by" ON "organization_contact_user_roles" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_organization_contact_user_roles_requested_by" ON "organization_contact_user_roles" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_organization_contact_user_roles_tags" ON "organization_contact_user_roles" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_organization_contact_user_roles_categories" ON "organization_contact_user_roles" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_organization_contact_user_roles_code" ON "organization_contact_user_roles" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_organization_contact_user_roles_sensitivity_level" ON "organization_contact_user_roles" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_organization_contact_user_roles_organization_contact_id" ON "organization_contact_user_roles" USING btree("organization_contact_id");
--> statement-breakpoint
CREATE INDEX "idx_organization_contact_user_roles_user_role_id" ON "organization_contact_user_roles" USING btree("user_role_id");
--> statement-breakpoint
CREATE INDEX "idx_grid_filters_tombstone" ON "grid_filters" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_grid_filters_status" ON "grid_filters" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_grid_filters_previous_status" ON "grid_filters" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_grid_filters_version" ON "grid_filters" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_grid_filters_created_date" ON "grid_filters" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_grid_filters_updated_date" ON "grid_filters" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_grid_filters_organization_id" ON "grid_filters" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_grid_filters_created_by" ON "grid_filters" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_grid_filters_updated_by" ON "grid_filters" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_grid_filters_deleted_by" ON "grid_filters" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_grid_filters_requested_by" ON "grid_filters" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_grid_filters_tags" ON "grid_filters" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_grid_filters_categories" ON "grid_filters" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_grid_filters_code" ON "grid_filters" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_grid_filters_sensitivity_level" ON "grid_filters" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_grid_filters_name" ON "grid_filters" USING btree("name");
--> statement-breakpoint
CREATE INDEX "idx_grid_filters_grid_id" ON "grid_filters" USING btree("grid_id");
--> statement-breakpoint
CREATE INDEX "idx_grid_filters_link" ON "grid_filters" USING btree("link");
--> statement-breakpoint
CREATE INDEX "idx_grid_filters_is_current" ON "grid_filters" USING btree("is_current");
--> statement-breakpoint
CREATE INDEX "idx_grid_filters_is_default" ON "grid_filters" USING btree("is_default");
--> statement-breakpoint
CREATE INDEX "idx_grid_filters_contact_id" ON "grid_filters" USING btree("contact_id");
--> statement-breakpoint
CREATE INDEX "idx_grid_filters_entity" ON "grid_filters" USING btree("entity");
--> statement-breakpoint
CREATE INDEX "idx_student_progress_tombstone" ON "student_progress" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_student_progress_status" ON "student_progress" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_student_progress_previous_status" ON "student_progress" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_student_progress_version" ON "student_progress" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_student_progress_created_date" ON "student_progress" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_student_progress_updated_date" ON "student_progress" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_student_progress_organization_id" ON "student_progress" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_student_progress_created_by" ON "student_progress" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_student_progress_updated_by" ON "student_progress" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_student_progress_deleted_by" ON "student_progress" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_student_progress_requested_by" ON "student_progress" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_student_progress_tags" ON "student_progress" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_student_progress_categories" ON "student_progress" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_student_progress_code" ON "student_progress" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_student_progress_sensitivity_level" ON "student_progress" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_student_progress_student_id" ON "student_progress" USING btree("student_id");
--> statement-breakpoint
CREATE INDEX "idx_student_progress_classroom_id" ON "student_progress" USING btree("classroom_id");
--> statement-breakpoint
CREATE INDEX "idx_student_progress_course_id" ON "student_progress" USING btree("course_id");
--> statement-breakpoint
CREATE INDEX "idx_student_progress_story_id" ON "student_progress" USING btree("story_id");
--> statement-breakpoint
CREATE INDEX "idx_student_progress_total_episodes_started" ON "student_progress" USING btree("total_episodes_started");
--> statement-breakpoint
CREATE INDEX "idx_student_progress_total_episodes_completed" ON "student_progress" USING btree("total_episodes_completed");
--> statement-breakpoint
CREATE INDEX "idx_student_progress_total_chapters_started" ON "student_progress" USING btree("total_chapters_started");
--> statement-breakpoint
CREATE INDEX "idx_student_progress_total_chapters_completed" ON "student_progress" USING btree("total_chapters_completed");
--> statement-breakpoint
CREATE INDEX "idx_student_progress_total_questions_answered" ON "student_progress" USING btree("total_questions_answered");
--> statement-breakpoint
CREATE INDEX "idx_student_progress_total_episodes_completed_in_string" ON "student_progress" USING btree("total_episodes_completed_in_string");
--> statement-breakpoint
CREATE INDEX "idx_student_progress_total_episodes" ON "student_progress" USING btree("total_episodes");
--> statement-breakpoint
CREATE INDEX "idx_student_progress_last_activity" ON "student_progress" USING btree("last_activity");
--> statement-breakpoint
CREATE INDEX "idx_conversation_topics_tombstone" ON "conversation_topics" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_conversation_topics_status" ON "conversation_topics" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_conversation_topics_previous_status" ON "conversation_topics" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_conversation_topics_version" ON "conversation_topics" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_conversation_topics_created_date" ON "conversation_topics" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_conversation_topics_updated_date" ON "conversation_topics" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_conversation_topics_organization_id" ON "conversation_topics" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_conversation_topics_created_by" ON "conversation_topics" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_conversation_topics_updated_by" ON "conversation_topics" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_conversation_topics_deleted_by" ON "conversation_topics" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_conversation_topics_requested_by" ON "conversation_topics" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_conversation_topics_tags" ON "conversation_topics" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_conversation_topics_categories" ON "conversation_topics" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_conversation_topics_code" ON "conversation_topics" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_conversation_topics_sensitivity_level" ON "conversation_topics" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_conversation_topics_title" ON "conversation_topics" USING btree("title");
--> statement-breakpoint
CREATE INDEX "idx_conversation_topics_topic_contact_categories" ON "conversation_topics" USING btree("topic_contact_categories");
--> statement-breakpoint
CREATE INDEX "idx_conversation_topics_topic_contact_roles" ON "conversation_topics" USING btree("topic_contact_roles");
--> statement-breakpoint
CREATE INDEX "idx_conversation_topics_age_start" ON "conversation_topics" USING btree("age_start");
--> statement-breakpoint
CREATE INDEX "idx_conversation_topics_age_end" ON "conversation_topics" USING btree("age_end");
--> statement-breakpoint
CREATE INDEX "idx_conversation_topics_topic_status" ON "conversation_topics" USING btree("topic_status");
--> statement-breakpoint
CREATE INDEX "idx_conversation_topics_is_show_to_new_contacts_only" ON "conversation_topics" USING btree("is_show_to_new_contacts_only");
--> statement-breakpoint
CREATE INDEX "idx_conversation_topics_order" ON "conversation_topics" USING btree("order");
--> statement-breakpoint
CREATE INDEX "idx_conversation_topics_source" ON "conversation_topics" USING btree("source");
--> statement-breakpoint
CREATE INDEX "idx_faq_questions_tombstone" ON "faq_questions" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_faq_questions_status" ON "faq_questions" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_faq_questions_previous_status" ON "faq_questions" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_faq_questions_version" ON "faq_questions" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_faq_questions_created_date" ON "faq_questions" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_faq_questions_updated_date" ON "faq_questions" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_faq_questions_organization_id" ON "faq_questions" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_faq_questions_created_by" ON "faq_questions" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_faq_questions_updated_by" ON "faq_questions" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_faq_questions_deleted_by" ON "faq_questions" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_faq_questions_requested_by" ON "faq_questions" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_faq_questions_tags" ON "faq_questions" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_faq_questions_categories" ON "faq_questions" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_faq_questions_code" ON "faq_questions" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_faq_questions_sensitivity_level" ON "faq_questions" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_faq_questions_faq_id" ON "faq_questions" USING btree("faq_id");
--> statement-breakpoint
CREATE INDEX "idx_faq_questions_category_id" ON "faq_questions" USING btree("category_id");
--> statement-breakpoint
CREATE INDEX "idx_faq_questions_question" ON "faq_questions" USING btree("question");
--> statement-breakpoint
CREATE INDEX "idx_faq_questions_order" ON "faq_questions" USING btree("order");
--> statement-breakpoint
CREATE INDEX "idx_smtp_contacts_tombstone" ON "smtp_contacts" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_smtp_contacts_status" ON "smtp_contacts" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_smtp_contacts_previous_status" ON "smtp_contacts" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_smtp_contacts_version" ON "smtp_contacts" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_smtp_contacts_created_date" ON "smtp_contacts" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_smtp_contacts_updated_date" ON "smtp_contacts" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_smtp_contacts_organization_id" ON "smtp_contacts" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_smtp_contacts_created_by" ON "smtp_contacts" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_smtp_contacts_updated_by" ON "smtp_contacts" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_smtp_contacts_deleted_by" ON "smtp_contacts" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_smtp_contacts_requested_by" ON "smtp_contacts" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_smtp_contacts_tags" ON "smtp_contacts" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_smtp_contacts_categories" ON "smtp_contacts" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_smtp_contacts_code" ON "smtp_contacts" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_smtp_contacts_sensitivity_level" ON "smtp_contacts" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_smtp_contacts_smtp_payload_id" ON "smtp_contacts" USING btree("smtp_payload_id");
--> statement-breakpoint
CREATE INDEX "idx_smtp_contacts_contact_id" ON "smtp_contacts" USING btree("contact_id");
--> statement-breakpoint
CREATE INDEX "idx_game_stats_tombstone" ON "game_stats" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_game_stats_status" ON "game_stats" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_game_stats_previous_status" ON "game_stats" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_game_stats_version" ON "game_stats" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_game_stats_created_date" ON "game_stats" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_game_stats_updated_date" ON "game_stats" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_game_stats_organization_id" ON "game_stats" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_game_stats_created_by" ON "game_stats" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_game_stats_updated_by" ON "game_stats" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_game_stats_deleted_by" ON "game_stats" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_game_stats_requested_by" ON "game_stats" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_game_stats_tags" ON "game_stats" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_game_stats_categories" ON "game_stats" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_game_stats_code" ON "game_stats" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_game_stats_sensitivity_level" ON "game_stats" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_game_stats_contact_id" ON "game_stats" USING btree("contact_id");
--> statement-breakpoint
CREATE INDEX "idx_game_stats_event_id" ON "game_stats" USING btree("event_id");
--> statement-breakpoint
CREATE INDEX "idx_game_stats_event_type" ON "game_stats"("event_type");
--> statement-breakpoint
CREATE INDEX "idx_game_stats_course_id" ON "game_stats" USING btree("course_id");
--> statement-breakpoint
CREATE INDEX "idx_game_stats_story_id" ON "game_stats" USING btree("story_id");
--> statement-breakpoint
CREATE INDEX "idx_game_stats_episode_id" ON "game_stats" USING btree("episode_id");
--> statement-breakpoint
CREATE INDEX "idx_game_stats_classroom_id" ON "game_stats" USING btree("classroom_id");
--> statement-breakpoint
CREATE INDEX "idx_game_stats_chapter_number" ON "game_stats" USING btree("chapter_number");
--> statement-breakpoint
CREATE INDEX "idx_game_stats_volume_level" ON "game_stats" USING btree("volume_level");
--> statement-breakpoint
CREATE INDEX "idx_game_stats_question" ON "game_stats" USING btree("question");
--> statement-breakpoint
CREATE INDEX "idx_game_stats_choices" ON "game_stats" USING btree("choices");
--> statement-breakpoint
CREATE INDEX "idx_game_stats_selected_choice" ON "game_stats" USING btree("selected_choice");
--> statement-breakpoint
CREATE INDEX "idx_related_contacts_tombstone" ON "related_contacts" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_related_contacts_status" ON "related_contacts" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_related_contacts_previous_status" ON "related_contacts" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_related_contacts_version" ON "related_contacts" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_related_contacts_created_date" ON "related_contacts" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_related_contacts_updated_date" ON "related_contacts" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_related_contacts_organization_id" ON "related_contacts" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_related_contacts_created_by" ON "related_contacts" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_related_contacts_updated_by" ON "related_contacts" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_related_contacts_deleted_by" ON "related_contacts" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_related_contacts_requested_by" ON "related_contacts" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_related_contacts_tags" ON "related_contacts" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_related_contacts_categories" ON "related_contacts" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_related_contacts_code" ON "related_contacts" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_related_contacts_sensitivity_level" ON "related_contacts" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_related_contacts_contact_id" ON "related_contacts" USING btree("contact_id");
--> statement-breakpoint
CREATE INDEX "idx_related_contacts_student_id" ON "related_contacts" USING btree("student_id");
--> statement-breakpoint
CREATE INDEX "idx_related_contacts_is_verified" ON "related_contacts" USING btree("is_verified");
--> statement-breakpoint
CREATE INDEX "idx_organization_accounts_organization_contact_id" ON "organization_accounts" USING btree("organization_contact_id");
--> statement-breakpoint
CREATE INDEX "idx_organization_accounts_contact_id" ON "organization_accounts" USING btree("contact_id");
--> statement-breakpoint
CREATE INDEX "idx_organization_accounts_email" ON "organization_accounts" USING btree("email");
--> statement-breakpoint
CREATE INDEX "idx_organization_accounts_password" ON "organization_accounts" USING btree("password");
--> statement-breakpoint
CREATE INDEX "idx_organization_accounts_account_id" ON "organization_accounts" USING btree("account_id");
--> statement-breakpoint
CREATE INDEX "idx_organization_accounts_account_secret" ON "organization_accounts" USING btree("account_secret");
--> statement-breakpoint
CREATE INDEX "idx_organization_accounts_role_id" ON "organization_accounts" USING btree("role_id");
--> statement-breakpoint
CREATE INDEX "idx_organization_accounts_account_organization_id" ON "organization_accounts" USING btree("account_organization_id");
--> statement-breakpoint
CREATE INDEX "idx_organization_accounts_is_new_user" ON "organization_accounts" USING btree("is_new_user");
--> statement-breakpoint
CREATE INDEX "idx_organization_accounts_account_status" ON "organization_accounts" USING btree("account_status");
--> statement-breakpoint
CREATE INDEX "idx_organization_accounts_external_contact_id" ON "organization_accounts" USING btree("external_contact_id");
--> statement-breakpoint
CREATE INDEX "idx_organization_accounts_device_id" ON "organization_accounts" USING btree("device_id");
--> statement-breakpoint
CREATE INDEX "idx_contacts_first_name" ON "contacts" USING btree("first_name");
--> statement-breakpoint
CREATE INDEX "idx_contacts_middle_name" ON "contacts" USING btree("middle_name");
--> statement-breakpoint
CREATE INDEX "idx_contacts_last_name" ON "contacts" USING btree("last_name");
--> statement-breakpoint
CREATE INDEX "idx_contacts_date_of_birth" ON "contacts" USING btree("date_of_birth");
--> statement-breakpoint
CREATE INDEX "idx_contacts_username" ON "contacts" USING btree("username");
--> statement-breakpoint
CREATE INDEX "idx_contacts_address_id" ON "contacts" USING btree("address_id");
--> statement-breakpoint
CREATE INDEX "idx_contacts_account_id" ON "contacts" USING btree("account_id");
--> statement-breakpoint
CREATE INDEX "idx_contacts_department_id" ON "contacts" USING btree("department_id");
--> statement-breakpoint
CREATE INDEX "idx_contacts_district_id" ON "contacts" USING btree("district_id");
--> statement-breakpoint
CREATE INDEX "idx_contacts_school_id" ON "contacts" USING btree("school_id");
--> statement-breakpoint
CREATE INDEX "idx_contacts_grade_level" ON "contacts" USING btree("grade_level");
--> statement-breakpoint
CREATE INDEX "idx_contacts_school_year" ON "contacts" USING btree("school_year");
--> statement-breakpoint
CREATE INDEX "idx_contacts_teacher_id" ON "contacts" USING btree("teacher_id");
--> statement-breakpoint
CREATE INDEX "idx_contacts_last_assistant_toggle_option_value" ON "contacts" USING btree("last_assistant_toggle_option_value");
--> statement-breakpoint
CREATE INDEX "idx_episode_events_tombstone" ON "episode_events" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_episode_events_status" ON "episode_events" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_episode_events_previous_status" ON "episode_events" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_episode_events_version" ON "episode_events" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_episode_events_created_date" ON "episode_events" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_episode_events_updated_date" ON "episode_events" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_episode_events_organization_id" ON "episode_events" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_episode_events_created_by" ON "episode_events" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_episode_events_updated_by" ON "episode_events" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_episode_events_deleted_by" ON "episode_events" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_episode_events_requested_by" ON "episode_events" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_episode_events_tags" ON "episode_events" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_episode_events_categories" ON "episode_events" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_episode_events_code" ON "episode_events" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_episode_events_sensitivity_level" ON "episode_events" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_episode_events_user_id" ON "episode_events" USING btree("user_id");
--> statement-breakpoint
CREATE INDEX "idx_episode_events_course_id" ON "episode_events" USING btree("course_id");
--> statement-breakpoint
CREATE INDEX "idx_episode_events_story_id" ON "episode_events" USING btree("story_id");
--> statement-breakpoint
CREATE INDEX "idx_episode_events_episode_id" ON "episode_events" USING btree("episode_id");
--> statement-breakpoint
CREATE INDEX "idx_episode_events_event_type" ON "episode_events"("event_type");
--> statement-breakpoint
CREATE INDEX "idx_episode_events_game_status" ON "episode_events" USING btree("game_status");
--> statement-breakpoint
CREATE INDEX "idx_teacher_schools_tombstone" ON "teacher_schools" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_teacher_schools_status" ON "teacher_schools" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_teacher_schools_previous_status" ON "teacher_schools" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_teacher_schools_version" ON "teacher_schools" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_teacher_schools_created_date" ON "teacher_schools" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_teacher_schools_updated_date" ON "teacher_schools" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_teacher_schools_organization_id" ON "teacher_schools" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_teacher_schools_created_by" ON "teacher_schools" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_teacher_schools_updated_by" ON "teacher_schools" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_teacher_schools_deleted_by" ON "teacher_schools" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_teacher_schools_requested_by" ON "teacher_schools" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_teacher_schools_tags" ON "teacher_schools" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_teacher_schools_categories" ON "teacher_schools" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_teacher_schools_code" ON "teacher_schools" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_teacher_schools_sensitivity_level" ON "teacher_schools" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_teacher_schools_school_id" ON "teacher_schools" USING btree("school_id");
--> statement-breakpoint
CREATE INDEX "idx_teacher_schools_teacher_id" ON "teacher_schools" USING btree("teacher_id");
--> statement-breakpoint
CREATE INDEX "idx_teacher_schools_district_id" ON "teacher_schools" USING btree("district_id");
--> statement-breakpoint
CREATE INDEX "idx_teacher_schools_department_id" ON "teacher_schools" USING btree("department_id");
--> statement-breakpoint
CREATE INDEX "idx_transport_provider_credentials_tombstone" ON "transport_provider_credentials" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_transport_provider_credentials_status" ON "transport_provider_credentials" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_transport_provider_credentials_previous_status" ON "transport_provider_credentials" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_transport_provider_credentials_version" ON "transport_provider_credentials" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_transport_provider_credentials_created_date" ON "transport_provider_credentials" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_transport_provider_credentials_updated_date" ON "transport_provider_credentials" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_transport_provider_credentials_organization_id" ON "transport_provider_credentials" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_transport_provider_credentials_created_by" ON "transport_provider_credentials" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_transport_provider_credentials_updated_by" ON "transport_provider_credentials" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_transport_provider_credentials_deleted_by" ON "transport_provider_credentials" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_transport_provider_credentials_requested_by" ON "transport_provider_credentials" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_transport_provider_credentials_tags" ON "transport_provider_credentials" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_transport_provider_credentials_categories" ON "transport_provider_credentials" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_transport_provider_credentials_code" ON "transport_provider_credentials" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_transport_provider_credentials_sensitivity_level" ON "transport_provider_credentials" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_transport_provider_credentials_transport_provider_id" ON "transport_provider_credentials" USING btree("transport_provider_id");
--> statement-breakpoint
CREATE INDEX "idx_transport_provider_credentials_host" ON "transport_provider_credentials" USING btree("host");
--> statement-breakpoint
CREATE INDEX "idx_transport_provider_credentials_port" ON "transport_provider_credentials" USING btree("port");
--> statement-breakpoint
CREATE INDEX "idx_transport_provider_credentials_is_secure" ON "transport_provider_credentials" USING btree("is_secure");
--> statement-breakpoint
CREATE INDEX "idx_transport_provider_credentials_username" ON "transport_provider_credentials" USING btree("username");
--> statement-breakpoint
CREATE INDEX "idx_transport_provider_credentials_password" ON "transport_provider_credentials" USING btree("password");
--> statement-breakpoint
CREATE INDEX "idx_transport_provider_credentials_api_key" ON "transport_provider_credentials" USING btree("api_key");
--> statement-breakpoint
CREATE INDEX "idx_organization_contacts_contact_organization_id" ON "organization_contacts" USING btree("contact_organization_id");
--> statement-breakpoint
CREATE INDEX "idx_organization_contacts_contact_id" ON "organization_contacts" USING btree("contact_id");
--> statement-breakpoint
CREATE INDEX "idx_organization_contacts_is_primary" ON "organization_contacts" USING btree("is_primary");
--> statement-breakpoint
CREATE INDEX "idx_smtp_attachments_tombstone" ON "smtp_attachments" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_smtp_attachments_status" ON "smtp_attachments" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_smtp_attachments_previous_status" ON "smtp_attachments" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_smtp_attachments_version" ON "smtp_attachments" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_smtp_attachments_created_date" ON "smtp_attachments" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_smtp_attachments_updated_date" ON "smtp_attachments" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_smtp_attachments_organization_id" ON "smtp_attachments" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_smtp_attachments_created_by" ON "smtp_attachments" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_smtp_attachments_updated_by" ON "smtp_attachments" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_smtp_attachments_deleted_by" ON "smtp_attachments" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_smtp_attachments_requested_by" ON "smtp_attachments" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_smtp_attachments_tags" ON "smtp_attachments" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_smtp_attachments_categories" ON "smtp_attachments" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_smtp_attachments_code" ON "smtp_attachments" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_smtp_attachments_sensitivity_level" ON "smtp_attachments" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_smtp_attachments_filename" ON "smtp_attachments" USING btree("filename");
--> statement-breakpoint
CREATE INDEX "idx_smtp_attachments_content_type" ON "smtp_attachments"("content_type");
--> statement-breakpoint
CREATE INDEX "idx_smtp_attachments_content_disposition" ON "smtp_attachments" USING btree("content_disposition");
--> statement-breakpoint
CREATE INDEX "idx_smtp_attachments_checksum" ON "smtp_attachments" USING btree("checksum");
--> statement-breakpoint
CREATE INDEX "idx_smtp_attachments_size" ON "smtp_attachments" USING btree("size");
--> statement-breakpoint
CREATE INDEX "idx_smtp_attachments_content_id" ON "smtp_attachments" USING btree("content_id");
--> statement-breakpoint
CREATE INDEX "idx_smtp_attachments_cid" ON "smtp_attachments" USING btree("cid");
--> statement-breakpoint
CREATE INDEX "idx_smtp_attachments_related" ON "smtp_attachments" USING btree("related");
--> statement-breakpoint
CREATE INDEX "idx_smtp_attachments_smtp_payload_id" ON "smtp_attachments" USING btree("smtp_payload_id");
--> statement-breakpoint
CREATE INDEX "idx_smtp_attachments_file_id" ON "smtp_attachments" USING btree("file_id");
--> statement-breakpoint
CREATE INDEX "idx_smtp_attachments_type_field" ON "smtp_attachments" USING btree("type_field");
--> statement-breakpoint
CREATE INDEX "idx_smtp_attachments_part_id" ON "smtp_attachments" USING btree("part_id");
--> statement-breakpoint
CREATE INDEX "idx_student_session_logs_tombstone" ON "student_session_logs" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_student_session_logs_status" ON "student_session_logs" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_student_session_logs_previous_status" ON "student_session_logs" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_student_session_logs_version" ON "student_session_logs" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_student_session_logs_created_date" ON "student_session_logs" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_student_session_logs_updated_date" ON "student_session_logs" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_student_session_logs_organization_id" ON "student_session_logs" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_student_session_logs_created_by" ON "student_session_logs" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_student_session_logs_updated_by" ON "student_session_logs" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_student_session_logs_deleted_by" ON "student_session_logs" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_student_session_logs_requested_by" ON "student_session_logs" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_student_session_logs_tags" ON "student_session_logs" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_student_session_logs_categories" ON "student_session_logs" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_student_session_logs_code" ON "student_session_logs" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_student_session_logs_sensitivity_level" ON "student_session_logs" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_student_session_logs_user_id" ON "student_session_logs" USING btree("user_id");
--> statement-breakpoint
CREATE INDEX "idx_student_session_logs_action" ON "student_session_logs" USING btree("action");
--> statement-breakpoint
CREATE INDEX "idx_student_session_logs_session_token" ON "student_session_logs" USING btree("session_token");
--> statement-breakpoint
CREATE INDEX "idx_student_session_logs_timestamp" ON "student_session_logs" USING btree("timestamp");
--> statement-breakpoint
CREATE INDEX "idx_student_session_logs_device_details" ON "student_session_logs" USING btree("device_details");
--> statement-breakpoint
CREATE INDEX "idx_student_session_logs_ip_address" ON "student_session_logs" USING btree("ip_address");
--> statement-breakpoint
CREATE INDEX "idx_student_session_logs_route" ON "student_session_logs" USING btree("route");
--> statement-breakpoint
CREATE INDEX "idx_student_session_logs_reason" ON "student_session_logs" USING btree("reason");
--> statement-breakpoint
CREATE INDEX "idx_game_states_tombstone" ON "game_states" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_game_states_status" ON "game_states" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_game_states_previous_status" ON "game_states" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_game_states_version" ON "game_states" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_game_states_created_date" ON "game_states" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_game_states_updated_date" ON "game_states" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_game_states_organization_id" ON "game_states" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_game_states_created_by" ON "game_states" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_game_states_updated_by" ON "game_states" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_game_states_deleted_by" ON "game_states" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_game_states_requested_by" ON "game_states" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_game_states_tags" ON "game_states" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_game_states_categories" ON "game_states" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_game_states_code" ON "game_states" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_game_states_sensitivity_level" ON "game_states" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_game_states_user_id" ON "game_states" USING btree("user_id");
--> statement-breakpoint
CREATE INDEX "idx_game_states_course_id" ON "game_states" USING btree("course_id");
--> statement-breakpoint
CREATE INDEX "idx_game_states_story_id" ON "game_states" USING btree("story_id");
--> statement-breakpoint
CREATE INDEX "idx_game_states_episode_id" ON "game_states" USING btree("episode_id");
--> statement-breakpoint
CREATE INDEX "idx_game_states_chapter_number" ON "game_states" USING btree("chapter_number");
--> statement-breakpoint
CREATE INDEX "idx_game_choices_tombstone" ON "game_choices" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_game_choices_status" ON "game_choices" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_game_choices_previous_status" ON "game_choices" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_game_choices_version" ON "game_choices" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_game_choices_created_date" ON "game_choices" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_game_choices_updated_date" ON "game_choices" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_game_choices_organization_id" ON "game_choices" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_game_choices_created_by" ON "game_choices" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_game_choices_updated_by" ON "game_choices" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_game_choices_deleted_by" ON "game_choices" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_game_choices_requested_by" ON "game_choices" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_game_choices_tags" ON "game_choices" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_game_choices_categories" ON "game_choices" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_game_choices_code" ON "game_choices" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_game_choices_sensitivity_level" ON "game_choices" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_game_choices_question_id" ON "game_choices" USING btree("question_id");
--> statement-breakpoint
CREATE INDEX "idx_game_choices_choice_text" ON "game_choices" USING btree("choice_text");
--> statement-breakpoint
CREATE INDEX "idx_game_choices_is_correct" ON "game_choices" USING btree("is_correct");
--> statement-breakpoint
CREATE INDEX "idx_classroom_students_tombstone" ON "classroom_students" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_classroom_students_status" ON "classroom_students" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_classroom_students_previous_status" ON "classroom_students" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_classroom_students_version" ON "classroom_students" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_classroom_students_created_date" ON "classroom_students" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_classroom_students_updated_date" ON "classroom_students" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_classroom_students_organization_id" ON "classroom_students" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_classroom_students_created_by" ON "classroom_students" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_classroom_students_updated_by" ON "classroom_students" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_classroom_students_deleted_by" ON "classroom_students" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_classroom_students_requested_by" ON "classroom_students" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_classroom_students_tags" ON "classroom_students" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_classroom_students_categories" ON "classroom_students" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_classroom_students_code" ON "classroom_students" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_classroom_students_sensitivity_level" ON "classroom_students" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_classroom_students_classroom_id" ON "classroom_students" USING btree("classroom_id");
--> statement-breakpoint
CREATE INDEX "idx_classroom_students_student_id" ON "classroom_students" USING btree("student_id");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_stories_tombstone" ON "classroom_course_stories" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_stories_status" ON "classroom_course_stories" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_stories_previous_status" ON "classroom_course_stories" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_stories_version" ON "classroom_course_stories" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_stories_created_date" ON "classroom_course_stories" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_stories_updated_date" ON "classroom_course_stories" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_stories_organization_id" ON "classroom_course_stories" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_stories_created_by" ON "classroom_course_stories" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_stories_updated_by" ON "classroom_course_stories" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_stories_deleted_by" ON "classroom_course_stories" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_stories_requested_by" ON "classroom_course_stories" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_stories_tags" ON "classroom_course_stories" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_stories_categories" ON "classroom_course_stories" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_stories_code" ON "classroom_course_stories" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_stories_sensitivity_level" ON "classroom_course_stories" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_stories_classroom_id" ON "classroom_course_stories" USING btree("classroom_id");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_stories_course_id" ON "classroom_course_stories" USING btree("course_id");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_stories_story_id" ON "classroom_course_stories" USING btree("story_id");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_stories_start_date" ON "classroom_course_stories" USING btree("start_date");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_stories_order" ON "classroom_course_stories" USING btree("order");
--> statement-breakpoint
CREATE INDEX "idx_avatar_selections_tombstone" ON "avatar_selections" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_avatar_selections_status" ON "avatar_selections" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_avatar_selections_previous_status" ON "avatar_selections" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_avatar_selections_version" ON "avatar_selections" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_avatar_selections_created_date" ON "avatar_selections" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_avatar_selections_updated_date" ON "avatar_selections" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_avatar_selections_organization_id" ON "avatar_selections" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_avatar_selections_created_by" ON "avatar_selections" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_avatar_selections_updated_by" ON "avatar_selections" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_avatar_selections_deleted_by" ON "avatar_selections" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_avatar_selections_requested_by" ON "avatar_selections" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_avatar_selections_tags" ON "avatar_selections" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_avatar_selections_categories" ON "avatar_selections" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_avatar_selections_code" ON "avatar_selections" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_avatar_selections_sensitivity_level" ON "avatar_selections" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_avatar_selections_user_id" ON "avatar_selections" USING btree("user_id");
--> statement-breakpoint
CREATE INDEX "idx_avatar_selections_gender" ON "avatar_selections" USING btree("gender");
--> statement-breakpoint
CREATE INDEX "idx_avatar_selections_skin_color" ON "avatar_selections" USING btree("skin_color");
--> statement-breakpoint
CREATE INDEX "idx_avatar_selections_hair_color" ON "avatar_selections" USING btree("hair_color");
--> statement-breakpoint
CREATE INDEX "idx_avatar_selections_eye_color" ON "avatar_selections" USING btree("eye_color");
--> statement-breakpoint
CREATE INDEX "idx_avatar_selections_hair_style" ON "avatar_selections" USING btree("hair_style");
--> statement-breakpoint
CREATE INDEX "idx_avatar_selections_clothing" ON "avatar_selections" USING btree("clothing");
--> statement-breakpoint
CREATE INDEX "idx_contact_phone_numbers_contact_id" ON "contact_phone_numbers" USING btree("contact_id");
--> statement-breakpoint
CREATE INDEX "idx_contact_phone_numbers_raw_phone_number" ON "contact_phone_numbers" USING btree("raw_phone_number");
--> statement-breakpoint
CREATE INDEX "idx_contact_phone_numbers_iso_code" ON "contact_phone_numbers" USING btree("iso_code");
--> statement-breakpoint
CREATE INDEX "idx_contact_phone_numbers_country_code" ON "contact_phone_numbers" USING btree("country_code");
--> statement-breakpoint
CREATE INDEX "idx_contact_phone_numbers_is_primary" ON "contact_phone_numbers" USING btree("is_primary");
--> statement-breakpoint
CREATE INDEX "idx_user_settings_tombstone" ON "user_settings" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_user_settings_status" ON "user_settings" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_user_settings_previous_status" ON "user_settings" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_user_settings_version" ON "user_settings" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_user_settings_created_date" ON "user_settings" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_user_settings_updated_date" ON "user_settings" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_user_settings_organization_id" ON "user_settings" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_user_settings_created_by" ON "user_settings" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_user_settings_updated_by" ON "user_settings" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_user_settings_deleted_by" ON "user_settings" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_user_settings_requested_by" ON "user_settings" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_user_settings_tags" ON "user_settings" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_user_settings_categories" ON "user_settings" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_user_settings_code" ON "user_settings" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_user_settings_sensitivity_level" ON "user_settings" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_user_settings_user_id" ON "user_settings" USING btree("user_id");
--> statement-breakpoint
CREATE INDEX "idx_user_settings_sound" ON "user_settings" USING btree("sound");
--> statement-breakpoint
CREATE INDEX "idx_chapter_events_tombstone" ON "chapter_events" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_chapter_events_status" ON "chapter_events" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_chapter_events_previous_status" ON "chapter_events" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_chapter_events_version" ON "chapter_events" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_chapter_events_created_date" ON "chapter_events" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_chapter_events_updated_date" ON "chapter_events" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_chapter_events_organization_id" ON "chapter_events" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_chapter_events_created_by" ON "chapter_events" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_chapter_events_updated_by" ON "chapter_events" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_chapter_events_deleted_by" ON "chapter_events" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_chapter_events_requested_by" ON "chapter_events" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_chapter_events_tags" ON "chapter_events" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_chapter_events_categories" ON "chapter_events" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_chapter_events_code" ON "chapter_events" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_chapter_events_sensitivity_level" ON "chapter_events" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_chapter_events_user_id" ON "chapter_events" USING btree("user_id");
--> statement-breakpoint
CREATE INDEX "idx_chapter_events_course_id" ON "chapter_events" USING btree("course_id");
--> statement-breakpoint
CREATE INDEX "idx_chapter_events_story_id" ON "chapter_events" USING btree("story_id");
--> statement-breakpoint
CREATE INDEX "idx_chapter_events_episode_id" ON "chapter_events" USING btree("episode_id");
--> statement-breakpoint
CREATE INDEX "idx_chapter_events_chapter_number" ON "chapter_events" USING btree("chapter_number");
--> statement-breakpoint
CREATE INDEX "idx_chapter_events_event_type" ON "chapter_events"("event_type");
--> statement-breakpoint
CREATE INDEX "idx_chapter_events_game_status" ON "chapter_events" USING btree("game_status");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_episodes_tombstone" ON "classroom_course_episodes" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_episodes_status" ON "classroom_course_episodes" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_episodes_previous_status" ON "classroom_course_episodes" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_episodes_version" ON "classroom_course_episodes" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_episodes_created_date" ON "classroom_course_episodes" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_episodes_updated_date" ON "classroom_course_episodes" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_episodes_organization_id" ON "classroom_course_episodes" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_episodes_created_by" ON "classroom_course_episodes" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_episodes_updated_by" ON "classroom_course_episodes" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_episodes_deleted_by" ON "classroom_course_episodes" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_episodes_requested_by" ON "classroom_course_episodes" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_episodes_tags" ON "classroom_course_episodes" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_episodes_categories" ON "classroom_course_episodes" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_episodes_code" ON "classroom_course_episodes" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_episodes_sensitivity_level" ON "classroom_course_episodes" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_episodes_classroom_id" ON "classroom_course_episodes" USING btree("classroom_id");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_episodes_course_id" ON "classroom_course_episodes" USING btree("course_id");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_episodes_story_id" ON "classroom_course_episodes" USING btree("story_id");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_episodes_episode_id" ON "classroom_course_episodes" USING btree("episode_id");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_episodes_start_date" ON "classroom_course_episodes" USING btree("start_date");
--> statement-breakpoint
CREATE INDEX "idx_classroom_course_episodes_order" ON "classroom_course_episodes" USING btree("order");
--> statement-breakpoint
CREATE INDEX "idx_faq_categories_tombstone" ON "faq_categories" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_faq_categories_status" ON "faq_categories" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_faq_categories_previous_status" ON "faq_categories" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_faq_categories_version" ON "faq_categories" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_faq_categories_created_date" ON "faq_categories" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_faq_categories_updated_date" ON "faq_categories" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_faq_categories_organization_id" ON "faq_categories" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_faq_categories_created_by" ON "faq_categories" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_faq_categories_updated_by" ON "faq_categories" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_faq_categories_deleted_by" ON "faq_categories" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_faq_categories_requested_by" ON "faq_categories" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_faq_categories_tags" ON "faq_categories" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_faq_categories_categories" ON "faq_categories" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_faq_categories_code" ON "faq_categories" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_faq_categories_sensitivity_level" ON "faq_categories" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_faq_categories_faq_id" ON "faq_categories" USING btree("faq_id");
--> statement-breakpoint
CREATE INDEX "idx_faq_categories_category" ON "faq_categories" USING btree("category");
--> statement-breakpoint
CREATE INDEX "idx_faq_categories_order" ON "faq_categories" USING btree("order");
--> statement-breakpoint
CREATE INDEX "idx_locations_tombstone" ON "locations" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_locations_status" ON "locations" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_locations_previous_status" ON "locations" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_locations_version" ON "locations" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_locations_created_date" ON "locations" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_locations_updated_date" ON "locations" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_locations_organization_id" ON "locations" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_locations_created_by" ON "locations" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_locations_updated_by" ON "locations" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_locations_deleted_by" ON "locations" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_locations_requested_by" ON "locations" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_locations_tags" ON "locations" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_locations_categories" ON "locations" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_locations_code" ON "locations" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_locations_sensitivity_level" ON "locations" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_locations_location_name" ON "locations" USING btree("location_name");
--> statement-breakpoint
CREATE INDEX "idx_locations_address_id" ON "locations" USING btree("address_id");
--> statement-breakpoint
CREATE INDEX "idx_conversations_tombstone" ON "conversations" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_conversations_status" ON "conversations" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_conversations_previous_status" ON "conversations" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_conversations_version" ON "conversations" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_conversations_created_date" ON "conversations" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_conversations_updated_date" ON "conversations" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_conversations_organization_id" ON "conversations" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_conversations_created_by" ON "conversations" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_conversations_updated_by" ON "conversations" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_conversations_deleted_by" ON "conversations" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_conversations_requested_by" ON "conversations" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_conversations_tags" ON "conversations" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_conversations_categories" ON "conversations" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_conversations_code" ON "conversations" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_conversations_sensitivity_level" ON "conversations" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_conversations_conversation_id" ON "conversations" USING btree("conversation_id");
--> statement-breakpoint
CREATE INDEX "idx_conversations_conversation_replica_name" ON "conversations" USING btree("conversation_replica_name");
--> statement-breakpoint
CREATE INDEX "idx_conversations_last_message_activity" ON "conversations" USING btree("last_message_activity");
--> statement-breakpoint
CREATE INDEX "idx_conversations_tavus_conversation_id" ON "conversations" USING btree("tavus_conversation_id");
--> statement-breakpoint
CREATE INDEX "idx_conversations_tavus_conversation_replica_name" ON "conversations" USING btree("tavus_conversation_replica_name");
--> statement-breakpoint
CREATE INDEX "idx_conversations_tavus_conversation_start_date" ON "conversations" USING btree("tavus_conversation_start_date");
--> statement-breakpoint
CREATE INDEX "idx_conversations_tavus_conversation_end_date" ON "conversations" USING btree("tavus_conversation_end_date");
--> statement-breakpoint
CREATE INDEX "idx_conversations_tavus_conversation_date_start" ON "conversations" USING btree("tavus_conversation_date_start");
--> statement-breakpoint
CREATE INDEX "idx_conversations_tavus_conversation_date_end" ON "conversations" USING btree("tavus_conversation_date_end");
--> statement-breakpoint
CREATE INDEX "idx_conversations_hume_ai_job_id" ON "conversations" USING btree("hume_ai_job_id");
--> statement-breakpoint
CREATE INDEX "idx_conversations_hume_ai_sync_status" ON "conversations" USING btree("hume_ai_sync_status");
--> statement-breakpoint
CREATE INDEX "idx_conversations_user_id" ON "conversations" USING btree("user_id");
--> statement-breakpoint
CREATE INDEX "idx_conversations_contact_id" ON "conversations" USING btree("contact_id");
--> statement-breakpoint
CREATE INDEX "idx_conversations_conversation_topic_id" ON "conversations" USING btree("conversation_topic_id");
--> statement-breakpoint
CREATE INDEX "idx_conversations_session_recording_url" ON "conversations" USING btree("session_recording_url");
--> statement-breakpoint
CREATE INDEX "idx_conversations_method" ON "conversations" USING btree("method");
--> statement-breakpoint
CREATE INDEX "idx_conversations_perception" ON "conversations" USING btree("perception");
--> statement-breakpoint
CREATE INDEX "idx_conversations_duration" ON "conversations" USING btree("duration");
--> statement-breakpoint
CREATE INDEX "idx_conversations_summary" ON "conversations" USING btree("summary");
--> statement-breakpoint
CREATE INDEX "idx_conversations_source" ON "conversations" USING btree("source");
--> statement-breakpoint
CREATE INDEX "idx_conversations_app_version" ON "conversations" USING btree("app_version");
--> statement-breakpoint
CREATE INDEX "idx_conversations_ip_address" ON "conversations" USING btree("ip_address");
--> statement-breakpoint
CREATE INDEX "idx_conversations_is_skip_select_topic" ON "conversations" USING btree("is_skip_select_topic");
--> statement-breakpoint
CREATE INDEX "idx_conversations_cybertipline_report_report_annotations" ON "conversations" USING btree("cybertipline_report_report_annotations");
--> statement-breakpoint
CREATE INDEX "idx_conversations_topic_name" ON "conversations" USING btree("topic_name");
--> statement-breakpoint
CREATE INDEX "idx_common_session_logs_tombstone" ON "common_session_logs" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_common_session_logs_status" ON "common_session_logs" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_common_session_logs_previous_status" ON "common_session_logs" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_common_session_logs_version" ON "common_session_logs" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_common_session_logs_created_date" ON "common_session_logs" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_common_session_logs_updated_date" ON "common_session_logs" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_common_session_logs_organization_id" ON "common_session_logs" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_common_session_logs_created_by" ON "common_session_logs" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_common_session_logs_updated_by" ON "common_session_logs" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_common_session_logs_deleted_by" ON "common_session_logs" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_common_session_logs_requested_by" ON "common_session_logs" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_common_session_logs_tags" ON "common_session_logs" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_common_session_logs_categories" ON "common_session_logs" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_common_session_logs_code" ON "common_session_logs" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_common_session_logs_sensitivity_level" ON "common_session_logs" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_common_session_logs_user_id" ON "common_session_logs" USING btree("user_id");
--> statement-breakpoint
CREATE INDEX "idx_common_session_logs_action" ON "common_session_logs" USING btree("action");
--> statement-breakpoint
CREATE INDEX "idx_common_session_logs_session_token" ON "common_session_logs" USING btree("session_token");
--> statement-breakpoint
CREATE INDEX "idx_common_session_logs_timestamp" ON "common_session_logs" USING btree("timestamp");
--> statement-breakpoint
CREATE INDEX "idx_common_session_logs_device_details" ON "common_session_logs" USING btree("device_details");
--> statement-breakpoint
CREATE INDEX "idx_common_session_logs_ip_address" ON "common_session_logs" USING btree("ip_address");
--> statement-breakpoint
CREATE INDEX "idx_common_session_logs_route" ON "common_session_logs" USING btree("route");
--> statement-breakpoint
CREATE INDEX "idx_common_session_logs_reason" ON "common_session_logs" USING btree("reason");
--> statement-breakpoint
CREATE INDEX "idx_game_questions_tombstone" ON "game_questions" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_game_questions_status" ON "game_questions" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_game_questions_previous_status" ON "game_questions" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_game_questions_version" ON "game_questions" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_game_questions_created_date" ON "game_questions" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_game_questions_updated_date" ON "game_questions" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_game_questions_organization_id" ON "game_questions" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_game_questions_created_by" ON "game_questions" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_game_questions_updated_by" ON "game_questions" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_game_questions_deleted_by" ON "game_questions" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_game_questions_requested_by" ON "game_questions" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_game_questions_tags" ON "game_questions" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_game_questions_categories" ON "game_questions" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_game_questions_code" ON "game_questions" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_game_questions_sensitivity_level" ON "game_questions" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_game_questions_course_id" ON "game_questions" USING btree("course_id");
--> statement-breakpoint
CREATE INDEX "idx_game_questions_course_title" ON "game_questions" USING btree("course_title");
--> statement-breakpoint
CREATE INDEX "idx_game_questions_story_id" ON "game_questions" USING btree("story_id");
--> statement-breakpoint
CREATE INDEX "idx_game_questions_story_title" ON "game_questions" USING btree("story_title");
--> statement-breakpoint
CREATE INDEX "idx_game_questions_episode_number" ON "game_questions" USING btree("episode_number");
--> statement-breakpoint
CREATE INDEX "idx_game_questions_question" ON "game_questions" USING btree("question");
--> statement-breakpoint
CREATE INDEX "idx_game_questions_topic" ON "game_questions" USING btree("topic");
--> statement-breakpoint
CREATE INDEX "idx_communication_templates_tombstone" ON "communication_templates" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_communication_templates_status" ON "communication_templates" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_communication_templates_previous_status" ON "communication_templates" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_communication_templates_version" ON "communication_templates" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_communication_templates_created_date" ON "communication_templates" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_communication_templates_updated_date" ON "communication_templates" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_communication_templates_organization_id" ON "communication_templates" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_communication_templates_created_by" ON "communication_templates" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_communication_templates_updated_by" ON "communication_templates" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_communication_templates_deleted_by" ON "communication_templates" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_communication_templates_requested_by" ON "communication_templates" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_communication_templates_tags" ON "communication_templates" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_communication_templates_categories" ON "communication_templates" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_communication_templates_code" ON "communication_templates" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_communication_templates_sensitivity_level" ON "communication_templates" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_communication_templates_name" ON "communication_templates" USING btree("name");
--> statement-breakpoint
CREATE INDEX "idx_communication_templates_communication_template_status" ON "communication_templates" USING btree("communication_template_status");
--> statement-breakpoint
CREATE INDEX "idx_communication_templates_event" ON "communication_templates" USING btree("event");
--> statement-breakpoint
CREATE INDEX "idx_communication_templates_content" ON "communication_templates" USING btree("content");
--> statement-breakpoint
CREATE INDEX "idx_communication_templates_subject" ON "communication_templates" USING btree("subject");
--> statement-breakpoint
CREATE INDEX "idx_classroom_courses_tombstone" ON "classroom_courses" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_classroom_courses_status" ON "classroom_courses" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_classroom_courses_previous_status" ON "classroom_courses" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_classroom_courses_version" ON "classroom_courses" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_classroom_courses_created_date" ON "classroom_courses" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_classroom_courses_updated_date" ON "classroom_courses" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_classroom_courses_organization_id" ON "classroom_courses" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_classroom_courses_created_by" ON "classroom_courses" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_classroom_courses_updated_by" ON "classroom_courses" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_classroom_courses_deleted_by" ON "classroom_courses" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_classroom_courses_requested_by" ON "classroom_courses" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_classroom_courses_tags" ON "classroom_courses" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_classroom_courses_categories" ON "classroom_courses" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_classroom_courses_code" ON "classroom_courses" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_classroom_courses_sensitivity_level" ON "classroom_courses" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_classroom_courses_classroom_id" ON "classroom_courses" USING btree("classroom_id");
--> statement-breakpoint
CREATE INDEX "idx_classroom_courses_course_id" ON "classroom_courses" USING btree("course_id");
--> statement-breakpoint
CREATE INDEX "idx_classroom_courses_start_date" ON "classroom_courses" USING btree("start_date");
--> statement-breakpoint
CREATE INDEX "idx_classroom_courses_order" ON "classroom_courses" USING btree("order");
--> statement-breakpoint
CREATE INDEX "idx_student_stats_tombstone" ON "student_stats" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_student_stats_status" ON "student_stats" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_student_stats_previous_status" ON "student_stats" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_student_stats_version" ON "student_stats" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_student_stats_created_date" ON "student_stats" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_student_stats_updated_date" ON "student_stats" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_student_stats_organization_id" ON "student_stats" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_student_stats_created_by" ON "student_stats" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_student_stats_updated_by" ON "student_stats" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_student_stats_deleted_by" ON "student_stats" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_student_stats_requested_by" ON "student_stats" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_student_stats_tags" ON "student_stats" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_student_stats_categories" ON "student_stats" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_student_stats_code" ON "student_stats" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_student_stats_sensitivity_level" ON "student_stats" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_student_stats_student_id" ON "student_stats" USING btree("student_id");
--> statement-breakpoint
CREATE INDEX "idx_student_stats_classroom_id" ON "student_stats" USING btree("classroom_id");
--> statement-breakpoint
CREATE INDEX "idx_student_stats_course_id" ON "student_stats" USING btree("course_id");
--> statement-breakpoint
CREATE INDEX "idx_student_stats_story_id" ON "student_stats" USING btree("story_id");
--> statement-breakpoint
CREATE INDEX "idx_student_stats_total_episodes_started" ON "student_stats" USING btree("total_episodes_started");
--> statement-breakpoint
CREATE INDEX "idx_student_stats_total_episodes_completed" ON "student_stats" USING btree("total_episodes_completed");
--> statement-breakpoint
CREATE INDEX "idx_student_stats_total_chapters_started" ON "student_stats" USING btree("total_chapters_started");
--> statement-breakpoint
CREATE INDEX "idx_student_stats_total_chapters_completed" ON "student_stats" USING btree("total_chapters_completed");
--> statement-breakpoint
CREATE INDEX "idx_student_stats_total_questions_answered" ON "student_stats" USING btree("total_questions_answered");
--> statement-breakpoint
CREATE INDEX "idx_student_stats_total_episodes_completed_in_string" ON "student_stats" USING btree("total_episodes_completed_in_string");
--> statement-breakpoint
CREATE INDEX "idx_student_stats_total_episodes" ON "student_stats" USING btree("total_episodes");
--> statement-breakpoint
CREATE INDEX "idx_student_stats_last_activity" ON "student_stats" USING btree("last_activity");
--> statement-breakpoint
CREATE INDEX "idx_student_stats_last_activity_date" ON "student_stats" USING btree("last_activity_date");
--> statement-breakpoint
CREATE INDEX "idx_common_sessions_tombstone" ON "common_sessions" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_common_sessions_status" ON "common_sessions" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_common_sessions_previous_status" ON "common_sessions" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_common_sessions_version" ON "common_sessions" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_common_sessions_created_date" ON "common_sessions" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_common_sessions_updated_date" ON "common_sessions" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_common_sessions_organization_id" ON "common_sessions" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_common_sessions_created_by" ON "common_sessions" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_common_sessions_updated_by" ON "common_sessions" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_common_sessions_deleted_by" ON "common_sessions" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_common_sessions_requested_by" ON "common_sessions" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_common_sessions_tags" ON "common_sessions" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_common_sessions_categories" ON "common_sessions" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_common_sessions_code" ON "common_sessions" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_common_sessions_sensitivity_level" ON "common_sessions" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_common_sessions_user_id" ON "common_sessions" USING btree("user_id");
--> statement-breakpoint
CREATE INDEX "idx_common_sessions_session_token" ON "common_sessions" USING btree("session_token");
--> statement-breakpoint
CREATE INDEX "idx_common_sessions_route" ON "common_sessions" USING btree("route");
--> statement-breakpoint
CREATE INDEX "idx_common_sessions_session_status" ON "common_sessions" USING btree("session_status");
--> statement-breakpoint
CREATE INDEX "idx_common_sessions_activity_status" ON "common_sessions" USING btree("activity_status");
--> statement-breakpoint
CREATE INDEX "idx_common_sessions_last_activity_at" ON "common_sessions" USING btree("last_activity_at");
--> statement-breakpoint
CREATE INDEX "idx_common_sessions_expires_at" ON "common_sessions" USING btree("expires_at");
--> statement-breakpoint
CREATE INDEX "idx_common_sessions_device_details" ON "common_sessions" USING btree("device_details");
--> statement-breakpoint
CREATE INDEX "idx_common_sessions_ip_address" ON "common_sessions" USING btree("ip_address");
--> statement-breakpoint
CREATE INDEX "idx_common_sessions_portal_application_type" ON "common_sessions"("portal_application_type");
--> statement-breakpoint
CREATE INDEX "idx_episodes_tombstone" ON "episodes" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_episodes_status" ON "episodes" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_episodes_previous_status" ON "episodes" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_episodes_version" ON "episodes" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_episodes_created_date" ON "episodes" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_episodes_updated_date" ON "episodes" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_episodes_organization_id" ON "episodes" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_episodes_created_by" ON "episodes" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_episodes_updated_by" ON "episodes" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_episodes_deleted_by" ON "episodes" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_episodes_requested_by" ON "episodes" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_episodes_tags" ON "episodes" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_episodes_categories" ON "episodes" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_episodes_code" ON "episodes" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_episodes_sensitivity_level" ON "episodes" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_episodes_name" ON "episodes" USING btree("name");
--> statement-breakpoint
CREATE INDEX "idx_episodes_story_id" ON "episodes" USING btree("story_id");
--> statement-breakpoint
CREATE INDEX "idx_episodes_order" ON "episodes" USING btree("order");
--> statement-breakpoint
CREATE INDEX "idx_episodes_course_id" ON "episodes" USING btree("course_id");
--> statement-breakpoint
CREATE INDEX "idx_episodes_description" ON "episodes" USING btree("description");
--> statement-breakpoint
CREATE INDEX "idx_smtp_attachment_headers_tombstone" ON "smtp_attachment_headers" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_smtp_attachment_headers_status" ON "smtp_attachment_headers" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_smtp_attachment_headers_previous_status" ON "smtp_attachment_headers" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_smtp_attachment_headers_version" ON "smtp_attachment_headers" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_smtp_attachment_headers_created_date" ON "smtp_attachment_headers" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_smtp_attachment_headers_updated_date" ON "smtp_attachment_headers" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_smtp_attachment_headers_organization_id" ON "smtp_attachment_headers" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_smtp_attachment_headers_created_by" ON "smtp_attachment_headers" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_smtp_attachment_headers_updated_by" ON "smtp_attachment_headers" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_smtp_attachment_headers_deleted_by" ON "smtp_attachment_headers" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_smtp_attachment_headers_requested_by" ON "smtp_attachment_headers" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_smtp_attachment_headers_tags" ON "smtp_attachment_headers" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_smtp_attachment_headers_categories" ON "smtp_attachment_headers" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_smtp_attachment_headers_code" ON "smtp_attachment_headers" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_smtp_attachment_headers_sensitivity_level" ON "smtp_attachment_headers" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_smtp_attachment_headers_header_key" ON "smtp_attachment_headers" USING btree("header_key");
--> statement-breakpoint
CREATE INDEX "idx_smtp_attachment_headers_header_value" ON "smtp_attachment_headers" USING btree("header_value");
--> statement-breakpoint
CREATE INDEX "idx_smtp_attachment_headers_smtp_attachment_id" ON "smtp_attachment_headers" USING btree("smtp_attachment_id");
--> statement-breakpoint
CREATE INDEX "idx_courses_tombstone" ON "courses" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_courses_status" ON "courses" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_courses_previous_status" ON "courses" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_courses_version" ON "courses" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_courses_created_date" ON "courses" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_courses_updated_date" ON "courses" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_courses_organization_id" ON "courses" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_courses_created_by" ON "courses" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_courses_updated_by" ON "courses" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_courses_deleted_by" ON "courses" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_courses_requested_by" ON "courses" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_courses_tags" ON "courses" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_courses_categories" ON "courses" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_courses_code" ON "courses" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_courses_sensitivity_level" ON "courses" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_courses_title" ON "courses" USING btree("title");
--> statement-breakpoint
CREATE INDEX "idx_courses_order" ON "courses" USING btree("order");
--> statement-breakpoint
CREATE INDEX "idx_courses_description" ON "courses" USING btree("description");
--> statement-breakpoint
CREATE INDEX "idx_courses_is_show_assistant" ON "courses" USING btree("is_show_assistant");
--> statement-breakpoint
CREATE INDEX "idx_smtp_payloads_tombstone" ON "smtp_payloads" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_smtp_payloads_status" ON "smtp_payloads" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_smtp_payloads_previous_status" ON "smtp_payloads" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_smtp_payloads_version" ON "smtp_payloads" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_smtp_payloads_created_date" ON "smtp_payloads" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_smtp_payloads_updated_date" ON "smtp_payloads" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_smtp_payloads_organization_id" ON "smtp_payloads" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_smtp_payloads_created_by" ON "smtp_payloads" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_smtp_payloads_updated_by" ON "smtp_payloads" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_smtp_payloads_deleted_by" ON "smtp_payloads" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_smtp_payloads_requested_by" ON "smtp_payloads" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_smtp_payloads_tags" ON "smtp_payloads" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_smtp_payloads_categories" ON "smtp_payloads" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_smtp_payloads_code" ON "smtp_payloads" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_smtp_payloads_sensitivity_level" ON "smtp_payloads" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_smtp_payloads_sender" ON "smtp_payloads" USING btree("sender");
--> statement-breakpoint
CREATE INDEX "idx_smtp_payloads_recipients" ON "smtp_payloads" USING btree("recipients");
--> statement-breakpoint
CREATE INDEX "idx_smtp_payloads_cc_recipients" ON "smtp_payloads" USING btree("cc_recipients");
--> statement-breakpoint
CREATE INDEX "idx_smtp_payloads_bcc_recipients" ON "smtp_payloads" USING btree("bcc_recipients");
--> statement-breakpoint
CREATE INDEX "idx_smtp_payloads_subject" ON "smtp_payloads" USING btree("subject");
--> statement-breakpoint
CREATE INDEX "idx_smtp_payloads_html" ON "smtp_payloads" USING btree("html");
--> statement-breakpoint
CREATE INDEX "idx_smtp_payloads_parent_id" ON "smtp_payloads" USING btree("parent_id");
--> statement-breakpoint
CREATE INDEX "idx_smtp_payloads_attachment_ids" ON "smtp_payloads" USING btree("attachment_ids");
--> statement-breakpoint
CREATE INDEX "idx_smtp_payloads_thread_id" ON "smtp_payloads" USING btree("thread_id");
--> statement-breakpoint
CREATE INDEX "idx_smtp_payloads_send_strategy" ON "smtp_payloads" USING btree("send_strategy");
--> statement-breakpoint
CREATE INDEX "idx_smtp_payloads_priority" ON "smtp_payloads" USING btree("priority");
--> statement-breakpoint
CREATE INDEX "idx_smtp_payloads_transport_provider_id" ON "smtp_payloads" USING btree("transport_provider_id");
--> statement-breakpoint
CREATE INDEX "idx_smtp_payloads_source" ON "smtp_payloads" USING btree("source");
--> statement-breakpoint
CREATE INDEX "idx_smtp_payloads_method" ON "smtp_payloads" USING btree("method");
--> statement-breakpoint
CREATE INDEX "idx_organizations_name" ON "organizations" USING btree("name");
--> statement-breakpoint
CREATE INDEX "idx_organizations_parent_organization_id" ON "organizations" USING btree("parent_organization_id");
--> statement-breakpoint
CREATE INDEX "idx_organizations_root_organization_id" ON "organizations" USING btree("root_organization_id");
--> statement-breakpoint
CREATE INDEX "idx_organizations_skyll_id" ON "organizations" USING btree("skyll_id");
--> statement-breakpoint
CREATE INDEX "idx_organizations_school_id" ON "organizations" USING btree("school_id");
--> statement-breakpoint
CREATE INDEX "idx_organizations_district_id" ON "organizations" USING btree("district_id");
--> statement-breakpoint
CREATE INDEX "idx_organizations_department_id" ON "organizations" USING btree("department_id");
--> statement-breakpoint
CREATE INDEX "idx_organizations_city" ON "organizations" USING btree("city");
--> statement-breakpoint
CREATE INDEX "idx_organizations_county" ON "organizations" USING btree("county");
--> statement-breakpoint
CREATE INDEX "idx_organizations_state" ON "organizations" USING btree("state");
--> statement-breakpoint
CREATE INDEX "idx_organizations_school_identifier" ON "organizations" USING btree("school_identifier");
--> statement-breakpoint
CREATE INDEX "idx_organizations_district_identifier" ON "organizations" USING btree("district_identifier");
--> statement-breakpoint
CREATE INDEX "idx_organizations_organization_level" ON "organizations" USING btree("organization_level");
--> statement-breakpoint
CREATE INDEX "idx_organizations_path_level" ON "organizations" USING btree("path_level");
--> statement-breakpoint
CREATE INDEX "idx_organizations_superintendent_id" ON "organizations" USING btree("superintendent_id");
--> statement-breakpoint
CREATE INDEX "idx_organizations_principal_id" ON "organizations" USING btree("principal_id");
--> statement-breakpoint
CREATE INDEX "idx_postgres_channels_channel_name" ON "postgres_channels" USING btree("channel_name");
--> statement-breakpoint
CREATE INDEX "idx_postgres_channels_channel_timestamp" ON "postgres_channels" USING btree("channel_timestamp");
--> statement-breakpoint
CREATE INDEX "idx_postgres_channels_function" ON "postgres_channels" USING btree("function");
--> statement-breakpoint
CREATE INDEX "idx_smtp_response_links_tombstone" ON "smtp_response_links" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_smtp_response_links_status" ON "smtp_response_links" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_smtp_response_links_previous_status" ON "smtp_response_links" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_smtp_response_links_version" ON "smtp_response_links" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_smtp_response_links_created_date" ON "smtp_response_links" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_smtp_response_links_updated_date" ON "smtp_response_links" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_smtp_response_links_organization_id" ON "smtp_response_links" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_smtp_response_links_created_by" ON "smtp_response_links" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_smtp_response_links_updated_by" ON "smtp_response_links" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_smtp_response_links_deleted_by" ON "smtp_response_links" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_smtp_response_links_requested_by" ON "smtp_response_links" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_smtp_response_links_tags" ON "smtp_response_links" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_smtp_response_links_categories" ON "smtp_response_links" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_smtp_response_links_code" ON "smtp_response_links" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_smtp_response_links_sensitivity_level" ON "smtp_response_links" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_smtp_response_links_smtp_payload_id" ON "smtp_response_links" USING btree("smtp_payload_id");
--> statement-breakpoint
CREATE INDEX "idx_smtp_response_links_callback_url" ON "smtp_response_links" USING btree("callback_url");
--> statement-breakpoint
CREATE INDEX "idx_smtp_response_links_expiry" ON "smtp_response_links" USING btree("expiry");
--> statement-breakpoint
CREATE INDEX "idx_smtp_transactions_tombstone" ON "smtp_transactions" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_smtp_transactions_status" ON "smtp_transactions" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_smtp_transactions_previous_status" ON "smtp_transactions" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_smtp_transactions_version" ON "smtp_transactions" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_smtp_transactions_created_date" ON "smtp_transactions" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_smtp_transactions_updated_date" ON "smtp_transactions" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_smtp_transactions_organization_id" ON "smtp_transactions" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_smtp_transactions_created_by" ON "smtp_transactions" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_smtp_transactions_updated_by" ON "smtp_transactions" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_smtp_transactions_deleted_by" ON "smtp_transactions" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_smtp_transactions_requested_by" ON "smtp_transactions" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_smtp_transactions_tags" ON "smtp_transactions" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_smtp_transactions_categories" ON "smtp_transactions" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_smtp_transactions_code" ON "smtp_transactions" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_smtp_transactions_sensitivity_level" ON "smtp_transactions" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_smtp_transactions_smtp_payload_id" ON "smtp_transactions" USING btree("smtp_payload_id");
--> statement-breakpoint
CREATE INDEX "idx_smtp_transactions_transaction_status" ON "smtp_transactions" USING btree("transaction_status");
--> statement-breakpoint
CREATE INDEX "idx_smtp_transactions_response_message" ON "smtp_transactions" USING btree("response_message");
--> statement-breakpoint
CREATE INDEX "idx_smtp_transactions_trigger_type" ON "smtp_transactions" USING btree("trigger_type");
--> statement-breakpoint
CREATE INDEX "idx_report_files_tombstone" ON "report_files" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_report_files_status" ON "report_files" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_report_files_previous_status" ON "report_files" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_report_files_version" ON "report_files" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_report_files_created_date" ON "report_files" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_report_files_updated_date" ON "report_files" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_report_files_organization_id" ON "report_files" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_report_files_created_by" ON "report_files" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_report_files_updated_by" ON "report_files" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_report_files_deleted_by" ON "report_files" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_report_files_requested_by" ON "report_files" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_report_files_tags" ON "report_files" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_report_files_categories" ON "report_files" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_report_files_code" ON "report_files" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_report_files_sensitivity_level" ON "report_files" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_report_files_report_id" ON "report_files" USING btree("report_id");
--> statement-breakpoint
CREATE INDEX "idx_report_files_cybertipline_report_file_id" ON "report_files" USING btree("cybertipline_report_file_id");
--> statement-breakpoint
CREATE INDEX "idx_report_files_evidence_url" ON "report_files" USING btree("evidence_url");
--> statement-breakpoint
CREATE INDEX "idx_invitations_tombstone" ON "invitations" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_invitations_status" ON "invitations" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_invitations_previous_status" ON "invitations" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_invitations_version" ON "invitations" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_invitations_created_date" ON "invitations" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_invitations_updated_date" ON "invitations" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_invitations_organization_id" ON "invitations" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_invitations_created_by" ON "invitations" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_invitations_updated_by" ON "invitations" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_invitations_deleted_by" ON "invitations" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_invitations_requested_by" ON "invitations" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_invitations_tags" ON "invitations" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_invitations_categories" ON "invitations" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_invitations_code" ON "invitations" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_invitations_sensitivity_level" ON "invitations" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_invitations_account_id" ON "invitations" USING btree("account_id");
--> statement-breakpoint
CREATE INDEX "idx_invitations_account_organization_id" ON "invitations" USING btree("account_organization_id");
--> statement-breakpoint
CREATE INDEX "idx_invitations_expiration_date" ON "invitations" USING btree("expiration_date");
--> statement-breakpoint
CREATE INDEX "idx_invitations_expiration_time" ON "invitations" USING btree("expiration_time");
--> statement-breakpoint
CREATE INDEX "idx_invitations_related_contact_id" ON "invitations" USING btree("related_contact_id");
--> statement-breakpoint
CREATE INDEX "idx_faq_answers_tombstone" ON "faq_answers" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_faq_answers_status" ON "faq_answers" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_faq_answers_previous_status" ON "faq_answers" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_faq_answers_version" ON "faq_answers" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_faq_answers_created_date" ON "faq_answers" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_faq_answers_updated_date" ON "faq_answers" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_faq_answers_organization_id" ON "faq_answers" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_faq_answers_created_by" ON "faq_answers" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_faq_answers_updated_by" ON "faq_answers" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_faq_answers_deleted_by" ON "faq_answers" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_faq_answers_requested_by" ON "faq_answers" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_faq_answers_tags" ON "faq_answers" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_faq_answers_categories" ON "faq_answers" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_faq_answers_code" ON "faq_answers" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_faq_answers_sensitivity_level" ON "faq_answers" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_faq_answers_faq_id" ON "faq_answers" USING btree("faq_id");
--> statement-breakpoint
CREATE INDEX "idx_faq_answers_category_id" ON "faq_answers" USING btree("category_id");
--> statement-breakpoint
CREATE INDEX "idx_faq_answers_question_id" ON "faq_answers" USING btree("question_id");
--> statement-breakpoint
CREATE INDEX "idx_faq_answers_answer" ON "faq_answers" USING btree("answer");
--> statement-breakpoint
CREATE INDEX "idx_faq_answers_order" ON "faq_answers" USING btree("order");
--> statement-breakpoint
CREATE INDEX "idx_classrooms_tombstone" ON "classrooms" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_classrooms_status" ON "classrooms" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_classrooms_previous_status" ON "classrooms" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_classrooms_version" ON "classrooms" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_classrooms_created_date" ON "classrooms" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_classrooms_updated_date" ON "classrooms" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_classrooms_organization_id" ON "classrooms" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_classrooms_created_by" ON "classrooms" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_classrooms_updated_by" ON "classrooms" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_classrooms_deleted_by" ON "classrooms" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_classrooms_requested_by" ON "classrooms" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_classrooms_tags" ON "classrooms" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_classrooms_categories" ON "classrooms" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_classrooms_code" ON "classrooms" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_classrooms_sensitivity_level" ON "classrooms" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_classrooms_name" ON "classrooms" USING btree("name");
--> statement-breakpoint
CREATE INDEX "idx_classrooms_description" ON "classrooms" USING btree("description");
--> statement-breakpoint
CREATE INDEX "idx_classrooms_grade" ON "classrooms" USING btree("grade");
--> statement-breakpoint
CREATE INDEX "idx_classrooms_color" ON "classrooms" USING btree("color");
--> statement-breakpoint
CREATE INDEX "idx_classrooms_avatar" ON "classrooms" USING btree("avatar");
--> statement-breakpoint
CREATE INDEX "idx_classrooms_department_id" ON "classrooms" USING btree("department_id");
--> statement-breakpoint
CREATE INDEX "idx_classrooms_district_id" ON "classrooms" USING btree("district_id");
--> statement-breakpoint
CREATE INDEX "idx_classrooms_school_id" ON "classrooms" USING btree("school_id");
--> statement-breakpoint
CREATE INDEX "idx_classrooms_teacher_id" ON "classrooms" USING btree("teacher_id");
--> statement-breakpoint
CREATE INDEX "idx_notifications_tombstone" ON "notifications" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_notifications_status" ON "notifications" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_notifications_previous_status" ON "notifications" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_notifications_version" ON "notifications" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_notifications_created_date" ON "notifications" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_notifications_updated_date" ON "notifications" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_notifications_organization_id" ON "notifications" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_notifications_created_by" ON "notifications" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_notifications_updated_by" ON "notifications" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_notifications_deleted_by" ON "notifications" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_notifications_requested_by" ON "notifications" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_notifications_tags" ON "notifications" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_notifications_categories" ON "notifications" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_notifications_code" ON "notifications" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_notifications_sensitivity_level" ON "notifications" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_notifications_title" ON "notifications" USING btree("title");
--> statement-breakpoint
CREATE INDEX "idx_notifications_description" ON "notifications" USING btree("description");
--> statement-breakpoint
CREATE INDEX "idx_notifications_event_timestamp" ON "notifications" USING btree("event_timestamp");
--> statement-breakpoint
CREATE INDEX "idx_notifications_link" ON "notifications" USING btree("link");
--> statement-breakpoint
CREATE INDEX "idx_notifications_icon" ON "notifications" USING btree("icon");
--> statement-breakpoint
CREATE INDEX "idx_notifications_source" ON "notifications" USING btree("source");
--> statement-breakpoint
CREATE INDEX "idx_notifications_is_pinned" ON "notifications" USING btree("is_pinned");
--> statement-breakpoint
CREATE INDEX "idx_notifications_recipient_id" ON "notifications" USING btree("recipient_id");
--> statement-breakpoint
CREATE INDEX "idx_notifications_notification_status" ON "notifications" USING btree("notification_status");
--> statement-breakpoint
CREATE INDEX "idx_notifications_priority_label" ON "notifications" USING btree("priority_label");
--> statement-breakpoint
CREATE INDEX "idx_notifications_priority_level" ON "notifications" USING btree("priority_level");
--> statement-breakpoint
CREATE INDEX "idx_notifications_expiry_date" ON "notifications" USING btree("expiry_date");
--> statement-breakpoint
CREATE INDEX "idx_classroom_stats_tombstone" ON "classroom_stats" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_classroom_stats_status" ON "classroom_stats" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_classroom_stats_previous_status" ON "classroom_stats" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_classroom_stats_version" ON "classroom_stats" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_classroom_stats_created_date" ON "classroom_stats" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_classroom_stats_updated_date" ON "classroom_stats" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_classroom_stats_organization_id" ON "classroom_stats" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_classroom_stats_created_by" ON "classroom_stats" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_classroom_stats_updated_by" ON "classroom_stats" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_classroom_stats_deleted_by" ON "classroom_stats" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_classroom_stats_requested_by" ON "classroom_stats" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_classroom_stats_tags" ON "classroom_stats" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_classroom_stats_categories" ON "classroom_stats" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_classroom_stats_code" ON "classroom_stats" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_classroom_stats_sensitivity_level" ON "classroom_stats" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_classroom_stats_classroom_id" ON "classroom_stats" USING btree("classroom_id");
--> statement-breakpoint
CREATE INDEX "idx_classroom_stats_story_id" ON "classroom_stats" USING btree("story_id");
--> statement-breakpoint
CREATE INDEX "idx_classroom_stats_student_id" ON "classroom_stats" USING btree("student_id");
--> statement-breakpoint
CREATE INDEX "idx_classroom_stats_course_id" ON "classroom_stats" USING btree("course_id");
--> statement-breakpoint
CREATE INDEX "idx_classroom_stats_number_of_students" ON "classroom_stats" USING btree("number_of_students");
--> statement-breakpoint
CREATE INDEX "idx_classroom_stats_student_completed" ON "classroom_stats" USING btree("student_completed");
--> statement-breakpoint
CREATE INDEX "idx_classroom_stats_progress_status" ON "classroom_stats" USING btree("progress_status");
--> statement-breakpoint
CREATE INDEX "idx_classroom_stats_progress_start_date" ON "classroom_stats" USING btree("progress_start_date");
--> statement-breakpoint
CREATE INDEX "idx_classroom_stats_progress_start_time" ON "classroom_stats" USING btree("progress_start_time");
--> statement-breakpoint
CREATE INDEX "idx_classroom_stats_progress_start_date_string" ON "classroom_stats" USING btree("progress_start_date_string");
--> statement-breakpoint
CREATE INDEX "idx_classroom_stats_completed_date" ON "classroom_stats" USING btree("completed_date");
--> statement-breakpoint
CREATE INDEX "idx_classroom_stats_completed_time" ON "classroom_stats" USING btree("completed_time");
--> statement-breakpoint
CREATE INDEX "idx_onboarding_contacts_tombstone" ON "onboarding_contacts" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_onboarding_contacts_status" ON "onboarding_contacts" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_onboarding_contacts_previous_status" ON "onboarding_contacts" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_onboarding_contacts_version" ON "onboarding_contacts" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_onboarding_contacts_created_date" ON "onboarding_contacts" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_onboarding_contacts_updated_date" ON "onboarding_contacts" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_onboarding_contacts_organization_id" ON "onboarding_contacts" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_onboarding_contacts_created_by" ON "onboarding_contacts" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_onboarding_contacts_updated_by" ON "onboarding_contacts" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_onboarding_contacts_deleted_by" ON "onboarding_contacts" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_onboarding_contacts_requested_by" ON "onboarding_contacts" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_onboarding_contacts_tags" ON "onboarding_contacts" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_onboarding_contacts_categories" ON "onboarding_contacts" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_onboarding_contacts_code" ON "onboarding_contacts" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_onboarding_contacts_sensitivity_level" ON "onboarding_contacts" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_onboarding_contacts_contact_id" ON "onboarding_contacts" USING btree("contact_id");
--> statement-breakpoint
CREATE INDEX "idx_onboarding_contacts_is_done" ON "onboarding_contacts" USING btree("is_done");
--> statement-breakpoint
CREATE INDEX "idx_conversation_messages_tombstone" ON "conversation_messages" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_conversation_messages_status" ON "conversation_messages" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_conversation_messages_previous_status" ON "conversation_messages" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_conversation_messages_version" ON "conversation_messages" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_conversation_messages_created_date" ON "conversation_messages" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_conversation_messages_updated_date" ON "conversation_messages" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_conversation_messages_organization_id" ON "conversation_messages" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_conversation_messages_created_by" ON "conversation_messages" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_conversation_messages_updated_by" ON "conversation_messages" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_conversation_messages_deleted_by" ON "conversation_messages" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_conversation_messages_requested_by" ON "conversation_messages" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_conversation_messages_tags" ON "conversation_messages" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_conversation_messages_categories" ON "conversation_messages" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_conversation_messages_code" ON "conversation_messages" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_conversation_messages_sensitivity_level" ON "conversation_messages" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_conversation_messages_conversation_id" ON "conversation_messages" USING btree("conversation_id");
--> statement-breakpoint
CREATE INDEX "idx_conversation_messages_content" ON "conversation_messages" USING btree("content");
--> statement-breakpoint
CREATE INDEX "idx_conversation_messages_role" ON "conversation_messages" USING btree("role");
--> statement-breakpoint
CREATE INDEX "idx_incident_reports_tombstone" ON "incident_reports" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_incident_reports_status" ON "incident_reports" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_incident_reports_previous_status" ON "incident_reports" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_incident_reports_version" ON "incident_reports" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_incident_reports_created_date" ON "incident_reports" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_incident_reports_updated_date" ON "incident_reports" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_incident_reports_organization_id" ON "incident_reports" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_incident_reports_created_by" ON "incident_reports" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_incident_reports_updated_by" ON "incident_reports" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_incident_reports_deleted_by" ON "incident_reports" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_incident_reports_requested_by" ON "incident_reports" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_incident_reports_tags" ON "incident_reports" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_incident_reports_categories" ON "incident_reports" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_incident_reports_code" ON "incident_reports" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_incident_reports_sensitivity_level" ON "incident_reports" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_incident_reports_user_id" ON "incident_reports" USING btree("user_id");
--> statement-breakpoint
CREATE INDEX "idx_incident_reports_report_status" ON "incident_reports" USING btree("report_status");
--> statement-breakpoint
CREATE INDEX "idx_incident_reports_source" ON "incident_reports" USING btree("source");
--> statement-breakpoint
CREATE INDEX "idx_incident_reports_app_version" ON "incident_reports" USING btree("app_version");
--> statement-breakpoint
CREATE INDEX "idx_incident_reports_conversation_id" ON "incident_reports" USING btree("conversation_id");
--> statement-breakpoint
CREATE INDEX "idx_incident_reports_cybertipline_report_id" ON "incident_reports" USING btree("cybertipline_report_id");
--> statement-breakpoint
CREATE INDEX "idx_incident_reports_cybertipline_report_case_number" ON "incident_reports" USING btree("cybertipline_report_case_number");
--> statement-breakpoint
CREATE INDEX "idx_incident_reports_cybertipline_report_type" ON "incident_reports"("cybertipline_report_type");
--> statement-breakpoint
CREATE INDEX "idx_incident_reports_cybertipline_report_status" ON "incident_reports" USING btree("cybertipline_report_status");
--> statement-breakpoint
CREATE INDEX "idx_incident_reports_cybertipline_api_response_status_code" ON "incident_reports" USING btree("cybertipline_api_response_status_code");
--> statement-breakpoint
CREATE INDEX "idx_incident_reports_additional_info" ON "incident_reports" USING btree("additional_info");
--> statement-breakpoint
CREATE INDEX "idx_incident_reports_reporter_first_name" ON "incident_reports" USING btree("reporter_first_name");
--> statement-breakpoint
CREATE INDEX "idx_incident_reports_reporter_last_name" ON "incident_reports" USING btree("reporter_last_name");
--> statement-breakpoint
CREATE INDEX "idx_incident_reports_reporter_email" ON "incident_reports" USING btree("reporter_email");
--> statement-breakpoint
CREATE INDEX "idx_incident_reports_reporter_type" ON "incident_reports"("reporter_type");
--> statement-breakpoint
CREATE INDEX "idx_incident_reports_incident_date" ON "incident_reports" USING btree("incident_date");
--> statement-breakpoint
CREATE INDEX "idx_incident_reports_incident_time" ON "incident_reports" USING btree("incident_time");
--> statement-breakpoint
CREATE INDEX "idx_incident_reports_incident_timezone" ON "incident_reports" USING btree("incident_timezone");
--> statement-breakpoint
CREATE INDEX "idx_incident_reports_incident_type" ON "incident_reports"("incident_type");
--> statement-breakpoint
CREATE INDEX "idx_incident_reports_incident_escalate_to_high_priority" ON "incident_reports" USING btree("incident_escalate_to_high_priority");
--> statement-breakpoint
CREATE INDEX "idx_incident_reports_incident_location_type" ON "incident_reports"("incident_location_type");
--> statement-breakpoint
CREATE INDEX "idx_incident_reports_incident_location_type_value" ON "incident_reports" USING btree("incident_location_type_value");
--> statement-breakpoint
CREATE INDEX "idx_incident_reports_reported_person_first_name" ON "incident_reports" USING btree("reported_person_first_name");
--> statement-breakpoint
CREATE INDEX "idx_incident_reports_reported_person_last_name" ON "incident_reports" USING btree("reported_person_last_name");
--> statement-breakpoint
CREATE INDEX "idx_incident_reports_reported_person_email" ON "incident_reports" USING btree("reported_person_email");
--> statement-breakpoint
CREATE INDEX "idx_incident_reports_reported_person_screen_name" ON "incident_reports" USING btree("reported_person_screen_name");
--> statement-breakpoint
CREATE INDEX "idx_incident_reports_reported_person_profile_url" ON "incident_reports" USING btree("reported_person_profile_url");
--> statement-breakpoint
CREATE INDEX "idx_incident_reports_reported_person_address_street" ON "incident_reports" USING btree("reported_person_address_street");
--> statement-breakpoint
CREATE INDEX "idx_incident_reports_reported_person_address_city" ON "incident_reports" USING btree("reported_person_address_city");
--> statement-breakpoint
CREATE INDEX "idx_incident_reports_reported_person_address_zip_code" ON "incident_reports" USING btree("reported_person_address_zip_code");
--> statement-breakpoint
CREATE INDEX "idx_incident_reports_reported_person_address_state" ON "incident_reports" USING btree("reported_person_address_state");
--> statement-breakpoint
CREATE INDEX "idx_incident_reports_reported_person_address_non_usa_state" ON "incident_reports" USING btree("reported_person_address_non_usa_state");
--> statement-breakpoint
CREATE INDEX "idx_incident_reports_reported_person_address_country" ON "incident_reports" USING btree("reported_person_address_country");
--> statement-breakpoint
CREATE INDEX "idx_incident_reports_reported_person_phone_country_code" ON "incident_reports" USING btree("reported_person_phone_country_code");
--> statement-breakpoint
CREATE INDEX "idx_incident_reports_reported_person_phone_extension" ON "incident_reports" USING btree("reported_person_phone_extension");
--> statement-breakpoint
CREATE INDEX "idx_incident_reports_ip_address" ON "incident_reports" USING btree("ip_address");
--> statement-breakpoint
CREATE INDEX "idx_incident_reports_date_submission" ON "incident_reports" USING btree("date_submission");
--> statement-breakpoint
CREATE INDEX "idx_incident_reports_submission_date" ON "incident_reports" USING btree("submission_date");
--> statement-breakpoint
CREATE INDEX "idx_incident_reports_submission_time" ON "incident_reports" USING btree("submission_time");
--> statement-breakpoint
CREATE INDEX "idx_incident_reports_evidence_urls" ON "incident_reports" USING btree("evidence_urls");
--> statement-breakpoint
CREATE INDEX "idx_incident_reports_reported_person_username" ON "incident_reports" USING btree("reported_person_username");
--> statement-breakpoint
CREATE INDEX "idx_reports_tombstone" ON "reports" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_reports_status" ON "reports" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_reports_previous_status" ON "reports" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_reports_version" ON "reports" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_reports_created_date" ON "reports" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_reports_updated_date" ON "reports" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_reports_organization_id" ON "reports" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_reports_created_by" ON "reports" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_reports_updated_by" ON "reports" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_reports_deleted_by" ON "reports" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_reports_requested_by" ON "reports" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_reports_tags" ON "reports" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_reports_categories" ON "reports" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_reports_code" ON "reports" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_reports_sensitivity_level" ON "reports" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_reports_report_type" ON "reports"("report_type");
--> statement-breakpoint
CREATE INDEX "idx_reports_job_status" ON "reports" USING btree("job_status");
--> statement-breakpoint
CREATE INDEX "idx_reports_start_date" ON "reports" USING btree("start_date");
--> statement-breakpoint
CREATE INDEX "idx_reports_start_time" ON "reports" USING btree("start_time");
--> statement-breakpoint
CREATE INDEX "idx_reports_end_date" ON "reports" USING btree("end_date");
--> statement-breakpoint
CREATE INDEX "idx_reports_end_time" ON "reports" USING btree("end_time");
--> statement-breakpoint
CREATE INDEX "idx_reports_progress" ON "reports" USING btree("progress");
--> statement-breakpoint
CREATE INDEX "idx_reports_reason" ON "reports" USING btree("reason");
--> statement-breakpoint
CREATE INDEX "idx_reports_department_id" ON "reports" USING btree("department_id");
--> statement-breakpoint
CREATE INDEX "idx_reports_district_id" ON "reports" USING btree("district_id");
--> statement-breakpoint
CREATE INDEX "idx_reports_school_id" ON "reports" USING btree("school_id");
--> statement-breakpoint
CREATE INDEX "idx_reports_course_id" ON "reports" USING btree("course_id");
--> statement-breakpoint
CREATE INDEX "idx_reports_story_id" ON "reports" USING btree("story_id");
--> statement-breakpoint
CREATE INDEX "idx_reports_record_count" ON "reports" USING btree("record_count");
--> statement-breakpoint
CREATE INDEX "idx_game_responses_tombstone" ON "game_responses" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_game_responses_status" ON "game_responses" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_game_responses_previous_status" ON "game_responses" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_game_responses_version" ON "game_responses" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_game_responses_created_date" ON "game_responses" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_game_responses_updated_date" ON "game_responses" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_game_responses_organization_id" ON "game_responses" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_game_responses_created_by" ON "game_responses" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_game_responses_updated_by" ON "game_responses" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_game_responses_deleted_by" ON "game_responses" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_game_responses_requested_by" ON "game_responses" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_game_responses_tags" ON "game_responses" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_game_responses_categories" ON "game_responses" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_game_responses_code" ON "game_responses" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_game_responses_sensitivity_level" ON "game_responses" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_game_responses_user_id" ON "game_responses" USING btree("user_id");
--> statement-breakpoint
CREATE INDEX "idx_game_responses_course_id" ON "game_responses" USING btree("course_id");
--> statement-breakpoint
CREATE INDEX "idx_game_responses_story_id" ON "game_responses" USING btree("story_id");
--> statement-breakpoint
CREATE INDEX "idx_game_responses_episode_id" ON "game_responses" USING btree("episode_id");
--> statement-breakpoint
CREATE INDEX "idx_game_responses_chapter_number" ON "game_responses" USING btree("chapter_number");
--> statement-breakpoint
CREATE INDEX "idx_game_responses_question_text" ON "game_responses" USING btree("question_text");
--> statement-breakpoint
CREATE INDEX "idx_game_responses_selected_answer" ON "game_responses" USING btree("selected_answer");
--> statement-breakpoint
CREATE INDEX "idx_game_responses_game_status" ON "game_responses" USING btree("game_status");
--> statement-breakpoint
CREATE INDEX "idx_game_responses_options" ON "game_responses" USING btree("options");
--> statement-breakpoint
CREATE INDEX "idx_emotion_predictions_tombstone" ON "emotion_predictions" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_emotion_predictions_status" ON "emotion_predictions" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_emotion_predictions_previous_status" ON "emotion_predictions" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_emotion_predictions_version" ON "emotion_predictions" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_emotion_predictions_created_date" ON "emotion_predictions" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_emotion_predictions_updated_date" ON "emotion_predictions" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_emotion_predictions_organization_id" ON "emotion_predictions" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_emotion_predictions_created_by" ON "emotion_predictions" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_emotion_predictions_updated_by" ON "emotion_predictions" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_emotion_predictions_deleted_by" ON "emotion_predictions" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_emotion_predictions_requested_by" ON "emotion_predictions" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_emotion_predictions_tags" ON "emotion_predictions" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_emotion_predictions_categories" ON "emotion_predictions" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_emotion_predictions_code" ON "emotion_predictions" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_emotion_predictions_sensitivity_level" ON "emotion_predictions" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_emotion_predictions_conversation_id" ON "emotion_predictions" USING btree("conversation_id");
--> statement-breakpoint
CREATE INDEX "idx_emotion_predictions_model" ON "emotion_predictions" USING btree("model");
--> statement-breakpoint
CREATE INDEX "idx_emotion_predictions_prob" ON "emotion_predictions" USING btree("prob");
--> statement-breakpoint
CREATE INDEX "idx_emotion_predictions_box" ON "emotion_predictions" USING btree("box");
--> statement-breakpoint
CREATE INDEX "idx_emotion_predictions_emotions" ON "emotion_predictions" USING btree("emotions");
--> statement-breakpoint
CREATE INDEX "idx_emotion_predictions_time" ON "emotion_predictions" USING btree("time");
--> statement-breakpoint
CREATE INDEX "idx_emotion_predictions_id" ON "emotion_predictions" USING btree("id");
--> statement-breakpoint
CREATE INDEX "idx_emotion_predictions_timestamp" ON "emotion_predictions" USING btree("timestamp");
--> statement-breakpoint
CREATE INDEX "idx_emotion_predictions_hypertable_timestamp" ON "emotion_predictions" USING btree("hypertable_timestamp");
--> statement-breakpoint
CREATE INDEX "idx_game_review_question_reports_tombstone" ON "game_review_question_reports" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_game_review_question_reports_status" ON "game_review_question_reports" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_game_review_question_reports_previous_status" ON "game_review_question_reports" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_game_review_question_reports_version" ON "game_review_question_reports" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_game_review_question_reports_created_date" ON "game_review_question_reports" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_game_review_question_reports_updated_date" ON "game_review_question_reports" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_game_review_question_reports_organization_id" ON "game_review_question_reports" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_game_review_question_reports_created_by" ON "game_review_question_reports" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_game_review_question_reports_updated_by" ON "game_review_question_reports" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_game_review_question_reports_deleted_by" ON "game_review_question_reports" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_game_review_question_reports_requested_by" ON "game_review_question_reports" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_game_review_question_reports_tags" ON "game_review_question_reports" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_game_review_question_reports_categories" ON "game_review_question_reports" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_game_review_question_reports_code" ON "game_review_question_reports" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_game_review_question_reports_sensitivity_level" ON "game_review_question_reports" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_game_review_question_reports_course_id" ON "game_review_question_reports" USING btree("course_id");
--> statement-breakpoint
CREATE INDEX "idx_game_review_question_reports_course_title" ON "game_review_question_reports" USING btree("course_title");
--> statement-breakpoint
CREATE INDEX "idx_game_review_question_reports_story_id" ON "game_review_question_reports" USING btree("story_id");
--> statement-breakpoint
CREATE INDEX "idx_game_review_question_reports_story_title" ON "game_review_question_reports" USING btree("story_title");
--> statement-breakpoint
CREATE INDEX "idx_game_review_question_reports_episode_number" ON "game_review_question_reports" USING btree("episode_number");
--> statement-breakpoint
CREATE INDEX "idx_game_review_question_reports_question" ON "game_review_question_reports" USING btree("question");
--> statement-breakpoint
CREATE INDEX "idx_game_review_question_reports_students" ON "game_review_question_reports" USING btree("students");
--> statement-breakpoint
CREATE INDEX "idx_game_review_question_reports_strongly_disagree" ON "game_review_question_reports" USING btree("strongly_disagree");
--> statement-breakpoint
CREATE INDEX "idx_game_review_question_reports_disagree" ON "game_review_question_reports" USING btree("disagree");
--> statement-breakpoint
CREATE INDEX "idx_game_review_question_reports_agree" ON "game_review_question_reports" USING btree("agree");
--> statement-breakpoint
CREATE INDEX "idx_game_review_question_reports_strongly_agree" ON "game_review_question_reports" USING btree("strongly_agree");
--> statement-breakpoint
CREATE INDEX "idx_game_review_question_reports_report_id" ON "game_review_question_reports" USING btree("report_id");
--> statement-breakpoint
CREATE INDEX "idx_student_sessions_tombstone" ON "student_sessions" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_student_sessions_status" ON "student_sessions" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_student_sessions_previous_status" ON "student_sessions" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_student_sessions_version" ON "student_sessions" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_student_sessions_created_date" ON "student_sessions" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_student_sessions_updated_date" ON "student_sessions" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_student_sessions_organization_id" ON "student_sessions" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_student_sessions_created_by" ON "student_sessions" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_student_sessions_updated_by" ON "student_sessions" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_student_sessions_deleted_by" ON "student_sessions" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_student_sessions_requested_by" ON "student_sessions" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_student_sessions_tags" ON "student_sessions" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_student_sessions_categories" ON "student_sessions" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_student_sessions_code" ON "student_sessions" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_student_sessions_sensitivity_level" ON "student_sessions" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_student_sessions_user_id" ON "student_sessions" USING btree("user_id");
--> statement-breakpoint
CREATE INDEX "idx_student_sessions_session_token" ON "student_sessions" USING btree("session_token");
--> statement-breakpoint
CREATE INDEX "idx_student_sessions_route" ON "student_sessions" USING btree("route");
--> statement-breakpoint
CREATE INDEX "idx_student_sessions_student_status" ON "student_sessions" USING btree("student_status");
--> statement-breakpoint
CREATE INDEX "idx_student_sessions_activity_status" ON "student_sessions" USING btree("activity_status");
--> statement-breakpoint
CREATE INDEX "idx_student_sessions_last_activity_at" ON "student_sessions" USING btree("last_activity_at");
--> statement-breakpoint
CREATE INDEX "idx_student_sessions_expires_at" ON "student_sessions" USING btree("expires_at");
--> statement-breakpoint
CREATE INDEX "idx_student_sessions_device_details" ON "student_sessions" USING btree("device_details");
--> statement-breakpoint
CREATE INDEX "idx_student_sessions_ip_address" ON "student_sessions" USING btree("ip_address");
--> statement-breakpoint
CREATE INDEX "idx_transport_providers_tombstone" ON "transport_providers" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_transport_providers_status" ON "transport_providers" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_transport_providers_previous_status" ON "transport_providers" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_transport_providers_version" ON "transport_providers" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_transport_providers_created_date" ON "transport_providers" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_transport_providers_updated_date" ON "transport_providers" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_transport_providers_organization_id" ON "transport_providers" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_transport_providers_created_by" ON "transport_providers" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_transport_providers_updated_by" ON "transport_providers" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_transport_providers_deleted_by" ON "transport_providers" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_transport_providers_requested_by" ON "transport_providers" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_transport_providers_tags" ON "transport_providers" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_transport_providers_categories" ON "transport_providers" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_transport_providers_code" ON "transport_providers" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_transport_providers_sensitivity_level" ON "transport_providers" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_transport_providers_name" ON "transport_providers" USING btree("name");
--> statement-breakpoint
CREATE INDEX "idx_faqs_tombstone" ON "faqs" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_faqs_status" ON "faqs" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_faqs_previous_status" ON "faqs" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_faqs_version" ON "faqs" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_faqs_created_date" ON "faqs" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_faqs_updated_date" ON "faqs" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_faqs_organization_id" ON "faqs" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_faqs_created_by" ON "faqs" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_faqs_updated_by" ON "faqs" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_faqs_deleted_by" ON "faqs" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_faqs_requested_by" ON "faqs" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_faqs_tags" ON "faqs" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_faqs_categories" ON "faqs" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_faqs_code" ON "faqs" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_faqs_sensitivity_level" ON "faqs" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_faqs_target_app" ON "faqs" USING btree("target_app");
--> statement-breakpoint
CREATE INDEX "idx_report_child_victims_tombstone" ON "report_child_victims" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_report_child_victims_status" ON "report_child_victims" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_report_child_victims_previous_status" ON "report_child_victims" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_report_child_victims_version" ON "report_child_victims" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_report_child_victims_created_date" ON "report_child_victims" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_report_child_victims_updated_date" ON "report_child_victims" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_report_child_victims_organization_id" ON "report_child_victims" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_report_child_victims_created_by" ON "report_child_victims" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_report_child_victims_updated_by" ON "report_child_victims" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_report_child_victims_deleted_by" ON "report_child_victims" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_report_child_victims_requested_by" ON "report_child_victims" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_report_child_victims_tags" ON "report_child_victims" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_report_child_victims_categories" ON "report_child_victims" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_report_child_victims_code" ON "report_child_victims" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_report_child_victims_sensitivity_level" ON "report_child_victims" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_report_child_victims_report_id" ON "report_child_victims" USING btree("report_id");
--> statement-breakpoint
CREATE INDEX "idx_report_child_victims_first_name" ON "report_child_victims" USING btree("first_name");
--> statement-breakpoint
CREATE INDEX "idx_report_child_victims_last_name" ON "report_child_victims" USING btree("last_name");
--> statement-breakpoint
CREATE INDEX "idx_report_child_victims_birth_date" ON "report_child_victims" USING btree("birth_date");
--> statement-breakpoint
CREATE INDEX "idx_report_child_victims_email" ON "report_child_victims" USING btree("email");
--> statement-breakpoint
CREATE INDEX "idx_report_child_victims_address_street" ON "report_child_victims" USING btree("address_street");
--> statement-breakpoint
CREATE INDEX "idx_report_child_victims_address_city" ON "report_child_victims" USING btree("address_city");
--> statement-breakpoint
CREATE INDEX "idx_report_child_victims_address_zip_code" ON "report_child_victims" USING btree("address_zip_code");
--> statement-breakpoint
CREATE INDEX "idx_report_child_victims_address_state" ON "report_child_victims" USING btree("address_state");
--> statement-breakpoint
CREATE INDEX "idx_report_child_victims_address_non_usa_state" ON "report_child_victims" USING btree("address_non_usa_state");
--> statement-breakpoint
CREATE INDEX "idx_report_child_victims_address_country" ON "report_child_victims" USING btree("address_country");
--> statement-breakpoint
CREATE INDEX "idx_report_child_victims_phone_country_code" ON "report_child_victims" USING btree("phone_country_code");
--> statement-breakpoint
CREATE INDEX "idx_report_child_victims_phone_extension" ON "report_child_victims" USING btree("phone_extension");
--> statement-breakpoint
CREATE INDEX "idx_report_child_victims_screen_name" ON "report_child_victims" USING btree("screen_name");
--> statement-breakpoint
ALTER TABLE "game_choice_reports" ADD CONSTRAINT "fk_game_choice_reports_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "game_choice_reports" ADD CONSTRAINT "fk_game_choice_reports_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "game_choice_reports" ADD CONSTRAINT "fk_game_choice_reports_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "game_choice_reports" ADD CONSTRAINT "fk_game_choice_reports_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "game_choice_reports" ADD CONSTRAINT "fk_game_choice_reports_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "game_choice_reports" ADD CONSTRAINT "fk_game_choice_reports_course_id" FOREIGN KEY ("course_id") REFERENCES "public"."courses"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "game_choice_reports" ADD CONSTRAINT "fk_game_choice_reports_story_id" FOREIGN KEY ("story_id") REFERENCES "public"."stories"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "game_choice_reports" ADD CONSTRAINT "fk_game_choice_reports_report_id" FOREIGN KEY ("report_id") REFERENCES "public"."reports"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "stories" ADD CONSTRAINT "fk_stories_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "stories" ADD CONSTRAINT "fk_stories_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "stories" ADD CONSTRAINT "fk_stories_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "stories" ADD CONSTRAINT "fk_stories_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "stories" ADD CONSTRAINT "fk_stories_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "stories" ADD CONSTRAINT "fk_stories_course_id" FOREIGN KEY ("course_id") REFERENCES "public"."courses"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "sponsorships" ADD CONSTRAINT "fk_sponsorships_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "sponsorships" ADD CONSTRAINT "fk_sponsorships_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "sponsorships" ADD CONSTRAINT "fk_sponsorships_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "sponsorships" ADD CONSTRAINT "fk_sponsorships_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "sponsorships" ADD CONSTRAINT "fk_sponsorships_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "sponsorships" ADD CONSTRAINT "fk_sponsorships_sponsor_organization_id" FOREIGN KEY ("sponsor_organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "sponsorships" ADD CONSTRAINT "fk_sponsorships_sponsor_sub_organization_id" FOREIGN KEY ("sponsor_sub_organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "user_progress" ADD CONSTRAINT "fk_user_progress_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "user_progress" ADD CONSTRAINT "fk_user_progress_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "user_progress" ADD CONSTRAINT "fk_user_progress_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "user_progress" ADD CONSTRAINT "fk_user_progress_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "user_progress" ADD CONSTRAINT "fk_user_progress_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "user_progress" ADD CONSTRAINT "fk_user_progress_user_id" FOREIGN KEY ("user_id") REFERENCES "public"."contacts"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "teacher_students" ADD CONSTRAINT "fk_teacher_students_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "teacher_students" ADD CONSTRAINT "fk_teacher_students_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "teacher_students" ADD CONSTRAINT "fk_teacher_students_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "teacher_students" ADD CONSTRAINT "fk_teacher_students_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "teacher_students" ADD CONSTRAINT "fk_teacher_students_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "teacher_students" ADD CONSTRAINT "fk_teacher_students_teacher_id" FOREIGN KEY ("teacher_id") REFERENCES "public"."contacts"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "teacher_students" ADD CONSTRAINT "fk_teacher_students_student_id" FOREIGN KEY ("student_id") REFERENCES "public"."contacts"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "course_completion_reports" ADD CONSTRAINT "fk_course_completion_reports_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "course_completion_reports" ADD CONSTRAINT "fk_course_completion_reports_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "course_completion_reports" ADD CONSTRAINT "fk_course_completion_reports_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "course_completion_reports" ADD CONSTRAINT "fk_course_completion_reports_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "course_completion_reports" ADD CONSTRAINT "fk_course_completion_reports_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "course_completion_reports" ADD CONSTRAINT "fk_course_completion_reports_report_id" FOREIGN KEY ("report_id") REFERENCES "public"."reports"("id") ON DELETE no action ON UPDATE no action;
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
--> statement-breakpoint
ALTER TABLE "organization_contact_user_roles" ADD CONSTRAINT "fk_organization_contact_user_roles_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "organization_contact_user_roles" ADD CONSTRAINT "fk_organization_contact_user_roles_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "organization_contact_user_roles" ADD CONSTRAINT "fk_organization_contact_user_roles_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "organization_contact_user_roles" ADD CONSTRAINT "fk_organization_contact_user_roles_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "organization_contact_user_roles" ADD CONSTRAINT "fk_organization_contact_user_roles_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "organization_contact_user_roles" ADD CONSTRAINT "fk_organization_contact_user_roles_organization_contact_id" FOREIGN KEY ("organization_contact_id") REFERENCES "public"."organization_contacts"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "organization_contact_user_roles" ADD CONSTRAINT "fk_organization_contact_user_roles_user_role_id" FOREIGN KEY ("user_role_id") REFERENCES "public"."user_roles"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "grid_filters" ADD CONSTRAINT "fk_grid_filters_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "grid_filters" ADD CONSTRAINT "fk_grid_filters_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "grid_filters" ADD CONSTRAINT "fk_grid_filters_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "grid_filters" ADD CONSTRAINT "fk_grid_filters_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "grid_filters" ADD CONSTRAINT "fk_grid_filters_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "grid_filters" ADD CONSTRAINT "fk_grid_filters_contact_id" FOREIGN KEY ("contact_id") REFERENCES "public"."contacts"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "grid_filters" ADD CONSTRAINT "fk_grid_filters_account_organization_id" FOREIGN KEY ("account_organization_id") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "student_progress" ADD CONSTRAINT "fk_student_progress_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "student_progress" ADD CONSTRAINT "fk_student_progress_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "student_progress" ADD CONSTRAINT "fk_student_progress_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "student_progress" ADD CONSTRAINT "fk_student_progress_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "student_progress" ADD CONSTRAINT "fk_student_progress_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "student_progress" ADD CONSTRAINT "fk_student_progress_student_id" FOREIGN KEY ("student_id") REFERENCES "public"."contacts"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "student_progress" ADD CONSTRAINT "fk_student_progress_classroom_id" FOREIGN KEY ("classroom_id") REFERENCES "public"."classrooms"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "student_progress" ADD CONSTRAINT "fk_student_progress_course_id" FOREIGN KEY ("course_id") REFERENCES "public"."courses"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "student_progress" ADD CONSTRAINT "fk_student_progress_story_id" FOREIGN KEY ("story_id") REFERENCES "public"."stories"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "conversation_topics" ADD CONSTRAINT "fk_conversation_topics_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "conversation_topics" ADD CONSTRAINT "fk_conversation_topics_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "conversation_topics" ADD CONSTRAINT "fk_conversation_topics_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "conversation_topics" ADD CONSTRAINT "fk_conversation_topics_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "conversation_topics" ADD CONSTRAINT "fk_conversation_topics_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "faq_questions" ADD CONSTRAINT "fk_faq_questions_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "faq_questions" ADD CONSTRAINT "fk_faq_questions_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "faq_questions" ADD CONSTRAINT "fk_faq_questions_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "faq_questions" ADD CONSTRAINT "fk_faq_questions_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "faq_questions" ADD CONSTRAINT "fk_faq_questions_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "faq_questions" ADD CONSTRAINT "fk_faq_questions_faq_id" FOREIGN KEY ("faq_id") REFERENCES "public"."faqs"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "faq_questions" ADD CONSTRAINT "fk_faq_questions_category_id" FOREIGN KEY ("category_id") REFERENCES "public"."faq_categories"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "smtp_contacts" ADD CONSTRAINT "fk_smtp_contacts_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "smtp_contacts" ADD CONSTRAINT "fk_smtp_contacts_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "smtp_contacts" ADD CONSTRAINT "fk_smtp_contacts_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "smtp_contacts" ADD CONSTRAINT "fk_smtp_contacts_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "smtp_contacts" ADD CONSTRAINT "fk_smtp_contacts_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "smtp_contacts" ADD CONSTRAINT "fk_smtp_contacts_smtp_payload_id" FOREIGN KEY ("smtp_payload_id") REFERENCES "public"."smtp_payloads"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "smtp_contacts" ADD CONSTRAINT "fk_smtp_contacts_contact_id" FOREIGN KEY ("contact_id") REFERENCES "public"."contacts"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "game_stats" ADD CONSTRAINT "fk_game_stats_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "game_stats" ADD CONSTRAINT "fk_game_stats_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "game_stats" ADD CONSTRAINT "fk_game_stats_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "game_stats" ADD CONSTRAINT "fk_game_stats_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "game_stats" ADD CONSTRAINT "fk_game_stats_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "game_stats" ADD CONSTRAINT "fk_game_stats_contact_id" FOREIGN KEY ("contact_id") REFERENCES "public"."contacts"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "game_stats" ADD CONSTRAINT "fk_game_stats_course_id" FOREIGN KEY ("course_id") REFERENCES "public"."courses"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "game_stats" ADD CONSTRAINT "fk_game_stats_story_id" FOREIGN KEY ("story_id") REFERENCES "public"."stories"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "game_stats" ADD CONSTRAINT "fk_game_stats_episode_id" FOREIGN KEY ("episode_id") REFERENCES "public"."episodes"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "game_stats" ADD CONSTRAINT "fk_game_stats_classroom_id" FOREIGN KEY ("classroom_id") REFERENCES "public"."classrooms"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "related_contacts" ADD CONSTRAINT "fk_related_contacts_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "related_contacts" ADD CONSTRAINT "fk_related_contacts_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "related_contacts" ADD CONSTRAINT "fk_related_contacts_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "related_contacts" ADD CONSTRAINT "fk_related_contacts_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "related_contacts" ADD CONSTRAINT "fk_related_contacts_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "related_contacts" ADD CONSTRAINT "fk_related_contacts_contact_id" FOREIGN KEY ("contact_id") REFERENCES "public"."contacts"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "related_contacts" ADD CONSTRAINT "fk_related_contacts_student_id" FOREIGN KEY ("student_id") REFERENCES "public"."contacts"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "organization_accounts" ADD CONSTRAINT "fk_organization_accounts_role_id" FOREIGN KEY ("role_id") REFERENCES "public"."user_roles"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "organization_accounts" ADD CONSTRAINT "fk_organization_accounts_account_organization_id" FOREIGN KEY ("account_organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "contacts" ADD CONSTRAINT "fk_contacts_address_id" FOREIGN KEY ("address_id") REFERENCES "public"."addresses"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "contacts" ADD CONSTRAINT "fk_contacts_district_id" FOREIGN KEY ("district_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "contacts" ADD CONSTRAINT "fk_contacts_school_id" FOREIGN KEY ("school_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "contacts" ADD CONSTRAINT "fk_contacts_teacher_id" FOREIGN KEY ("teacher_id") REFERENCES "public"."contacts"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "episode_events" ADD CONSTRAINT "fk_episode_events_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "episode_events" ADD CONSTRAINT "fk_episode_events_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "episode_events" ADD CONSTRAINT "fk_episode_events_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "episode_events" ADD CONSTRAINT "fk_episode_events_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "episode_events" ADD CONSTRAINT "fk_episode_events_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "episode_events" ADD CONSTRAINT "fk_episode_events_user_id" FOREIGN KEY ("user_id") REFERENCES "public"."contacts"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "episode_events" ADD CONSTRAINT "fk_episode_events_course_id" FOREIGN KEY ("course_id") REFERENCES "public"."courses"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "episode_events" ADD CONSTRAINT "fk_episode_events_story_id" FOREIGN KEY ("story_id") REFERENCES "public"."stories"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "episode_events" ADD CONSTRAINT "fk_episode_events_episode_id" FOREIGN KEY ("episode_id") REFERENCES "public"."episodes"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "teacher_schools" ADD CONSTRAINT "fk_teacher_schools_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "teacher_schools" ADD CONSTRAINT "fk_teacher_schools_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "teacher_schools" ADD CONSTRAINT "fk_teacher_schools_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "teacher_schools" ADD CONSTRAINT "fk_teacher_schools_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "teacher_schools" ADD CONSTRAINT "fk_teacher_schools_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "teacher_schools" ADD CONSTRAINT "fk_teacher_schools_school_id" FOREIGN KEY ("school_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "teacher_schools" ADD CONSTRAINT "fk_teacher_schools_teacher_id" FOREIGN KEY ("teacher_id") REFERENCES "public"."contacts"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "teacher_schools" ADD CONSTRAINT "fk_teacher_schools_district_id" FOREIGN KEY ("district_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "teacher_schools" ADD CONSTRAINT "fk_teacher_schools_department_id" FOREIGN KEY ("department_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "transport_provider_credentials" ADD CONSTRAINT "fk_transport_provider_credentials_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "transport_provider_credentials" ADD CONSTRAINT "fk_transport_provider_credentials_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "transport_provider_credentials" ADD CONSTRAINT "fk_transport_provider_credentials_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "transport_provider_credentials" ADD CONSTRAINT "fk_transport_provider_credentials_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "transport_provider_credentials" ADD CONSTRAINT "fk_transport_provider_credentials_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "transport_provider_credentials" ADD CONSTRAINT "fk_transport_provider_credentials_transport_provider_id" FOREIGN KEY ("transport_provider_id") REFERENCES "public"."transport_providers"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "organization_contacts" ADD CONSTRAINT "fk_organization_contacts_contact_organization_id" FOREIGN KEY ("contact_organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "smtp_attachments" ADD CONSTRAINT "fk_smtp_attachments_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "smtp_attachments" ADD CONSTRAINT "fk_smtp_attachments_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "smtp_attachments" ADD CONSTRAINT "fk_smtp_attachments_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "smtp_attachments" ADD CONSTRAINT "fk_smtp_attachments_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "smtp_attachments" ADD CONSTRAINT "fk_smtp_attachments_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "smtp_attachments" ADD CONSTRAINT "fk_smtp_attachments_file_id" FOREIGN KEY ("file_id") REFERENCES "public"."files"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "student_session_logs" ADD CONSTRAINT "fk_student_session_logs_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "student_session_logs" ADD CONSTRAINT "fk_student_session_logs_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "student_session_logs" ADD CONSTRAINT "fk_student_session_logs_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "student_session_logs" ADD CONSTRAINT "fk_student_session_logs_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "student_session_logs" ADD CONSTRAINT "fk_student_session_logs_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "student_session_logs" ADD CONSTRAINT "fk_student_session_logs_user_id" FOREIGN KEY ("user_id") REFERENCES "public"."contacts"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "game_states" ADD CONSTRAINT "fk_game_states_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "game_states" ADD CONSTRAINT "fk_game_states_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "game_states" ADD CONSTRAINT "fk_game_states_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "game_states" ADD CONSTRAINT "fk_game_states_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "game_states" ADD CONSTRAINT "fk_game_states_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "game_choices" ADD CONSTRAINT "fk_game_choices_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "game_choices" ADD CONSTRAINT "fk_game_choices_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "game_choices" ADD CONSTRAINT "fk_game_choices_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "game_choices" ADD CONSTRAINT "fk_game_choices_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "game_choices" ADD CONSTRAINT "fk_game_choices_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "game_choices" ADD CONSTRAINT "fk_game_choices_question_id" FOREIGN KEY ("question_id") REFERENCES "public"."game_questions"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classroom_students" ADD CONSTRAINT "fk_classroom_students_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classroom_students" ADD CONSTRAINT "fk_classroom_students_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classroom_students" ADD CONSTRAINT "fk_classroom_students_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classroom_students" ADD CONSTRAINT "fk_classroom_students_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classroom_students" ADD CONSTRAINT "fk_classroom_students_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classroom_students" ADD CONSTRAINT "fk_classroom_students_classroom_id" FOREIGN KEY ("classroom_id") REFERENCES "public"."classrooms"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classroom_students" ADD CONSTRAINT "fk_classroom_students_student_id" FOREIGN KEY ("student_id") REFERENCES "public"."contacts"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classroom_course_stories" ADD CONSTRAINT "fk_classroom_course_stories_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classroom_course_stories" ADD CONSTRAINT "fk_classroom_course_stories_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classroom_course_stories" ADD CONSTRAINT "fk_classroom_course_stories_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classroom_course_stories" ADD CONSTRAINT "fk_classroom_course_stories_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classroom_course_stories" ADD CONSTRAINT "fk_classroom_course_stories_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classroom_course_stories" ADD CONSTRAINT "fk_classroom_course_stories_classroom_id" FOREIGN KEY ("classroom_id") REFERENCES "public"."classrooms"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classroom_course_stories" ADD CONSTRAINT "fk_classroom_course_stories_course_id" FOREIGN KEY ("course_id") REFERENCES "public"."courses"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classroom_course_stories" ADD CONSTRAINT "fk_classroom_course_stories_story_id" FOREIGN KEY ("story_id") REFERENCES "public"."stories"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "avatar_selections" ADD CONSTRAINT "fk_avatar_selections_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "avatar_selections" ADD CONSTRAINT "fk_avatar_selections_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "avatar_selections" ADD CONSTRAINT "fk_avatar_selections_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "avatar_selections" ADD CONSTRAINT "fk_avatar_selections_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "avatar_selections" ADD CONSTRAINT "fk_avatar_selections_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "avatar_selections" ADD CONSTRAINT "fk_avatar_selections_user_id" FOREIGN KEY ("user_id") REFERENCES "public"."contacts"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "user_settings" ADD CONSTRAINT "fk_user_settings_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "user_settings" ADD CONSTRAINT "fk_user_settings_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "user_settings" ADD CONSTRAINT "fk_user_settings_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "user_settings" ADD CONSTRAINT "fk_user_settings_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "user_settings" ADD CONSTRAINT "fk_user_settings_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "user_settings" ADD CONSTRAINT "fk_user_settings_user_id" FOREIGN KEY ("user_id") REFERENCES "public"."contacts"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "chapter_events" ADD CONSTRAINT "fk_chapter_events_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "chapter_events" ADD CONSTRAINT "fk_chapter_events_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "chapter_events" ADD CONSTRAINT "fk_chapter_events_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "chapter_events" ADD CONSTRAINT "fk_chapter_events_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "chapter_events" ADD CONSTRAINT "fk_chapter_events_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "chapter_events" ADD CONSTRAINT "fk_chapter_events_user_id" FOREIGN KEY ("user_id") REFERENCES "public"."contacts"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "chapter_events" ADD CONSTRAINT "fk_chapter_events_course_id" FOREIGN KEY ("course_id") REFERENCES "public"."courses"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "chapter_events" ADD CONSTRAINT "fk_chapter_events_story_id" FOREIGN KEY ("story_id") REFERENCES "public"."stories"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "chapter_events" ADD CONSTRAINT "fk_chapter_events_episode_id" FOREIGN KEY ("episode_id") REFERENCES "public"."episodes"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classroom_course_episodes" ADD CONSTRAINT "fk_classroom_course_episodes_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classroom_course_episodes" ADD CONSTRAINT "fk_classroom_course_episodes_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classroom_course_episodes" ADD CONSTRAINT "fk_classroom_course_episodes_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classroom_course_episodes" ADD CONSTRAINT "fk_classroom_course_episodes_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classroom_course_episodes" ADD CONSTRAINT "fk_classroom_course_episodes_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classroom_course_episodes" ADD CONSTRAINT "fk_classroom_course_episodes_classroom_id" FOREIGN KEY ("classroom_id") REFERENCES "public"."classrooms"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classroom_course_episodes" ADD CONSTRAINT "fk_classroom_course_episodes_course_id" FOREIGN KEY ("course_id") REFERENCES "public"."courses"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classroom_course_episodes" ADD CONSTRAINT "fk_classroom_course_episodes_story_id" FOREIGN KEY ("story_id") REFERENCES "public"."stories"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classroom_course_episodes" ADD CONSTRAINT "fk_classroom_course_episodes_episode_id" FOREIGN KEY ("episode_id") REFERENCES "public"."episodes"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "faq_categories" ADD CONSTRAINT "fk_faq_categories_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "faq_categories" ADD CONSTRAINT "fk_faq_categories_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "faq_categories" ADD CONSTRAINT "fk_faq_categories_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "faq_categories" ADD CONSTRAINT "fk_faq_categories_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "faq_categories" ADD CONSTRAINT "fk_faq_categories_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "faq_categories" ADD CONSTRAINT "fk_faq_categories_faq_id" FOREIGN KEY ("faq_id") REFERENCES "public"."faqs"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "locations" ADD CONSTRAINT "fk_locations_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "locations" ADD CONSTRAINT "fk_locations_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "locations" ADD CONSTRAINT "fk_locations_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "locations" ADD CONSTRAINT "fk_locations_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "locations" ADD CONSTRAINT "fk_locations_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "locations" ADD CONSTRAINT "fk_locations_address_id" FOREIGN KEY ("address_id") REFERENCES "public"."addresses"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "conversations" ADD CONSTRAINT "fk_conversations_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "conversations" ADD CONSTRAINT "fk_conversations_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "conversations" ADD CONSTRAINT "fk_conversations_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "conversations" ADD CONSTRAINT "fk_conversations_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "conversations" ADD CONSTRAINT "fk_conversations_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "conversations" ADD CONSTRAINT "fk_conversations_contact_id" FOREIGN KEY ("contact_id") REFERENCES "public"."contacts"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "conversations" ADD CONSTRAINT "fk_conversations_conversation_topic_id" FOREIGN KEY ("conversation_topic_id") REFERENCES "public"."conversation_topics"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "common_session_logs" ADD CONSTRAINT "fk_common_session_logs_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "common_session_logs" ADD CONSTRAINT "fk_common_session_logs_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "common_session_logs" ADD CONSTRAINT "fk_common_session_logs_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "common_session_logs" ADD CONSTRAINT "fk_common_session_logs_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "common_session_logs" ADD CONSTRAINT "fk_common_session_logs_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "common_session_logs" ADD CONSTRAINT "fk_common_session_logs_user_id" FOREIGN KEY ("user_id") REFERENCES "public"."contacts"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "game_questions" ADD CONSTRAINT "fk_game_questions_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "game_questions" ADD CONSTRAINT "fk_game_questions_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "game_questions" ADD CONSTRAINT "fk_game_questions_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "game_questions" ADD CONSTRAINT "fk_game_questions_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "game_questions" ADD CONSTRAINT "fk_game_questions_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "game_questions" ADD CONSTRAINT "fk_game_questions_course_id" FOREIGN KEY ("course_id") REFERENCES "public"."courses"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "game_questions" ADD CONSTRAINT "fk_game_questions_story_id" FOREIGN KEY ("story_id") REFERENCES "public"."stories"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "communication_templates" ADD CONSTRAINT "fk_communication_templates_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "communication_templates" ADD CONSTRAINT "fk_communication_templates_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "communication_templates" ADD CONSTRAINT "fk_communication_templates_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "communication_templates" ADD CONSTRAINT "fk_communication_templates_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "communication_templates" ADD CONSTRAINT "fk_communication_templates_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classroom_courses" ADD CONSTRAINT "fk_classroom_courses_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classroom_courses" ADD CONSTRAINT "fk_classroom_courses_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classroom_courses" ADD CONSTRAINT "fk_classroom_courses_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classroom_courses" ADD CONSTRAINT "fk_classroom_courses_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classroom_courses" ADD CONSTRAINT "fk_classroom_courses_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classroom_courses" ADD CONSTRAINT "fk_classroom_courses_classroom_id" FOREIGN KEY ("classroom_id") REFERENCES "public"."classrooms"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classroom_courses" ADD CONSTRAINT "fk_classroom_courses_course_id" FOREIGN KEY ("course_id") REFERENCES "public"."courses"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "student_stats" ADD CONSTRAINT "fk_student_stats_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "student_stats" ADD CONSTRAINT "fk_student_stats_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "student_stats" ADD CONSTRAINT "fk_student_stats_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "student_stats" ADD CONSTRAINT "fk_student_stats_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "student_stats" ADD CONSTRAINT "fk_student_stats_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "student_stats" ADD CONSTRAINT "fk_student_stats_student_id" FOREIGN KEY ("student_id") REFERENCES "public"."contacts"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "student_stats" ADD CONSTRAINT "fk_student_stats_classroom_id" FOREIGN KEY ("classroom_id") REFERENCES "public"."classrooms"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "student_stats" ADD CONSTRAINT "fk_student_stats_course_id" FOREIGN KEY ("course_id") REFERENCES "public"."courses"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "student_stats" ADD CONSTRAINT "fk_student_stats_story_id" FOREIGN KEY ("story_id") REFERENCES "public"."stories"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "common_sessions" ADD CONSTRAINT "fk_common_sessions_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "common_sessions" ADD CONSTRAINT "fk_common_sessions_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "common_sessions" ADD CONSTRAINT "fk_common_sessions_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "common_sessions" ADD CONSTRAINT "fk_common_sessions_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "common_sessions" ADD CONSTRAINT "fk_common_sessions_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "common_sessions" ADD CONSTRAINT "fk_common_sessions_user_id" FOREIGN KEY ("user_id") REFERENCES "public"."contacts"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "episodes" ADD CONSTRAINT "fk_episodes_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "episodes" ADD CONSTRAINT "fk_episodes_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "episodes" ADD CONSTRAINT "fk_episodes_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "episodes" ADD CONSTRAINT "fk_episodes_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "episodes" ADD CONSTRAINT "fk_episodes_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "episodes" ADD CONSTRAINT "fk_episodes_story_id" FOREIGN KEY ("story_id") REFERENCES "public"."stories"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "episodes" ADD CONSTRAINT "fk_episodes_course_id" FOREIGN KEY ("course_id") REFERENCES "public"."courses"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "smtp_attachment_headers" ADD CONSTRAINT "fk_smtp_attachment_headers_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "smtp_attachment_headers" ADD CONSTRAINT "fk_smtp_attachment_headers_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "smtp_attachment_headers" ADD CONSTRAINT "fk_smtp_attachment_headers_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "smtp_attachment_headers" ADD CONSTRAINT "fk_smtp_attachment_headers_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "smtp_attachment_headers" ADD CONSTRAINT "fk_smtp_attachment_headers_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "smtp_attachment_headers" ADD CONSTRAINT "fk_smtp_attachment_headers_smtp_attachment_id" FOREIGN KEY ("smtp_attachment_id") REFERENCES "public"."smtp_attachments"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "courses" ADD CONSTRAINT "fk_courses_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "courses" ADD CONSTRAINT "fk_courses_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "courses" ADD CONSTRAINT "fk_courses_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "courses" ADD CONSTRAINT "fk_courses_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "courses" ADD CONSTRAINT "fk_courses_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "smtp_payloads" ADD CONSTRAINT "fk_smtp_payloads_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "smtp_payloads" ADD CONSTRAINT "fk_smtp_payloads_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "smtp_payloads" ADD CONSTRAINT "fk_smtp_payloads_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "smtp_payloads" ADD CONSTRAINT "fk_smtp_payloads_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "smtp_payloads" ADD CONSTRAINT "fk_smtp_payloads_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "smtp_payloads" ADD CONSTRAINT "fk_smtp_payloads_transport_provider_id" FOREIGN KEY ("transport_provider_id") REFERENCES "public"."transport_providers"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "smtp_response_links" ADD CONSTRAINT "fk_smtp_response_links_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "smtp_response_links" ADD CONSTRAINT "fk_smtp_response_links_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "smtp_response_links" ADD CONSTRAINT "fk_smtp_response_links_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "smtp_response_links" ADD CONSTRAINT "fk_smtp_response_links_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "smtp_response_links" ADD CONSTRAINT "fk_smtp_response_links_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "smtp_response_links" ADD CONSTRAINT "fk_smtp_response_links_smtp_payload_id" FOREIGN KEY ("smtp_payload_id") REFERENCES "public"."smtp_payloads"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "smtp_transactions" ADD CONSTRAINT "fk_smtp_transactions_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "smtp_transactions" ADD CONSTRAINT "fk_smtp_transactions_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "smtp_transactions" ADD CONSTRAINT "fk_smtp_transactions_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "smtp_transactions" ADD CONSTRAINT "fk_smtp_transactions_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "smtp_transactions" ADD CONSTRAINT "fk_smtp_transactions_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "smtp_transactions" ADD CONSTRAINT "fk_smtp_transactions_smtp_payload_id" FOREIGN KEY ("smtp_payload_id") REFERENCES "public"."smtp_payloads"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "report_files" ADD CONSTRAINT "fk_report_files_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "report_files" ADD CONSTRAINT "fk_report_files_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "report_files" ADD CONSTRAINT "fk_report_files_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "report_files" ADD CONSTRAINT "fk_report_files_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "report_files" ADD CONSTRAINT "fk_report_files_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "invitations" ADD CONSTRAINT "fk_invitations_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "invitations" ADD CONSTRAINT "fk_invitations_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "invitations" ADD CONSTRAINT "fk_invitations_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "invitations" ADD CONSTRAINT "fk_invitations_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "invitations" ADD CONSTRAINT "fk_invitations_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "invitations" ADD CONSTRAINT "fk_invitations_account_id" FOREIGN KEY ("account_id") REFERENCES "public"."accounts"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "invitations" ADD CONSTRAINT "fk_invitations_account_organization_id" FOREIGN KEY ("account_organization_id") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "invitations" ADD CONSTRAINT "fk_invitations_related_contact_id" FOREIGN KEY ("related_contact_id") REFERENCES "public"."related_contacts"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "faq_answers" ADD CONSTRAINT "fk_faq_answers_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "faq_answers" ADD CONSTRAINT "fk_faq_answers_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "faq_answers" ADD CONSTRAINT "fk_faq_answers_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "faq_answers" ADD CONSTRAINT "fk_faq_answers_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "faq_answers" ADD CONSTRAINT "fk_faq_answers_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "faq_answers" ADD CONSTRAINT "fk_faq_answers_faq_id" FOREIGN KEY ("faq_id") REFERENCES "public"."faqs"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "faq_answers" ADD CONSTRAINT "fk_faq_answers_category_id" FOREIGN KEY ("category_id") REFERENCES "public"."faq_categories"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "faq_answers" ADD CONSTRAINT "fk_faq_answers_question_id" FOREIGN KEY ("question_id") REFERENCES "public"."faq_questions"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classrooms" ADD CONSTRAINT "fk_classrooms_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classrooms" ADD CONSTRAINT "fk_classrooms_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classrooms" ADD CONSTRAINT "fk_classrooms_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classrooms" ADD CONSTRAINT "fk_classrooms_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classrooms" ADD CONSTRAINT "fk_classrooms_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classrooms" ADD CONSTRAINT "fk_classrooms_department_id" FOREIGN KEY ("department_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classrooms" ADD CONSTRAINT "fk_classrooms_district_id" FOREIGN KEY ("district_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classrooms" ADD CONSTRAINT "fk_classrooms_school_id" FOREIGN KEY ("school_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classrooms" ADD CONSTRAINT "fk_classrooms_teacher_id" FOREIGN KEY ("teacher_id") REFERENCES "public"."contacts"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "notifications" ADD CONSTRAINT "fk_notifications_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "notifications" ADD CONSTRAINT "fk_notifications_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "notifications" ADD CONSTRAINT "fk_notifications_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "notifications" ADD CONSTRAINT "fk_notifications_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "notifications" ADD CONSTRAINT "fk_notifications_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "notifications" ADD CONSTRAINT "fk_notifications_recipient_id" FOREIGN KEY ("recipient_id") REFERENCES "public"."contacts"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classroom_stats" ADD CONSTRAINT "fk_classroom_stats_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classroom_stats" ADD CONSTRAINT "fk_classroom_stats_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classroom_stats" ADD CONSTRAINT "fk_classroom_stats_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classroom_stats" ADD CONSTRAINT "fk_classroom_stats_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classroom_stats" ADD CONSTRAINT "fk_classroom_stats_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classroom_stats" ADD CONSTRAINT "fk_classroom_stats_classroom_id" FOREIGN KEY ("classroom_id") REFERENCES "public"."classrooms"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classroom_stats" ADD CONSTRAINT "fk_classroom_stats_story_id" FOREIGN KEY ("story_id") REFERENCES "public"."stories"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classroom_stats" ADD CONSTRAINT "fk_classroom_stats_student_id" FOREIGN KEY ("student_id") REFERENCES "public"."contacts"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "classroom_stats" ADD CONSTRAINT "fk_classroom_stats_course_id" FOREIGN KEY ("course_id") REFERENCES "public"."courses"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "onboarding_contacts" ADD CONSTRAINT "fk_onboarding_contacts_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "onboarding_contacts" ADD CONSTRAINT "fk_onboarding_contacts_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "onboarding_contacts" ADD CONSTRAINT "fk_onboarding_contacts_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "onboarding_contacts" ADD CONSTRAINT "fk_onboarding_contacts_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "onboarding_contacts" ADD CONSTRAINT "fk_onboarding_contacts_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "onboarding_contacts" ADD CONSTRAINT "fk_onboarding_contacts_contact_id" FOREIGN KEY ("contact_id") REFERENCES "public"."contacts"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "conversation_messages" ADD CONSTRAINT "fk_conversation_messages_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "conversation_messages" ADD CONSTRAINT "fk_conversation_messages_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "conversation_messages" ADD CONSTRAINT "fk_conversation_messages_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "conversation_messages" ADD CONSTRAINT "fk_conversation_messages_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "conversation_messages" ADD CONSTRAINT "fk_conversation_messages_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "conversation_messages" ADD CONSTRAINT "fk_conversation_messages_conversation_id" FOREIGN KEY ("conversation_id") REFERENCES "public"."conversations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "incident_reports" ADD CONSTRAINT "fk_incident_reports_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "incident_reports" ADD CONSTRAINT "fk_incident_reports_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "incident_reports" ADD CONSTRAINT "fk_incident_reports_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "incident_reports" ADD CONSTRAINT "fk_incident_reports_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "incident_reports" ADD CONSTRAINT "fk_incident_reports_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "incident_reports" ADD CONSTRAINT "fk_incident_reports_conversation_id" FOREIGN KEY ("conversation_id") REFERENCES "public"."conversations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "reports" ADD CONSTRAINT "fk_reports_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "reports" ADD CONSTRAINT "fk_reports_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "reports" ADD CONSTRAINT "fk_reports_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "reports" ADD CONSTRAINT "fk_reports_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "reports" ADD CONSTRAINT "fk_reports_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "reports" ADD CONSTRAINT "fk_reports_department_id" FOREIGN KEY ("department_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "reports" ADD CONSTRAINT "fk_reports_district_id" FOREIGN KEY ("district_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "reports" ADD CONSTRAINT "fk_reports_school_id" FOREIGN KEY ("school_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "reports" ADD CONSTRAINT "fk_reports_course_id" FOREIGN KEY ("course_id") REFERENCES "public"."courses"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "reports" ADD CONSTRAINT "fk_reports_story_id" FOREIGN KEY ("story_id") REFERENCES "public"."stories"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "game_responses" ADD CONSTRAINT "fk_game_responses_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "game_responses" ADD CONSTRAINT "fk_game_responses_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "game_responses" ADD CONSTRAINT "fk_game_responses_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "game_responses" ADD CONSTRAINT "fk_game_responses_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "game_responses" ADD CONSTRAINT "fk_game_responses_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "game_responses" ADD CONSTRAINT "fk_game_responses_user_id" FOREIGN KEY ("user_id") REFERENCES "public"."contacts"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "game_responses" ADD CONSTRAINT "fk_game_responses_course_id" FOREIGN KEY ("course_id") REFERENCES "public"."courses"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "game_responses" ADD CONSTRAINT "fk_game_responses_story_id" FOREIGN KEY ("story_id") REFERENCES "public"."stories"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "game_responses" ADD CONSTRAINT "fk_game_responses_episode_id" FOREIGN KEY ("episode_id") REFERENCES "public"."episodes"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "emotion_predictions" ADD CONSTRAINT "fk_emotion_predictions_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "emotion_predictions" ADD CONSTRAINT "fk_emotion_predictions_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "emotion_predictions" ADD CONSTRAINT "fk_emotion_predictions_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "emotion_predictions" ADD CONSTRAINT "fk_emotion_predictions_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "emotion_predictions" ADD CONSTRAINT "fk_emotion_predictions_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "emotion_predictions" ADD CONSTRAINT "fk_emotion_predictions_conversation_id" FOREIGN KEY ("conversation_id") REFERENCES "public"."conversations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "game_review_question_reports" ADD CONSTRAINT "fk_game_review_question_reports_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "game_review_question_reports" ADD CONSTRAINT "fk_game_review_question_reports_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "game_review_question_reports" ADD CONSTRAINT "fk_game_review_question_reports_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "game_review_question_reports" ADD CONSTRAINT "fk_game_review_question_reports_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "game_review_question_reports" ADD CONSTRAINT "fk_game_review_question_reports_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "game_review_question_reports" ADD CONSTRAINT "fk_game_review_question_reports_course_id" FOREIGN KEY ("course_id") REFERENCES "public"."courses"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "game_review_question_reports" ADD CONSTRAINT "fk_game_review_question_reports_story_id" FOREIGN KEY ("story_id") REFERENCES "public"."stories"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "game_review_question_reports" ADD CONSTRAINT "fk_game_review_question_reports_report_id" FOREIGN KEY ("report_id") REFERENCES "public"."reports"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "student_sessions" ADD CONSTRAINT "fk_student_sessions_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "student_sessions" ADD CONSTRAINT "fk_student_sessions_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "student_sessions" ADD CONSTRAINT "fk_student_sessions_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "student_sessions" ADD CONSTRAINT "fk_student_sessions_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "student_sessions" ADD CONSTRAINT "fk_student_sessions_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "student_sessions" ADD CONSTRAINT "fk_student_sessions_user_id" FOREIGN KEY ("user_id") REFERENCES "public"."contacts"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "transport_providers" ADD CONSTRAINT "fk_transport_providers_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "transport_providers" ADD CONSTRAINT "fk_transport_providers_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "transport_providers" ADD CONSTRAINT "fk_transport_providers_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "transport_providers" ADD CONSTRAINT "fk_transport_providers_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "transport_providers" ADD CONSTRAINT "fk_transport_providers_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "faqs" ADD CONSTRAINT "fk_faqs_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "faqs" ADD CONSTRAINT "fk_faqs_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "faqs" ADD CONSTRAINT "fk_faqs_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "faqs" ADD CONSTRAINT "fk_faqs_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "faqs" ADD CONSTRAINT "fk_faqs_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "report_child_victims" ADD CONSTRAINT "fk_report_child_victims_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "report_child_victims" ADD CONSTRAINT "fk_report_child_victims_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "report_child_victims" ADD CONSTRAINT "fk_report_child_victims_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "report_child_victims" ADD CONSTRAINT "fk_report_child_victims_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "report_child_victims" ADD CONSTRAINT "fk_report_child_victims_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
