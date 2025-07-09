use crate::{generate_get_by_id_match, generate_hypertable_timestamp_match, generate_insert_record_match, generate_upsert_record_match, generate_upsert_record_with_timestamp_match};
use crate::models::external_contact_model::ExternalContactModel;
use crate::models::organization_model::OrganizationModel;
use crate::models::organization_contact_model::OrganizationContactModel;
use crate::models::organization_account_model::OrganizationAccountModel;
use crate::models::account_organization_model::AccountOrganizationModel;
use crate::models::account_profile_model::AccountProfileModel;
use crate::models::account_model::AccountModel;
use crate::models::address_model::AddressModel;
use crate::models::sample_model::SampleModel;
use crate::models::app_firewall_model::AppFirewallModel;
use crate::models::appguard_log_model::AppguardLogModel;
use crate::models::temp_appguard_log_model::TempAppguardLogModel;
use crate::models::device_alias_model::DeviceAliasModel;
use crate::models::temp_device_alias_model::TempDeviceAliasModel;
use crate::models::device_configuration_model::DeviceConfigurationModel;
use crate::models::device_interface_address_model::DeviceInterfaceAddressModel;
use crate::models::temp_device_interface_address_model::TempDeviceInterfaceAddressModel;
use crate::models::device_interface_model::DeviceInterfaceModel;
use crate::models::temp_device_interface_model::TempDeviceInterfaceModel;
use crate::models::device_remote_access_session_model::DeviceRemoteAccessSessionModel;
use crate::models::temp_device_remote_access_session_model::TempDeviceRemoteAccessSessionModel;
use crate::models::device_rule_model::DeviceRuleModel;
use crate::models::temp_device_rule_model::TempDeviceRuleModel;
use crate::models::packet_model::PacketModel;
use crate::models::temp_packet_model::TempPacketModel;
use crate::models::connection_model::ConnectionModel;
use crate::models::temp_connection_model::TempConnectionModel;
use crate::models::device_ssh_key_model::DeviceSshKeyModel;
use crate::models::device_model::DeviceModel;
use crate::models::ip_info_model::IpInfoModel;
use crate::models::resolution_model::ResolutionModel;
use crate::models::wallguard_log_model::WallguardLogModel;
use crate::models::temp_wallguard_log_model::TempWallguardLogModel;
use crate::models::device_group_setting_model::DeviceGroupSettingModel;
use crate::models::contact_model::ContactModel;
use crate::models::contact_phone_number_model::ContactPhoneNumberModel;
use crate::models::contact_email_model::ContactEmailModel;
use crate::schema::schema;
use crate::schema::verify::field_exists_in_table;
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
    ExternalContacts,
    Organizations,
    OrganizationContacts,
    OrganizationAccounts,
    AccountOrganizations,
    AccountProfiles,
    Accounts,
    Addresses,
    Samples,
    AppFirewalls,
    AppguardLogs,
    TempAppguardLogs,
    DeviceAliases,
    TempDeviceAliases,
    DeviceConfigurations,
    DeviceInterfaceAddresses,
    TempDeviceInterfaceAddresses,
    DeviceInterfaces,
    TempDeviceInterfaces,
    DeviceRemoteAccessSessions,
    TempDeviceRemoteAccessSessions,
    DeviceRules,
    TempDeviceRules,
    Packets,
    TempPackets,
    Connections,
    TempConnections,
    DeviceSshKeys,
    Devices,
    IpInfos,
    Resolutions,
    WallguardLogs,
    TempWallguardLogs,
    DeviceGroupSettings,
    Contacts,
    ContactPhoneNumbers,
    ContactEmails,
    // Add other tables here
}

impl Table {
    pub fn from_str(name: &str) -> Option<Self> {
        match name {
            "external_contacts" => Some(Table::ExternalContacts),
            "organizations" => Some(Table::Organizations),
            "organization_contacts" => Some(Table::OrganizationContacts),
            "organization_accounts" => Some(Table::OrganizationAccounts),
            "account_organizations" => Some(Table::AccountOrganizations),
            "account_profiles" => Some(Table::AccountProfiles),
            "accounts" => Some(Table::Accounts),
            "addresses" => Some(Table::Addresses),
            "samples" => Some(Table::Samples),
            "app_firewalls" => Some(Table::AppFirewalls),
            "appguard_logs" => Some(Table::AppguardLogs),
            "temp_appguard_logs" => Some(Table::TempAppguardLogs),
            "device_aliases" => Some(Table::DeviceAliases),
            "temp_device_aliases" => Some(Table::TempDeviceAliases),
            "device_configurations" => Some(Table::DeviceConfigurations),
            "device_interface_addresses" => Some(Table::DeviceInterfaceAddresses),
            "temp_device_interface_addresses" => Some(Table::TempDeviceInterfaceAddresses),
            "device_interfaces" => Some(Table::DeviceInterfaces),
            "temp_device_interfaces" => Some(Table::TempDeviceInterfaces),
            "device_remote_access_sessions" => Some(Table::DeviceRemoteAccessSessions),
            "temp_device_remote_access_sessions" => Some(Table::TempDeviceRemoteAccessSessions),
            "device_rules" => Some(Table::DeviceRules),
            "temp_device_rules" => Some(Table::TempDeviceRules),
            "packets" => Some(Table::Packets),
            "temp_packets" => Some(Table::TempPackets),
            "connections" => Some(Table::Connections),
            "temp_connections" => Some(Table::TempConnections),
            "device_ssh_keys" => Some(Table::DeviceSshKeys),
            "devices" => Some(Table::Devices),
            "ip_infos" => Some(Table::IpInfos),
            "resolutions" => Some(Table::Resolutions),
            "wallguard_logs" => Some(Table::WallguardLogs),
            "temp_wallguard_logs" => Some(Table::TempWallguardLogs),
            "device_group_settings" => Some(Table::DeviceGroupSettings),
            "contacts" => Some(Table::Contacts),
            "contact_phone_numbers" => Some(Table::ContactPhoneNumbers),
            "contact_emails" => Some(Table::ContactEmails),
            // Add other tables here
            _ => None,
        }
    }

    pub fn pluck_fields(&self, record_value: &Value, pluck_fields: Vec<String>) -> Value {
        if !pluck_fields.is_empty() && record_value.is_object() {
            let obj = record_value.as_object().unwrap();
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
    }

    pub async fn get_hypertable_timestamp(
        &self,
        conn: &mut AsyncPgConnection,
        id: &str,
    ) -> Result<Option<String>, DieselError> {
        generate_hypertable_timestamp_match!(self, conn, id, Packets, TempPackets, Connections, TempConnections, WallguardLogs, TempWallguardLogs)
    }

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
            ExternalContacts, ExternalContactModel, Organizations, OrganizationModel, OrganizationContacts, OrganizationContactModel, OrganizationAccounts, OrganizationAccountModel, AccountOrganizations, AccountOrganizationModel, AccountProfiles, AccountProfileModel, Accounts, AccountModel, Addresses, AddressModel, Samples, SampleModel, AppFirewalls, AppFirewallModel, AppguardLogs, AppguardLogModel, TempAppguardLogs, TempAppguardLogModel, DeviceAliases, DeviceAliasModel, TempDeviceAliases, TempDeviceAliasModel, DeviceConfigurations, DeviceConfigurationModel, DeviceInterfaceAddresses, DeviceInterfaceAddressModel, TempDeviceInterfaceAddresses, TempDeviceInterfaceAddressModel, DeviceInterfaces, DeviceInterfaceModel, TempDeviceInterfaces, TempDeviceInterfaceModel, DeviceRemoteAccessSessions, DeviceRemoteAccessSessionModel, TempDeviceRemoteAccessSessions, TempDeviceRemoteAccessSessionModel, DeviceRules, DeviceRuleModel, TempDeviceRules, TempDeviceRuleModel, Packets, PacketModel, TempPackets, TempPacketModel, Connections, ConnectionModel, TempConnections, TempConnectionModel, DeviceSshKeys, DeviceSshKeyModel, Devices, DeviceModel, IpInfos, IpInfoModel, Resolutions, ResolutionModel, WallguardLogs, WallguardLogModel, TempWallguardLogs, TempWallguardLogModel, DeviceGroupSettings, DeviceGroupSettingModel, Contacts, ContactModel, ContactPhoneNumbers, ContactPhoneNumberModel, ContactEmails, ContactEmailModel // Add other tables and their models here as needed
        )
    }

    pub async fn get_by_id(
        &self,
        conn: &mut AsyncPgConnection,
        id: &str,
    ) -> Result<Option<Value>, DieselError> {
        generate_get_by_id_match!(
            self,
            conn,
            id,
            ExternalContacts, ExternalContactModel, Organizations, OrganizationModel, OrganizationContacts, OrganizationContactModel, OrganizationAccounts, OrganizationAccountModel, AccountOrganizations, AccountOrganizationModel, AccountProfiles, AccountProfileModel, Accounts, AccountModel, Addresses, AddressModel, Samples, SampleModel, AppFirewalls, AppFirewallModel, AppguardLogs, AppguardLogModel, TempAppguardLogs, TempAppguardLogModel, DeviceAliases, DeviceAliasModel, TempDeviceAliases, TempDeviceAliasModel, DeviceConfigurations, DeviceConfigurationModel, DeviceInterfaceAddresses, DeviceInterfaceAddressModel, TempDeviceInterfaceAddresses, TempDeviceInterfaceAddressModel, DeviceInterfaces, DeviceInterfaceModel, TempDeviceInterfaces, TempDeviceInterfaceModel, DeviceRemoteAccessSessions, DeviceRemoteAccessSessionModel, TempDeviceRemoteAccessSessions, TempDeviceRemoteAccessSessionModel, DeviceRules, DeviceRuleModel, TempDeviceRules, TempDeviceRuleModel, Packets, PacketModel, TempPackets, TempPacketModel, Connections, ConnectionModel, TempConnections, TempConnectionModel, DeviceSshKeys, DeviceSshKeyModel, Devices, DeviceModel, IpInfos, IpInfoModel, Resolutions, ResolutionModel, WallguardLogs, WallguardLogModel, TempWallguardLogs, TempWallguardLogModel, DeviceGroupSettings, DeviceGroupSettingModel, Contacts, ContactModel, ContactPhoneNumbers, ContactPhoneNumberModel, ContactEmails, ContactEmailModel // Add other tables and their models here as needed
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
            ExternalContacts, ExternalContactModel, Organizations, OrganizationModel, OrganizationContacts, OrganizationContactModel, OrganizationAccounts, OrganizationAccountModel, AccountOrganizations, AccountOrganizationModel, AccountProfiles, AccountProfileModel, Accounts, AccountModel, Addresses, AddressModel, Samples, SampleModel, AppFirewalls, AppFirewallModel, AppguardLogs, AppguardLogModel, TempAppguardLogs, TempAppguardLogModel, DeviceAliases, DeviceAliasModel, TempDeviceAliases, TempDeviceAliasModel, DeviceConfigurations, DeviceConfigurationModel, DeviceInterfaceAddresses, DeviceInterfaceAddressModel, TempDeviceInterfaceAddresses, TempDeviceInterfaceAddressModel, DeviceInterfaces, DeviceInterfaceModel, TempDeviceInterfaces, TempDeviceInterfaceModel, DeviceRemoteAccessSessions, DeviceRemoteAccessSessionModel, TempDeviceRemoteAccessSessions, TempDeviceRemoteAccessSessionModel, DeviceRules, DeviceRuleModel, TempDeviceRules, TempDeviceRuleModel, Packets, PacketModel, TempPackets, TempPacketModel, Connections, ConnectionModel, TempConnections, TempConnectionModel, DeviceSshKeys, DeviceSshKeyModel, Devices, DeviceModel, IpInfos, IpInfoModel, Resolutions, ResolutionModel, WallguardLogs, WallguardLogModel, TempWallguardLogs, TempWallguardLogModel, DeviceGroupSettings, DeviceGroupSettingModel, Contacts, ContactModel, ContactPhoneNumbers, ContactPhoneNumberModel, ContactEmails, ContactEmailModel // Add other tables and their models here as needed
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
            ExternalContacts, ExternalContactModel, Organizations, OrganizationModel, OrganizationContacts, OrganizationContactModel, OrganizationAccounts, OrganizationAccountModel, AccountOrganizations, AccountOrganizationModel, AccountProfiles, AccountProfileModel, Accounts, AccountModel, Addresses, AddressModel, Samples, SampleModel, AppFirewalls, AppFirewallModel, AppguardLogs, AppguardLogModel, TempAppguardLogs, TempAppguardLogModel, DeviceAliases, DeviceAliasModel, TempDeviceAliases, TempDeviceAliasModel, DeviceConfigurations, DeviceConfigurationModel, DeviceInterfaceAddresses, DeviceInterfaceAddressModel, TempDeviceInterfaceAddresses, TempDeviceInterfaceAddressModel, DeviceInterfaces, DeviceInterfaceModel, TempDeviceInterfaces, TempDeviceInterfaceModel, DeviceRemoteAccessSessions, DeviceRemoteAccessSessionModel, TempDeviceRemoteAccessSessions, TempDeviceRemoteAccessSessionModel, DeviceRules, DeviceRuleModel, TempDeviceRules, TempDeviceRuleModel, Packets, PacketModel, TempPackets, TempPacketModel, Connections, ConnectionModel, TempConnections, TempConnectionModel, DeviceSshKeys, DeviceSshKeyModel, Devices, DeviceModel, IpInfos, IpInfoModel, Resolutions, ResolutionModel, WallguardLogs, WallguardLogModel, TempWallguardLogs, TempWallguardLogModel, DeviceGroupSettings, DeviceGroupSettingModel, Contacts, ContactModel, ContactPhoneNumbers, ContactPhoneNumberModel, ContactEmails, ContactEmailModel // Add other tables and their models here as needed
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
