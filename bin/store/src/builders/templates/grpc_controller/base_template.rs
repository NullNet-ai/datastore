use super::common_controller::{
    convert_json_to_csv, execute_copy, perform_upsert,
    process_and_get_record_by_id, process_and_insert_record, process_and_update_record,
    process_record_for_update, process_records, sanitize_updates,
};
use crate::with_session_management;
use crate::database::db;
use crate::database::db::create_connection;
use diesel_async::RunQueryDsl;
use crate::generated::store::store_service_server::{StoreService, StoreServiceServer};
use crate::middleware::auth_middleware::GrpcAuthInterceptor;
use crate::middleware::session_middleware::{GrpcSessionInterceptor, InterceptorChain};
use crate::middleware::shutdown_middleware::GrpcShutdownInterceptor;
use crate::providers::queries::find::DynamicResult;
use crate::providers::queries::find::SQLConstructor;
use log::info;

use crate::structs::structs::RequestBody;
use crate::providers::operations::sync::sync_service::update;
use crate::table_enum::Table;
use crate::utils::utils::table_exists;
use serde_json::Value;
use std::net::SocketAddr;
use std::pin::Pin;
use tonic::{transport::Server, Request, Response, Status};
// Note: AggregationFilterWrapper has been moved to providers::aggregation_filter
// Note: Converter functions have been moved to grpc_struct_converter.rs
use crate::generated::store::{
    AccountOrganizations, AccountProfiles, Accounts, Addresses, AggregationFilterRequest,
    AggregationFilterResponse, BatchDeleteAccountOrganizationsRequest,
    BatchDeleteAccountOrganizationsResponse, BatchDeleteAccountProfilesRequest,
    BatchDeleteAccountProfilesResponse, BatchDeleteAccountsRequest, BatchDeleteAccountsResponse,
    BatchDeleteAddressesRequest, BatchDeleteAddressesResponse, BatchDeleteAppFirewallsRequest,
    BatchDeleteAppFirewallsResponse,
    BatchDeleteContactEmailsRequest, BatchDeleteContactEmailsResponse,
    BatchDeleteContactPhoneNumbersRequest, BatchDeleteContactPhoneNumbersResponse,
    BatchDeleteContactsRequest, BatchDeleteContactsResponse, BatchDeleteDevicesRequest, BatchDeleteDevicesResponse,
    BatchDeleteExternalContactsRequest, BatchDeleteExternalContactsResponse,
    BatchDeleteIpInfosRequest, BatchDeleteIpInfosResponse, BatchDeleteOrganizationAccountsRequest,
    BatchDeleteOrganizationAccountsResponse, BatchDeleteOrganizationContactsRequest,
    BatchDeleteOrganizationContactsResponse, BatchDeleteOrganizationsRequest,
    BatchDeleteOrganizationsResponse, BatchDeletePostgresChannelsRequest, BatchDeletePostgresChannelsResponse, BatchDeleteSamplesRequest,
    BatchDeleteSamplesResponse, BatchInsertAccountOrganizationsRequest,
    BatchInsertAccountOrganizationsResponse, BatchInsertAccountProfilesRequest,
    BatchInsertAccountProfilesResponse, BatchInsertAccountsRequest, BatchInsertAccountsResponse,
    BatchInsertAddressesRequest, BatchInsertAddressesResponse, BatchInsertAppFirewallsRequest,
    BatchInsertAppFirewallsResponse,
    BatchInsertContactEmailsRequest, BatchInsertContactEmailsResponse,
    BatchInsertContactPhoneNumbersRequest, BatchInsertContactPhoneNumbersResponse,
    BatchInsertContactsRequest, BatchInsertContactsResponse, BatchInsertDevicesRequest, BatchInsertDevicesResponse,
    BatchInsertExternalContactsRequest, BatchInsertExternalContactsResponse,
    BatchInsertIpInfosRequest, BatchInsertIpInfosResponse, BatchInsertOrganizationAccountsRequest,
    BatchInsertOrganizationAccountsResponse, BatchInsertOrganizationContactsRequest,
    BatchInsertOrganizationContactsResponse, BatchInsertOrganizationsRequest,
    BatchInsertOrganizationsResponse, BatchInsertPostgresChannelsRequest, BatchInsertPostgresChannelsResponse, BatchInsertSamplesRequest,
    BatchInsertSamplesResponse, BatchUpdateAccountOrganizationsRequest,
    BatchUpdateAccountOrganizationsResponse, BatchUpdateAccountProfilesRequest,
    BatchUpdateAccountProfilesResponse, BatchUpdateAccountsRequest, BatchUpdateAccountsResponse,
    BatchUpdateAddressesRequest, BatchUpdateAddressesResponse, BatchUpdateAppFirewallsRequest,
    BatchUpdateAppFirewallsResponse,
    BatchUpdateContactEmailsRequest, BatchUpdateContactEmailsResponse,
    BatchUpdateContactPhoneNumbersRequest, BatchUpdateContactPhoneNumbersResponse,
    BatchUpdateContactsRequest, BatchUpdateContactsResponse, BatchUpdateDevicesRequest, BatchUpdateDevicesResponse,
    BatchUpdateExternalContactsRequest, BatchUpdateExternalContactsResponse,
    BatchUpdateIpInfosRequest, BatchUpdateIpInfosResponse, BatchUpdateOrganizationAccountsRequest,
    BatchUpdateOrganizationAccountsResponse, BatchUpdateOrganizationContactsRequest,
    BatchUpdateOrganizationContactsResponse, BatchUpdateOrganizationsRequest,
    BatchUpdateOrganizationsResponse, BatchUpdatePostgresChannelsRequest, BatchUpdatePostgresChannelsResponse, BatchUpdateSamplesRequest,
    BatchUpdateSamplesResponse, ContactEmails, ContactPhoneNumbers, Contacts,
    CreateAccountOrganizationsRequest, CreateAccountOrganizationsResponse,
    CreateAccountProfilesRequest, CreateAccountProfilesResponse, CreateAccountsRequest,
    CreateAccountsResponse, CreateAddressesRequest, CreateAddressesResponse,
    CreateAppFirewallsRequest, CreateAppFirewallsResponse,
    CreateContactEmailsRequest, CreateContactEmailsResponse, CreateContactPhoneNumbersRequest,
    CreateContactPhoneNumbersResponse, CreateContactsRequest, CreateContactsResponse, CreateDevicesRequest,
    CreateDevicesResponse, CreateExternalContactsRequest, CreateExternalContactsResponse,
    CreateIpInfosRequest, CreateIpInfosResponse, CreateOrganizationAccountsRequest,
    CreateOrganizationAccountsResponse, CreateOrganizationContactsRequest,
    CreateOrganizationContactsResponse, CreateOrganizationsRequest, CreateOrganizationsResponse, CreatePostgresChannelsRequest,
    CreatePostgresChannelsResponse,
    CreateSamplesRequest, CreateSamplesResponse,
    DeleteAccountOrganizationsRequest,
    DeleteAccountOrganizationsResponse, DeleteAccountProfilesRequest,
    DeleteAccountProfilesResponse, DeleteAccountsRequest, DeleteAccountsResponse,
    DeleteAddressesRequest, DeleteAddressesResponse, DeleteAppFirewallsRequest,
    DeleteAppFirewallsResponse, DeleteContactEmailsRequest,
    DeleteContactEmailsResponse, DeleteContactPhoneNumbersRequest, DeleteContactPhoneNumbersResponse, DeleteContactsRequest,
    DeleteContactsResponse, DeleteDevicesRequest, DeleteDevicesResponse, DeleteExternalContactsRequest, DeleteExternalContactsResponse,
    DeleteIpInfosRequest, DeleteIpInfosResponse, DeleteOrganizationAccountsRequest,
    DeleteOrganizationAccountsResponse, DeleteOrganizationContactsRequest,
    DeleteOrganizationContactsResponse, DeleteOrganizationsRequest, DeleteOrganizationsResponse,
    DeletePostgresChannelsRequest,
    DeletePostgresChannelsResponse,
    DeleteSamplesRequest, DeleteSamplesResponse, Devices, ExternalContacts, GetAccountOrganizationsRequest,
    GetAccountOrganizationsResponse, GetAccountProfilesRequest, GetAccountProfilesResponse,
    GetAccountsRequest, GetAccountsResponse, GetAddressesRequest, GetAddressesResponse,
    GetAppFirewallsRequest, GetAppFirewallsResponse,
    GetContactEmailsRequest, GetContactEmailsResponse, GetContactPhoneNumbersRequest,
    GetContactPhoneNumbersResponse, GetContactsRequest, GetContactsResponse, GetDevicesRequest, GetDevicesResponse,
    GetExternalContactsRequest, GetExternalContactsResponse, GetIpInfosRequest, GetIpInfosResponse,
    GetOrganizationAccountsRequest, GetOrganizationAccountsResponse,
    GetOrganizationContactsRequest, GetOrganizationContactsResponse, GetOrganizationsRequest,
    GetOrganizationsResponse,
    GetPostgresChannelsRequest, GetPostgresChannelsResponse,
    GetSamplesRequest, GetSamplesResponse, IpInfos, OrganizationAccounts, OrganizationContacts, Organizations,
    PostgresChannels, Samples,
    UpdateAccountOrganizationsRequest, UpdateAccountOrganizationsResponse,
    UpdateAccountProfilesRequest, UpdateAccountProfilesResponse, UpdateAccountsRequest,
    UpdateAccountsResponse, UpdateAddressesRequest, UpdateAddressesResponse,
    UpdateContactEmailsRequest, UpdateContactEmailsResponse, UpdateContactPhoneNumbersRequest,
    UpdateContactPhoneNumbersResponse, UpdateContactsRequest, UpdateContactsResponse,
    UpdateDevicesRequest,
    UpdateDevicesResponse, UpdateExternalContactsRequest, UpdateExternalContactsResponse,
    UpdateIpInfosRequest, UpdateIpInfosResponse, UpdateOrganizationAccountsRequest,
    UpdateOrganizationAccountsResponse, UpdateOrganizationContactsRequest,
    UpdateOrganizationContactsResponse, UpdateOrganizationsRequest, UpdateOrganizationsResponse,
    UpdatePostgresChannelsRequest,
    UpdatePostgresChannelsResponse,
    UpdateSamplesRequest, UpdateSamplesResponse,
    UpsertAccountOrganizationsRequest,
    UpsertAccountOrganizationsResponse, UpsertAccountProfilesRequest,
    UpsertAccountProfilesResponse, UpsertAccountsRequest, UpsertAccountsResponse,
    UpsertAddressesRequest, UpsertAddressesResponse, UpsertAppFirewallsRequest,
    UpsertContactEmailsRequest,
    UpsertContactEmailsResponse, UpsertContactPhoneNumbersRequest,
    UpsertContactPhoneNumbersResponse, UpsertContactsRequest, UpsertContactsResponse,
    UpsertDevicesRequest,
    UpsertDevicesResponse, UpsertExternalContactsRequest, UpsertExternalContactsResponse,
    UpsertIpInfosRequest, UpsertIpInfosResponse, UpsertOrganizationAccountsRequest,
    UpsertOrganizationAccountsResponse, UpsertOrganizationContactsRequest,
    UpsertOrganizationContactsResponse, UpsertOrganizationsRequest, UpsertOrganizationsResponse,
    UpsertPostgresChannelsRequest,
    UpsertPostgresChannelsResponse,
    UpsertSamplesRequest, UpsertSamplesResponse,
};
use crate::{
    generate_aggregation_filter_method, generate_batch_delete_method, generate_batch_insert_method,
    generate_batch_update_method, generate_create_method, generate_delete_method,
    generate_get_method, generate_update_method, generate_upsert_method,
};
pub struct GrpcController {}

impl GrpcController {
    pub fn new() -> Self {
        GrpcController {}
    }

    pub async fn init(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
        let addr: SocketAddr = addr.parse()?;
        let grpc_controller = GrpcController::new();
        info!("gRPC Server listening on {}", addr);
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
                interceptor_chain,
            ))
            .serve(addr)
            .await?;
        Ok(())
    }
}

#[tonic::async_trait]
impl StoreService for GrpcController {
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
    // CRUD methods for ip_infos
    generate_create_method!(ip_infos);
    generate_update_method!(ip_infos, ip_info);
    generate_batch_insert_method!(ip_infos);
    generate_batch_update_method!(ip_infos);
    generate_get_method!(ip_infos);
    generate_delete_method!(ip_infos);
    generate_batch_delete_method!(ip_infos);
    generate_upsert_method!(ip_infos);
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
    // Aggregation filter method
    generate_aggregation_filter_method!();
}
