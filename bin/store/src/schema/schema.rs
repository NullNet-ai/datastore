use diesel::table;

table! {
    items (id) {
        tombstone -> Int4,
        status -> Text,
        previous_status -> Nullable<Text>,
        version -> Int4,
        created_date -> Nullable<Text>,
        created_time -> Nullable<Text>,
        updated_date -> Nullable<Text>,
        updated_time -> Nullable<Text>,
        organization_id -> Nullable<Text>,
        created_by -> Nullable<Text>,
        updated_by -> Nullable<Text>,
        deleted_by -> Nullable<Text>,
        requested_by -> Nullable<Text>,
        tags -> Array<Text>,

        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
    }
}

table! {
    packets (id, timestamp) {
         tombstone -> Int4,
        status -> Text,
        previous_status -> Nullable<Text>,
        version -> Int4,
        created_date -> Nullable<Text>,
        created_time -> Nullable<Text>,
        updated_date -> Nullable<Text>,
        updated_time -> Nullable<Text>,
        organization_id -> Nullable<Text>,
        created_by -> Nullable<Text>,
        updated_by -> Nullable<Text>,
        deleted_by -> Nullable<Text>,
        requested_by -> Nullable<Text>,
        tags -> Array<Text>,

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