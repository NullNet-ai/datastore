pub mod connection_model;
pub mod counter_model;
pub mod crdt_merkle_model;
pub mod crdt_message_model;
pub mod device_ssh_key_model;
pub mod packet_model;
pub mod queue_item_model;
pub mod queue_model;
pub mod sync_endpoint_model;
pub mod transaction_model;
pub mod device_group_setting_model;
pub mod device_model;
pub mod address_model;
pub mod app_firewall_model;
pub mod appguard_log_model;
pub mod device_alias_model;
// diesel_ext --model --derive "Queryable, Selectable, Insertable, Serialize, Deserialize, Clone" --import-types "diesel::prelude::*" --import-types "serde::{Deserialize, Serialize}" --add-table-name > models.rs
