pub mod account_model;
pub mod account_organization_model;
pub mod account_profile_model;
pub mod address_model;

pub mod contact_email_model;
pub mod contact_model;
pub mod contact_phone_number_model;
pub mod counter_model;
pub mod crdt_merkle_model;
pub mod crdt_message_model;
pub mod data_permission_model;

pub mod device_model;

pub mod encryption_key_model;
pub mod entity_field_model;
pub mod entity_model;
pub mod external_contact_model;
pub mod field_model;
pub mod file_model;
pub mod organization_account_model;
pub mod organization_contact_model;
pub mod organization_domain_model;
pub mod organization_model;

pub mod permission_model;
pub mod postgres_channel_model;

pub mod queue_item_model;
pub mod queue_model;
pub mod record_permission_model;
pub mod test_hypertable_model;
pub mod transaction_model;
pub mod role_permission_model;
pub mod sample_model;
pub mod session_model;
pub mod stream_queue_item_model;
pub mod stream_queue_model;
pub mod sync_endpoint_model;
pub mod system_config_field_model;
pub mod table_index_model;

pub mod user_role_model;

// diesel_ext --model --derive "Queryable, Selectable, Insertable, Serialize, Deserialize, Clone" --import-types "diesel::prelude::*" --import-types "serde::{Deserialize, Serialize}" --add-table-name > models.rs