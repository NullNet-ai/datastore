use super::common_controller::{
    convert_json_to_csv, execute_copy, perform_batch_update, perform_upsert,
    process_and_insert_record, process_and_update_record, process_record_for_insert,
    process_record_for_update, process_records, sanitize_updates,
};
use crate::db::create_connection;
use crate::generated::store::store_service_server::{StoreService, StoreServiceServer};
use crate::generated::store::{
    AccountOrganizations, AccountProfiles, Accounts, Addresses, AppFirewalls, AppguardLogs,
    BatchDeleteAccountOrganizationsRequest, BatchDeleteAccountOrganizationsResponse,
    BatchDeleteAccountProfilesRequest, BatchDeleteAccountProfilesResponse,
    BatchDeleteAccountsRequest, BatchDeleteAccountsResponse, BatchDeleteAddressesRequest,
    BatchDeleteAddressesResponse, BatchDeleteAppFirewallsRequest, BatchDeleteAppFirewallsResponse,
    BatchDeleteAppguardLogsRequest, BatchDeleteAppguardLogsResponse, BatchDeleteConnectionsRequest,
    BatchDeleteConnectionsResponse, BatchDeleteContactEmailsRequest,
    BatchDeleteContactEmailsResponse, BatchDeleteContactPhoneNumbersRequest,
    BatchDeleteContactPhoneNumbersResponse, BatchDeleteContactsRequest,
    BatchDeleteContactsResponse, BatchDeleteDeviceAliasesRequest, BatchDeleteDeviceAliasesResponse,
    BatchDeleteDeviceConfigurationsRequest, BatchDeleteDeviceConfigurationsResponse,
    BatchDeleteDeviceGroupSettingsRequest, BatchDeleteDeviceGroupSettingsResponse,
    BatchDeleteDeviceInterfaceAddressesRequest, BatchDeleteDeviceInterfaceAddressesResponse,
    BatchDeleteDeviceInterfacesRequest, BatchDeleteDeviceInterfacesResponse,
    BatchDeleteDeviceRemoteAccessSessionsRequest, BatchDeleteDeviceRemoteAccessSessionsResponse,
    BatchDeleteDeviceRulesRequest, BatchDeleteDeviceRulesResponse, BatchDeleteDeviceSshKeysRequest,
    BatchDeleteDeviceSshKeysResponse, BatchDeleteDevicesRequest, BatchDeleteDevicesResponse,
    BatchDeleteExternalContactsRequest, BatchDeleteExternalContactsResponse,
    BatchDeleteIpInfosRequest, BatchDeleteIpInfosResponse, BatchDeleteOrganizationAccountsRequest,
    BatchDeleteOrganizationAccountsResponse, BatchDeleteOrganizationContactsRequest,
    BatchDeleteOrganizationContactsResponse, BatchDeleteOrganizationsRequest,
    BatchDeleteOrganizationsResponse, BatchDeletePacketsRequest, BatchDeletePacketsResponse,
    BatchDeleteResolutionsRequest, BatchDeleteResolutionsResponse,
    BatchDeleteTempAppguardLogsRequest, BatchDeleteTempAppguardLogsResponse,
    BatchDeleteTempConnectionsRequest, BatchDeleteTempConnectionsResponse,
    BatchDeleteTempDeviceAliasesRequest, BatchDeleteTempDeviceAliasesResponse,
    BatchDeleteTempDeviceInterfaceAddressesRequest,
    BatchDeleteTempDeviceInterfaceAddressesResponse, BatchDeleteTempDeviceInterfacesRequest,
    BatchDeleteTempDeviceInterfacesResponse, BatchDeleteTempDeviceRemoteAccessSessionsRequest,
    BatchDeleteTempDeviceRemoteAccessSessionsResponse, BatchDeleteTempDeviceRulesRequest,
    BatchDeleteTempDeviceRulesResponse, BatchDeleteTempPacketsRequest,
    BatchDeleteTempPacketsResponse, BatchDeleteTempWallguardLogsRequest,
    BatchDeleteTempWallguardLogsResponse, BatchDeleteWallguardLogsRequest,
    BatchDeleteWallguardLogsResponse, BatchInsertAccountOrganizationsRequest,
    BatchInsertAccountOrganizationsResponse, BatchInsertAccountProfilesRequest,
    BatchInsertAccountProfilesResponse, BatchInsertAccountsRequest, BatchInsertAccountsResponse,
    BatchInsertAddressesRequest, BatchInsertAddressesResponse, BatchInsertAppFirewallsRequest,
    BatchInsertAppFirewallsResponse, BatchInsertAppguardLogsRequest,
    BatchInsertAppguardLogsResponse, BatchInsertConnectionsRequest, BatchInsertConnectionsResponse,
    BatchInsertContactEmailsRequest, BatchInsertContactEmailsResponse,
    BatchInsertContactPhoneNumbersRequest, BatchInsertContactPhoneNumbersResponse,
    BatchInsertContactsRequest, BatchInsertContactsResponse, BatchInsertDeviceAliasesRequest,
    BatchInsertDeviceAliasesResponse, BatchInsertDeviceConfigurationsRequest,
    BatchInsertDeviceConfigurationsResponse, BatchInsertDeviceGroupSettingsRequest,
    BatchInsertDeviceGroupSettingsResponse, BatchInsertDeviceInterfaceAddressesRequest,
    BatchInsertDeviceInterfaceAddressesResponse, BatchInsertDeviceInterfacesRequest,
    BatchInsertDeviceInterfacesResponse, BatchInsertDeviceRemoteAccessSessionsRequest,
    BatchInsertDeviceRemoteAccessSessionsResponse, BatchInsertDeviceRulesRequest,
    BatchInsertDeviceRulesResponse, BatchInsertDeviceSshKeysRequest,
    BatchInsertDeviceSshKeysResponse, BatchInsertDevicesRequest, BatchInsertDevicesResponse,
    BatchInsertExternalContactsRequest, BatchInsertExternalContactsResponse,
    BatchInsertIpInfosRequest, BatchInsertIpInfosResponse, BatchInsertOrganizationAccountsRequest,
    BatchInsertOrganizationAccountsResponse, BatchInsertOrganizationContactsRequest,
    BatchInsertOrganizationContactsResponse, BatchInsertOrganizationsRequest,
    BatchInsertOrganizationsResponse, BatchInsertPacketsRequest, BatchInsertPacketsResponse,
    BatchInsertResolutionsRequest, BatchInsertResolutionsResponse,
    BatchInsertTempAppguardLogsRequest, BatchInsertTempAppguardLogsResponse,
    BatchInsertTempConnectionsRequest, BatchInsertTempConnectionsResponse,
    BatchInsertTempDeviceAliasesRequest, BatchInsertTempDeviceAliasesResponse,
    BatchInsertTempDeviceInterfaceAddressesRequest,
    BatchInsertTempDeviceInterfaceAddressesResponse, BatchInsertTempDeviceInterfacesRequest,
    BatchInsertTempDeviceInterfacesResponse, BatchInsertTempDeviceRemoteAccessSessionsRequest,
    BatchInsertTempDeviceRemoteAccessSessionsResponse, BatchInsertTempDeviceRulesRequest,
    BatchInsertTempDeviceRulesResponse, BatchInsertTempPacketsRequest,
    BatchInsertTempPacketsResponse, BatchInsertTempWallguardLogsRequest,
    BatchInsertTempWallguardLogsResponse, BatchInsertWallguardLogsRequest,
    BatchInsertWallguardLogsResponse, BatchUpdateAccountOrganizationsRequest,
    BatchUpdateAccountOrganizationsResponse, BatchUpdateAccountProfilesRequest,
    BatchUpdateAccountProfilesResponse, BatchUpdateAccountsRequest, BatchUpdateAccountsResponse,
    BatchUpdateAddressesRequest, BatchUpdateAddressesResponse, BatchUpdateAppFirewallsRequest,
    BatchUpdateAppFirewallsResponse, BatchUpdateAppguardLogsRequest,
    BatchUpdateAppguardLogsResponse, BatchUpdateConnectionsRequest, BatchUpdateConnectionsResponse,
    BatchUpdateContactEmailsRequest, BatchUpdateContactEmailsResponse,
    BatchUpdateContactPhoneNumbersRequest, BatchUpdateContactPhoneNumbersResponse,
    BatchUpdateContactsRequest, BatchUpdateContactsResponse, BatchUpdateDeviceAliasesRequest,
    BatchUpdateDeviceAliasesResponse, BatchUpdateDeviceConfigurationsRequest,
    BatchUpdateDeviceConfigurationsResponse, BatchUpdateDeviceGroupSettingsRequest,
    BatchUpdateDeviceGroupSettingsResponse, BatchUpdateDeviceInterfaceAddressesRequest,
    BatchUpdateDeviceInterfaceAddressesResponse, BatchUpdateDeviceInterfacesRequest,
    BatchUpdateDeviceInterfacesResponse, BatchUpdateDeviceRemoteAccessSessionsRequest,
    BatchUpdateDeviceRemoteAccessSessionsResponse, BatchUpdateDeviceRulesRequest,
    BatchUpdateDeviceRulesResponse, BatchUpdateDeviceSshKeysRequest,
    BatchUpdateDeviceSshKeysResponse, BatchUpdateDevicesRequest, BatchUpdateDevicesResponse,
    BatchUpdateExternalContactsRequest, BatchUpdateExternalContactsResponse,
    BatchUpdateIpInfosRequest, BatchUpdateIpInfosResponse, BatchUpdateOrganizationAccountsRequest,
    BatchUpdateOrganizationAccountsResponse, BatchUpdateOrganizationContactsRequest,
    BatchUpdateOrganizationContactsResponse, BatchUpdateOrganizationsRequest,
    BatchUpdateOrganizationsResponse, BatchUpdatePacketsRequest, BatchUpdatePacketsResponse,
    BatchUpdateResolutionsRequest, BatchUpdateResolutionsResponse,
    BatchUpdateTempAppguardLogsRequest, BatchUpdateTempAppguardLogsResponse,
    BatchUpdateTempConnectionsRequest, BatchUpdateTempConnectionsResponse,
    BatchUpdateTempDeviceAliasesRequest, BatchUpdateTempDeviceAliasesResponse,
    BatchUpdateTempDeviceInterfaceAddressesRequest,
    BatchUpdateTempDeviceInterfaceAddressesResponse, BatchUpdateTempDeviceInterfacesRequest,
    BatchUpdateTempDeviceInterfacesResponse, BatchUpdateTempDeviceRemoteAccessSessionsRequest,
    BatchUpdateTempDeviceRemoteAccessSessionsResponse, BatchUpdateTempDeviceRulesRequest,
    BatchUpdateTempDeviceRulesResponse, BatchUpdateTempPacketsRequest,
    BatchUpdateTempPacketsResponse, BatchUpdateTempWallguardLogsRequest,
    BatchUpdateTempWallguardLogsResponse, BatchUpdateWallguardLogsRequest,
    BatchUpdateWallguardLogsResponse, Connections, ContactEmails, ContactPhoneNumbers, Contacts,
    CreateAccountOrganizationsRequest, CreateAccountOrganizationsResponse,
    CreateAccountProfilesRequest, CreateAccountProfilesResponse, CreateAccountsRequest,
    CreateAccountsResponse, CreateAddressesRequest, CreateAddressesResponse,
    CreateAppFirewallsRequest, CreateAppFirewallsResponse, CreateAppguardLogsRequest,
    CreateAppguardLogsResponse, CreateConnectionsRequest, CreateConnectionsResponse,
    CreateContactEmailsRequest, CreateContactEmailsResponse, CreateContactPhoneNumbersRequest,
    CreateContactPhoneNumbersResponse, CreateContactsRequest, CreateContactsResponse,
    CreateDeviceAliasesRequest, CreateDeviceAliasesResponse, CreateDeviceConfigurationsRequest,
    CreateDeviceConfigurationsResponse, CreateDeviceGroupSettingsRequest,
    CreateDeviceGroupSettingsResponse, CreateDeviceInterfaceAddressesRequest,
    CreateDeviceInterfaceAddressesResponse, CreateDeviceInterfacesRequest,
    CreateDeviceInterfacesResponse, CreateDeviceRemoteAccessSessionsRequest,
    CreateDeviceRemoteAccessSessionsResponse, CreateDeviceRulesRequest, CreateDeviceRulesResponse,
    CreateDeviceSshKeysRequest, CreateDeviceSshKeysResponse, CreateDevicesRequest,
    CreateDevicesResponse, CreateExternalContactsRequest, CreateExternalContactsResponse,
    CreateIpInfosRequest, CreateIpInfosResponse, CreateOrganizationAccountsRequest,
    CreateOrganizationAccountsResponse, CreateOrganizationContactsRequest,
    CreateOrganizationContactsResponse, CreateOrganizationsRequest, CreateOrganizationsResponse,
    CreatePacketsRequest, CreatePacketsResponse, CreateResolutionsRequest,
    CreateResolutionsResponse, CreateTempAppguardLogsRequest, CreateTempAppguardLogsResponse,
    CreateTempConnectionsRequest, CreateTempConnectionsResponse, CreateTempDeviceAliasesRequest,
    CreateTempDeviceAliasesResponse, CreateTempDeviceInterfaceAddressesRequest,
    CreateTempDeviceInterfaceAddressesResponse, CreateTempDeviceInterfacesRequest,
    CreateTempDeviceInterfacesResponse, CreateTempDeviceRemoteAccessSessionsRequest,
    CreateTempDeviceRemoteAccessSessionsResponse, CreateTempDeviceRulesRequest,
    CreateTempDeviceRulesResponse, CreateTempPacketsRequest, CreateTempPacketsResponse,
    CreateTempWallguardLogsRequest, CreateTempWallguardLogsResponse, CreateWallguardLogsRequest,
    CreateWallguardLogsResponse, DeleteAccountOrganizationsRequest,
    DeleteAccountOrganizationsResponse, DeleteAccountProfilesRequest,
    DeleteAccountProfilesResponse, DeleteAccountsRequest, DeleteAccountsResponse,
    DeleteAddressesRequest, DeleteAddressesResponse, DeleteAppFirewallsRequest,
    DeleteAppFirewallsResponse, DeleteAppguardLogsRequest, DeleteAppguardLogsResponse,
    DeleteConnectionsRequest, DeleteConnectionsResponse, DeleteContactEmailsRequest,
    DeleteContactEmailsResponse, DeleteContactPhoneNumbersRequest,
    DeleteContactPhoneNumbersResponse, DeleteContactsRequest, DeleteContactsResponse,
    DeleteDeviceAliasesRequest, DeleteDeviceAliasesResponse, DeleteDeviceConfigurationsRequest,
    DeleteDeviceConfigurationsResponse, DeleteDeviceGroupSettingsRequest,
    DeleteDeviceGroupSettingsResponse, DeleteDeviceInterfaceAddressesRequest,
    DeleteDeviceInterfaceAddressesResponse, DeleteDeviceInterfacesRequest,
    DeleteDeviceInterfacesResponse, DeleteDeviceRemoteAccessSessionsRequest,
    DeleteDeviceRemoteAccessSessionsResponse, DeleteDeviceRulesRequest, DeleteDeviceRulesResponse,
    DeleteDeviceSshKeysRequest, DeleteDeviceSshKeysResponse, DeleteDevicesRequest,
    DeleteDevicesResponse, DeleteExternalContactsRequest, DeleteExternalContactsResponse,
    DeleteIpInfosRequest, DeleteIpInfosResponse, DeleteOrganizationAccountsRequest,
    DeleteOrganizationAccountsResponse, DeleteOrganizationContactsRequest,
    DeleteOrganizationContactsResponse, DeleteOrganizationsRequest, DeleteOrganizationsResponse,
    DeletePacketsRequest, DeletePacketsResponse, DeleteResolutionsRequest,
    DeleteResolutionsResponse, DeleteTempAppguardLogsRequest, DeleteTempAppguardLogsResponse,
    DeleteTempConnectionsRequest, DeleteTempConnectionsResponse, DeleteTempDeviceAliasesRequest,
    DeleteTempDeviceAliasesResponse, DeleteTempDeviceInterfaceAddressesRequest,
    DeleteTempDeviceInterfaceAddressesResponse, DeleteTempDeviceInterfacesRequest,
    DeleteTempDeviceInterfacesResponse, DeleteTempDeviceRemoteAccessSessionsRequest,
    DeleteTempDeviceRemoteAccessSessionsResponse, DeleteTempDeviceRulesRequest,
    DeleteTempDeviceRulesResponse, DeleteTempPacketsRequest, DeleteTempPacketsResponse,
    DeleteTempWallguardLogsRequest, DeleteTempWallguardLogsResponse, DeleteWallguardLogsRequest,
    DeleteWallguardLogsResponse, DeviceAliases, DeviceConfigurations, DeviceGroupSettings,
    DeviceInterfaceAddresses, DeviceInterfaces, DeviceRemoteAccessSessions, DeviceRules,
    DeviceSshKeys, Devices, ExternalContacts, GetAccountOrganizationsRequest,
    GetAccountOrganizationsResponse, GetAccountProfilesRequest, GetAccountProfilesResponse,
    GetAccountsRequest, GetAccountsResponse, GetAddressesRequest, GetAddressesResponse,
    GetAppFirewallsRequest, GetAppFirewallsResponse, GetAppguardLogsRequest,
    GetAppguardLogsResponse, GetConnectionsRequest, GetConnectionsResponse,
    GetContactEmailsRequest, GetContactEmailsResponse, GetContactPhoneNumbersRequest,
    GetContactPhoneNumbersResponse, GetContactsRequest, GetContactsResponse,
    GetDeviceAliasesRequest, GetDeviceAliasesResponse, GetDeviceConfigurationsRequest,
    GetDeviceConfigurationsResponse, GetDeviceGroupSettingsRequest, GetDeviceGroupSettingsResponse,
    GetDeviceInterfaceAddressesRequest, GetDeviceInterfaceAddressesResponse,
    GetDeviceInterfacesRequest, GetDeviceInterfacesResponse, GetDeviceRemoteAccessSessionsRequest,
    GetDeviceRemoteAccessSessionsResponse, GetDeviceRulesRequest, GetDeviceRulesResponse,
    GetDeviceSshKeysRequest, GetDeviceSshKeysResponse, GetDevicesRequest, GetDevicesResponse,
    GetExternalContactsRequest, GetExternalContactsResponse, GetIpInfosRequest, GetIpInfosResponse,
    GetOrganizationAccountsRequest, GetOrganizationAccountsResponse,
    GetOrganizationContactsRequest, GetOrganizationContactsResponse, GetOrganizationsRequest,
    GetOrganizationsResponse, GetPacketsRequest, GetPacketsResponse, GetResolutionsRequest,
    GetResolutionsResponse, GetTempAppguardLogsRequest, GetTempAppguardLogsResponse,
    GetTempConnectionsRequest, GetTempConnectionsResponse, GetTempDeviceAliasesRequest,
    GetTempDeviceAliasesResponse, GetTempDeviceInterfaceAddressesRequest,
    GetTempDeviceInterfaceAddressesResponse, GetTempDeviceInterfacesRequest,
    GetTempDeviceInterfacesResponse, GetTempDeviceRemoteAccessSessionsRequest,
    GetTempDeviceRemoteAccessSessionsResponse, GetTempDeviceRulesRequest,
    GetTempDeviceRulesResponse, GetTempPacketsRequest, GetTempPacketsResponse,
    GetTempWallguardLogsRequest, GetTempWallguardLogsResponse, GetWallguardLogsRequest,
    GetWallguardLogsResponse, IpInfos, OrganizationAccounts, OrganizationContacts, Organizations,
    Packets, Resolutions, TempAppguardLogs, TempConnections, TempDeviceAliases,
    TempDeviceInterfaceAddresses, TempDeviceInterfaces, TempDeviceRemoteAccessSessions,
    TempDeviceRules, TempPackets, TempWallguardLogs, UpdateAccountOrganizationsRequest,
    UpdateAccountOrganizationsResponse, UpdateAccountProfilesRequest,
    UpdateAccountProfilesResponse, UpdateAccountsRequest, UpdateAccountsResponse,
    UpdateAddressesRequest, UpdateAddressesResponse, UpdateAppFirewallsRequest,
    UpdateAppFirewallsResponse, UpdateAppguardLogsRequest, UpdateAppguardLogsResponse,
    UpdateConnectionsRequest, UpdateConnectionsResponse, UpdateContactEmailsRequest,
    UpdateContactEmailsResponse, UpdateContactPhoneNumbersRequest,
    UpdateContactPhoneNumbersResponse, UpdateContactsRequest, UpdateContactsResponse,
    UpdateDeviceAliasesRequest, UpdateDeviceAliasesResponse, UpdateDeviceConfigurationsRequest,
    UpdateDeviceConfigurationsResponse, UpdateDeviceGroupSettingsRequest,
    UpdateDeviceGroupSettingsResponse, UpdateDeviceInterfaceAddressesRequest,
    UpdateDeviceInterfaceAddressesResponse, UpdateDeviceInterfacesRequest,
    UpdateDeviceInterfacesResponse, UpdateDeviceRemoteAccessSessionsRequest,
    UpdateDeviceRemoteAccessSessionsResponse, UpdateDeviceRulesRequest, UpdateDeviceRulesResponse,
    UpdateDeviceSshKeysRequest, UpdateDeviceSshKeysResponse, UpdateDevicesRequest,
    UpdateDevicesResponse, UpdateExternalContactsRequest, UpdateExternalContactsResponse,
    UpdateIpInfosRequest, UpdateIpInfosResponse, UpdateOrganizationAccountsRequest,
    UpdateOrganizationAccountsResponse, UpdateOrganizationContactsRequest,
    UpdateOrganizationContactsResponse, UpdateOrganizationsRequest, UpdateOrganizationsResponse,
    UpdatePacketsRequest, UpdatePacketsResponse, UpdateResolutionsRequest,
    UpdateResolutionsResponse, UpdateTempAppguardLogsRequest, UpdateTempAppguardLogsResponse,
    UpdateTempConnectionsRequest, UpdateTempConnectionsResponse, UpdateTempDeviceAliasesRequest,
    UpdateTempDeviceAliasesResponse, UpdateTempDeviceInterfaceAddressesRequest,
    UpdateTempDeviceInterfaceAddressesResponse, UpdateTempDeviceInterfacesRequest,
    UpdateTempDeviceInterfacesResponse, UpdateTempDeviceRemoteAccessSessionsRequest,
    UpdateTempDeviceRemoteAccessSessionsResponse, UpdateTempDeviceRulesRequest,
    UpdateTempDeviceRulesResponse, UpdateTempPacketsRequest, UpdateTempPacketsResponse,
    UpdateTempWallguardLogsRequest, UpdateTempWallguardLogsResponse, UpdateWallguardLogsRequest,
    UpdateWallguardLogsResponse, UpsertAccountOrganizationsRequest,
    UpsertAccountOrganizationsResponse, UpsertAccountProfilesRequest,
    UpsertAccountProfilesResponse, UpsertAccountsRequest, UpsertAccountsResponse,
    UpsertAddressesRequest, UpsertAddressesResponse, UpsertAppFirewallsRequest,
    UpsertAppFirewallsResponse, UpsertAppguardLogsRequest, UpsertAppguardLogsResponse,
    UpsertConnectionsRequest, UpsertConnectionsResponse, UpsertContactEmailsRequest,
    UpsertContactEmailsResponse, UpsertContactPhoneNumbersRequest,
    UpsertContactPhoneNumbersResponse, UpsertContactsRequest, UpsertContactsResponse,
    UpsertDeviceAliasesRequest, UpsertDeviceAliasesResponse, UpsertDeviceConfigurationsRequest,
    UpsertDeviceConfigurationsResponse, UpsertDeviceGroupSettingsRequest,
    UpsertDeviceGroupSettingsResponse, UpsertDeviceInterfaceAddressesRequest,
    UpsertDeviceInterfaceAddressesResponse, UpsertDeviceInterfacesRequest,
    UpsertDeviceInterfacesResponse, UpsertDeviceRemoteAccessSessionsRequest,
    UpsertDeviceRemoteAccessSessionsResponse, UpsertDeviceRulesRequest, UpsertDeviceRulesResponse,
    UpsertDeviceSshKeysRequest, UpsertDeviceSshKeysResponse, UpsertDevicesRequest,
    UpsertDevicesResponse, UpsertExternalContactsRequest, UpsertExternalContactsResponse,
    UpsertIpInfosRequest, UpsertIpInfosResponse, UpsertOrganizationAccountsRequest,
    UpsertOrganizationAccountsResponse, UpsertOrganizationContactsRequest,
    UpsertOrganizationContactsResponse, UpsertOrganizationsRequest, UpsertOrganizationsResponse,
    UpsertPacketsRequest, UpsertPacketsResponse, UpsertResolutionsRequest,
    UpsertResolutionsResponse, UpsertTempAppguardLogsRequest, UpsertTempAppguardLogsResponse,
    UpsertTempConnectionsRequest, UpsertTempConnectionsResponse, UpsertTempDeviceAliasesRequest,
    UpsertTempDeviceAliasesResponse, UpsertTempDeviceInterfaceAddressesRequest,
    UpsertTempDeviceInterfaceAddressesResponse, UpsertTempDeviceInterfacesRequest,
    UpsertTempDeviceInterfacesResponse, UpsertTempDeviceRemoteAccessSessionsRequest,
    UpsertTempDeviceRemoteAccessSessionsResponse, UpsertTempDeviceRulesRequest,
    UpsertTempDeviceRulesResponse, UpsertTempPacketsRequest, UpsertTempPacketsResponse,
    UpsertTempWallguardLogsRequest, UpsertTempWallguardLogsResponse, UpsertWallguardLogsRequest,
    UpsertWallguardLogsResponse, WallguardLogs,
};
use crate::middlewares::auth_middleware::GrpcAuthInterceptor;
use crate::middlewares::interceptor_chain::InterceptorChain;
use crate::middlewares::shutdown_middleware::GrpcShutdownInterceptor;
use crate::structs::structs::Auth;
use crate::structs::structs::RequestBody;
use crate::sync::sync_service::{insert, update};
use crate::table_enum::Table;
use crate::utils::utils::table_exists;
use crate::{
    generate_batch_delete_method, generate_batch_insert_method, generate_batch_update_method,
    generate_create_method, generate_delete_method, generate_get_method, generate_update_method,
    generate_upsert_method,
};
use actix_web::{web, HttpResponse, Responder};
use serde_json::Value;
use std::net::SocketAddr;
use std::pin::Pin;
use tonic::{transport::Server, Request, Response, Status};
pub struct GrpcController {}

impl GrpcController {
    pub fn new() -> Self {
        GrpcController {}
    }

    pub async fn init(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
        let addr: SocketAddr = addr.parse()?;
        let grpc_controller = GrpcController::new();
        println!("gRPC Server listening on {}", addr);
        // Create a chain of interceptors
        let auth_interceptor = GrpcAuthInterceptor;
        let shutdown_interceptor = GrpcShutdownInterceptor;
        let interceptor_chain = InterceptorChain::new(shutdown_interceptor, auth_interceptor);
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
    // CRUD methods for app_firewalls
    generate_create_method!(app_firewalls);
    generate_update_method!(app_firewalls, app_firewall);
    generate_batch_insert_method!(app_firewalls);
    generate_batch_update_method!(app_firewalls);
    generate_get_method!(app_firewalls);
    generate_delete_method!(app_firewalls);
    generate_batch_delete_method!(app_firewalls);
    generate_upsert_method!(app_firewalls);
    // CRUD methods for appguard_logs
    generate_create_method!(appguard_logs);
    generate_update_method!(appguard_logs, appguard_log);
    generate_batch_insert_method!(appguard_logs);
    generate_batch_update_method!(appguard_logs);
    generate_get_method!(appguard_logs);
    generate_delete_method!(appguard_logs);
    generate_batch_delete_method!(appguard_logs);
    generate_upsert_method!(appguard_logs);
    // CRUD methods for temp_appguard_logs
    generate_create_method!(temp_appguard_logs);
    generate_update_method!(temp_appguard_logs, temp_appguard_log);
    generate_batch_insert_method!(temp_appguard_logs);
    generate_batch_update_method!(temp_appguard_logs);
    generate_get_method!(temp_appguard_logs);
    generate_delete_method!(temp_appguard_logs);
    generate_batch_delete_method!(temp_appguard_logs);
    generate_upsert_method!(temp_appguard_logs);
    // CRUD methods for device_aliases
    generate_create_method!(device_aliases);
    generate_update_method!(device_aliases, device_alias);
    generate_batch_insert_method!(device_aliases);
    generate_batch_update_method!(device_aliases);
    generate_get_method!(device_aliases);
    generate_delete_method!(device_aliases);
    generate_batch_delete_method!(device_aliases);
    generate_upsert_method!(device_aliases);
    // CRUD methods for temp_device_aliases
    generate_create_method!(temp_device_aliases);
    generate_update_method!(temp_device_aliases, temp_device_alias);
    generate_batch_insert_method!(temp_device_aliases);
    generate_batch_update_method!(temp_device_aliases);
    generate_get_method!(temp_device_aliases);
    generate_delete_method!(temp_device_aliases);
    generate_batch_delete_method!(temp_device_aliases);
    generate_upsert_method!(temp_device_aliases);
    // CRUD methods for device_configurations
    generate_create_method!(device_configurations);
    generate_update_method!(device_configurations, device_configuration);
    generate_batch_insert_method!(device_configurations);
    generate_batch_update_method!(device_configurations);
    generate_get_method!(device_configurations);
    generate_delete_method!(device_configurations);
    generate_batch_delete_method!(device_configurations);
    generate_upsert_method!(device_configurations);
    // CRUD methods for device_interface_addresses
    generate_create_method!(device_interface_addresses);
    generate_update_method!(device_interface_addresses, device_interface_address);
    generate_batch_insert_method!(device_interface_addresses);
    generate_batch_update_method!(device_interface_addresses);
    generate_get_method!(device_interface_addresses);
    generate_delete_method!(device_interface_addresses);
    generate_batch_delete_method!(device_interface_addresses);
    generate_upsert_method!(device_interface_addresses);
    // CRUD methods for temp_device_interface_addresses
    generate_create_method!(temp_device_interface_addresses);
    generate_update_method!(
        temp_device_interface_addresses,
        temp_device_interface_address
    );
    generate_batch_insert_method!(temp_device_interface_addresses);
    generate_batch_update_method!(temp_device_interface_addresses);
    generate_get_method!(temp_device_interface_addresses);
    generate_delete_method!(temp_device_interface_addresses);
    generate_batch_delete_method!(temp_device_interface_addresses);
    generate_upsert_method!(temp_device_interface_addresses);
    // CRUD methods for device_interfaces
    generate_create_method!(device_interfaces);
    generate_update_method!(device_interfaces, device_interface);
    generate_batch_insert_method!(device_interfaces);
    generate_batch_update_method!(device_interfaces);
    generate_get_method!(device_interfaces);
    generate_delete_method!(device_interfaces);
    generate_batch_delete_method!(device_interfaces);
    generate_upsert_method!(device_interfaces);
    // CRUD methods for temp_device_interfaces
    generate_create_method!(temp_device_interfaces);
    generate_update_method!(temp_device_interfaces, temp_device_interface);
    generate_batch_insert_method!(temp_device_interfaces);
    generate_batch_update_method!(temp_device_interfaces);
    generate_get_method!(temp_device_interfaces);
    generate_delete_method!(temp_device_interfaces);
    generate_batch_delete_method!(temp_device_interfaces);
    generate_upsert_method!(temp_device_interfaces);
    // CRUD methods for device_remote_access_sessions
    generate_create_method!(device_remote_access_sessions);
    generate_update_method!(device_remote_access_sessions, device_remote_access_session);
    generate_batch_insert_method!(device_remote_access_sessions);
    generate_batch_update_method!(device_remote_access_sessions);
    generate_get_method!(device_remote_access_sessions);
    generate_delete_method!(device_remote_access_sessions);
    generate_batch_delete_method!(device_remote_access_sessions);
    generate_upsert_method!(device_remote_access_sessions);
    // CRUD methods for temp_device_remote_access_sessions
    generate_create_method!(temp_device_remote_access_sessions);
    generate_update_method!(
        temp_device_remote_access_sessions,
        temp_device_remote_access_session
    );
    generate_batch_insert_method!(temp_device_remote_access_sessions);
    generate_batch_update_method!(temp_device_remote_access_sessions);
    generate_get_method!(temp_device_remote_access_sessions);
    generate_delete_method!(temp_device_remote_access_sessions);
    generate_batch_delete_method!(temp_device_remote_access_sessions);
    generate_upsert_method!(temp_device_remote_access_sessions);
    // CRUD methods for device_rules
    generate_create_method!(device_rules);
    generate_update_method!(device_rules, device_rule);
    generate_batch_insert_method!(device_rules);
    generate_batch_update_method!(device_rules);
    generate_get_method!(device_rules);
    generate_delete_method!(device_rules);
    generate_batch_delete_method!(device_rules);
    generate_upsert_method!(device_rules);
    // CRUD methods for temp_device_rules
    generate_create_method!(temp_device_rules);
    generate_update_method!(temp_device_rules, temp_device_rule);
    generate_batch_insert_method!(temp_device_rules);
    generate_batch_update_method!(temp_device_rules);
    generate_get_method!(temp_device_rules);
    generate_delete_method!(temp_device_rules);
    generate_batch_delete_method!(temp_device_rules);
    generate_upsert_method!(temp_device_rules);
    // CRUD methods for packets
    generate_create_method!(packets);
    generate_update_method!(packets, packet);
    generate_batch_insert_method!(packets);
    generate_batch_update_method!(packets);
    generate_get_method!(packets);
    generate_delete_method!(packets);
    generate_batch_delete_method!(packets);
    generate_upsert_method!(packets);
    // CRUD methods for temp_packets
    generate_create_method!(temp_packets);
    generate_update_method!(temp_packets, temp_packet);
    generate_batch_insert_method!(temp_packets);
    generate_batch_update_method!(temp_packets);
    generate_get_method!(temp_packets);
    generate_delete_method!(temp_packets);
    generate_batch_delete_method!(temp_packets);
    generate_upsert_method!(temp_packets);
    // CRUD methods for connections
    generate_create_method!(connections);
    generate_update_method!(connections, connection);
    generate_batch_insert_method!(connections);
    generate_batch_update_method!(connections);
    generate_get_method!(connections);
    generate_delete_method!(connections);
    generate_batch_delete_method!(connections);
    generate_upsert_method!(connections);
    // CRUD methods for temp_connections
    generate_create_method!(temp_connections);
    generate_update_method!(temp_connections, temp_connection);
    generate_batch_insert_method!(temp_connections);
    generate_batch_update_method!(temp_connections);
    generate_get_method!(temp_connections);
    generate_delete_method!(temp_connections);
    generate_batch_delete_method!(temp_connections);
    generate_upsert_method!(temp_connections);
    // CRUD methods for device_ssh_keys
    generate_create_method!(device_ssh_keys);
    generate_update_method!(device_ssh_keys, device_ssh_key);
    generate_batch_insert_method!(device_ssh_keys);
    generate_batch_update_method!(device_ssh_keys);
    generate_get_method!(device_ssh_keys);
    generate_delete_method!(device_ssh_keys);
    generate_batch_delete_method!(device_ssh_keys);
    generate_upsert_method!(device_ssh_keys);
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
    // CRUD methods for resolutions
    generate_create_method!(resolutions);
    generate_update_method!(resolutions, resolution);
    generate_batch_insert_method!(resolutions);
    generate_batch_update_method!(resolutions);
    generate_get_method!(resolutions);
    generate_delete_method!(resolutions);
    generate_batch_delete_method!(resolutions);
    generate_upsert_method!(resolutions);
    // CRUD methods for wallguard_logs
    generate_create_method!(wallguard_logs);
    generate_update_method!(wallguard_logs, wallguard_log);
    generate_batch_insert_method!(wallguard_logs);
    generate_batch_update_method!(wallguard_logs);
    generate_get_method!(wallguard_logs);
    generate_delete_method!(wallguard_logs);
    generate_batch_delete_method!(wallguard_logs);
    generate_upsert_method!(wallguard_logs);
    // CRUD methods for temp_wallguard_logs
    generate_create_method!(temp_wallguard_logs);
    generate_update_method!(temp_wallguard_logs, temp_wallguard_log);
    generate_batch_insert_method!(temp_wallguard_logs);
    generate_batch_update_method!(temp_wallguard_logs);
    generate_get_method!(temp_wallguard_logs);
    generate_delete_method!(temp_wallguard_logs);
    generate_batch_delete_method!(temp_wallguard_logs);
    generate_upsert_method!(temp_wallguard_logs);
    // CRUD methods for device_group_settings
    generate_create_method!(device_group_settings);
    generate_update_method!(device_group_settings, device_group_setting);
    generate_batch_insert_method!(device_group_settings);
    generate_batch_update_method!(device_group_settings);
    generate_get_method!(device_group_settings);
    generate_delete_method!(device_group_settings);
    generate_batch_delete_method!(device_group_settings);
    generate_upsert_method!(device_group_settings);
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
}

// You can add HTTP endpoints to configure or check gRPC status
pub async fn grpc_status() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "running",
        "message": "gRPC server is operational"
    }))
}

// Function to configure and register HTTP routes related to gRPC
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/api/grpc/status").route(web::get().to(grpc_status)));
}
