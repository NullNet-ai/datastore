// @generated automatically by Diesel CLI.

diesel::table! {
    allowed_fields (id) {
        id -> Text,
        categories -> Nullable<Array<Nullable<Text>>>,
        code -> Nullable<Text>,
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
        timestamp -> Nullable<Text>,
        tags -> Nullable<Array<Nullable<Text>>>,
        label -> Nullable<Text>,
        name -> Nullable<Text>,
        #[sql_name = "type"]
        type_ -> Nullable<Text>,
        class_type_id -> Nullable<Text>,
        is_optional -> Nullable<Bool>,
        is_primary_key -> Nullable<Bool>,
        reference_to -> Nullable<Text>,
        data_type -> Nullable<Text>,
        default_value -> Nullable<Text>,
    }
}

diesel::table! {
    class_types (id) {
        id -> Text,
        categories -> Nullable<Array<Nullable<Text>>>,
        code -> Nullable<Text>,
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
        timestamp -> Nullable<Text>,
        tags -> Nullable<Array<Nullable<Text>>>,
        #[sql_name = "type"]
        type_ -> Nullable<Text>,
        company -> Nullable<Text>,
        entity -> Nullable<Text>,
        is_list -> Nullable<Bool>,
        is_with_version -> Nullable<Bool>,
        schema_version -> Nullable<Text>,
    }
}

diesel::table! {
    config_applications (id) {
        id -> Text,
        categories -> Nullable<Array<Nullable<Text>>>,
        code -> Nullable<Text>,
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
        timestamp -> Nullable<Text>,
        tags -> Nullable<Array<Nullable<Text>>>,
        #[sql_name = "type"]
        type_ -> Nullable<Text>,
        value -> Nullable<Text>,
    }
}

diesel::table! {
    config_sync (id) {
        id -> Text,
        categories -> Nullable<Array<Nullable<Text>>>,
        code -> Nullable<Text>,
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
        timestamp -> Nullable<Text>,
        tags -> Nullable<Array<Nullable<Text>>>,
        #[sql_name = "type"]
        type_ -> Nullable<Text>,
        value -> Nullable<Text>,
    }
}

diesel::table! {
    contact_emails (id) {
        id -> Text,
        categories -> Nullable<Array<Nullable<Text>>>,
        code -> Nullable<Text>,
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
        timestamp -> Nullable<Text>,
        tags -> Nullable<Array<Nullable<Text>>>,
        contact_id -> Nullable<Text>,
        email -> Nullable<Text>,
    }
}

diesel::table! {
    contact_phone_numbers (id) {
        id -> Text,
        categories -> Nullable<Array<Nullable<Text>>>,
        code -> Nullable<Text>,
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
        timestamp -> Nullable<Text>,
        tags -> Nullable<Array<Nullable<Text>>>,
        contact_id -> Nullable<Text>,
        phone_number_raw -> Nullable<Text>,
    }
}

diesel::table! {
    contacts (id) {
        id -> Text,
        categories -> Nullable<Array<Nullable<Text>>>,
        code -> Nullable<Text>,
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
        timestamp -> Nullable<Text>,
        tags -> Nullable<Array<Nullable<Text>>>,
        first_name -> Nullable<Text>,
        middle_name -> Nullable<Text>,
        last_name -> Nullable<Text>,
        date_of_birth -> Nullable<Text>,
    }
}

diesel::table! {
    counters (entity) {
        entity -> Text,
        default_code -> Nullable<Int4>,
        prefix -> Nullable<Text>,
        counter -> Nullable<Int4>,
    }
}

diesel::table! {
    crdt_merkles (group_id) {
        group_id -> Text,
        timestamp -> Text,
        merkle -> Text,
    }
}

diesel::table! {
    crdt_messages (timestamp, group_id, row, column) {
        database -> Nullable<Text>,
        dataset -> Text,
        group_id -> Text,
        timestamp -> Text,
        row -> Text,
        column -> Text,
        client_id -> Text,
        value -> Text,
        operation -> Nullable<Text>,
        hypertable_timestamp -> Nullable<Text>,
    }
}

diesel::table! {
    dead_letter_queue (id) {
        id -> Uuid,
        record_id -> Uuid,
        created_date -> Nullable<Timestamp>,
        table -> Text,
        prefix -> Text,
        error -> Text,
    }
}

diesel::table! {
    devices (id) {
        id -> Text,
        categories -> Nullable<Array<Nullable<Text>>>,
        code -> Nullable<Text>,
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
        timestamp -> Nullable<Text>,
        tags -> Nullable<Array<Nullable<Text>>>,
    }
}

diesel::table! {
    fields (id) {
        id -> Text,
        categories -> Nullable<Array<Nullable<Text>>>,
        code -> Nullable<Text>,
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
        timestamp -> Nullable<Text>,
        tags -> Nullable<Array<Nullable<Text>>>,
        label -> Nullable<Text>,
        name -> Nullable<Text>,
        #[sql_name = "type"]
        type_ -> Nullable<Text>,
        assigned_to -> Nullable<Text>,
    }
}

diesel::table! {
    files (id) {
        id -> Text,
        categories -> Nullable<Array<Nullable<Text>>>,
        code -> Nullable<Text>,
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
        timestamp -> Nullable<Text>,
        tags -> Nullable<Array<Nullable<Text>>>,
        fieldname -> Nullable<Text>,
        originalname -> Nullable<Text>,
        encoding -> Nullable<Text>,
        mimetype -> Nullable<Text>,
        destination -> Nullable<Text>,
        filename -> Nullable<Text>,
        path -> Nullable<Text>,
        size -> Nullable<Int4>,
        uploaded_by -> Nullable<Text>,
        downloaded_by -> Nullable<Text>,
        etag -> Nullable<Text>,
        versionId -> Nullable<Text>,
        download_path -> Nullable<Text>,
    }
}

diesel::table! {
    organization_accounts (id) {
        id -> Text,
        categories -> Nullable<Array<Nullable<Text>>>,
        code -> Nullable<Text>,
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
        timestamp -> Nullable<Text>,
        tags -> Nullable<Array<Nullable<Text>>>,
        organization_contact_id -> Nullable<Text>,
        account_id -> Nullable<Text>,
        account_secret -> Nullable<Text>,
        role_id -> Nullable<Text>,
        contact_id -> Nullable<Text>,
        device_id -> Nullable<Text>,
    }
}

diesel::table! {
    organization_contacts (id) {
        id -> Text,
        categories -> Nullable<Array<Nullable<Text>>>,
        code -> Nullable<Text>,
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
        timestamp -> Nullable<Text>,
        tags -> Nullable<Array<Nullable<Text>>>,
        contact_id -> Nullable<Text>,
    }
}

diesel::table! {
    organization_domains (id) {
        id -> Text,
        categories -> Nullable<Array<Nullable<Text>>>,
        code -> Nullable<Text>,
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
        timestamp -> Nullable<Text>,
        tags -> Nullable<Array<Nullable<Text>>>,
        domain_name -> Nullable<Text>,
    }
}

diesel::table! {
    organization_files (id) {
        id -> Text,
        categories -> Nullable<Array<Nullable<Text>>>,
        code -> Nullable<Text>,
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
        timestamp -> Nullable<Text>,
        tags -> Nullable<Array<Nullable<Text>>>,
        organizaion_id -> Nullable<Text>,
        organization_contact_id -> Nullable<Text>,
        url -> Nullable<Text>,
        name -> Nullable<Text>,
        mime_type -> Nullable<Text>,
        size -> Nullable<Text>,
        #[sql_name = "type"]
        type_ -> Nullable<Text>,
    }
}

diesel::table! {
    organizations (id) {
        id -> Text,
        categories -> Nullable<Array<Nullable<Text>>>,
        code -> Nullable<Text>,
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
        timestamp -> Nullable<Text>,
        tags -> Nullable<Array<Nullable<Text>>>,
        parent_organization_id -> Nullable<Text>,
        name -> Nullable<Text>,
    }
}

diesel::table! {
    packets (id, timestamp) {
        id -> Uuid,
        categories -> Nullable<Array<Nullable<Text>>>,
        code -> Nullable<Text>,
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
        timestamp -> Timestamptz,
        tags -> Nullable<Array<Nullable<Text>>>,
        hypertable_timestamp -> Nullable<Text>,
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

diesel::table! {
    queue_items (id) {
        id -> Text,
        order -> Int4,
        queue_id -> Text,
        value -> Text,
    }
}

diesel::table! {
    queues (id) {
        id -> Text,
        name -> Text,
        count -> Int4,
        size -> Int4,
    }
}

diesel::table! {
    samples (id) {
        id -> Text,
        categories -> Nullable<Array<Nullable<Text>>>,
        code -> Nullable<Text>,
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
        timestamp -> Nullable<Text>,
        tags -> Nullable<Array<Nullable<Text>>>,
        sample_text -> Nullable<Text>,
    }
}

diesel::table! {
    sync_endpoints (id) {
        id -> Text,
        name -> Nullable<Text>,
        url -> Nullable<Text>,
        group_id -> Nullable<Text>,
        username -> Nullable<Text>,
        password -> Nullable<Text>,
        status -> Nullable<Text>,
    }
}

diesel::table! {
    temp_packets (id) {
        id -> Uuid,
        categories -> Nullable<Array<Nullable<Text>>>,
        code -> Nullable<Text>,
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
        timestamp -> Timestamptz,
        tags -> Nullable<Array<Nullable<Text>>>,
        hypertable_timestamp -> Nullable<Text>,
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
    }
}

diesel::table! {
    temp_wallguard_logs (id, timestamp) {
        id -> Uuid,
        categories -> Nullable<Array<Nullable<Text>>>,
        code -> Nullable<Text>,
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
        timestamp -> Timestamptz,
        tags -> Nullable<Array<Nullable<Text>>>,
        level -> Nullable<Text>,
        message -> Nullable<Text>,
    }
}

diesel::table! {
    transactions (id) {
        id -> Text,
        timestamp -> Text,
        status -> Text,
        expiry -> Nullable<Int8>,
    }
}

diesel::table! {
    wallguard_logs (id, timestamp) {
        id -> Uuid,
        categories -> Nullable<Array<Nullable<Text>>>,
        code -> Nullable<Text>,
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
        timestamp -> Timestamptz,
        tags -> Nullable<Array<Nullable<Text>>>,
        level -> Nullable<Text>,
        message -> Nullable<Text>,
    }
}

diesel::joinable!(allowed_fields -> class_types (class_type_id));
diesel::joinable!(allowed_fields -> organizations (organization_id));
diesel::joinable!(class_types -> organizations (organization_id));
diesel::joinable!(config_applications -> organizations (organization_id));
diesel::joinable!(config_sync -> organizations (organization_id));
diesel::joinable!(contact_emails -> contacts (contact_id));
diesel::joinable!(contact_emails -> organizations (organization_id));
diesel::joinable!(contact_phone_numbers -> contacts (contact_id));
diesel::joinable!(contact_phone_numbers -> organizations (organization_id));
diesel::joinable!(contacts -> organizations (organization_id));
diesel::joinable!(devices -> organizations (organization_id));
diesel::joinable!(fields -> organizations (organization_id));
diesel::joinable!(files -> organizations (organization_id));
diesel::joinable!(organization_contacts -> contacts (contact_id));
diesel::joinable!(organization_contacts -> organizations (organization_id));
diesel::joinable!(organization_domains -> organizations (organization_id));
diesel::joinable!(organization_files -> organization_contacts (organization_contact_id));
diesel::joinable!(packets -> organizations (organization_id));
diesel::joinable!(samples -> organizations (organization_id));
diesel::joinable!(temp_packets -> organizations (organization_id));
diesel::joinable!(temp_wallguard_logs -> organizations (organization_id));
diesel::joinable!(wallguard_logs -> organizations (organization_id));

diesel::allow_tables_to_appear_in_same_query!(
    allowed_fields,
    class_types,
    config_applications,
    config_sync,
    contact_emails,
    contact_phone_numbers,
    contacts,
    counters,
    crdt_merkles,
    crdt_messages,
    dead_letter_queue,
    devices,
    fields,
    files,
    organization_accounts,
    organization_contacts,
    organization_domains,
    organization_files,
    organizations,
    packets,
    queue_items,
    queues,
    samples,
    sync_endpoints,
    temp_packets,
    temp_wallguard_logs,
    transactions,
    wallguard_logs,
);
