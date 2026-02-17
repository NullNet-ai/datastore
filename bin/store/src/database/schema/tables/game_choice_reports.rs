use crate::define_table_schema;
use crate::schema::generator::diesel_schema_definition::{types::*, DieselTableDefinition};

pub struct GameChoiceReportsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        system_fields!(),
        course_id: nullable(text()),
        course_title: nullable(text()), default: "''",
        story_id: nullable(text()),
        story_title: nullable(text()), default: "''",
        episode_number: nullable(integer()), default: "0",
        topic: nullable(text()), default: "''",
        question: nullable(text()), default: "''",
        students: nullable(integer()), default: "0",
        correct_choice: nullable(text()), default: "''",
        avg_tries: nullable(integer()), default: "0",
        report_id: nullable(text()),
    },
    indexes: {
        system_indexes!("game_choice_reports"),
        idx_game_choice_reports_course_id: { columns: ["course_id"], unique: false, type: "btree" },
        idx_game_choice_reports_course_title: { columns: ["course_title"], unique: false, type: "btree" },
        idx_game_choice_reports_story_id: { columns: ["story_id"], unique: false, type: "btree" },
        idx_game_choice_reports_story_title: { columns: ["story_title"], unique: false, type: "btree" },
        idx_game_choice_reports_episode_number: { columns: ["episode_number"], unique: false, type: "btree" },
        idx_game_choice_reports_topic: { columns: ["topic"], unique: false, type: "btree" },
        idx_game_choice_reports_question: { columns: ["question"], unique: false, type: "btree" },
        idx_game_choice_reports_students: { columns: ["students"], unique: false, type: "btree" },
        idx_game_choice_reports_correct_choice: { columns: ["correct_choice"], unique: false, type: "btree" },
        idx_game_choice_reports_avg_tries: { columns: ["avg_tries"], unique: false, type: "btree" },
        idx_game_choice_reports_report_id: { columns: ["report_id"], unique: false, type: "btree" },
    },
    foreign_keys: {
        system_foreign_keys!("game_choice_reports"),
        fk_game_choice_reports_course_id: { columns: ["course_id"], foreign_table: "courses", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
        fk_game_choice_reports_story_id: { columns: ["story_id"], foreign_table: "stories", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
        fk_game_choice_reports_report_id: { columns: ["report_id"], foreign_table: "reports", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}
