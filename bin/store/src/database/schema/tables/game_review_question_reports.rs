use crate::schema::generator::diesel_schema_definition::{
    DieselTableDefinition, types::*
};
use crate::define_table_schema;

pub struct GameReviewQuestionReportsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        system_fields!(),
        course_id: nullable(text()),
        course_title: nullable(text()), default: "''",
        story_id: nullable(text()),
        story_title: nullable(text()), default: "''",
        episode_number: nullable(integer()), default: "0",
        question: nullable(text()), default: "''",
        students: nullable(integer()), default: "0",
        strongly_disagree: nullable(integer()), default: "0",
        disagree: nullable(integer()), default: "0",
        agree: nullable(integer()), default: "0",
        strongly_agree: nullable(integer()), default: "0",
        report_id: nullable(text()),
    },
    indexes: {
        system_indexes!("game_review_question_reports"),
        idx_game_review_question_reports_course_id: { columns: ["course_id"], unique: false, type: "btree" },
        idx_game_review_question_reports_course_title: { columns: ["course_title"], unique: false, type: "btree" },
        idx_game_review_question_reports_story_id: { columns: ["story_id"], unique: false, type: "btree" },
        idx_game_review_question_reports_story_title: { columns: ["story_title"], unique: false, type: "btree" },
        idx_game_review_question_reports_episode_number: { columns: ["episode_number"], unique: false, type: "btree" },
        idx_game_review_question_reports_question: { columns: ["question"], unique: false, type: "btree" },
        idx_game_review_question_reports_students: { columns: ["students"], unique: false, type: "btree" },
        idx_game_review_question_reports_strongly_disagree: { columns: ["strongly_disagree"], unique: false, type: "btree" },
        idx_game_review_question_reports_disagree: { columns: ["disagree"], unique: false, type: "btree" },
        idx_game_review_question_reports_agree: { columns: ["agree"], unique: false, type: "btree" },
        idx_game_review_question_reports_strongly_agree: { columns: ["strongly_agree"], unique: false, type: "btree" },
        idx_game_review_question_reports_report_id: { columns: ["report_id"], unique: false, type: "btree" },
    },
    foreign_keys: {
        system_foreign_keys!("game_review_question_reports"),
        fk_game_review_question_reports_course_id: { columns: ["course_id"], foreign_table: "courses", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
        fk_game_review_question_reports_story_id: { columns: ["story_id"], foreign_table: "stories", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
        fk_game_review_question_reports_report_id: { columns: ["report_id"], foreign_table: "reports", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}