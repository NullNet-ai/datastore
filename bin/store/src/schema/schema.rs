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
    packets (id) {
        tombstone -> Integer,
        status -> Text,
        previous_status -> Nullable<Text>,
        version -> Integer,
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
        timestamp -> Timestamp,
        hypertable_timestamp -> Text,
        interface_name -> Text,
        total_length -> Nullable<Integer>,
        device_id -> Nullable<Uuid>,
        source_mac -> Nullable<Text>,
        destination_mac -> Nullable<Text>,
        ether_type -> Nullable<Text>,
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
    }
}

table! {
    crdt_messages (timestamp, group_id, row, column) {
        database -> Nullable<Text>,
        dataset -> Text,
        group_id -> Text,
        timestamp -> Text,
        row -> Text,
        column -> Text,
        client_id -> Text,
        value -> Text,
        operation -> Text,
        hypertable_timestamp -> Nullable<Text>,
    }
}

table! {
    crdt_merkles (group_id) {
        group_id -> Text,
        timestamp -> Text,
        merkle -> Text,
    }
}

table! {
    sync_endpoints (id) {
        id -> Text,
        name -> Text,
        url -> Text,
        group_id -> Text,
        username -> Text,
        password -> Text,
        status -> Text,
    }
}

// Add these table definitions to your schema.rs file

table! {
    queues (id) {
        id -> Text,
        name -> Text,
        size -> Integer,
        count -> Integer,
    }
}

table! {
    queue_items (id) {
        id -> Text,
        order -> Integer,
        queue_id -> Text,
        value -> Text,
    }
}

// Add this table definition to your schema.rs file

table! {
    transactions (id) {
        id -> Text,
        timestamp -> Text,
        status -> Text,
        expiry -> BigInt,
    }
}
