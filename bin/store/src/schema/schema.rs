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
        tombstone -> Nullable<Int4>,
        status -> Nullable<Text>,
        previous_status -> Nullable<Text>,
        version -> Nullable<Int4>,
        created_date -> Nullable<Text>,
        created_time -> Nullable<Text>,
        updated_date -> Nullable<Text>,
        updated_time -> Nullable<Text>,
        organization_id -> Nullable<Text>,
        created_by -> Nullable<Text>,
        updated_by -> Nullable<Text>,
        deleted_by -> Nullable<Text>,
        requested_by -> Nullable<Text>,
        tags -> Nullable<Array<Text>>,

        id -> Text,
        timestamp -> Timestamp,
        hypertable_timestamp -> Nullable<Text>,
        interface_name -> Nullable<Text>,
        device_id -> Nullable<Uuid>,
        source_mac -> Nullable<Text>,
        destination_mac -> Nullable<Text>,
        ether_type -> Nullable<Text>,
        protocol -> Nullable<Text>,
        total_length -> Nullable<Int4>,
        source_ip -> Nullable<Inet>,
        destination_ip -> Nullable<Inet>,
        source_port -> Nullable<Int4>,
        destination_port -> Nullable<Int4>,
        tcp_header_length -> Nullable<Int4>,
        tcp_sequence_number -> Nullable<BigInt>,
        tcp_acknowledgment_number -> Nullable<BigInt>,
        tcp_data_offset -> Nullable<Int4>,
        tcp_flags -> Nullable<Int4>,
        tcp_window_size -> Nullable<Int4>,
        tcp_urgent_pointer -> Nullable<Int4>,
        icmp_type -> Nullable<Int4>,
        icmp_code -> Nullable<Int4>,
    }
}

table! {
    connections (id, timestamp) {
        tombstone -> Nullable<Int4>,
        status -> Nullable<Text>,
        previous_status -> Nullable<Text>,
        version -> Nullable<Int4>,
        created_date -> Nullable<Text>,
        created_time -> Nullable<Text>,
        updated_date -> Nullable<Text>,
        updated_time -> Nullable<Text>,
        organization_id -> Nullable<Text>,
        created_by -> Nullable<Text>,
        updated_by -> Nullable<Text>,
        deleted_by -> Nullable<Text>,
        requested_by -> Nullable<Text>,
        tags -> Nullable<Array<Text>>,

        id -> Text,
        timestamp -> Timestamp,
        interface_name -> Nullable<Text>,
        hypertable_timestamp -> Nullable<Text>,
        total_packet -> Nullable<Int4>,
        total_byte -> Nullable<Int4>,
        device_id -> Nullable<Text>,
        protocol -> Nullable<Text>,
        source_ip -> Nullable<Inet>,
        destination_ip -> Nullable<Inet>,
        remote_ip -> Nullable<Inet>,
        source_port -> Nullable<Int4>,
        destination_port -> Nullable<Int4>,
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
        size -> Int4,
        count -> Int4,
    }
}

table! {
    queue_items (id) {
        id -> Text,
        order -> Int4,
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
