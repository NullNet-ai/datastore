
use crate::generated::store;
use std::convert::From;
use crate::models::item_model::Item;
use crate::models::packet_model::Packet;
use chrono::NaiveDateTime;
use uuid::Uuid;



impl From<store::Items> for Item {
    fn from(i: store::Items) -> Self {
        Item{
            tombstone: i.tombstone,
            status: i.status,
            previous_status: i.previous_status,
            version: i.version,
            created_date: i.created_date,
            created_time: i.created_time,
            updated_date: i.updated_date,
            updated_time: i.updated_time,
            organization_id: i.organization_id,
            created_by: i.created_by,
            updated_by: i.updated_by,
            deleted_by: i.deleted_by,
            requested_by: i.requested_by,
            tags: i.tags,
            id: i.id,
            name: i.name,
            description: i.description,
        }
    }
}

impl From<store::Packets> for Packet {
    fn from(i: store::Packets) -> Self {
        Packet{
            tombstone: i.tombstone,
            status: i.status,
            previous_status: i.previous_status,
            version: i.version,
            created_date: i.created_date,
            created_time: i.created_time,
            updated_date: i.updated_date,
            updated_time: i.updated_time,
            organization_id: i.organization_id,
            created_by: i.created_by,
            updated_by: i.updated_by,
            deleted_by: i.deleted_by,
            requested_by: i.requested_by,
            tags: i.tags,
            id: Uuid::parse_str(&i.id).unwrap_or_default(),
            timestamp: match i.timestamp {
                Some(ts) => NaiveDateTime::from_timestamp_opt(ts.seconds, ts.nanos as u32)
                    .unwrap_or_default(),
                None => NaiveDateTime::default(),
            },
            hypertable_timestamp: i.hypertable_timestamp,
            interface_name: i.interface_name,
            total_length: i.total_length,
            device_id: i.device_id.as_ref().and_then(|id| {
                if id.is_empty() {
                    None
                } else {
                    Some(Uuid::parse_str(id).unwrap_or_default())
                }
            }),
            source_mac: i.source_mac,
            destination_mac: i.destination_mac,
            ether_type: i.ether_type,
            ip_header_length: i.ip_header_length,
            payload_length: i.payload_length,
            protocol: i.protocol,
            source_ip: i.source_ip,
            destination_ip: i.destination_ip,
            source_port: i.source_port,
            destination_port: i.destination_port,
            tcp_header_length: i.tcp_header_length,
            tcp_sequence_number: i.tcp_sequence_number,
            tcp_acknowledgment_number: i.tcp_acknowledgment_number,
            tcp_data_offset: i.tcp_data_offset,
            tcp_flags: i.tcp_flags,
            tcp_window_size: i.tcp_window_size,
            tcp_urgent_pointer: i.tcp_urgent_pointer,
            icmp_type: i.icmp_type,
            icmp_code: i.icmp_code,
        }
    }
}

