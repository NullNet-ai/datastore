diesel::table! {
    items (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    packets (id, timestamp) {
        id -> Uuid,
        timestamp -> Timestamptz,
        hypertable_timestamp -> Text,
        interface_name -> Text,
        total_length -> Nullable<Int4>,
        source_mac -> Nullable<Text>,
        destination_mac -> Nullable<Text>,
        ether_type -> Nullable<Text>,
        ip_header_length -> Int4,
        payload_length -> Int4,
        protocol -> Text,
        source_ip -> Text,
        destination_ip -> Text,
        source_port -> Nullable<Int4>,
        destination_port -> Nullable<Int4>,
        tcp_header_length -> Nullable<Int4>,
        tcp_sequence_number -> Nullable<Int8>,
        tcp_acknowledgment_number -> Nullable<Int8>,
        tcp_data_offset -> Nullable<Int4>,
        tcp_flags -> Nullable<Int4>,
        tcp_window_size -> Nullable<Int4>,
        tcp_urgent_pointer -> Nullable<Int4>,
        icmp_type -> Nullable<Int4>,
        icmp_code -> Nullable<Int4>,
        order -> Nullable<Text>,
    }
}