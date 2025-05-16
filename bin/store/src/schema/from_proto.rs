use crate::generated::store;
use crate::models::packet_model::PacketModel;
use chrono::NaiveDateTime;
use std::convert::From;
use uuid::Uuid;

// impl From<store::Items> for Item {
//     fn from(i: store::Items) -> Self {
//         Item {
//             tombstone: i.tombstone,
//             status: i.status,
//             previous_status: i.previous_status,
//             version: i.version,
//             created_date: i.created_date,
//             created_time: i.created_time,
//             updated_date: i.updated_date,
//             updated_time: i.updated_time,
//             organization_id: i.organization_id,
//             created_by: i.created_by,
//             updated_by: i.updated_by,
//             deleted_by: i.deleted_by,
//             requested_by: i.requested_by,
//             tags: i.tags,
//             id: i.id,
//             name: i.name,
//             description: i.description,
//         }
//     }
// }
