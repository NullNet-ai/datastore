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
use crate::generated::schema;
use crate::structs::core::{Auth, RequestBody};
use actix_web::web;
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel_async::AsyncPgConnection;
use diesel_async::RunQueryDsl;
use serde_json::{Map, Value};
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
        generate_hypertable_timestamp_match!(self, conn, id, SignedInActivities, TestHypertables)
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
            UserRoles, UserRoleModel, Sessions, SessionModel, SignedInActivities, SignedInActivityModel, ExternalContacts, ExternalContactModel, Organizations, OrganizationModel, OrganizationContacts, OrganizationContactModel, OrganizationAccounts, OrganizationAccountModel, AccountOrganizations, AccountOrganizationModel, AccountProfiles, AccountProfileModel, Accounts, AccountModel, Addresses, AddressModel, Samples, SampleModel, Devices, DeviceModel, PostgresChannels, PostgresChannelModel, Contacts, ContactModel, ContactPhoneNumbers, ContactPhoneNumberModel, ContactEmails, ContactEmailModel, Files, FileModel, TestHypertables, TestHypertableModel, AccountPhoneNumbers, AccountPhoneNumberModel, AccountSignatures, AccountSignatureModel // Add other tables and their models here as needed
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
            UserRoles, UserRoleModel, Sessions, SessionModel, SignedInActivities, SignedInActivityModel, ExternalContacts, ExternalContactModel, Organizations, OrganizationModel, OrganizationContacts, OrganizationContactModel, OrganizationAccounts, OrganizationAccountModel, AccountOrganizations, AccountOrganizationModel, AccountProfiles, AccountProfileModel, Accounts, AccountModel, Addresses, AddressModel, Samples, SampleModel, Devices, DeviceModel, PostgresChannels, PostgresChannelModel, Contacts, ContactModel, ContactPhoneNumbers, ContactPhoneNumberModel, ContactEmails, ContactEmailModel, Files, FileModel, TestHypertables, TestHypertableModel, AccountPhoneNumbers, AccountPhoneNumberModel, AccountSignatures, AccountSignatureModel // Add other tables and their models here as needed
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
            UserRoles, UserRoleModel, Sessions, SessionModel, SignedInActivities, SignedInActivityModel, ExternalContacts, ExternalContactModel, Organizations, OrganizationModel, OrganizationContacts, OrganizationContactModel, OrganizationAccounts, OrganizationAccountModel, AccountOrganizations, AccountOrganizationModel, AccountProfiles, AccountProfileModel, Accounts, AccountModel, Addresses, AddressModel, Samples, SampleModel, Devices, DeviceModel, PostgresChannels, PostgresChannelModel, Contacts, ContactModel, ContactPhoneNumbers, ContactPhoneNumberModel, ContactEmails, ContactEmailModel, Files, FileModel, TestHypertables, TestHypertableModel, AccountPhoneNumbers, AccountPhoneNumberModel, AccountSignatures, AccountSignatureModel // Add other tables and their models here as needed
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
            UserRoles, UserRoleModel, Sessions, SessionModel, SignedInActivities, SignedInActivityModel, ExternalContacts, ExternalContactModel, Organizations, OrganizationModel, OrganizationContacts, OrganizationContactModel, OrganizationAccounts, OrganizationAccountModel, AccountOrganizations, AccountOrganizationModel, AccountProfiles, AccountProfileModel, Accounts, AccountModel, Addresses, AddressModel, Samples, SampleModel, Devices, DeviceModel, PostgresChannels, PostgresChannelModel, Contacts, ContactModel, ContactPhoneNumbers, ContactPhoneNumberModel, ContactEmails, ContactEmailModel, Files, FileModel, TestHypertables, TestHypertableModel, AccountPhoneNumbers, AccountPhoneNumberModel, AccountSignatures, AccountSignatureModel // Add other tables and their models here as needed
        )
    }
}
/// Generate next unique code for the table via counter-service.
pub async fn generate_code(
    table: &str,
    prefix_param: &str,
    default_code_param: i32,
) -> Result<String, DieselError> {
    crate::utils::code_generator::generate_code(table, prefix_param, default_code_param).await
}
