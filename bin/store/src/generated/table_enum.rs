use crate::{generate_get_by_id_match, generate_hypertable_timestamp_match, generate_insert_record_match, generate_upsert_record_match, generate_upsert_record_with_timestamp_match};
use crate::generated::models::user_role_model::UserRoleModel;
use crate::generated::models::session_model::SessionModel;
use crate::generated::models::signed_in_activity_model::SignedInActivityModel;
use crate::generated::models::external_contact_model::ExternalContactModel;
use crate::generated::models::organization_model::OrganizationModel;
use crate::generated::models::organization_contact_model::OrganizationContactModel;
use crate::generated::models::organization_account_model::OrganizationAccountModel;
use crate::generated::models::account_organization_model::AccountOrganizationModel;
use crate::generated::models::account_profile_model::AccountProfileModel;
use crate::generated::models::account_model::AccountModel;
use crate::generated::models::address_model::AddressModel;
use crate::generated::models::sample_model::SampleModel;
use crate::generated::models::device_model::DeviceModel;
use crate::generated::models::postgres_channel_model::PostgresChannelModel;
use crate::generated::models::contact_model::ContactModel;
use crate::generated::models::contact_phone_number_model::ContactPhoneNumberModel;
use crate::generated::models::contact_email_model::ContactEmailModel;
use crate::generated::models::file_model::FileModel;
use crate::generated::models::test_hypertable_model::TestHypertableModel;
use crate::generated::models::account_phone_number_model::AccountPhoneNumberModel;
use crate::generated::models::account_signature_model::AccountSignatureModel;
use crate::generated::models::game_choice_report_model::GameChoiceReportModel;
use crate::generated::models::story_model::StoryModel;
use crate::generated::models::sponsorship_model::SponsorshipModel;
use crate::generated::models::user_progress_model::UserProgressModel;
use crate::generated::models::teacher_student_model::TeacherStudentModel;
use crate::generated::models::course_completion_report_model::CourseCompletionReportModel;
use crate::generated::models::classroom_course_stories_episode_model::ClassroomCourseStoriesEpisodeModel;
use crate::generated::models::organization_contact_user_role_model::OrganizationContactUserRoleModel;
use crate::generated::models::grid_filter_model::GridFilterModel;
use crate::generated::models::student_progress_model::StudentProgressModel;
use crate::generated::models::conversation_topic_model::ConversationTopicModel;
use crate::generated::models::faq_question_model::FaqQuestionModel;
use crate::generated::models::smtp_contact_model::SmtpContactModel;
use crate::generated::models::game_stat_model::GameStatModel;
use crate::generated::models::related_contact_model::RelatedContactModel;
use crate::generated::models::episode_event_model::EpisodeEventModel;
use crate::generated::models::teacher_school_model::TeacherSchoolModel;
use crate::generated::models::transport_provider_credential_model::TransportProviderCredentialModel;
use crate::generated::models::smtp_attachment_model::SmtpAttachmentModel;
use crate::generated::models::student_session_log_model::StudentSessionLogModel;
use crate::generated::models::game_state_model::GameStateModel;
use crate::generated::models::game_choice_model::GameChoiceModel;
use crate::generated::models::classroom_student_model::ClassroomStudentModel;
use crate::generated::models::classroom_course_story_model::ClassroomCourseStoryModel;
use crate::generated::models::avatar_selection_model::AvatarSelectionModel;
use crate::generated::models::user_setting_model::UserSettingModel;
use crate::generated::models::chapter_event_model::ChapterEventModel;
use crate::generated::models::classroom_course_episode_model::ClassroomCourseEpisodeModel;
use crate::generated::models::faq_category_model::FaqCategoryModel;
use crate::generated::models::location_model::LocationModel;
use crate::generated::models::conversation_model::ConversationModel;
use crate::generated::models::common_session_log_model::CommonSessionLogModel;
use crate::generated::models::game_question_model::GameQuestionModel;
use crate::generated::models::communication_template_model::CommunicationTemplateModel;
use crate::generated::models::classroom_coursis_model::ClassroomCoursisModel;
use crate::generated::models::student_stat_model::StudentStatModel;
use crate::generated::models::common_session_model::CommonSessionModel;
use crate::generated::models::episode_model::EpisodeModel;
use crate::generated::models::smtp_attachment_header_model::SmtpAttachmentHeaderModel;
use crate::generated::models::coursis_model::CoursisModel;
use crate::generated::models::smtp_payload_model::SmtpPayloadModel;
use crate::generated::models::smtp_response_link_model::SmtpResponseLinkModel;
use crate::generated::models::smtp_transaction_model::SmtpTransactionModel;
use crate::generated::models::report_file_model::ReportFileModel;
use crate::generated::models::invitation_model::InvitationModel;
use crate::generated::models::faq_answer_model::FaqAnswerModel;
use crate::generated::models::classroom_model::ClassroomModel;
use crate::generated::models::notification_model::NotificationModel;
use crate::generated::models::classroom_stat_model::ClassroomStatModel;
use crate::generated::models::onboarding_contact_model::OnboardingContactModel;
use crate::generated::models::conversation_message_model::ConversationMessageModel;
use crate::generated::models::incident_report_model::IncidentReportModel;
use crate::generated::models::report_model::ReportModel;
use crate::generated::models::game_responsis_model::GameResponsisModel;
use crate::generated::models::emotion_prediction_model::EmotionPredictionModel;
use crate::generated::models::game_review_question_report_model::GameReviewQuestionReportModel;
use crate::generated::models::student_session_model::StudentSessionModel;
use crate::generated::models::transport_provider_model::TransportProviderModel;
use crate::generated::models::faq_model::FaqModel;
use crate::generated::models::report_child_victim_model::ReportChildVictimModel;
use crate::generated::schema;
use crate::structs::core::{Auth, RequestBody};
use actix_web::web;
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel_async::AsyncPgConnection;
use diesel_async::RunQueryDsl;
use serde_json::{Map, Value};
use crate::database::db;
use crate::generated::models::counter_model::CounterModel;

#[derive(Debug)]
pub enum Table {
    UserRoles,
    Sessions,
    SignedInActivities,
    ExternalContacts,
    Organizations,
    OrganizationContacts,
    OrganizationAccounts,
    AccountOrganizations,
    AccountProfiles,
    Accounts,
    Addresses,
    Samples,
    Devices,
    PostgresChannels,
    Contacts,
    ContactPhoneNumbers,
    ContactEmails,
    Files,
    TestHypertables,
    AccountPhoneNumbers,
    AccountSignatures,
    GameChoiceReports,
    Stories,
    Sponsorships,
    UserProgress,
    TeacherStudents,
    CourseCompletionReports,
    ClassroomCourseStoriesEpisodes,
    OrganizationContactUserRoles,
    GridFilters,
    StudentProgress,
    ConversationTopics,
    FaqQuestions,
    SmtpContacts,
    GameStats,
    RelatedContacts,
    EpisodeEvents,
    TeacherSchools,
    TransportProviderCredentials,
    SmtpAttachments,
    StudentSessionLogs,
    GameStates,
    GameChoices,
    ClassroomStudents,
    ClassroomCourseStories,
    AvatarSelections,
    UserSettings,
    ChapterEvents,
    ClassroomCourseEpisodes,
    FaqCategories,
    Locations,
    Conversations,
    CommonSessionLogs,
    GameQuestions,
    CommunicationTemplates,
    ClassroomCourses,
    StudentStats,
    CommonSessions,
    Episodes,
    SmtpAttachmentHeaders,
    Courses,
    SmtpPayloads,
    SmtpResponseLinks,
    SmtpTransactions,
    ReportFiles,
    Invitations,
    FaqAnswers,
    Classrooms,
    Notifications,
    ClassroomStats,
    OnboardingContacts,
    ConversationMessages,
    IncidentReports,
    Reports,
    GameResponses,
    EmotionPredictions,
    GameReviewQuestionReports,
    StudentSessions,
    TransportProviders,
    Faqs,
    ReportChildVictims,
    // Add other tables here
}

impl Table {
    pub fn from_str(name: &str) -> Option<Self> {
        match name {
            "user_roles" => Some(Table::UserRoles),
            "sessions" => Some(Table::Sessions),
            "signed_in_activities" => Some(Table::SignedInActivities),
            "external_contacts" => Some(Table::ExternalContacts),
            "organizations" => Some(Table::Organizations),
            "organization_contacts" => Some(Table::OrganizationContacts),
            "organization_accounts" => Some(Table::OrganizationAccounts),
            "account_organizations" => Some(Table::AccountOrganizations),
            "account_profiles" => Some(Table::AccountProfiles),
            "accounts" => Some(Table::Accounts),
            "addresses" => Some(Table::Addresses),
            "samples" => Some(Table::Samples),
            "devices" => Some(Table::Devices),
            "postgres_channels" => Some(Table::PostgresChannels),
            "contacts" => Some(Table::Contacts),
            "contact_phone_numbers" => Some(Table::ContactPhoneNumbers),
            "contact_emails" => Some(Table::ContactEmails),
            "files" => Some(Table::Files),
            "test_hypertables" => Some(Table::TestHypertables),
            "account_phone_numbers" => Some(Table::AccountPhoneNumbers),
            "account_signatures" => Some(Table::AccountSignatures),
            "game_choice_reports" => Some(Table::GameChoiceReports),
            "stories" => Some(Table::Stories),
            "sponsorships" => Some(Table::Sponsorships),
            "user_progress" => Some(Table::UserProgress),
            "teacher_students" => Some(Table::TeacherStudents),
            "course_completion_reports" => Some(Table::CourseCompletionReports),
            "classroom_course_stories_episodes" => Some(Table::ClassroomCourseStoriesEpisodes),
            "organization_contact_user_roles" => Some(Table::OrganizationContactUserRoles),
            "grid_filters" => Some(Table::GridFilters),
            "student_progress" => Some(Table::StudentProgress),
            "conversation_topics" => Some(Table::ConversationTopics),
            "faq_questions" => Some(Table::FaqQuestions),
            "smtp_contacts" => Some(Table::SmtpContacts),
            "game_stats" => Some(Table::GameStats),
            "related_contacts" => Some(Table::RelatedContacts),
            "episode_events" => Some(Table::EpisodeEvents),
            "teacher_schools" => Some(Table::TeacherSchools),
            "transport_provider_credentials" => Some(Table::TransportProviderCredentials),
            "smtp_attachments" => Some(Table::SmtpAttachments),
            "student_session_logs" => Some(Table::StudentSessionLogs),
            "game_states" => Some(Table::GameStates),
            "game_choices" => Some(Table::GameChoices),
            "classroom_students" => Some(Table::ClassroomStudents),
            "classroom_course_stories" => Some(Table::ClassroomCourseStories),
            "avatar_selections" => Some(Table::AvatarSelections),
            "user_settings" => Some(Table::UserSettings),
            "chapter_events" => Some(Table::ChapterEvents),
            "classroom_course_episodes" => Some(Table::ClassroomCourseEpisodes),
            "faq_categories" => Some(Table::FaqCategories),
            "locations" => Some(Table::Locations),
            "conversations" => Some(Table::Conversations),
            "common_session_logs" => Some(Table::CommonSessionLogs),
            "game_questions" => Some(Table::GameQuestions),
            "communication_templates" => Some(Table::CommunicationTemplates),
            "classroom_courses" => Some(Table::ClassroomCourses),
            "student_stats" => Some(Table::StudentStats),
            "common_sessions" => Some(Table::CommonSessions),
            "episodes" => Some(Table::Episodes),
            "smtp_attachment_headers" => Some(Table::SmtpAttachmentHeaders),
            "courses" => Some(Table::Courses),
            "smtp_payloads" => Some(Table::SmtpPayloads),
            "smtp_response_links" => Some(Table::SmtpResponseLinks),
            "smtp_transactions" => Some(Table::SmtpTransactions),
            "report_files" => Some(Table::ReportFiles),
            "invitations" => Some(Table::Invitations),
            "faq_answers" => Some(Table::FaqAnswers),
            "classrooms" => Some(Table::Classrooms),
            "notifications" => Some(Table::Notifications),
            "classroom_stats" => Some(Table::ClassroomStats),
            "onboarding_contacts" => Some(Table::OnboardingContacts),
            "conversation_messages" => Some(Table::ConversationMessages),
            "incident_reports" => Some(Table::IncidentReports),
            "reports" => Some(Table::Reports),
            "game_responses" => Some(Table::GameResponses),
            "emotion_predictions" => Some(Table::EmotionPredictions),
            "game_review_question_reports" => Some(Table::GameReviewQuestionReports),
            "student_sessions" => Some(Table::StudentSessions),
            "transport_providers" => Some(Table::TransportProviders),
            "faqs" => Some(Table::Faqs),
            "report_child_victims" => Some(Table::ReportChildVictims),
            // Add other tables here
            _ => None,
        }
    }

    pub fn pluck_fields(&self, record_value: &Value, pluck_fields: Vec<String>) -> Value {
        if !pluck_fields.is_empty() && record_value.is_object() {
            if let Some(obj) = record_value.as_object() {
                let mut filtered = Map::new();

                for field in pluck_fields {
                    if let Some(val) = obj.get(&field) {
                        filtered.insert(field, val.clone());
                    }
                }

                Value::Object(filtered)
            } else {
                record_value.clone() // fallback: return original value
            }
        } else {
            record_value.clone()
        }
    }

    pub async fn get_hypertable_timestamp(
        &self,
        conn: &mut AsyncPgConnection,
        id: &str,
    ) -> Result<Option<String>, DieselError> {
        generate_hypertable_timestamp_match!(self, conn, id, SignedInActivities, TestHypertables, EmotionPredictions)
    }

    #[allow(dead_code)]
    pub async fn insert_record(
        &self,
        conn: &mut AsyncPgConnection,
        record: Value,
        request: web::Json<RequestBody>,
        auth: &Auth,
    ) -> Result<String, DieselError> {
        generate_insert_record_match!(
            self,
            auth,
            conn,
            record,
            request,
            UserRoles, UserRoleModel, Sessions, SessionModel, SignedInActivities, SignedInActivityModel, ExternalContacts, ExternalContactModel, Organizations, OrganizationModel, OrganizationContacts, OrganizationContactModel, OrganizationAccounts, OrganizationAccountModel, AccountOrganizations, AccountOrganizationModel, AccountProfiles, AccountProfileModel, Accounts, AccountModel, Addresses, AddressModel, Samples, SampleModel, Devices, DeviceModel, PostgresChannels, PostgresChannelModel, Contacts, ContactModel, ContactPhoneNumbers, ContactPhoneNumberModel, ContactEmails, ContactEmailModel, Files, FileModel, TestHypertables, TestHypertableModel, AccountPhoneNumbers, AccountPhoneNumberModel, AccountSignatures, AccountSignatureModel, GameChoiceReports, GameChoiceReportModel, Stories, StoryModel, Sponsorships, SponsorshipModel, UserProgress, UserProgressModel, TeacherStudents, TeacherStudentModel, CourseCompletionReports, CourseCompletionReportModel, ClassroomCourseStoriesEpisodes, ClassroomCourseStoriesEpisodeModel, OrganizationContactUserRoles, OrganizationContactUserRoleModel, GridFilters, GridFilterModel, StudentProgress, StudentProgressModel, ConversationTopics, ConversationTopicModel, FaqQuestions, FaqQuestionModel, SmtpContacts, SmtpContactModel, GameStats, GameStatModel, RelatedContacts, RelatedContactModel, EpisodeEvents, EpisodeEventModel, TeacherSchools, TeacherSchoolModel, TransportProviderCredentials, TransportProviderCredentialModel, SmtpAttachments, SmtpAttachmentModel, StudentSessionLogs, StudentSessionLogModel, GameStates, GameStateModel, GameChoices, GameChoiceModel, ClassroomStudents, ClassroomStudentModel, ClassroomCourseStories, ClassroomCourseStoryModel, AvatarSelections, AvatarSelectionModel, UserSettings, UserSettingModel, ChapterEvents, ChapterEventModel, ClassroomCourseEpisodes, ClassroomCourseEpisodeModel, FaqCategories, FaqCategoryModel, Locations, LocationModel, Conversations, ConversationModel, CommonSessionLogs, CommonSessionLogModel, GameQuestions, GameQuestionModel, CommunicationTemplates, CommunicationTemplateModel, ClassroomCourses, ClassroomCoursisModel, StudentStats, StudentStatModel, CommonSessions, CommonSessionModel, Episodes, EpisodeModel, SmtpAttachmentHeaders, SmtpAttachmentHeaderModel, Courses, CoursisModel, SmtpPayloads, SmtpPayloadModel, SmtpResponseLinks, SmtpResponseLinkModel, SmtpTransactions, SmtpTransactionModel, ReportFiles, ReportFileModel, Invitations, InvitationModel, FaqAnswers, FaqAnswerModel, Classrooms, ClassroomModel, Notifications, NotificationModel, ClassroomStats, ClassroomStatModel, OnboardingContacts, OnboardingContactModel, ConversationMessages, ConversationMessageModel, IncidentReports, IncidentReportModel, Reports, ReportModel, GameResponses, GameResponsisModel, EmotionPredictions, EmotionPredictionModel, GameReviewQuestionReports, GameReviewQuestionReportModel, StudentSessions, StudentSessionModel, TransportProviders, TransportProviderModel, Faqs, FaqModel, ReportChildVictims, ReportChildVictimModel // Add other tables and their models here as needed
        )
    }

    pub async fn get_by_id(
        &self,
        conn: &mut AsyncPgConnection,
        id: &str,
        is_root_account: bool,
        organization_id: Option<String>,
    ) -> Result<Option<Value>, DieselError> {
        generate_get_by_id_match!(
            self,
            conn,
            id,
            is_root_account,
            organization_id,
            UserRoles, UserRoleModel, Sessions, SessionModel, SignedInActivities, SignedInActivityModel, ExternalContacts, ExternalContactModel, Organizations, OrganizationModel, OrganizationContacts, OrganizationContactModel, OrganizationAccounts, OrganizationAccountModel, AccountOrganizations, AccountOrganizationModel, AccountProfiles, AccountProfileModel, Accounts, AccountModel, Addresses, AddressModel, Samples, SampleModel, Devices, DeviceModel, PostgresChannels, PostgresChannelModel, Contacts, ContactModel, ContactPhoneNumbers, ContactPhoneNumberModel, ContactEmails, ContactEmailModel, Files, FileModel, TestHypertables, TestHypertableModel, AccountPhoneNumbers, AccountPhoneNumberModel, AccountSignatures, AccountSignatureModel, GameChoiceReports, GameChoiceReportModel, Stories, StoryModel, Sponsorships, SponsorshipModel, UserProgress, UserProgressModel, TeacherStudents, TeacherStudentModel, CourseCompletionReports, CourseCompletionReportModel, ClassroomCourseStoriesEpisodes, ClassroomCourseStoriesEpisodeModel, OrganizationContactUserRoles, OrganizationContactUserRoleModel, GridFilters, GridFilterModel, StudentProgress, StudentProgressModel, ConversationTopics, ConversationTopicModel, FaqQuestions, FaqQuestionModel, SmtpContacts, SmtpContactModel, GameStats, GameStatModel, RelatedContacts, RelatedContactModel, EpisodeEvents, EpisodeEventModel, TeacherSchools, TeacherSchoolModel, TransportProviderCredentials, TransportProviderCredentialModel, SmtpAttachments, SmtpAttachmentModel, StudentSessionLogs, StudentSessionLogModel, GameStates, GameStateModel, GameChoices, GameChoiceModel, ClassroomStudents, ClassroomStudentModel, ClassroomCourseStories, ClassroomCourseStoryModel, AvatarSelections, AvatarSelectionModel, UserSettings, UserSettingModel, ChapterEvents, ChapterEventModel, ClassroomCourseEpisodes, ClassroomCourseEpisodeModel, FaqCategories, FaqCategoryModel, Locations, LocationModel, Conversations, ConversationModel, CommonSessionLogs, CommonSessionLogModel, GameQuestions, GameQuestionModel, CommunicationTemplates, CommunicationTemplateModel, ClassroomCourses, ClassroomCoursisModel, StudentStats, StudentStatModel, CommonSessions, CommonSessionModel, Episodes, EpisodeModel, SmtpAttachmentHeaders, SmtpAttachmentHeaderModel, Courses, CoursisModel, SmtpPayloads, SmtpPayloadModel, SmtpResponseLinks, SmtpResponseLinkModel, SmtpTransactions, SmtpTransactionModel, ReportFiles, ReportFileModel, Invitations, InvitationModel, FaqAnswers, FaqAnswerModel, Classrooms, ClassroomModel, Notifications, NotificationModel, ClassroomStats, ClassroomStatModel, OnboardingContacts, OnboardingContactModel, ConversationMessages, ConversationMessageModel, IncidentReports, IncidentReportModel, Reports, ReportModel, GameResponses, GameResponsisModel, EmotionPredictions, EmotionPredictionModel, GameReviewQuestionReports, GameReviewQuestionReportModel, StudentSessions, StudentSessionModel, TransportProviders, TransportProviderModel, Faqs, FaqModel, ReportChildVictims, ReportChildVictimModel // Add other tables and their models here as needed
        )
    }

    pub async fn upsert_record_with_id(
        &self,
        conn: &mut AsyncPgConnection,
        record: Value,
    ) -> Result<(), DieselError> {
        generate_upsert_record_match!(
            self,
            conn,
            record,
            UserRoles, UserRoleModel, Sessions, SessionModel, SignedInActivities, SignedInActivityModel, ExternalContacts, ExternalContactModel, Organizations, OrganizationModel, OrganizationContacts, OrganizationContactModel, OrganizationAccounts, OrganizationAccountModel, AccountOrganizations, AccountOrganizationModel, AccountProfiles, AccountProfileModel, Accounts, AccountModel, Addresses, AddressModel, Samples, SampleModel, Devices, DeviceModel, PostgresChannels, PostgresChannelModel, Contacts, ContactModel, ContactPhoneNumbers, ContactPhoneNumberModel, ContactEmails, ContactEmailModel, Files, FileModel, TestHypertables, TestHypertableModel, AccountPhoneNumbers, AccountPhoneNumberModel, AccountSignatures, AccountSignatureModel, GameChoiceReports, GameChoiceReportModel, Stories, StoryModel, Sponsorships, SponsorshipModel, UserProgress, UserProgressModel, TeacherStudents, TeacherStudentModel, CourseCompletionReports, CourseCompletionReportModel, ClassroomCourseStoriesEpisodes, ClassroomCourseStoriesEpisodeModel, OrganizationContactUserRoles, OrganizationContactUserRoleModel, GridFilters, GridFilterModel, StudentProgress, StudentProgressModel, ConversationTopics, ConversationTopicModel, FaqQuestions, FaqQuestionModel, SmtpContacts, SmtpContactModel, GameStats, GameStatModel, RelatedContacts, RelatedContactModel, EpisodeEvents, EpisodeEventModel, TeacherSchools, TeacherSchoolModel, TransportProviderCredentials, TransportProviderCredentialModel, SmtpAttachments, SmtpAttachmentModel, StudentSessionLogs, StudentSessionLogModel, GameStates, GameStateModel, GameChoices, GameChoiceModel, ClassroomStudents, ClassroomStudentModel, ClassroomCourseStories, ClassroomCourseStoryModel, AvatarSelections, AvatarSelectionModel, UserSettings, UserSettingModel, ChapterEvents, ChapterEventModel, ClassroomCourseEpisodes, ClassroomCourseEpisodeModel, FaqCategories, FaqCategoryModel, Locations, LocationModel, Conversations, ConversationModel, CommonSessionLogs, CommonSessionLogModel, GameQuestions, GameQuestionModel, CommunicationTemplates, CommunicationTemplateModel, ClassroomCourses, ClassroomCoursisModel, StudentStats, StudentStatModel, CommonSessions, CommonSessionModel, Episodes, EpisodeModel, SmtpAttachmentHeaders, SmtpAttachmentHeaderModel, Courses, CoursisModel, SmtpPayloads, SmtpPayloadModel, SmtpResponseLinks, SmtpResponseLinkModel, SmtpTransactions, SmtpTransactionModel, ReportFiles, ReportFileModel, Invitations, InvitationModel, FaqAnswers, FaqAnswerModel, Classrooms, ClassroomModel, Notifications, NotificationModel, ClassroomStats, ClassroomStatModel, OnboardingContacts, OnboardingContactModel, ConversationMessages, ConversationMessageModel, IncidentReports, IncidentReportModel, Reports, ReportModel, GameResponses, GameResponsisModel, EmotionPredictions, EmotionPredictionModel, GameReviewQuestionReports, GameReviewQuestionReportModel, StudentSessions, StudentSessionModel, TransportProviders, TransportProviderModel, Faqs, FaqModel, ReportChildVictims, ReportChildVictimModel // Add other tables and their models here as needed
        )
    }

    pub async fn upsert_record_with_id_timestamp(
        &self,
        conn: &mut AsyncPgConnection,
        record: Value,
    ) -> Result<(), DieselError> {
        generate_upsert_record_with_timestamp_match!(
            self,
            conn,
            record,
            UserRoles, UserRoleModel, Sessions, SessionModel, SignedInActivities, SignedInActivityModel, ExternalContacts, ExternalContactModel, Organizations, OrganizationModel, OrganizationContacts, OrganizationContactModel, OrganizationAccounts, OrganizationAccountModel, AccountOrganizations, AccountOrganizationModel, AccountProfiles, AccountProfileModel, Accounts, AccountModel, Addresses, AddressModel, Samples, SampleModel, Devices, DeviceModel, PostgresChannels, PostgresChannelModel, Contacts, ContactModel, ContactPhoneNumbers, ContactPhoneNumberModel, ContactEmails, ContactEmailModel, Files, FileModel, TestHypertables, TestHypertableModel, AccountPhoneNumbers, AccountPhoneNumberModel, AccountSignatures, AccountSignatureModel, GameChoiceReports, GameChoiceReportModel, Stories, StoryModel, Sponsorships, SponsorshipModel, UserProgress, UserProgressModel, TeacherStudents, TeacherStudentModel, CourseCompletionReports, CourseCompletionReportModel, ClassroomCourseStoriesEpisodes, ClassroomCourseStoriesEpisodeModel, OrganizationContactUserRoles, OrganizationContactUserRoleModel, GridFilters, GridFilterModel, StudentProgress, StudentProgressModel, ConversationTopics, ConversationTopicModel, FaqQuestions, FaqQuestionModel, SmtpContacts, SmtpContactModel, GameStats, GameStatModel, RelatedContacts, RelatedContactModel, EpisodeEvents, EpisodeEventModel, TeacherSchools, TeacherSchoolModel, TransportProviderCredentials, TransportProviderCredentialModel, SmtpAttachments, SmtpAttachmentModel, StudentSessionLogs, StudentSessionLogModel, GameStates, GameStateModel, GameChoices, GameChoiceModel, ClassroomStudents, ClassroomStudentModel, ClassroomCourseStories, ClassroomCourseStoryModel, AvatarSelections, AvatarSelectionModel, UserSettings, UserSettingModel, ChapterEvents, ChapterEventModel, ClassroomCourseEpisodes, ClassroomCourseEpisodeModel, FaqCategories, FaqCategoryModel, Locations, LocationModel, Conversations, ConversationModel, CommonSessionLogs, CommonSessionLogModel, GameQuestions, GameQuestionModel, CommunicationTemplates, CommunicationTemplateModel, ClassroomCourses, ClassroomCoursisModel, StudentStats, StudentStatModel, CommonSessions, CommonSessionModel, Episodes, EpisodeModel, SmtpAttachmentHeaders, SmtpAttachmentHeaderModel, Courses, CoursisModel, SmtpPayloads, SmtpPayloadModel, SmtpResponseLinks, SmtpResponseLinkModel, SmtpTransactions, SmtpTransactionModel, ReportFiles, ReportFileModel, Invitations, InvitationModel, FaqAnswers, FaqAnswerModel, Classrooms, ClassroomModel, Notifications, NotificationModel, ClassroomStats, ClassroomStatModel, OnboardingContacts, OnboardingContactModel, ConversationMessages, ConversationMessageModel, IncidentReports, IncidentReportModel, Reports, ReportModel, GameResponses, GameResponsisModel, EmotionPredictions, EmotionPredictionModel, GameReviewQuestionReports, GameReviewQuestionReportModel, StudentSessions, StudentSessionModel, TransportProviders, TransportProviderModel, Faqs, FaqModel, ReportChildVictims, ReportChildVictimModel // Add other tables and their models here as needed
        )
    }
}
pub async fn generate_code(
    table: &str,
    prefix_param: &str,
    default_code_param: i32,
) -> Result<String, DieselError> {

    let mut conn = db::get_async_connection().await;

    let new_counter = CounterModel {
        entity: table.to_string(),
        counter: 1,
        prefix: prefix_param.to_string(),
        default_code: default_code_param,
        digits_number: 1,
    };
    
    // Attempt the insert with conflict handling
    let result = diesel::insert_into(schema::counters::dsl::counters::table())
    .values(&new_counter)
        .on_conflict(schema::counters::entity)
        .do_update()
        .set(schema::counters::counter.eq(schema::counters::counter + 1))
        .returning((schema::counters::prefix, schema::counters::default_code, schema::counters::counter))
        .get_result::<(String, i32, i32)>(&mut conn).await
        .map_err(|e| {
            log::error!("Error generating code: {}", e);
            e
        })?;
    
    // Format the code
    let (prefix_val, default_code_val, counter_val) = result;
    let code = format!(
        "{}{}",
        prefix_val,
        default_code_val + counter_val
    );
    
    Ok(code)
}
