use crate::{generate_get_by_id_match, generate_hypertable_timestamp_match, generate_insert_record_match, generate_upsert_record_match, generate_upsert_record_migration_match, generate_upsert_record_migration_with_timestamp_match, generate_upsert_record_with_timestamp_match};
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
use crate::generated::models::ip_alias_model::IpAliasModel;
use crate::generated::models::smtp_responsis_model::SmtpResponsisModel;
use crate::generated::models::appguard_log_model::AppguardLogModel;
use crate::generated::models::packet_model::PacketModel;
use crate::generated::models::temp_device_interface_address_model::TempDeviceInterfaceAddressModel;
use crate::generated::models::http_responsis_model::HttpResponsisModel;
use crate::generated::models::system_resource_model::SystemResourceModel;
use crate::generated::models::organization_contact_user_role_model::OrganizationContactUserRoleModel;
use crate::generated::models::temp_system_resource_model::TempSystemResourceModel;
use crate::generated::models::device_configuration_model::DeviceConfigurationModel;
use crate::generated::models::grid_filter_model::GridFilterModel;
use crate::generated::models::connection_model::ConnectionModel;
use crate::generated::models::dummy_packet_model::DummyPacketModel;
use crate::generated::models::app_firewall_model::AppFirewallModel;
use crate::generated::models::device_group_setting_model::DeviceGroupSettingModel;
use crate::generated::models::device_tunnel_model::DeviceTunnelModel;
use crate::generated::models::device_service_model::DeviceServiceModel;
use crate::generated::models::device_group_model::DeviceGroupModel;
use crate::generated::models::device_heartbeat_model::DeviceHeartbeatModel;
use crate::generated::models::temp_device_instance_model::TempDeviceInstanceModel;
use crate::generated::models::resolution_model::ResolutionModel;
use crate::generated::models::temp_device_interface_model::TempDeviceInterfaceModel;
use crate::generated::models::temp_appguard_log_model::TempAppguardLogModel;
use crate::generated::models::device_filter_rule_model::DeviceFilterRuleModel;
use crate::generated::models::alias_model::AliasModel;
use crate::generated::models::tcp_connection_model::TcpConnectionModel;
use crate::generated::models::temp_device_nat_rule_model::TempDeviceNatRuleModel;
use crate::generated::models::device_instance_model::DeviceInstanceModel;
use crate::generated::models::temp_device_service_model::TempDeviceServiceModel;
use crate::generated::models::temp_connection_model::TempConnectionModel;
use crate::generated::models::http_request_model::HttpRequestModel;
use crate::generated::models::temp_port_alias_model::TempPortAliasModel;
use crate::generated::models::version_model::VersionModel;
use crate::generated::models::temp_device_tunnel_model::TempDeviceTunnelModel;
use crate::generated::models::temp_device_filter_rule_model::TempDeviceFilterRuleModel;
use crate::generated::models::port_alias_model::PortAliasModel;
use crate::generated::models::ip_info_model::IpInfoModel;
use crate::generated::models::device_interface_model::DeviceInterfaceModel;
use crate::generated::models::temp_wallguard_log_model::TempWallguardLogModel;
use crate::generated::models::notification_model::NotificationModel;
use crate::generated::models::device_nat_rule_model::DeviceNatRuleModel;
use crate::generated::models::temp_ip_alias_model::TempIpAliasModel;
use crate::generated::models::appguard_config_model::AppguardConfigModel;
use crate::generated::models::device_interface_address_model::DeviceInterfaceAddressModel;
use crate::generated::models::smtp_request_model::SmtpRequestModel;
use crate::generated::models::invitation_model::InvitationModel;
use crate::generated::models::setup_instruction_model::SetupInstructionModel;
use crate::generated::models::installation_code_model::InstallationCodeModel;
use crate::generated::models::temp_device_remote_access_session_model::TempDeviceRemoteAccessSessionModel;
use crate::generated::models::temp_packet_model::TempPacketModel;
use crate::generated::models::wallguard_log_model::WallguardLogModel;
use crate::generated::models::communication_template_model::CommunicationTemplateModel;
use crate::generated::models::temp_alias_model::TempAliasModel;
use crate::generated::models::location_model::LocationModel;
use crate::generated::models::device_remote_access_session_model::DeviceRemoteAccessSessionModel;
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
    IpAliases,
    SmtpResponses,
    AppguardLogs,
    Packets,
    TempDeviceInterfaceAddresses,
    HttpResponses,
    SystemResources,
    OrganizationContactUserRoles,
    TempSystemResources,
    DeviceConfigurations,
    GridFilters,
    Connections,
    DummyPackets,
    AppFirewalls,
    DeviceGroupSettings,
    DeviceTunnels,
    DeviceServices,
    DeviceGroups,
    DeviceHeartbeats,
    TempDeviceInstances,
    Resolutions,
    TempDeviceInterfaces,
    TempAppguardLogs,
    DeviceFilterRules,
    Aliases,
    TcpConnections,
    TempDeviceNatRules,
    DeviceInstances,
    TempDeviceServices,
    TempConnections,
    HttpRequests,
    TempPortAliases,
    Versions,
    TempDeviceTunnels,
    TempDeviceFilterRules,
    PortAliases,
    IpInfos,
    DeviceInterfaces,
    TempWallguardLogs,
    Notifications,
    DeviceNatRules,
    TempIpAliases,
    AppguardConfigs,
    DeviceInterfaceAddresses,
    SmtpRequests,
    Invitations,
    SetupInstructions,
    InstallationCodes,
    TempDeviceRemoteAccessSessions,
    TempPackets,
    WallguardLogs,
    CommunicationTemplates,
    TempAliases,
    Locations,
    DeviceRemoteAccessSessions,
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
            "ip_aliases" => Some(Table::IpAliases),
            "smtp_responses" => Some(Table::SmtpResponses),
            "appguard_logs" => Some(Table::AppguardLogs),
            "packets" => Some(Table::Packets),
            "temp_device_interface_addresses" => Some(Table::TempDeviceInterfaceAddresses),
            "http_responses" => Some(Table::HttpResponses),
            "system_resources" => Some(Table::SystemResources),
            "organization_contact_user_roles" => Some(Table::OrganizationContactUserRoles),
            "temp_system_resources" => Some(Table::TempSystemResources),
            "device_configurations" => Some(Table::DeviceConfigurations),
            "grid_filters" => Some(Table::GridFilters),
            "connections" => Some(Table::Connections),
            "dummy_packets" => Some(Table::DummyPackets),
            "app_firewalls" => Some(Table::AppFirewalls),
            "device_group_settings" => Some(Table::DeviceGroupSettings),
            "device_tunnels" => Some(Table::DeviceTunnels),
            "device_services" => Some(Table::DeviceServices),
            "device_groups" => Some(Table::DeviceGroups),
            "device_heartbeats" => Some(Table::DeviceHeartbeats),
            "temp_device_instances" => Some(Table::TempDeviceInstances),
            "resolutions" => Some(Table::Resolutions),
            "temp_device_interfaces" => Some(Table::TempDeviceInterfaces),
            "temp_appguard_logs" => Some(Table::TempAppguardLogs),
            "device_filter_rules" => Some(Table::DeviceFilterRules),
            "aliases" => Some(Table::Aliases),
            "tcp_connections" => Some(Table::TcpConnections),
            "temp_device_nat_rules" => Some(Table::TempDeviceNatRules),
            "device_instances" => Some(Table::DeviceInstances),
            "temp_device_services" => Some(Table::TempDeviceServices),
            "temp_connections" => Some(Table::TempConnections),
            "http_requests" => Some(Table::HttpRequests),
            "temp_port_aliases" => Some(Table::TempPortAliases),
            "versions" => Some(Table::Versions),
            "temp_device_tunnels" => Some(Table::TempDeviceTunnels),
            "temp_device_filter_rules" => Some(Table::TempDeviceFilterRules),
            "port_aliases" => Some(Table::PortAliases),
            "ip_infos" => Some(Table::IpInfos),
            "device_interfaces" => Some(Table::DeviceInterfaces),
            "temp_wallguard_logs" => Some(Table::TempWallguardLogs),
            "notifications" => Some(Table::Notifications),
            "device_nat_rules" => Some(Table::DeviceNatRules),
            "temp_ip_aliases" => Some(Table::TempIpAliases),
            "appguard_configs" => Some(Table::AppguardConfigs),
            "device_interface_addresses" => Some(Table::DeviceInterfaceAddresses),
            "smtp_requests" => Some(Table::SmtpRequests),
            "invitations" => Some(Table::Invitations),
            "setup_instructions" => Some(Table::SetupInstructions),
            "installation_codes" => Some(Table::InstallationCodes),
            "temp_device_remote_access_sessions" => Some(Table::TempDeviceRemoteAccessSessions),
            "temp_packets" => Some(Table::TempPackets),
            "wallguard_logs" => Some(Table::WallguardLogs),
            "communication_templates" => Some(Table::CommunicationTemplates),
            "temp_aliases" => Some(Table::TempAliases),
            "locations" => Some(Table::Locations),
            "device_remote_access_sessions" => Some(Table::DeviceRemoteAccessSessions),
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
        generate_hypertable_timestamp_match!(self, conn, id, SignedInActivities, TestHypertables, Packets, Connections, DummyPackets, DeviceHeartbeats, TempConnections, TempPackets)
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
            UserRoles, UserRoleModel, Sessions, SessionModel, SignedInActivities, SignedInActivityModel, ExternalContacts, ExternalContactModel, Organizations, OrganizationModel, OrganizationContacts, OrganizationContactModel, OrganizationAccounts, OrganizationAccountModel, AccountOrganizations, AccountOrganizationModel, AccountProfiles, AccountProfileModel, Accounts, AccountModel, Addresses, AddressModel, Samples, SampleModel, Devices, DeviceModel, PostgresChannels, PostgresChannelModel, Contacts, ContactModel, ContactPhoneNumbers, ContactPhoneNumberModel, ContactEmails, ContactEmailModel, Files, FileModel, TestHypertables, TestHypertableModel, AccountPhoneNumbers, AccountPhoneNumberModel, AccountSignatures, AccountSignatureModel, IpAliases, IpAliasModel, SmtpResponses, SmtpResponsisModel, AppguardLogs, AppguardLogModel, Packets, PacketModel, TempDeviceInterfaceAddresses, TempDeviceInterfaceAddressModel, HttpResponses, HttpResponsisModel, SystemResources, SystemResourceModel, OrganizationContactUserRoles, OrganizationContactUserRoleModel, TempSystemResources, TempSystemResourceModel, DeviceConfigurations, DeviceConfigurationModel, GridFilters, GridFilterModel, Connections, ConnectionModel, DummyPackets, DummyPacketModel, AppFirewalls, AppFirewallModel, DeviceGroupSettings, DeviceGroupSettingModel, DeviceTunnels, DeviceTunnelModel, DeviceServices, DeviceServiceModel, DeviceGroups, DeviceGroupModel, DeviceHeartbeats, DeviceHeartbeatModel, TempDeviceInstances, TempDeviceInstanceModel, Resolutions, ResolutionModel, TempDeviceInterfaces, TempDeviceInterfaceModel, TempAppguardLogs, TempAppguardLogModel, DeviceFilterRules, DeviceFilterRuleModel, Aliases, AliasModel, TcpConnections, TcpConnectionModel, TempDeviceNatRules, TempDeviceNatRuleModel, DeviceInstances, DeviceInstanceModel, TempDeviceServices, TempDeviceServiceModel, TempConnections, TempConnectionModel, HttpRequests, HttpRequestModel, TempPortAliases, TempPortAliasModel, Versions, VersionModel, TempDeviceTunnels, TempDeviceTunnelModel, TempDeviceFilterRules, TempDeviceFilterRuleModel, PortAliases, PortAliasModel, IpInfos, IpInfoModel, DeviceInterfaces, DeviceInterfaceModel, TempWallguardLogs, TempWallguardLogModel, Notifications, NotificationModel, DeviceNatRules, DeviceNatRuleModel, TempIpAliases, TempIpAliasModel, AppguardConfigs, AppguardConfigModel, DeviceInterfaceAddresses, DeviceInterfaceAddressModel, SmtpRequests, SmtpRequestModel, Invitations, InvitationModel, SetupInstructions, SetupInstructionModel, InstallationCodes, InstallationCodeModel, TempDeviceRemoteAccessSessions, TempDeviceRemoteAccessSessionModel, TempPackets, TempPacketModel, WallguardLogs, WallguardLogModel, CommunicationTemplates, CommunicationTemplateModel, TempAliases, TempAliasModel, Locations, LocationModel, DeviceRemoteAccessSessions, DeviceRemoteAccessSessionModel // Add other tables and their models here as needed
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
            UserRoles, UserRoleModel, Sessions, SessionModel, SignedInActivities, SignedInActivityModel, ExternalContacts, ExternalContactModel, Organizations, OrganizationModel, OrganizationContacts, OrganizationContactModel, OrganizationAccounts, OrganizationAccountModel, AccountOrganizations, AccountOrganizationModel, AccountProfiles, AccountProfileModel, Accounts, AccountModel, Addresses, AddressModel, Samples, SampleModel, Devices, DeviceModel, PostgresChannels, PostgresChannelModel, Contacts, ContactModel, ContactPhoneNumbers, ContactPhoneNumberModel, ContactEmails, ContactEmailModel, Files, FileModel, TestHypertables, TestHypertableModel, AccountPhoneNumbers, AccountPhoneNumberModel, AccountSignatures, AccountSignatureModel, IpAliases, IpAliasModel, SmtpResponses, SmtpResponsisModel, AppguardLogs, AppguardLogModel, Packets, PacketModel, TempDeviceInterfaceAddresses, TempDeviceInterfaceAddressModel, HttpResponses, HttpResponsisModel, SystemResources, SystemResourceModel, OrganizationContactUserRoles, OrganizationContactUserRoleModel, TempSystemResources, TempSystemResourceModel, DeviceConfigurations, DeviceConfigurationModel, GridFilters, GridFilterModel, Connections, ConnectionModel, DummyPackets, DummyPacketModel, AppFirewalls, AppFirewallModel, DeviceGroupSettings, DeviceGroupSettingModel, DeviceTunnels, DeviceTunnelModel, DeviceServices, DeviceServiceModel, DeviceGroups, DeviceGroupModel, DeviceHeartbeats, DeviceHeartbeatModel, TempDeviceInstances, TempDeviceInstanceModel, Resolutions, ResolutionModel, TempDeviceInterfaces, TempDeviceInterfaceModel, TempAppguardLogs, TempAppguardLogModel, DeviceFilterRules, DeviceFilterRuleModel, Aliases, AliasModel, TcpConnections, TcpConnectionModel, TempDeviceNatRules, TempDeviceNatRuleModel, DeviceInstances, DeviceInstanceModel, TempDeviceServices, TempDeviceServiceModel, TempConnections, TempConnectionModel, HttpRequests, HttpRequestModel, TempPortAliases, TempPortAliasModel, Versions, VersionModel, TempDeviceTunnels, TempDeviceTunnelModel, TempDeviceFilterRules, TempDeviceFilterRuleModel, PortAliases, PortAliasModel, IpInfos, IpInfoModel, DeviceInterfaces, DeviceInterfaceModel, TempWallguardLogs, TempWallguardLogModel, Notifications, NotificationModel, DeviceNatRules, DeviceNatRuleModel, TempIpAliases, TempIpAliasModel, AppguardConfigs, AppguardConfigModel, DeviceInterfaceAddresses, DeviceInterfaceAddressModel, SmtpRequests, SmtpRequestModel, Invitations, InvitationModel, SetupInstructions, SetupInstructionModel, InstallationCodes, InstallationCodeModel, TempDeviceRemoteAccessSessions, TempDeviceRemoteAccessSessionModel, TempPackets, TempPacketModel, WallguardLogs, WallguardLogModel, CommunicationTemplates, CommunicationTemplateModel, TempAliases, TempAliasModel, Locations, LocationModel, DeviceRemoteAccessSessions, DeviceRemoteAccessSessionModel // Add other tables and their models here as needed
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
            UserRoles, UserRoleModel, Sessions, SessionModel, SignedInActivities, SignedInActivityModel, ExternalContacts, ExternalContactModel, Organizations, OrganizationModel, OrganizationContacts, OrganizationContactModel, OrganizationAccounts, OrganizationAccountModel, AccountOrganizations, AccountOrganizationModel, AccountProfiles, AccountProfileModel, Accounts, AccountModel, Addresses, AddressModel, Samples, SampleModel, Devices, DeviceModel, PostgresChannels, PostgresChannelModel, Contacts, ContactModel, ContactPhoneNumbers, ContactPhoneNumberModel, ContactEmails, ContactEmailModel, Files, FileModel, TestHypertables, TestHypertableModel, AccountPhoneNumbers, AccountPhoneNumberModel, AccountSignatures, AccountSignatureModel, IpAliases, IpAliasModel, SmtpResponses, SmtpResponsisModel, AppguardLogs, AppguardLogModel, Packets, PacketModel, TempDeviceInterfaceAddresses, TempDeviceInterfaceAddressModel, HttpResponses, HttpResponsisModel, SystemResources, SystemResourceModel, OrganizationContactUserRoles, OrganizationContactUserRoleModel, TempSystemResources, TempSystemResourceModel, DeviceConfigurations, DeviceConfigurationModel, GridFilters, GridFilterModel, Connections, ConnectionModel, DummyPackets, DummyPacketModel, AppFirewalls, AppFirewallModel, DeviceGroupSettings, DeviceGroupSettingModel, DeviceTunnels, DeviceTunnelModel, DeviceServices, DeviceServiceModel, DeviceGroups, DeviceGroupModel, DeviceHeartbeats, DeviceHeartbeatModel, TempDeviceInstances, TempDeviceInstanceModel, Resolutions, ResolutionModel, TempDeviceInterfaces, TempDeviceInterfaceModel, TempAppguardLogs, TempAppguardLogModel, DeviceFilterRules, DeviceFilterRuleModel, Aliases, AliasModel, TcpConnections, TcpConnectionModel, TempDeviceNatRules, TempDeviceNatRuleModel, DeviceInstances, DeviceInstanceModel, TempDeviceServices, TempDeviceServiceModel, TempConnections, TempConnectionModel, HttpRequests, HttpRequestModel, TempPortAliases, TempPortAliasModel, Versions, VersionModel, TempDeviceTunnels, TempDeviceTunnelModel, TempDeviceFilterRules, TempDeviceFilterRuleModel, PortAliases, PortAliasModel, IpInfos, IpInfoModel, DeviceInterfaces, DeviceInterfaceModel, TempWallguardLogs, TempWallguardLogModel, Notifications, NotificationModel, DeviceNatRules, DeviceNatRuleModel, TempIpAliases, TempIpAliasModel, AppguardConfigs, AppguardConfigModel, DeviceInterfaceAddresses, DeviceInterfaceAddressModel, SmtpRequests, SmtpRequestModel, Invitations, InvitationModel, SetupInstructions, SetupInstructionModel, InstallationCodes, InstallationCodeModel, TempDeviceRemoteAccessSessions, TempDeviceRemoteAccessSessionModel, TempPackets, TempPacketModel, WallguardLogs, WallguardLogModel, CommunicationTemplates, CommunicationTemplateModel, TempAliases, TempAliasModel, Locations, LocationModel, DeviceRemoteAccessSessions, DeviceRemoteAccessSessionModel // Add other tables and their models here as needed
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
            UserRoles, UserRoleModel, Sessions, SessionModel, SignedInActivities, SignedInActivityModel, ExternalContacts, ExternalContactModel, Organizations, OrganizationModel, OrganizationContacts, OrganizationContactModel, OrganizationAccounts, OrganizationAccountModel, AccountOrganizations, AccountOrganizationModel, AccountProfiles, AccountProfileModel, Accounts, AccountModel, Addresses, AddressModel, Samples, SampleModel, Devices, DeviceModel, PostgresChannels, PostgresChannelModel, Contacts, ContactModel, ContactPhoneNumbers, ContactPhoneNumberModel, ContactEmails, ContactEmailModel, Files, FileModel, TestHypertables, TestHypertableModel, AccountPhoneNumbers, AccountPhoneNumberModel, AccountSignatures, AccountSignatureModel, IpAliases, IpAliasModel, SmtpResponses, SmtpResponsisModel, AppguardLogs, AppguardLogModel, Packets, PacketModel, TempDeviceInterfaceAddresses, TempDeviceInterfaceAddressModel, HttpResponses, HttpResponsisModel, SystemResources, SystemResourceModel, OrganizationContactUserRoles, OrganizationContactUserRoleModel, TempSystemResources, TempSystemResourceModel, DeviceConfigurations, DeviceConfigurationModel, GridFilters, GridFilterModel, Connections, ConnectionModel, DummyPackets, DummyPacketModel, AppFirewalls, AppFirewallModel, DeviceGroupSettings, DeviceGroupSettingModel, DeviceTunnels, DeviceTunnelModel, DeviceServices, DeviceServiceModel, DeviceGroups, DeviceGroupModel, DeviceHeartbeats, DeviceHeartbeatModel, TempDeviceInstances, TempDeviceInstanceModel, Resolutions, ResolutionModel, TempDeviceInterfaces, TempDeviceInterfaceModel, TempAppguardLogs, TempAppguardLogModel, DeviceFilterRules, DeviceFilterRuleModel, Aliases, AliasModel, TcpConnections, TcpConnectionModel, TempDeviceNatRules, TempDeviceNatRuleModel, DeviceInstances, DeviceInstanceModel, TempDeviceServices, TempDeviceServiceModel, TempConnections, TempConnectionModel, HttpRequests, HttpRequestModel, TempPortAliases, TempPortAliasModel, Versions, VersionModel, TempDeviceTunnels, TempDeviceTunnelModel, TempDeviceFilterRules, TempDeviceFilterRuleModel, PortAliases, PortAliasModel, IpInfos, IpInfoModel, DeviceInterfaces, DeviceInterfaceModel, TempWallguardLogs, TempWallguardLogModel, Notifications, NotificationModel, DeviceNatRules, DeviceNatRuleModel, TempIpAliases, TempIpAliasModel, AppguardConfigs, AppguardConfigModel, DeviceInterfaceAddresses, DeviceInterfaceAddressModel, SmtpRequests, SmtpRequestModel, Invitations, InvitationModel, SetupInstructions, SetupInstructionModel, InstallationCodes, InstallationCodeModel, TempDeviceRemoteAccessSessions, TempDeviceRemoteAccessSessionModel, TempPackets, TempPacketModel, WallguardLogs, WallguardLogModel, CommunicationTemplates, CommunicationTemplateModel, TempAliases, TempAliasModel, Locations, LocationModel, DeviceRemoteAccessSessions, DeviceRemoteAccessSessionModel // Add other tables and their models here as needed
        )
    }

    /// Migration-mode upsert: always sets all non-None fields, no version/status special handling.
    pub async fn upsert_record_migration(
        &self,
        conn: &mut AsyncPgConnection,
        record: Value,
    ) -> Result<(), DieselError> {
        generate_upsert_record_migration_match!(
            self,
            conn,
            record,
            UserRoles, UserRoleModel, Sessions, SessionModel, SignedInActivities, SignedInActivityModel, ExternalContacts, ExternalContactModel, Organizations, OrganizationModel, OrganizationContacts, OrganizationContactModel, OrganizationAccounts, OrganizationAccountModel, AccountOrganizations, AccountOrganizationModel, AccountProfiles, AccountProfileModel, Accounts, AccountModel, Addresses, AddressModel, Samples, SampleModel, Devices, DeviceModel, PostgresChannels, PostgresChannelModel, Contacts, ContactModel, ContactPhoneNumbers, ContactPhoneNumberModel, ContactEmails, ContactEmailModel, Files, FileModel, TestHypertables, TestHypertableModel, AccountPhoneNumbers, AccountPhoneNumberModel, AccountSignatures, AccountSignatureModel, IpAliases, IpAliasModel, SmtpResponses, SmtpResponsisModel, AppguardLogs, AppguardLogModel, Packets, PacketModel, TempDeviceInterfaceAddresses, TempDeviceInterfaceAddressModel, HttpResponses, HttpResponsisModel, SystemResources, SystemResourceModel, OrganizationContactUserRoles, OrganizationContactUserRoleModel, TempSystemResources, TempSystemResourceModel, DeviceConfigurations, DeviceConfigurationModel, GridFilters, GridFilterModel, Connections, ConnectionModel, DummyPackets, DummyPacketModel, AppFirewalls, AppFirewallModel, DeviceGroupSettings, DeviceGroupSettingModel, DeviceTunnels, DeviceTunnelModel, DeviceServices, DeviceServiceModel, DeviceGroups, DeviceGroupModel, DeviceHeartbeats, DeviceHeartbeatModel, TempDeviceInstances, TempDeviceInstanceModel, Resolutions, ResolutionModel, TempDeviceInterfaces, TempDeviceInterfaceModel, TempAppguardLogs, TempAppguardLogModel, DeviceFilterRules, DeviceFilterRuleModel, Aliases, AliasModel, TcpConnections, TcpConnectionModel, TempDeviceNatRules, TempDeviceNatRuleModel, DeviceInstances, DeviceInstanceModel, TempDeviceServices, TempDeviceServiceModel, TempConnections, TempConnectionModel, HttpRequests, HttpRequestModel, TempPortAliases, TempPortAliasModel, Versions, VersionModel, TempDeviceTunnels, TempDeviceTunnelModel, TempDeviceFilterRules, TempDeviceFilterRuleModel, PortAliases, PortAliasModel, IpInfos, IpInfoModel, DeviceInterfaces, DeviceInterfaceModel, TempWallguardLogs, TempWallguardLogModel, Notifications, NotificationModel, DeviceNatRules, DeviceNatRuleModel, TempIpAliases, TempIpAliasModel, AppguardConfigs, AppguardConfigModel, DeviceInterfaceAddresses, DeviceInterfaceAddressModel, SmtpRequests, SmtpRequestModel, Invitations, InvitationModel, SetupInstructions, SetupInstructionModel, InstallationCodes, InstallationCodeModel, TempDeviceRemoteAccessSessions, TempDeviceRemoteAccessSessionModel, TempPackets, TempPacketModel, WallguardLogs, WallguardLogModel, CommunicationTemplates, CommunicationTemplateModel, TempAliases, TempAliasModel, Locations, LocationModel, DeviceRemoteAccessSessions, DeviceRemoteAccessSessionModel // Add other tables and their models here as needed
        )
    }

    /// Migration-mode upsert with timestamp conflict: always sets all non-None fields.
    pub async fn upsert_record_migration_with_timestamp(
        &self,
        conn: &mut AsyncPgConnection,
        record: Value,
    ) -> Result<(), DieselError> {
        generate_upsert_record_migration_with_timestamp_match!(
            self,
            conn,
            record,
            UserRoles, UserRoleModel, Sessions, SessionModel, SignedInActivities, SignedInActivityModel, ExternalContacts, ExternalContactModel, Organizations, OrganizationModel, OrganizationContacts, OrganizationContactModel, OrganizationAccounts, OrganizationAccountModel, AccountOrganizations, AccountOrganizationModel, AccountProfiles, AccountProfileModel, Accounts, AccountModel, Addresses, AddressModel, Samples, SampleModel, Devices, DeviceModel, PostgresChannels, PostgresChannelModel, Contacts, ContactModel, ContactPhoneNumbers, ContactPhoneNumberModel, ContactEmails, ContactEmailModel, Files, FileModel, TestHypertables, TestHypertableModel, AccountPhoneNumbers, AccountPhoneNumberModel, AccountSignatures, AccountSignatureModel, IpAliases, IpAliasModel, SmtpResponses, SmtpResponsisModel, AppguardLogs, AppguardLogModel, Packets, PacketModel, TempDeviceInterfaceAddresses, TempDeviceInterfaceAddressModel, HttpResponses, HttpResponsisModel, SystemResources, SystemResourceModel, OrganizationContactUserRoles, OrganizationContactUserRoleModel, TempSystemResources, TempSystemResourceModel, DeviceConfigurations, DeviceConfigurationModel, GridFilters, GridFilterModel, Connections, ConnectionModel, DummyPackets, DummyPacketModel, AppFirewalls, AppFirewallModel, DeviceGroupSettings, DeviceGroupSettingModel, DeviceTunnels, DeviceTunnelModel, DeviceServices, DeviceServiceModel, DeviceGroups, DeviceGroupModel, DeviceHeartbeats, DeviceHeartbeatModel, TempDeviceInstances, TempDeviceInstanceModel, Resolutions, ResolutionModel, TempDeviceInterfaces, TempDeviceInterfaceModel, TempAppguardLogs, TempAppguardLogModel, DeviceFilterRules, DeviceFilterRuleModel, Aliases, AliasModel, TcpConnections, TcpConnectionModel, TempDeviceNatRules, TempDeviceNatRuleModel, DeviceInstances, DeviceInstanceModel, TempDeviceServices, TempDeviceServiceModel, TempConnections, TempConnectionModel, HttpRequests, HttpRequestModel, TempPortAliases, TempPortAliasModel, Versions, VersionModel, TempDeviceTunnels, TempDeviceTunnelModel, TempDeviceFilterRules, TempDeviceFilterRuleModel, PortAliases, PortAliasModel, IpInfos, IpInfoModel, DeviceInterfaces, DeviceInterfaceModel, TempWallguardLogs, TempWallguardLogModel, Notifications, NotificationModel, DeviceNatRules, DeviceNatRuleModel, TempIpAliases, TempIpAliasModel, AppguardConfigs, AppguardConfigModel, DeviceInterfaceAddresses, DeviceInterfaceAddressModel, SmtpRequests, SmtpRequestModel, Invitations, InvitationModel, SetupInstructions, SetupInstructionModel, InstallationCodes, InstallationCodeModel, TempDeviceRemoteAccessSessions, TempDeviceRemoteAccessSessionModel, TempPackets, TempPacketModel, WallguardLogs, WallguardLogModel, CommunicationTemplates, CommunicationTemplateModel, TempAliases, TempAliasModel, Locations, LocationModel, DeviceRemoteAccessSessions, DeviceRemoteAccessSessionModel // Add other tables and their models here as needed
        )
    }
}
/// Generate next unique code for the table via code-service (CODE_SERVICE_GRPC_URL).
pub async fn generate_code(
    table: &str,
    prefix_param: &str,
    default_code_param: i32,
) -> Result<String, DieselError> {
    crate::utils::code_generator::generate_code(table, prefix_param, default_code_param).await
}
