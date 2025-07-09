use crate::db::create_connection;
use actix_web::{HttpResponse, Responder, web};
use std::pin::Pin;
use std::net::SocketAddr;
use crate::table_enum::Table;
use crate::utils::utils::table_exists;
use crate::sync::sync_service::{insert, update};
use crate::structs::structs::Auth;
use serde_json::Value;
use crate::middlewares::shutdown_middleware::GrpcShutdownInterceptor;
use crate::middlewares::interceptor_chain::InterceptorChain;
use crate::structs::structs::RequestBody;
use tonic::{Request, Response, Status, transport::Server};
use crate::middlewares::auth_middleware::GrpcAuthInterceptor;
use super::common_controller::{perform_batch_update, process_record_for_insert, process_record_for_update, sanitize_updates, convert_json_to_csv, process_records, execute_copy, perform_upsert, process_and_update_record, process_and_insert_record, process_and_get_record_by_id};
use crate::generated::store::store_service_server::{StoreServiceServer, StoreService };
use crate::{ generate_batch_delete_method, generate_batch_insert_method, generate_batch_update_method, generate_create_method, generate_update_method, generate_get_method, generate_delete_method, generate_upsert_method};
use crate::generated::store::{ExternalContacts, CreateExternalContactsRequest, CreateExternalContactsResponse, GetExternalContactsRequest, GetExternalContactsResponse, UpdateExternalContactsRequest, UpdateExternalContactsResponse, DeleteExternalContactsRequest, DeleteExternalContactsResponse, BatchInsertExternalContactsRequest, BatchInsertExternalContactsResponse, BatchUpdateExternalContactsRequest, BatchUpdateExternalContactsResponse, BatchDeleteExternalContactsRequest, BatchDeleteExternalContactsResponse, UpsertExternalContactsRequest, UpsertExternalContactsResponse, Organizations, CreateOrganizationsRequest, CreateOrganizationsResponse, GetOrganizationsRequest, GetOrganizationsResponse, UpdateOrganizationsRequest, UpdateOrganizationsResponse, DeleteOrganizationsRequest, DeleteOrganizationsResponse, BatchInsertOrganizationsRequest, BatchInsertOrganizationsResponse, BatchUpdateOrganizationsRequest, BatchUpdateOrganizationsResponse, BatchDeleteOrganizationsRequest, BatchDeleteOrganizationsResponse, UpsertOrganizationsRequest, UpsertOrganizationsResponse, OrganizationContacts, CreateOrganizationContactsRequest, CreateOrganizationContactsResponse, GetOrganizationContactsRequest, GetOrganizationContactsResponse, UpdateOrganizationContactsRequest, UpdateOrganizationContactsResponse, DeleteOrganizationContactsRequest, DeleteOrganizationContactsResponse, BatchInsertOrganizationContactsRequest, BatchInsertOrganizationContactsResponse, BatchUpdateOrganizationContactsRequest, BatchUpdateOrganizationContactsResponse, BatchDeleteOrganizationContactsRequest, BatchDeleteOrganizationContactsResponse, UpsertOrganizationContactsRequest, UpsertOrganizationContactsResponse, OrganizationAccounts, CreateOrganizationAccountsRequest, CreateOrganizationAccountsResponse, GetOrganizationAccountsRequest, GetOrganizationAccountsResponse, UpdateOrganizationAccountsRequest, UpdateOrganizationAccountsResponse, DeleteOrganizationAccountsRequest, DeleteOrganizationAccountsResponse, BatchInsertOrganizationAccountsRequest, BatchInsertOrganizationAccountsResponse, BatchUpdateOrganizationAccountsRequest, BatchUpdateOrganizationAccountsResponse, BatchDeleteOrganizationAccountsRequest, BatchDeleteOrganizationAccountsResponse, UpsertOrganizationAccountsRequest, UpsertOrganizationAccountsResponse, AccountOrganizations, CreateAccountOrganizationsRequest, CreateAccountOrganizationsResponse, GetAccountOrganizationsRequest, GetAccountOrganizationsResponse, UpdateAccountOrganizationsRequest, UpdateAccountOrganizationsResponse, DeleteAccountOrganizationsRequest, DeleteAccountOrganizationsResponse, BatchInsertAccountOrganizationsRequest, BatchInsertAccountOrganizationsResponse, BatchUpdateAccountOrganizationsRequest, BatchUpdateAccountOrganizationsResponse, BatchDeleteAccountOrganizationsRequest, BatchDeleteAccountOrganizationsResponse, UpsertAccountOrganizationsRequest, UpsertAccountOrganizationsResponse, AccountProfiles, CreateAccountProfilesRequest, CreateAccountProfilesResponse, GetAccountProfilesRequest, GetAccountProfilesResponse, UpdateAccountProfilesRequest, UpdateAccountProfilesResponse, DeleteAccountProfilesRequest, DeleteAccountProfilesResponse, BatchInsertAccountProfilesRequest, BatchInsertAccountProfilesResponse, BatchUpdateAccountProfilesRequest, BatchUpdateAccountProfilesResponse, BatchDeleteAccountProfilesRequest, BatchDeleteAccountProfilesResponse, UpsertAccountProfilesRequest, UpsertAccountProfilesResponse, Accounts, CreateAccountsRequest, CreateAccountsResponse, GetAccountsRequest, GetAccountsResponse, UpdateAccountsRequest, UpdateAccountsResponse, DeleteAccountsRequest, DeleteAccountsResponse, BatchInsertAccountsRequest, BatchInsertAccountsResponse, BatchUpdateAccountsRequest, BatchUpdateAccountsResponse, BatchDeleteAccountsRequest, BatchDeleteAccountsResponse, UpsertAccountsRequest, UpsertAccountsResponse, Addresses, CreateAddressesRequest, CreateAddressesResponse, GetAddressesRequest, GetAddressesResponse, UpdateAddressesRequest, UpdateAddressesResponse, DeleteAddressesRequest, DeleteAddressesResponse, BatchInsertAddressesRequest, BatchInsertAddressesResponse, BatchUpdateAddressesRequest, BatchUpdateAddressesResponse, BatchDeleteAddressesRequest, BatchDeleteAddressesResponse, UpsertAddressesRequest, UpsertAddressesResponse, Samples, CreateSamplesRequest, CreateSamplesResponse, GetSamplesRequest, GetSamplesResponse, UpdateSamplesRequest, UpdateSamplesResponse, DeleteSamplesRequest, DeleteSamplesResponse, BatchInsertSamplesRequest, BatchInsertSamplesResponse, BatchUpdateSamplesRequest, BatchUpdateSamplesResponse, BatchDeleteSamplesRequest, BatchDeleteSamplesResponse, UpsertSamplesRequest, UpsertSamplesResponse, AppFirewalls, CreateAppFirewallsRequest, CreateAppFirewallsResponse, GetAppFirewallsRequest, GetAppFirewallsResponse, UpdateAppFirewallsRequest, UpdateAppFirewallsResponse, DeleteAppFirewallsRequest, DeleteAppFirewallsResponse, BatchInsertAppFirewallsRequest, BatchInsertAppFirewallsResponse, BatchUpdateAppFirewallsRequest, BatchUpdateAppFirewallsResponse, BatchDeleteAppFirewallsRequest, BatchDeleteAppFirewallsResponse, UpsertAppFirewallsRequest, UpsertAppFirewallsResponse, AppguardLogs, CreateAppguardLogsRequest, CreateAppguardLogsResponse, GetAppguardLogsRequest, GetAppguardLogsResponse, UpdateAppguardLogsRequest, UpdateAppguardLogsResponse, DeleteAppguardLogsRequest, DeleteAppguardLogsResponse, BatchInsertAppguardLogsRequest, BatchInsertAppguardLogsResponse, BatchUpdateAppguardLogsRequest, BatchUpdateAppguardLogsResponse, BatchDeleteAppguardLogsRequest, BatchDeleteAppguardLogsResponse, UpsertAppguardLogsRequest, UpsertAppguardLogsResponse, TempAppguardLogs, CreateTempAppguardLogsRequest, CreateTempAppguardLogsResponse, GetTempAppguardLogsRequest, GetTempAppguardLogsResponse, UpdateTempAppguardLogsRequest, UpdateTempAppguardLogsResponse, DeleteTempAppguardLogsRequest, DeleteTempAppguardLogsResponse, BatchInsertTempAppguardLogsRequest, BatchInsertTempAppguardLogsResponse, BatchUpdateTempAppguardLogsRequest, BatchUpdateTempAppguardLogsResponse, BatchDeleteTempAppguardLogsRequest, BatchDeleteTempAppguardLogsResponse, UpsertTempAppguardLogsRequest, UpsertTempAppguardLogsResponse, DeviceAliases, CreateDeviceAliasesRequest, CreateDeviceAliasesResponse, GetDeviceAliasesRequest, GetDeviceAliasesResponse, UpdateDeviceAliasesRequest, UpdateDeviceAliasesResponse, DeleteDeviceAliasesRequest, DeleteDeviceAliasesResponse, BatchInsertDeviceAliasesRequest, BatchInsertDeviceAliasesResponse, BatchUpdateDeviceAliasesRequest, BatchUpdateDeviceAliasesResponse, BatchDeleteDeviceAliasesRequest, BatchDeleteDeviceAliasesResponse, UpsertDeviceAliasesRequest, UpsertDeviceAliasesResponse, TempDeviceAliases, CreateTempDeviceAliasesRequest, CreateTempDeviceAliasesResponse, GetTempDeviceAliasesRequest, GetTempDeviceAliasesResponse, UpdateTempDeviceAliasesRequest, UpdateTempDeviceAliasesResponse, DeleteTempDeviceAliasesRequest, DeleteTempDeviceAliasesResponse, BatchInsertTempDeviceAliasesRequest, BatchInsertTempDeviceAliasesResponse, BatchUpdateTempDeviceAliasesRequest, BatchUpdateTempDeviceAliasesResponse, BatchDeleteTempDeviceAliasesRequest, BatchDeleteTempDeviceAliasesResponse, UpsertTempDeviceAliasesRequest, UpsertTempDeviceAliasesResponse, DeviceConfigurations, CreateDeviceConfigurationsRequest, CreateDeviceConfigurationsResponse, GetDeviceConfigurationsRequest, GetDeviceConfigurationsResponse, UpdateDeviceConfigurationsRequest, UpdateDeviceConfigurationsResponse, DeleteDeviceConfigurationsRequest, DeleteDeviceConfigurationsResponse, BatchInsertDeviceConfigurationsRequest, BatchInsertDeviceConfigurationsResponse, BatchUpdateDeviceConfigurationsRequest, BatchUpdateDeviceConfigurationsResponse, BatchDeleteDeviceConfigurationsRequest, BatchDeleteDeviceConfigurationsResponse, UpsertDeviceConfigurationsRequest, UpsertDeviceConfigurationsResponse, DeviceInterfaceAddresses, CreateDeviceInterfaceAddressesRequest, CreateDeviceInterfaceAddressesResponse, GetDeviceInterfaceAddressesRequest, GetDeviceInterfaceAddressesResponse, UpdateDeviceInterfaceAddressesRequest, UpdateDeviceInterfaceAddressesResponse, DeleteDeviceInterfaceAddressesRequest, DeleteDeviceInterfaceAddressesResponse, BatchInsertDeviceInterfaceAddressesRequest, BatchInsertDeviceInterfaceAddressesResponse, BatchUpdateDeviceInterfaceAddressesRequest, BatchUpdateDeviceInterfaceAddressesResponse, BatchDeleteDeviceInterfaceAddressesRequest, BatchDeleteDeviceInterfaceAddressesResponse, UpsertDeviceInterfaceAddressesRequest, UpsertDeviceInterfaceAddressesResponse, TempDeviceInterfaceAddresses, CreateTempDeviceInterfaceAddressesRequest, CreateTempDeviceInterfaceAddressesResponse, GetTempDeviceInterfaceAddressesRequest, GetTempDeviceInterfaceAddressesResponse, UpdateTempDeviceInterfaceAddressesRequest, UpdateTempDeviceInterfaceAddressesResponse, DeleteTempDeviceInterfaceAddressesRequest, DeleteTempDeviceInterfaceAddressesResponse, BatchInsertTempDeviceInterfaceAddressesRequest, BatchInsertTempDeviceInterfaceAddressesResponse, BatchUpdateTempDeviceInterfaceAddressesRequest, BatchUpdateTempDeviceInterfaceAddressesResponse, BatchDeleteTempDeviceInterfaceAddressesRequest, BatchDeleteTempDeviceInterfaceAddressesResponse, UpsertTempDeviceInterfaceAddressesRequest, UpsertTempDeviceInterfaceAddressesResponse, DeviceInterfaces, CreateDeviceInterfacesRequest, CreateDeviceInterfacesResponse, GetDeviceInterfacesRequest, GetDeviceInterfacesResponse, UpdateDeviceInterfacesRequest, UpdateDeviceInterfacesResponse, DeleteDeviceInterfacesRequest, DeleteDeviceInterfacesResponse, BatchInsertDeviceInterfacesRequest, BatchInsertDeviceInterfacesResponse, BatchUpdateDeviceInterfacesRequest, BatchUpdateDeviceInterfacesResponse, BatchDeleteDeviceInterfacesRequest, BatchDeleteDeviceInterfacesResponse, UpsertDeviceInterfacesRequest, UpsertDeviceInterfacesResponse, TempDeviceInterfaces, CreateTempDeviceInterfacesRequest, CreateTempDeviceInterfacesResponse, GetTempDeviceInterfacesRequest, GetTempDeviceInterfacesResponse, UpdateTempDeviceInterfacesRequest, UpdateTempDeviceInterfacesResponse, DeleteTempDeviceInterfacesRequest, DeleteTempDeviceInterfacesResponse, BatchInsertTempDeviceInterfacesRequest, BatchInsertTempDeviceInterfacesResponse, BatchUpdateTempDeviceInterfacesRequest, BatchUpdateTempDeviceInterfacesResponse, BatchDeleteTempDeviceInterfacesRequest, BatchDeleteTempDeviceInterfacesResponse, UpsertTempDeviceInterfacesRequest, UpsertTempDeviceInterfacesResponse, DeviceRemoteAccessSessions, CreateDeviceRemoteAccessSessionsRequest, CreateDeviceRemoteAccessSessionsResponse, GetDeviceRemoteAccessSessionsRequest, GetDeviceRemoteAccessSessionsResponse, UpdateDeviceRemoteAccessSessionsRequest, UpdateDeviceRemoteAccessSessionsResponse, DeleteDeviceRemoteAccessSessionsRequest, DeleteDeviceRemoteAccessSessionsResponse, BatchInsertDeviceRemoteAccessSessionsRequest, BatchInsertDeviceRemoteAccessSessionsResponse, BatchUpdateDeviceRemoteAccessSessionsRequest, BatchUpdateDeviceRemoteAccessSessionsResponse, BatchDeleteDeviceRemoteAccessSessionsRequest, BatchDeleteDeviceRemoteAccessSessionsResponse, UpsertDeviceRemoteAccessSessionsRequest, UpsertDeviceRemoteAccessSessionsResponse, TempDeviceRemoteAccessSessions, CreateTempDeviceRemoteAccessSessionsRequest, CreateTempDeviceRemoteAccessSessionsResponse, GetTempDeviceRemoteAccessSessionsRequest, GetTempDeviceRemoteAccessSessionsResponse, UpdateTempDeviceRemoteAccessSessionsRequest, UpdateTempDeviceRemoteAccessSessionsResponse, DeleteTempDeviceRemoteAccessSessionsRequest, DeleteTempDeviceRemoteAccessSessionsResponse, BatchInsertTempDeviceRemoteAccessSessionsRequest, BatchInsertTempDeviceRemoteAccessSessionsResponse, BatchUpdateTempDeviceRemoteAccessSessionsRequest, BatchUpdateTempDeviceRemoteAccessSessionsResponse, BatchDeleteTempDeviceRemoteAccessSessionsRequest, BatchDeleteTempDeviceRemoteAccessSessionsResponse, UpsertTempDeviceRemoteAccessSessionsRequest, UpsertTempDeviceRemoteAccessSessionsResponse, DeviceRules, CreateDeviceRulesRequest, CreateDeviceRulesResponse, GetDeviceRulesRequest, GetDeviceRulesResponse, UpdateDeviceRulesRequest, UpdateDeviceRulesResponse, DeleteDeviceRulesRequest, DeleteDeviceRulesResponse, BatchInsertDeviceRulesRequest, BatchInsertDeviceRulesResponse, BatchUpdateDeviceRulesRequest, BatchUpdateDeviceRulesResponse, BatchDeleteDeviceRulesRequest, BatchDeleteDeviceRulesResponse, UpsertDeviceRulesRequest, UpsertDeviceRulesResponse, TempDeviceRules, CreateTempDeviceRulesRequest, CreateTempDeviceRulesResponse, GetTempDeviceRulesRequest, GetTempDeviceRulesResponse, UpdateTempDeviceRulesRequest, UpdateTempDeviceRulesResponse, DeleteTempDeviceRulesRequest, DeleteTempDeviceRulesResponse, BatchInsertTempDeviceRulesRequest, BatchInsertTempDeviceRulesResponse, BatchUpdateTempDeviceRulesRequest, BatchUpdateTempDeviceRulesResponse, BatchDeleteTempDeviceRulesRequest, BatchDeleteTempDeviceRulesResponse, UpsertTempDeviceRulesRequest, UpsertTempDeviceRulesResponse, Packets, CreatePacketsRequest, CreatePacketsResponse, GetPacketsRequest, GetPacketsResponse, UpdatePacketsRequest, UpdatePacketsResponse, DeletePacketsRequest, DeletePacketsResponse, BatchInsertPacketsRequest, BatchInsertPacketsResponse, BatchUpdatePacketsRequest, BatchUpdatePacketsResponse, BatchDeletePacketsRequest, BatchDeletePacketsResponse, UpsertPacketsRequest, UpsertPacketsResponse, TempPackets, CreateTempPacketsRequest, CreateTempPacketsResponse, GetTempPacketsRequest, GetTempPacketsResponse, UpdateTempPacketsRequest, UpdateTempPacketsResponse, DeleteTempPacketsRequest, DeleteTempPacketsResponse, BatchInsertTempPacketsRequest, BatchInsertTempPacketsResponse, BatchUpdateTempPacketsRequest, BatchUpdateTempPacketsResponse, BatchDeleteTempPacketsRequest, BatchDeleteTempPacketsResponse, UpsertTempPacketsRequest, UpsertTempPacketsResponse, Connections, CreateConnectionsRequest, CreateConnectionsResponse, GetConnectionsRequest, GetConnectionsResponse, UpdateConnectionsRequest, UpdateConnectionsResponse, DeleteConnectionsRequest, DeleteConnectionsResponse, BatchInsertConnectionsRequest, BatchInsertConnectionsResponse, BatchUpdateConnectionsRequest, BatchUpdateConnectionsResponse, BatchDeleteConnectionsRequest, BatchDeleteConnectionsResponse, UpsertConnectionsRequest, UpsertConnectionsResponse, TempConnections, CreateTempConnectionsRequest, CreateTempConnectionsResponse, GetTempConnectionsRequest, GetTempConnectionsResponse, UpdateTempConnectionsRequest, UpdateTempConnectionsResponse, DeleteTempConnectionsRequest, DeleteTempConnectionsResponse, BatchInsertTempConnectionsRequest, BatchInsertTempConnectionsResponse, BatchUpdateTempConnectionsRequest, BatchUpdateTempConnectionsResponse, BatchDeleteTempConnectionsRequest, BatchDeleteTempConnectionsResponse, UpsertTempConnectionsRequest, UpsertTempConnectionsResponse, DeviceSshKeys, CreateDeviceSshKeysRequest, CreateDeviceSshKeysResponse, GetDeviceSshKeysRequest, GetDeviceSshKeysResponse, UpdateDeviceSshKeysRequest, UpdateDeviceSshKeysResponse, DeleteDeviceSshKeysRequest, DeleteDeviceSshKeysResponse, BatchInsertDeviceSshKeysRequest, BatchInsertDeviceSshKeysResponse, BatchUpdateDeviceSshKeysRequest, BatchUpdateDeviceSshKeysResponse, BatchDeleteDeviceSshKeysRequest, BatchDeleteDeviceSshKeysResponse, UpsertDeviceSshKeysRequest, UpsertDeviceSshKeysResponse, Devices, CreateDevicesRequest, CreateDevicesResponse, GetDevicesRequest, GetDevicesResponse, UpdateDevicesRequest, UpdateDevicesResponse, DeleteDevicesRequest, DeleteDevicesResponse, BatchInsertDevicesRequest, BatchInsertDevicesResponse, BatchUpdateDevicesRequest, BatchUpdateDevicesResponse, BatchDeleteDevicesRequest, BatchDeleteDevicesResponse, UpsertDevicesRequest, UpsertDevicesResponse, IpInfos, CreateIpInfosRequest, CreateIpInfosResponse, GetIpInfosRequest, GetIpInfosResponse, UpdateIpInfosRequest, UpdateIpInfosResponse, DeleteIpInfosRequest, DeleteIpInfosResponse, BatchInsertIpInfosRequest, BatchInsertIpInfosResponse, BatchUpdateIpInfosRequest, BatchUpdateIpInfosResponse, BatchDeleteIpInfosRequest, BatchDeleteIpInfosResponse, UpsertIpInfosRequest, UpsertIpInfosResponse, Resolutions, CreateResolutionsRequest, CreateResolutionsResponse, GetResolutionsRequest, GetResolutionsResponse, UpdateResolutionsRequest, UpdateResolutionsResponse, DeleteResolutionsRequest, DeleteResolutionsResponse, BatchInsertResolutionsRequest, BatchInsertResolutionsResponse, BatchUpdateResolutionsRequest, BatchUpdateResolutionsResponse, BatchDeleteResolutionsRequest, BatchDeleteResolutionsResponse, UpsertResolutionsRequest, UpsertResolutionsResponse, WallguardLogs, CreateWallguardLogsRequest, CreateWallguardLogsResponse, GetWallguardLogsRequest, GetWallguardLogsResponse, UpdateWallguardLogsRequest, UpdateWallguardLogsResponse, DeleteWallguardLogsRequest, DeleteWallguardLogsResponse, BatchInsertWallguardLogsRequest, BatchInsertWallguardLogsResponse, BatchUpdateWallguardLogsRequest, BatchUpdateWallguardLogsResponse, BatchDeleteWallguardLogsRequest, BatchDeleteWallguardLogsResponse, UpsertWallguardLogsRequest, UpsertWallguardLogsResponse, TempWallguardLogs, CreateTempWallguardLogsRequest, CreateTempWallguardLogsResponse, GetTempWallguardLogsRequest, GetTempWallguardLogsResponse, UpdateTempWallguardLogsRequest, UpdateTempWallguardLogsResponse, DeleteTempWallguardLogsRequest, DeleteTempWallguardLogsResponse, BatchInsertTempWallguardLogsRequest, BatchInsertTempWallguardLogsResponse, BatchUpdateTempWallguardLogsRequest, BatchUpdateTempWallguardLogsResponse, BatchDeleteTempWallguardLogsRequest, BatchDeleteTempWallguardLogsResponse, UpsertTempWallguardLogsRequest, UpsertTempWallguardLogsResponse, DeviceGroupSettings, CreateDeviceGroupSettingsRequest, CreateDeviceGroupSettingsResponse, GetDeviceGroupSettingsRequest, GetDeviceGroupSettingsResponse, UpdateDeviceGroupSettingsRequest, UpdateDeviceGroupSettingsResponse, DeleteDeviceGroupSettingsRequest, DeleteDeviceGroupSettingsResponse, BatchInsertDeviceGroupSettingsRequest, BatchInsertDeviceGroupSettingsResponse, BatchUpdateDeviceGroupSettingsRequest, BatchUpdateDeviceGroupSettingsResponse, BatchDeleteDeviceGroupSettingsRequest, BatchDeleteDeviceGroupSettingsResponse, UpsertDeviceGroupSettingsRequest, UpsertDeviceGroupSettingsResponse, Contacts, CreateContactsRequest, CreateContactsResponse, GetContactsRequest, GetContactsResponse, UpdateContactsRequest, UpdateContactsResponse, DeleteContactsRequest, DeleteContactsResponse, BatchInsertContactsRequest, BatchInsertContactsResponse, BatchUpdateContactsRequest, BatchUpdateContactsResponse, BatchDeleteContactsRequest, BatchDeleteContactsResponse, UpsertContactsRequest, UpsertContactsResponse, ContactPhoneNumbers, CreateContactPhoneNumbersRequest, CreateContactPhoneNumbersResponse, GetContactPhoneNumbersRequest, GetContactPhoneNumbersResponse, UpdateContactPhoneNumbersRequest, UpdateContactPhoneNumbersResponse, DeleteContactPhoneNumbersRequest, DeleteContactPhoneNumbersResponse, BatchInsertContactPhoneNumbersRequest, BatchInsertContactPhoneNumbersResponse, BatchUpdateContactPhoneNumbersRequest, BatchUpdateContactPhoneNumbersResponse, BatchDeleteContactPhoneNumbersRequest, BatchDeleteContactPhoneNumbersResponse, UpsertContactPhoneNumbersRequest, UpsertContactPhoneNumbersResponse, ContactEmails, CreateContactEmailsRequest, CreateContactEmailsResponse, GetContactEmailsRequest, GetContactEmailsResponse, UpdateContactEmailsRequest, UpdateContactEmailsResponse, DeleteContactEmailsRequest, DeleteContactEmailsResponse, BatchInsertContactEmailsRequest, BatchInsertContactEmailsResponse, BatchUpdateContactEmailsRequest, BatchUpdateContactEmailsResponse, BatchDeleteContactEmailsRequest, BatchDeleteContactEmailsResponse, UpsertContactEmailsRequest, UpsertContactEmailsResponse};
pub struct GrpcController {}

impl GrpcController {
    pub fn new() -> Self { GrpcController {} }

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
                    interceptor_chain
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
    generate_update_method!(temp_device_interface_addresses, temp_device_interface_address);
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
    generate_update_method!(temp_device_remote_access_sessions, temp_device_remote_access_session);
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
