use crate::{generate_get_by_id_match, generate_hypertable_timestamp_match, generate_insert_record_match, generate_upsert_record_match, generate_upsert_record_with_timestamp_match};
use crate::models::session_model::SessionModel;
use crate::models::signed_in_activity_model::SignedInActivityModel;
use crate::models::external_contact_model::ExternalContactModel;
use crate::models::organization_model::OrganizationModel;
use crate::models::organization_contact_model::OrganizationContactModel;
use crate::models::organization_account_model::OrganizationAccountModel;
use crate::models::account_organization_model::AccountOrganizationModel;
use crate::models::account_profile_model::AccountProfileModel;
use crate::models::account_model::AccountModel;
use crate::models::address_model::AddressModel;
use crate::models::sample_model::SampleModel;
use crate::models::device_model::DeviceModel;
use crate::models::postgres_channel_model::PostgresChannelModel;
use crate::models::contact_model::ContactModel;
use crate::models::contact_phone_number_model::ContactPhoneNumberModel;
use crate::models::contact_email_model::ContactEmailModel;
use crate::models::file_model::FileModel;
use crate::models::test_hypertable_model::TestHypertableModel;
use crate::models::account_phone_number_model::AccountPhoneNumberModel;
use crate::models::account_signature_model::AccountSignatureModel;
use crate::schema::schema;
use crate::structs::structs::{Auth, RequestBody};
use actix_web::web;
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel_async::AsyncPgConnection;
use diesel_async::RunQueryDsl;
use serde_json::{Map, Value};
use crate::db;
use crate::models::counter_model::CounterModel;

#[derive(Debug)]
pub enum Table {
    Sessions,
    SignedInActivity,
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
    TestHypertable,
    AccountPhoneNumbers,
    AccountSignatures,
    // Add other tables here
}

impl Table {
    pub fn from_str(name: &str) -> Option<Self> {
        match name {
            "sessions" => Some(Table::Sessions),
            "signed_in_activity" => Some(Table::SignedInActivity),
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
            "test_hypertable" => Some(Table::TestHypertable),
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
        generate_hypertable_timestamp_match!(self, conn, id, TestHypertable)
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
            Sessions, SessionModel, SignedInActivity, SignedInActivityModel, ExternalContacts, ExternalContactModel, Organizations, OrganizationModel, OrganizationContacts, OrganizationContactModel, OrganizationAccounts, OrganizationAccountModel, AccountOrganizations, AccountOrganizationModel, AccountProfiles, AccountProfileModel, Accounts, AccountModel, Addresses, AddressModel, Samples, SampleModel, Devices, DeviceModel, PostgresChannels, PostgresChannelModel, Contacts, ContactModel, ContactPhoneNumbers, ContactPhoneNumberModel, ContactEmails, ContactEmailModel, Files, FileModel, TestHypertable, TestHypertableModel, AccountPhoneNumbers, AccountPhoneNumberModel, AccountSignatures, AccountSignatureModel // Add other tables and their models here as needed
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
            Sessions, SessionModel, SignedInActivity, SignedInActivityModel, ExternalContacts, ExternalContactModel, Organizations, OrganizationModel, OrganizationContacts, OrganizationContactModel, OrganizationAccounts, OrganizationAccountModel, AccountOrganizations, AccountOrganizationModel, AccountProfiles, AccountProfileModel, Accounts, AccountModel, Addresses, AddressModel, Samples, SampleModel, Devices, DeviceModel, PostgresChannels, PostgresChannelModel, Contacts, ContactModel, ContactPhoneNumbers, ContactPhoneNumberModel, ContactEmails, ContactEmailModel, Files, FileModel, TestHypertable, TestHypertableModel, AccountPhoneNumbers, AccountPhoneNumberModel, AccountSignatures, AccountSignatureModel // Add other tables and their models here as needed
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
            Sessions, SessionModel, SignedInActivity, SignedInActivityModel, ExternalContacts, ExternalContactModel, Organizations, OrganizationModel, OrganizationContacts, OrganizationContactModel, OrganizationAccounts, OrganizationAccountModel, AccountOrganizations, AccountOrganizationModel, AccountProfiles, AccountProfileModel, Accounts, AccountModel, Addresses, AddressModel, Samples, SampleModel, Devices, DeviceModel, PostgresChannels, PostgresChannelModel, Contacts, ContactModel, ContactPhoneNumbers, ContactPhoneNumberModel, ContactEmails, ContactEmailModel, Files, FileModel, TestHypertable, TestHypertableModel, AccountPhoneNumbers, AccountPhoneNumberModel, AccountSignatures, AccountSignatureModel // Add other tables and their models here as needed
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
            Sessions, SessionModel, SignedInActivity, SignedInActivityModel, ExternalContacts, ExternalContactModel, Organizations, OrganizationModel, OrganizationContacts, OrganizationContactModel, OrganizationAccounts, OrganizationAccountModel, AccountOrganizations, AccountOrganizationModel, AccountProfiles, AccountProfileModel, Accounts, AccountModel, Addresses, AddressModel, Samples, SampleModel, Devices, DeviceModel, PostgresChannels, PostgresChannelModel, Contacts, ContactModel, ContactPhoneNumbers, ContactPhoneNumberModel, ContactEmails, ContactEmailModel, Files, FileModel, TestHypertable, TestHypertableModel, AccountPhoneNumbers, AccountPhoneNumberModel, AccountSignatures, AccountSignatureModel // Add other tables and their models here as needed
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
