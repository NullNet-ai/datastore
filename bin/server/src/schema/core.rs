use diesel::{allow_tables_to_appear_in_same_query, joinable, table};

table! {
    sync_endpoint_groups (group_id, sync_endpoint_id) {
        sync_endpoint_id -> Integer,
        group_id -> Text,
        status -> Text,
    }
}

table! {
    sync_endpoints (id) {
        id -> Serial,
        url -> Text,
        auth_username -> Text,
        auth_password -> Text,
        sync_interval -> Integer,
    }
}

table! {
    sync_queue_items (id) {
        id -> Text,
        order -> Integer,
        group_id -> Text,
        value -> Text,
    }
}

table! {
    sync_queues (group_id) {
        group_id -> Text,
        count -> Integer,
        size -> Integer,
    }
}

table! {
    sync_transactions (id) {
        id -> Text,
        timestamp -> Text,
        group_id -> Text,
        sync_endpoint_id -> Integer,
        status -> Text,
        expiry -> Nullable<BigInt>,
    }
}

table! {
    crdt_client_messages (record_id) {
        record_id -> Text,
        client_id -> Text,
        message -> Text,
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
        hypertable_timestamp -> Nullable<Text>,
    }
}

table! {
    crdt_messages_merkles (group_id) {
        group_id -> Text,
        merkle -> Text,
    }
}

joinable!(sync_endpoint_groups -> sync_endpoints (sync_endpoint_id));
allow_tables_to_appear_in_same_query!(
    sync_endpoint_groups,
    sync_endpoints,
    sync_queue_items,
    sync_queues,
);
