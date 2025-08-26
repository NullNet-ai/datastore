use crate::controllers::common_controller::{
    convert_json_to_csv, execute_copy, perform_upsert,
    process_and_get_record_by_id, process_and_insert_record, process_and_update_record,
    process_record_for_update, process_records, sanitize_updates,
};
use crate::with_session_management;
use crate::database::db;
use crate::database::db::create_connection;
use diesel_async::RunQueryDsl;
use crate::{generate_create_method, generate_update_method, generate_batch_insert_method,
    generate_batch_update_method, generate_get_method, generate_delete_method,
    generate_batch_delete_method, generate_upsert_method, generate_aggregation_filter_method};
use crate::generated::store::store_service_server::{StoreServiceServer, StoreService };
use crate::middlewares::auth_middleware::GrpcAuthInterceptor;
use crate::middlewares::session_middleware::{GrpcSessionInterceptor, InterceptorChain};
use crate::middlewares::shutdown_middleware::GrpcShutdownInterceptor;
use crate::providers::queries::find::DynamicResult;
use crate::structs::structs::RequestBody;
use crate::providers::operations::sync::sync_service::update;
use crate::generated::table_enum::Table;
use crate::utils::utils::table_exists;
use serde_json::Value;
use std::net::SocketAddr;
use std::pin::Pin;
use tonic::{transport::Server, Request, Response, Status};
// Note: AggregationFilterWrapper has been moved to providers::aggregation_filter
// Note: Converter functions have been moved to grpc_struct_converter.rs
use crate::generated::store::{UserRoles, CreateUserRolesRequest, CreateUserRolesResponse, GetUserRolesRequest, GetUserRolesResponse, UpdateUserRolesRequest, UpdateUserRolesResponse, DeleteUserRolesRequest, DeleteUserRolesResponse, BatchInsertUserRolesRequest, BatchInsertUserRolesResponse, BatchUpdateUserRolesRequest, BatchUpdateUserRolesResponse, BatchDeleteUserRolesRequest, BatchDeleteUserRolesResponse, UpsertUserRolesRequest, UpsertUserRolesResponse, Sessions, CreateSessionsRequest, CreateSessionsResponse, GetSessionsRequest, GetSessionsResponse, UpdateSessionsRequest, UpdateSessionsResponse, DeleteSessionsRequest, DeleteSessionsResponse, BatchInsertSessionsRequest, BatchInsertSessionsResponse, BatchUpdateSessionsRequest, BatchUpdateSessionsResponse, BatchDeleteSessionsRequest, BatchDeleteSessionsResponse, UpsertSessionsRequest, UpsertSessionsResponse, SignedInActivities, CreateSignedInActivitiesRequest, CreateSignedInActivitiesResponse, GetSignedInActivitiesRequest, GetSignedInActivitiesResponse, UpdateSignedInActivitiesRequest, UpdateSignedInActivitiesResponse, DeleteSignedInActivitiesRequest, DeleteSignedInActivitiesResponse, BatchInsertSignedInActivitiesRequest, BatchInsertSignedInActivitiesResponse, BatchUpdateSignedInActivitiesRequest, BatchUpdateSignedInActivitiesResponse, BatchDeleteSignedInActivitiesRequest, BatchDeleteSignedInActivitiesResponse, UpsertSignedInActivitiesRequest, UpsertSignedInActivitiesResponse, ExternalContacts, CreateExternalContactsRequest, CreateExternalContactsResponse, GetExternalContactsRequest, GetExternalContactsResponse, UpdateExternalContactsRequest, UpdateExternalContactsResponse, DeleteExternalContactsRequest, DeleteExternalContactsResponse, BatchInsertExternalContactsRequest, BatchInsertExternalContactsResponse, BatchUpdateExternalContactsRequest, BatchUpdateExternalContactsResponse, BatchDeleteExternalContactsRequest, BatchDeleteExternalContactsResponse, UpsertExternalContactsRequest, UpsertExternalContactsResponse, Organizations, CreateOrganizationsRequest, CreateOrganizationsResponse, GetOrganizationsRequest, GetOrganizationsResponse, UpdateOrganizationsRequest, UpdateOrganizationsResponse, DeleteOrganizationsRequest, DeleteOrganizationsResponse, BatchInsertOrganizationsRequest, BatchInsertOrganizationsResponse, BatchUpdateOrganizationsRequest, BatchUpdateOrganizationsResponse, BatchDeleteOrganizationsRequest, BatchDeleteOrganizationsResponse, UpsertOrganizationsRequest, UpsertOrganizationsResponse, OrganizationContacts, CreateOrganizationContactsRequest, CreateOrganizationContactsResponse, GetOrganizationContactsRequest, GetOrganizationContactsResponse, UpdateOrganizationContactsRequest, UpdateOrganizationContactsResponse, DeleteOrganizationContactsRequest, DeleteOrganizationContactsResponse, BatchInsertOrganizationContactsRequest, BatchInsertOrganizationContactsResponse, BatchUpdateOrganizationContactsRequest, BatchUpdateOrganizationContactsResponse, BatchDeleteOrganizationContactsRequest, BatchDeleteOrganizationContactsResponse, UpsertOrganizationContactsRequest, UpsertOrganizationContactsResponse, OrganizationAccounts, CreateOrganizationAccountsRequest, CreateOrganizationAccountsResponse, GetOrganizationAccountsRequest, GetOrganizationAccountsResponse, UpdateOrganizationAccountsRequest, UpdateOrganizationAccountsResponse, DeleteOrganizationAccountsRequest, DeleteOrganizationAccountsResponse, BatchInsertOrganizationAccountsRequest, BatchInsertOrganizationAccountsResponse, BatchUpdateOrganizationAccountsRequest, BatchUpdateOrganizationAccountsResponse, BatchDeleteOrganizationAccountsRequest, BatchDeleteOrganizationAccountsResponse, UpsertOrganizationAccountsRequest, UpsertOrganizationAccountsResponse, AccountOrganizations, CreateAccountOrganizationsRequest, CreateAccountOrganizationsResponse, GetAccountOrganizationsRequest, GetAccountOrganizationsResponse, UpdateAccountOrganizationsRequest, UpdateAccountOrganizationsResponse, DeleteAccountOrganizationsRequest, DeleteAccountOrganizationsResponse, BatchInsertAccountOrganizationsRequest, BatchInsertAccountOrganizationsResponse, BatchUpdateAccountOrganizationsRequest, BatchUpdateAccountOrganizationsResponse, BatchDeleteAccountOrganizationsRequest, BatchDeleteAccountOrganizationsResponse, UpsertAccountOrganizationsRequest, UpsertAccountOrganizationsResponse, AccountProfiles, CreateAccountProfilesRequest, CreateAccountProfilesResponse, GetAccountProfilesRequest, GetAccountProfilesResponse, UpdateAccountProfilesRequest, UpdateAccountProfilesResponse, DeleteAccountProfilesRequest, DeleteAccountProfilesResponse, BatchInsertAccountProfilesRequest, BatchInsertAccountProfilesResponse, BatchUpdateAccountProfilesRequest, BatchUpdateAccountProfilesResponse, BatchDeleteAccountProfilesRequest, BatchDeleteAccountProfilesResponse, UpsertAccountProfilesRequest, UpsertAccountProfilesResponse, Accounts, CreateAccountsRequest, CreateAccountsResponse, GetAccountsRequest, GetAccountsResponse, UpdateAccountsRequest, UpdateAccountsResponse, DeleteAccountsRequest, DeleteAccountsResponse, BatchInsertAccountsRequest, BatchInsertAccountsResponse, BatchUpdateAccountsRequest, BatchUpdateAccountsResponse, BatchDeleteAccountsRequest, BatchDeleteAccountsResponse, UpsertAccountsRequest, UpsertAccountsResponse, Addresses, CreateAddressesRequest, CreateAddressesResponse, GetAddressesRequest, GetAddressesResponse, UpdateAddressesRequest, UpdateAddressesResponse, DeleteAddressesRequest, DeleteAddressesResponse, BatchInsertAddressesRequest, BatchInsertAddressesResponse, BatchUpdateAddressesRequest, BatchUpdateAddressesResponse, BatchDeleteAddressesRequest, BatchDeleteAddressesResponse, UpsertAddressesRequest, UpsertAddressesResponse, Samples, CreateSamplesRequest, CreateSamplesResponse, GetSamplesRequest, GetSamplesResponse, UpdateSamplesRequest, UpdateSamplesResponse, DeleteSamplesRequest, DeleteSamplesResponse, BatchInsertSamplesRequest, BatchInsertSamplesResponse, BatchUpdateSamplesRequest, BatchUpdateSamplesResponse, BatchDeleteSamplesRequest, BatchDeleteSamplesResponse, UpsertSamplesRequest, UpsertSamplesResponse, Devices, CreateDevicesRequest, CreateDevicesResponse, GetDevicesRequest, GetDevicesResponse, UpdateDevicesRequest, UpdateDevicesResponse, DeleteDevicesRequest, DeleteDevicesResponse, BatchInsertDevicesRequest, BatchInsertDevicesResponse, BatchUpdateDevicesRequest, BatchUpdateDevicesResponse, BatchDeleteDevicesRequest, BatchDeleteDevicesResponse, UpsertDevicesRequest, UpsertDevicesResponse, PostgresChannels, CreatePostgresChannelsRequest, CreatePostgresChannelsResponse, GetPostgresChannelsRequest, GetPostgresChannelsResponse, UpdatePostgresChannelsRequest, UpdatePostgresChannelsResponse, DeletePostgresChannelsRequest, DeletePostgresChannelsResponse, BatchInsertPostgresChannelsRequest, BatchInsertPostgresChannelsResponse, BatchUpdatePostgresChannelsRequest, BatchUpdatePostgresChannelsResponse, BatchDeletePostgresChannelsRequest, BatchDeletePostgresChannelsResponse, UpsertPostgresChannelsRequest, UpsertPostgresChannelsResponse, Contacts, CreateContactsRequest, CreateContactsResponse, GetContactsRequest, GetContactsResponse, UpdateContactsRequest, UpdateContactsResponse, DeleteContactsRequest, DeleteContactsResponse, BatchInsertContactsRequest, BatchInsertContactsResponse, BatchUpdateContactsRequest, BatchUpdateContactsResponse, BatchDeleteContactsRequest, BatchDeleteContactsResponse, UpsertContactsRequest, UpsertContactsResponse, ContactPhoneNumbers, CreateContactPhoneNumbersRequest, CreateContactPhoneNumbersResponse, GetContactPhoneNumbersRequest, GetContactPhoneNumbersResponse, UpdateContactPhoneNumbersRequest, UpdateContactPhoneNumbersResponse, DeleteContactPhoneNumbersRequest, DeleteContactPhoneNumbersResponse, BatchInsertContactPhoneNumbersRequest, BatchInsertContactPhoneNumbersResponse, BatchUpdateContactPhoneNumbersRequest, BatchUpdateContactPhoneNumbersResponse, BatchDeleteContactPhoneNumbersRequest, BatchDeleteContactPhoneNumbersResponse, UpsertContactPhoneNumbersRequest, UpsertContactPhoneNumbersResponse, ContactEmails, CreateContactEmailsRequest, CreateContactEmailsResponse, GetContactEmailsRequest, GetContactEmailsResponse, UpdateContactEmailsRequest, UpdateContactEmailsResponse, DeleteContactEmailsRequest, DeleteContactEmailsResponse, BatchInsertContactEmailsRequest, BatchInsertContactEmailsResponse, BatchUpdateContactEmailsRequest, BatchUpdateContactEmailsResponse, BatchDeleteContactEmailsRequest, BatchDeleteContactEmailsResponse, UpsertContactEmailsRequest, UpsertContactEmailsResponse, Files, CreateFilesRequest, CreateFilesResponse, GetFilesRequest, GetFilesResponse, UpdateFilesRequest, UpdateFilesResponse, DeleteFilesRequest, DeleteFilesResponse, BatchInsertFilesRequest, BatchInsertFilesResponse, BatchUpdateFilesRequest, BatchUpdateFilesResponse, BatchDeleteFilesRequest, BatchDeleteFilesResponse, UpsertFilesRequest, UpsertFilesResponse, TestHypertable, CreateTestHypertableRequest, CreateTestHypertableResponse, GetTestHypertableRequest, GetTestHypertableResponse, UpdateTestHypertableRequest, UpdateTestHypertableResponse, DeleteTestHypertableRequest, DeleteTestHypertableResponse, BatchInsertTestHypertableRequest, BatchInsertTestHypertableResponse, BatchUpdateTestHypertableRequest, BatchUpdateTestHypertableResponse, BatchDeleteTestHypertableRequest, BatchDeleteTestHypertableResponse, UpsertTestHypertableRequest, UpsertTestHypertableResponse, AccountPhoneNumbers, CreateAccountPhoneNumbersRequest, CreateAccountPhoneNumbersResponse, GetAccountPhoneNumbersRequest, GetAccountPhoneNumbersResponse, UpdateAccountPhoneNumbersRequest, UpdateAccountPhoneNumbersResponse, DeleteAccountPhoneNumbersRequest, DeleteAccountPhoneNumbersResponse, BatchInsertAccountPhoneNumbersRequest, BatchInsertAccountPhoneNumbersResponse, BatchUpdateAccountPhoneNumbersRequest, BatchUpdateAccountPhoneNumbersResponse, BatchDeleteAccountPhoneNumbersRequest, BatchDeleteAccountPhoneNumbersResponse, UpsertAccountPhoneNumbersRequest, UpsertAccountPhoneNumbersResponse, AccountSignatures, CreateAccountSignaturesRequest, CreateAccountSignaturesResponse, GetAccountSignaturesRequest, GetAccountSignaturesResponse, UpdateAccountSignaturesRequest, UpdateAccountSignaturesResponse, DeleteAccountSignaturesRequest, DeleteAccountSignaturesResponse, BatchInsertAccountSignaturesRequest, BatchInsertAccountSignaturesResponse, BatchUpdateAccountSignaturesRequest, BatchUpdateAccountSignaturesResponse, BatchDeleteAccountSignaturesRequest, BatchDeleteAccountSignaturesResponse, UpsertAccountSignaturesRequest, UpsertAccountSignaturesResponse, AggregationFilterRequest, AggregationFilterResponse
};
pub struct GrpcController {}

impl GrpcController {
    pub fn new() -> Self { GrpcController {} }

    pub async fn init(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
        let addr: SocketAddr = addr.parse()?;
        let grpc_controller = GrpcController::new();
        println!("gRPC Server listening on {}", addr);
        // Create a chain of interceptors
        let session_interceptor = GrpcSessionInterceptor::new();
        let auth_interceptor = GrpcAuthInterceptor;
        let shutdown_interceptor = GrpcShutdownInterceptor;
        
        // Chain interceptors: shutdown -> session -> auth
        let session_auth_chain = InterceptorChain::new(session_interceptor, auth_interceptor);
        let interceptor_chain = InterceptorChain::new(shutdown_interceptor, session_auth_chain);
        Server::builder()
                .add_service(StoreServiceServer::with_interceptor(
                    grpc_controller,
                    interceptor_chain
                ))
                .serve(addr)
                .await?;
        Ok(())
    }
}

#[tonic::async_trait]
impl StoreService for GrpcController {
    // CRUD methods for user_roles
    generate_create_method!(user_roles);
    generate_update_method!(user_roles, user_role);
    generate_batch_insert_method!(user_roles);
    generate_batch_update_method!(user_roles);
    generate_get_method!(user_roles);
    generate_delete_method!(user_roles);
    generate_batch_delete_method!(user_roles);
    generate_upsert_method!(user_roles);
    // CRUD methods for sessions
    generate_create_method!(sessions);
    generate_update_method!(sessions, session);
    generate_batch_insert_method!(sessions);
    generate_batch_update_method!(sessions);
    generate_get_method!(sessions);
    generate_delete_method!(sessions);
    generate_batch_delete_method!(sessions);
    generate_upsert_method!(sessions);
    // CRUD methods for signed_in_activities
    generate_create_method!(signed_in_activities);
    generate_update_method!(signed_in_activities, signed_in_activity);
    generate_batch_insert_method!(signed_in_activities);
    generate_batch_update_method!(signed_in_activities);
    generate_get_method!(signed_in_activities);
    generate_delete_method!(signed_in_activities);
    generate_batch_delete_method!(signed_in_activities);
    generate_upsert_method!(signed_in_activities);
    // CRUD methods for external_contacts
    generate_create_method!(external_contacts);
    generate_update_method!(external_contacts, external_contact);
    generate_batch_insert_method!(external_contacts);
    generate_batch_update_method!(external_contacts);
    generate_get_method!(external_contacts);
    generate_delete_method!(external_contacts);
    generate_batch_delete_method!(external_contacts);
    generate_upsert_method!(external_contacts);
    // CRUD methods for organizations
    generate_create_method!(organizations);
    generate_update_method!(organizations, organization);
    generate_batch_insert_method!(organizations);
    generate_batch_update_method!(organizations);
    generate_get_method!(organizations);
    generate_delete_method!(organizations);
    generate_batch_delete_method!(organizations);
    generate_upsert_method!(organizations);
    // CRUD methods for organization_contacts
    generate_create_method!(organization_contacts);
    generate_update_method!(organization_contacts, organization_contact);
    generate_batch_insert_method!(organization_contacts);
    generate_batch_update_method!(organization_contacts);
    generate_get_method!(organization_contacts);
    generate_delete_method!(organization_contacts);
    generate_batch_delete_method!(organization_contacts);
    generate_upsert_method!(organization_contacts);
    // CRUD methods for organization_accounts
    generate_create_method!(organization_accounts);
    generate_update_method!(organization_accounts, organization_account);
    generate_batch_insert_method!(organization_accounts);
    generate_batch_update_method!(organization_accounts);
    generate_get_method!(organization_accounts);
    generate_delete_method!(organization_accounts);
    generate_batch_delete_method!(organization_accounts);
    generate_upsert_method!(organization_accounts);
    // CRUD methods for account_organizations
    generate_create_method!(account_organizations);
    generate_update_method!(account_organizations, account_organization);
    generate_batch_insert_method!(account_organizations);
    generate_batch_update_method!(account_organizations);
    generate_get_method!(account_organizations);
    generate_delete_method!(account_organizations);
    generate_batch_delete_method!(account_organizations);
    generate_upsert_method!(account_organizations);
    // CRUD methods for account_profiles
    generate_create_method!(account_profiles);
    generate_update_method!(account_profiles, account_profile);
    generate_batch_insert_method!(account_profiles);
    generate_batch_update_method!(account_profiles);
    generate_get_method!(account_profiles);
    generate_delete_method!(account_profiles);
    generate_batch_delete_method!(account_profiles);
    generate_upsert_method!(account_profiles);
    // CRUD methods for accounts
    generate_create_method!(accounts);
    generate_update_method!(accounts, account);
    generate_batch_insert_method!(accounts);
    generate_batch_update_method!(accounts);
    generate_get_method!(accounts);
    generate_delete_method!(accounts);
    generate_batch_delete_method!(accounts);
    generate_upsert_method!(accounts);
    // CRUD methods for addresses
    generate_create_method!(addresses);
    generate_update_method!(addresses, address);
    generate_batch_insert_method!(addresses);
    generate_batch_update_method!(addresses);
    generate_get_method!(addresses);
    generate_delete_method!(addresses);
    generate_batch_delete_method!(addresses);
    generate_upsert_method!(addresses);
    // CRUD methods for samples
    generate_create_method!(samples);
    generate_update_method!(samples, sample);
    generate_batch_insert_method!(samples);
    generate_batch_update_method!(samples);
    generate_get_method!(samples);
    generate_delete_method!(samples);
    generate_batch_delete_method!(samples);
    generate_upsert_method!(samples);
    // CRUD methods for devices
    generate_create_method!(devices);
    generate_update_method!(devices, device);
    generate_batch_insert_method!(devices);
    generate_batch_update_method!(devices);
    generate_get_method!(devices);
    generate_delete_method!(devices);
    generate_batch_delete_method!(devices);
    generate_upsert_method!(devices);
    // CRUD methods for postgres_channels
    generate_create_method!(postgres_channels);
    generate_update_method!(postgres_channels, postgres_channel);
    generate_batch_insert_method!(postgres_channels);
    generate_batch_update_method!(postgres_channels);
    generate_get_method!(postgres_channels);
    generate_delete_method!(postgres_channels);
    generate_batch_delete_method!(postgres_channels);
    generate_upsert_method!(postgres_channels);
    // CRUD methods for contacts
    generate_create_method!(contacts);
    generate_update_method!(contacts, contact);
    generate_batch_insert_method!(contacts);
    generate_batch_update_method!(contacts);
    generate_get_method!(contacts);
    generate_delete_method!(contacts);
    generate_batch_delete_method!(contacts);
    generate_upsert_method!(contacts);
    // CRUD methods for contact_phone_numbers
    generate_create_method!(contact_phone_numbers);
    generate_update_method!(contact_phone_numbers, contact_phone_number);
    generate_batch_insert_method!(contact_phone_numbers);
    generate_batch_update_method!(contact_phone_numbers);
    generate_get_method!(contact_phone_numbers);
    generate_delete_method!(contact_phone_numbers);
    generate_batch_delete_method!(contact_phone_numbers);
    generate_upsert_method!(contact_phone_numbers);
    // CRUD methods for contact_emails
    generate_create_method!(contact_emails);
    generate_update_method!(contact_emails, contact_email);
    generate_batch_insert_method!(contact_emails);
    generate_batch_update_method!(contact_emails);
    generate_get_method!(contact_emails);
    generate_delete_method!(contact_emails);
    generate_batch_delete_method!(contact_emails);
    generate_upsert_method!(contact_emails);
    // CRUD methods for files
    generate_create_method!(files);
    generate_update_method!(files, file);
    generate_batch_insert_method!(files);
    generate_batch_update_method!(files);
    generate_get_method!(files);
    generate_delete_method!(files);
    generate_batch_delete_method!(files);
    generate_upsert_method!(files);
    // CRUD methods for test_hypertable
    generate_create_method!(test_hypertable);
    generate_update_method!(test_hypertable, test_hypertable);
    generate_batch_insert_method!(test_hypertable);
    generate_batch_update_method!(test_hypertable);
    generate_get_method!(test_hypertable);
    generate_delete_method!(test_hypertable);
    generate_batch_delete_method!(test_hypertable);
    generate_upsert_method!(test_hypertable);
    // CRUD methods for account_phone_numbers
    generate_create_method!(account_phone_numbers);
    generate_update_method!(account_phone_numbers, account_phone_number);
    generate_batch_insert_method!(account_phone_numbers);
    generate_batch_update_method!(account_phone_numbers);
    generate_get_method!(account_phone_numbers);
    generate_delete_method!(account_phone_numbers);
    generate_batch_delete_method!(account_phone_numbers);
    generate_upsert_method!(account_phone_numbers);
    // CRUD methods for account_signatures
    generate_create_method!(account_signatures);
    generate_update_method!(account_signatures, account_signature);
    generate_batch_insert_method!(account_signatures);
    generate_batch_update_method!(account_signatures);
    generate_get_method!(account_signatures);
    generate_delete_method!(account_signatures);
    generate_batch_delete_method!(account_signatures);
    generate_upsert_method!(account_signatures);
    // Aggregation filter method
    generate_aggregation_filter_method!();
}
