#[macro_use]
mod system;

diesel::table! {
    packets (id, timestamp) {
        id -> Uuid,
        timestamp -> Timestamptz, // Timestamp with timezone
        hypertable_timestamp -> Text,
        interface_name -> Text,
        total_length -> Integer,
        source_mac -> Text,
        destination_mac -> Text,
        ether_type -> Text,
        ip_header_length -> Integer,
        payload_length -> Integer,
        protocol -> Text,
        source_ip -> Text,
        destination_ip -> Text,
        source_port -> Nullable<Integer>,
        destination_port -> Nullable<Integer>,
        tcp_header_length -> Nullable<Integer>,
        tcp_sequence_number -> Nullable<BigInt>,
        tcp_acknowledgment_number -> Nullable<BigInt>,
        tcp_data_offset -> Nullable<Integer>,
        tcp_flags -> Nullable<Integer>,
        tcp_window_size -> Nullable<Integer>,
        tcp_urgent_pointer -> Nullable<Integer>,
        icmp_type -> Nullable<Integer>,
        icmp_code -> Nullable<Integer>,
        order -> Text,
        system_fields!(),
    }
}

