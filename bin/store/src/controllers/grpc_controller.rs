use crate::db::create_connection;
use actix_web::{HttpResponse, Responder, web};
use std::pin::Pin;
use std::net::SocketAddr;
use crate::table_enum::Table;
use crate::utils::utils::table_exists;
use crate::sync::sync_service::{insert, update};
use crate::structs::structs::Auth;
use serde_json::Value;
use crate::structs::structs::RequestBody;
use tonic::{Request, Response, Status, transport::Server};
use crate::middlewares::auth_middleware::GrpcAuthInterceptor;
use super::common_controller::{perform_batch_update, process_record_for_insert, process_record_for_update, sanitize_updates, convert_json_to_csv, process_records, execute_copy, perform_upsert, process_and_update_record, process_and_insert_record};
use crate::generated::store::store_service_server::{StoreServiceServer, StoreService };
use crate::{ generate_batch_delete_method, generate_batch_insert_method, generate_batch_update_method, generate_create_method, generate_update_method, generate_get_method, generate_delete_method, generate_upsert_method};
use crate::generated::store::{Addresses, CreateAddressesRequest, CreateAddressesResponse, GetAddressesRequest, GetAddressesResponse, UpdateAddressesRequest, UpdateAddressesResponse, DeleteAddressesRequest, DeleteAddressesResponse, BatchInsertAddressesRequest, BatchInsertAddressesResponse, BatchUpdateAddressesRequest, BatchUpdateAddressesResponse, BatchDeleteAddressesRequest, BatchDeleteAddressesResponse, UpsertAddressesRequest, UpsertAddressesResponse, AppFirewalls, CreateAppFirewallsRequest, CreateAppFirewallsResponse, GetAppFirewallsRequest, GetAppFirewallsResponse, UpdateAppFirewallsRequest, UpdateAppFirewallsResponse, DeleteAppFirewallsRequest, DeleteAppFirewallsResponse, BatchInsertAppFirewallsRequest, BatchInsertAppFirewallsResponse, BatchUpdateAppFirewallsRequest, BatchUpdateAppFirewallsResponse, BatchDeleteAppFirewallsRequest, BatchDeleteAppFirewallsResponse, UpsertAppFirewallsRequest, UpsertAppFirewallsResponse, AppguardLogs, CreateAppguardLogsRequest, CreateAppguardLogsResponse, GetAppguardLogsRequest, GetAppguardLogsResponse, UpdateAppguardLogsRequest, UpdateAppguardLogsResponse, DeleteAppguardLogsRequest, DeleteAppguardLogsResponse, BatchInsertAppguardLogsRequest, BatchInsertAppguardLogsResponse, BatchUpdateAppguardLogsRequest, BatchUpdateAppguardLogsResponse, BatchDeleteAppguardLogsRequest, BatchDeleteAppguardLogsResponse, UpsertAppguardLogsRequest, UpsertAppguardLogsResponse, DeviceAliases, CreateDeviceAliasesRequest, CreateDeviceAliasesResponse, GetDeviceAliasesRequest, GetDeviceAliasesResponse, UpdateDeviceAliasesRequest, UpdateDeviceAliasesResponse, DeleteDeviceAliasesRequest, DeleteDeviceAliasesResponse, BatchInsertDeviceAliasesRequest, BatchInsertDeviceAliasesResponse, BatchUpdateDeviceAliasesRequest, BatchUpdateDeviceAliasesResponse, BatchDeleteDeviceAliasesRequest, BatchDeleteDeviceAliasesResponse, UpsertDeviceAliasesRequest, UpsertDeviceAliasesResponse, DeviceConfigurations, CreateDeviceConfigurationsRequest, CreateDeviceConfigurationsResponse, GetDeviceConfigurationsRequest, GetDeviceConfigurationsResponse, UpdateDeviceConfigurationsRequest, UpdateDeviceConfigurationsResponse, DeleteDeviceConfigurationsRequest, DeleteDeviceConfigurationsResponse, BatchInsertDeviceConfigurationsRequest, BatchInsertDeviceConfigurationsResponse, BatchUpdateDeviceConfigurationsRequest, BatchUpdateDeviceConfigurationsResponse, BatchDeleteDeviceConfigurationsRequest, BatchDeleteDeviceConfigurationsResponse, UpsertDeviceConfigurationsRequest, UpsertDeviceConfigurationsResponse, DeviceInterfaceAddresses, CreateDeviceInterfaceAddressesRequest, CreateDeviceInterfaceAddressesResponse, GetDeviceInterfaceAddressesRequest, GetDeviceInterfaceAddressesResponse, UpdateDeviceInterfaceAddressesRequest, UpdateDeviceInterfaceAddressesResponse, DeleteDeviceInterfaceAddressesRequest, DeleteDeviceInterfaceAddressesResponse, BatchInsertDeviceInterfaceAddressesRequest, BatchInsertDeviceInterfaceAddressesResponse, BatchUpdateDeviceInterfaceAddressesRequest, BatchUpdateDeviceInterfaceAddressesResponse, BatchDeleteDeviceInterfaceAddressesRequest, BatchDeleteDeviceInterfaceAddressesResponse, UpsertDeviceInterfaceAddressesRequest, UpsertDeviceInterfaceAddressesResponse, DeviceInterfaces, CreateDeviceInterfacesRequest, CreateDeviceInterfacesResponse, GetDeviceInterfacesRequest, GetDeviceInterfacesResponse, UpdateDeviceInterfacesRequest, UpdateDeviceInterfacesResponse, DeleteDeviceInterfacesRequest, DeleteDeviceInterfacesResponse, BatchInsertDeviceInterfacesRequest, BatchInsertDeviceInterfacesResponse, BatchUpdateDeviceInterfacesRequest, BatchUpdateDeviceInterfacesResponse, BatchDeleteDeviceInterfacesRequest, BatchDeleteDeviceInterfacesResponse, UpsertDeviceInterfacesRequest, UpsertDeviceInterfacesResponse, DeviceRemoteAccessSessions, CreateDeviceRemoteAccessSessionsRequest, CreateDeviceRemoteAccessSessionsResponse, GetDeviceRemoteAccessSessionsRequest, GetDeviceRemoteAccessSessionsResponse, UpdateDeviceRemoteAccessSessionsRequest, UpdateDeviceRemoteAccessSessionsResponse, DeleteDeviceRemoteAccessSessionsRequest, DeleteDeviceRemoteAccessSessionsResponse, BatchInsertDeviceRemoteAccessSessionsRequest, BatchInsertDeviceRemoteAccessSessionsResponse, BatchUpdateDeviceRemoteAccessSessionsRequest, BatchUpdateDeviceRemoteAccessSessionsResponse, BatchDeleteDeviceRemoteAccessSessionsRequest, BatchDeleteDeviceRemoteAccessSessionsResponse, UpsertDeviceRemoteAccessSessionsRequest, UpsertDeviceRemoteAccessSessionsResponse, DeviceRules, CreateDeviceRulesRequest, CreateDeviceRulesResponse, GetDeviceRulesRequest, GetDeviceRulesResponse, UpdateDeviceRulesRequest, UpdateDeviceRulesResponse, DeleteDeviceRulesRequest, DeleteDeviceRulesResponse, BatchInsertDeviceRulesRequest, BatchInsertDeviceRulesResponse, BatchUpdateDeviceRulesRequest, BatchUpdateDeviceRulesResponse, BatchDeleteDeviceRulesRequest, BatchDeleteDeviceRulesResponse, UpsertDeviceRulesRequest, UpsertDeviceRulesResponse, Packets, CreatePacketsRequest, CreatePacketsResponse, GetPacketsRequest, GetPacketsResponse, UpdatePacketsRequest, UpdatePacketsResponse, DeletePacketsRequest, DeletePacketsResponse, BatchInsertPacketsRequest, BatchInsertPacketsResponse, BatchUpdatePacketsRequest, BatchUpdatePacketsResponse, BatchDeletePacketsRequest, BatchDeletePacketsResponse, UpsertPacketsRequest, UpsertPacketsResponse, Connections, CreateConnectionsRequest, CreateConnectionsResponse, GetConnectionsRequest, GetConnectionsResponse, UpdateConnectionsRequest, UpdateConnectionsResponse, DeleteConnectionsRequest, DeleteConnectionsResponse, BatchInsertConnectionsRequest, BatchInsertConnectionsResponse, BatchUpdateConnectionsRequest, BatchUpdateConnectionsResponse, BatchDeleteConnectionsRequest, BatchDeleteConnectionsResponse, UpsertConnectionsRequest, UpsertConnectionsResponse, DeviceSshKeys, CreateDeviceSshKeysRequest, CreateDeviceSshKeysResponse, GetDeviceSshKeysRequest, GetDeviceSshKeysResponse, UpdateDeviceSshKeysRequest, UpdateDeviceSshKeysResponse, DeleteDeviceSshKeysRequest, DeleteDeviceSshKeysResponse, BatchInsertDeviceSshKeysRequest, BatchInsertDeviceSshKeysResponse, BatchUpdateDeviceSshKeysRequest, BatchUpdateDeviceSshKeysResponse, BatchDeleteDeviceSshKeysRequest, BatchDeleteDeviceSshKeysResponse, UpsertDeviceSshKeysRequest, UpsertDeviceSshKeysResponse, Devices, CreateDevicesRequest, CreateDevicesResponse, GetDevicesRequest, GetDevicesResponse, UpdateDevicesRequest, UpdateDevicesResponse, DeleteDevicesRequest, DeleteDevicesResponse, BatchInsertDevicesRequest, BatchInsertDevicesResponse, BatchUpdateDevicesRequest, BatchUpdateDevicesResponse, BatchDeleteDevicesRequest, BatchDeleteDevicesResponse, UpsertDevicesRequest, UpsertDevicesResponse, IpInfos, CreateIpInfosRequest, CreateIpInfosResponse, GetIpInfosRequest, GetIpInfosResponse, UpdateIpInfosRequest, UpdateIpInfosResponse, DeleteIpInfosRequest, DeleteIpInfosResponse, BatchInsertIpInfosRequest, BatchInsertIpInfosResponse, BatchUpdateIpInfosRequest, BatchUpdateIpInfosResponse, BatchDeleteIpInfosRequest, BatchDeleteIpInfosResponse, UpsertIpInfosRequest, UpsertIpInfosResponse, OrganizationAccounts, CreateOrganizationAccountsRequest, CreateOrganizationAccountsResponse, GetOrganizationAccountsRequest, GetOrganizationAccountsResponse, UpdateOrganizationAccountsRequest, UpdateOrganizationAccountsResponse, DeleteOrganizationAccountsRequest, DeleteOrganizationAccountsResponse, BatchInsertOrganizationAccountsRequest, BatchInsertOrganizationAccountsResponse, BatchUpdateOrganizationAccountsRequest, BatchUpdateOrganizationAccountsResponse, BatchDeleteOrganizationAccountsRequest, BatchDeleteOrganizationAccountsResponse, UpsertOrganizationAccountsRequest, UpsertOrganizationAccountsResponse, DeviceGroupSettings, CreateDeviceGroupSettingsRequest, CreateDeviceGroupSettingsResponse, GetDeviceGroupSettingsRequest, GetDeviceGroupSettingsResponse, UpdateDeviceGroupSettingsRequest, UpdateDeviceGroupSettingsResponse, DeleteDeviceGroupSettingsRequest, DeleteDeviceGroupSettingsResponse, BatchInsertDeviceGroupSettingsRequest, BatchInsertDeviceGroupSettingsResponse, BatchUpdateDeviceGroupSettingsRequest, BatchUpdateDeviceGroupSettingsResponse, BatchDeleteDeviceGroupSettingsRequest, BatchDeleteDeviceGroupSettingsResponse, UpsertDeviceGroupSettingsRequest, UpsertDeviceGroupSettingsResponse};
pub struct GrpcController {}

impl GrpcController {
    pub fn new() -> Self { GrpcController {} }

    pub async fn init(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
        let addr: SocketAddr = addr.parse()?;
        let grpc_controller = GrpcController::new();
        println!("gRPC Server listening on {}", addr);
        Server::builder()
            .add_service(StoreServiceServer::with_interceptor(grpc_controller, GrpcAuthInterceptor))
            .serve(addr)
            .await?;
        Ok(())
    }
}

#[tonic::async_trait]
impl StoreService for GrpcController {
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
    // CRUD methods for device_aliases
    generate_create_method!(device_aliases);
    generate_update_method!(device_aliases, device_alias);
    generate_batch_insert_method!(device_aliases);
    generate_batch_update_method!(device_aliases);
    generate_get_method!(device_aliases);
    generate_delete_method!(device_aliases);
    generate_batch_delete_method!(device_aliases);
    generate_upsert_method!(device_aliases);
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
    // CRUD methods for device_interfaces
    generate_create_method!(device_interfaces);
    generate_update_method!(device_interfaces, device_interface);
    generate_batch_insert_method!(device_interfaces);
    generate_batch_update_method!(device_interfaces);
    generate_get_method!(device_interfaces);
    generate_delete_method!(device_interfaces);
    generate_batch_delete_method!(device_interfaces);
    generate_upsert_method!(device_interfaces);
    // CRUD methods for device_remote_access_sessions
    generate_create_method!(device_remote_access_sessions);
    generate_update_method!(device_remote_access_sessions, device_remote_access_session);
    generate_batch_insert_method!(device_remote_access_sessions);
    generate_batch_update_method!(device_remote_access_sessions);
    generate_get_method!(device_remote_access_sessions);
    generate_delete_method!(device_remote_access_sessions);
    generate_batch_delete_method!(device_remote_access_sessions);
    generate_upsert_method!(device_remote_access_sessions);
    // CRUD methods for device_rules
    generate_create_method!(device_rules);
    generate_update_method!(device_rules, device_rule);
    generate_batch_insert_method!(device_rules);
    generate_batch_update_method!(device_rules);
    generate_get_method!(device_rules);
    generate_delete_method!(device_rules);
    generate_batch_delete_method!(device_rules);
    generate_upsert_method!(device_rules);
    // CRUD methods for packets
    generate_create_method!(packets);
    generate_update_method!(packets, packet);
    generate_batch_insert_method!(packets);
    generate_batch_update_method!(packets);
    generate_get_method!(packets);
    generate_delete_method!(packets);
    generate_batch_delete_method!(packets);
    generate_upsert_method!(packets);
    // CRUD methods for connections
    generate_create_method!(connections);
    generate_update_method!(connections, connection);
    generate_batch_insert_method!(connections);
    generate_batch_update_method!(connections);
    generate_get_method!(connections);
    generate_delete_method!(connections);
    generate_batch_delete_method!(connections);
    generate_upsert_method!(connections);
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
    // CRUD methods for organization_accounts
    generate_create_method!(organization_accounts);
    generate_update_method!(organization_accounts, organization_account);
    generate_batch_insert_method!(organization_accounts);
    generate_batch_update_method!(organization_accounts);
    generate_get_method!(organization_accounts);
    generate_delete_method!(organization_accounts);
    generate_batch_delete_method!(organization_accounts);
    generate_upsert_method!(organization_accounts);
    // CRUD methods for device_group_settings
    generate_create_method!(device_group_settings);
    generate_update_method!(device_group_settings, device_group_setting);
    generate_batch_insert_method!(device_group_settings);
    generate_batch_update_method!(device_group_settings);
    generate_get_method!(device_group_settings);
    generate_delete_method!(device_group_settings);
    generate_batch_delete_method!(device_group_settings);
    generate_upsert_method!(device_group_settings);
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
