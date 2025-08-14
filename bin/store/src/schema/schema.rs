use diesel::{allow_tables_to_appear_in_same_query, joinable, table};

//permissions

table! {
    system_config_fields(field_id) {
        field_id -> Nullable<Text>,
        is_searchable -> Nullable<Bool>,
        is_system_field -> Nullable<Bool>,
        is_encryptable -> Nullable<Bool>,
        is_allowed_to_return -> Nullable<Bool>,
        created_by -> Nullable<Text>,
        updated_by -> Nullable<Text>,
        deleted_by -> Nullable<Text>,
        timestamp -> Nullable<Timestamp>,
        tombstone -> Nullable<Int4>,
    }
}

table! {
    data_permissions(id) {
        id -> Nullable<Text>,
        entity_field_id -> Nullable<Text>,
        account_organization_id -> Nullable<Text>,
        version -> Nullable<Int4>,
        created_by -> Nullable<Text>,
        updated_by -> Nullable<Text>,
        deleted_by -> Nullable<Text>,
        timestamp -> Nullable<Text>,
        tombstone -> Nullable<Int4>,

        permission_id -> Nullable<Text>,
        record_id -> Nullable<Text>,
        record_entity -> Nullable<Text>,

    }
}

table! {
    role_permissions(id) {
        id -> Nullable<Text>,
        role_id -> Nullable<Text>,
        permission_id -> Nullable<Text>,
        created_by -> Nullable<Text>,
        updated_by -> Nullable<Text>,
        deleted_by -> Nullable<Text>,
        timestamp -> Nullable<Timestamp>,
        tombstone -> Nullable<Int4>,
    }
}
table! {
    record_permissions(id) {
        id -> Nullable<Text>,
        record_id -> Nullable<Text>,
        record_entity -> Nullable<Text>,
        permission_id -> Nullable<Text>,
        created_by -> Nullable<Text>,
        updated_by -> Nullable<Text>,
        deleted_by -> Nullable<Text>,
        timestamp -> Nullable<Timestamp>,
        tombstone -> Nullable<Int4>,
    }
}

table! {
    table_indexes(id) {
        id -> Nullable<Text>,
        entity_id -> Nullable<Text>,
        secondary_index -> Nullable<Text>,
        compound_index -> Nullable<Jsonb>,
        created_by -> Nullable<Text>,
        updated_by -> Nullable<Text>,
        deleted_by -> Nullable<Text>,
        timestamp -> Nullable<Timestamp>,
        tombstone -> Nullable<Int4>,
    }
}

table! {
    user_roles(id) {
        id -> Nullable<Text>,
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
        categories -> Nullable<Array<Text>>,
        code -> Nullable<Text>,
        timestamp -> Nullable<Timestamp>,
        sync_status -> Nullable<Text>,
        is_batch -> Nullable<Bool>,

        role -> Nullable<Text>,
        entity -> Nullable<Text>,
        sensitivity_level -> Nullable<Int4>,


    }
}

table! {
    entities(id) {
        id -> Nullable<Text>,
        organization_id -> Nullable<Text>,
        version -> Nullable<Int4>,
        created_by -> Nullable<Text>,
        updated_by -> Nullable<Text>,
        deleted_by -> Nullable<Text>,
        timestamp -> Nullable<Text>,
        tombstone -> Nullable<Int4>,

        name -> Nullable<Text>,
    }
}

table! {
    fields(id) {
        id -> Nullable<Text>,
        label -> Nullable<Text>,
        name -> Nullable<Text>,
        field_type -> Nullable<Text>,
        constraints -> Nullable<Jsonb>,
        _default -> Nullable<Text>,
        reference_to -> Nullable<Text>,
        version -> Nullable<Int4>,
        created_by -> Nullable<Text>,
        updated_by -> Nullable<Text>,
        deleted_by -> Nullable<Text>,
        timestamp -> Nullable<Text>,
        tombstone -> Nullable<Int4>,
    }
}

table! {
    entity_fields(id) {
        id -> Nullable<Text>,
        entity_id -> Nullable<Text>,
        field_id -> Nullable<Text>,
        version -> Nullable<Int4>,
        schema_version -> Nullable<Int4>,
        created_by -> Nullable<Text>,
        updated_by -> Nullable<Text>,
        deleted_by -> Nullable<Text>,
        timestamp -> Nullable<Text>,
        tombstone -> Nullable<Int4>,
        sensitivity_level -> Nullable<Int4>,
        is_encryptable -> Nullable<Bool>,
    }
}

table! {
    permissions(id) {
        id -> Nullable<Text>,
        read -> Nullable<Bool>,
        write -> Nullable<Bool>,
        encrypt -> Nullable<Bool>,
        decrypt -> Nullable<Bool>,
        required -> Nullable<Bool>,
        sensitive -> Nullable<Bool>,
        archive -> Nullable<Bool>,
        delete -> Nullable<Bool>,
        version -> Nullable<Int4>,
        created_by -> Nullable<Text>,
        updated_by -> Nullable<Text>,
        deleted_by -> Nullable<Text>,
        timestamp -> Nullable<Text>,
        tombstone -> Nullable<Int4>,
    }
}

table! {
    encryption_keys(id) {
        id -> Nullable<Text>,
        organization_id -> Nullable<Text>,
        entity -> Nullable<Text>,
        created_by -> Nullable<Text>,
        timestamp -> Nullable<Text>,
        tombstone -> Nullable<Int4>,
    }
}

table! {
    sessions(id) {
        id -> Nullable<Text>,
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
        timestamp -> Nullable<Timestamp>,
        tags -> Nullable<Array<Text>>,
        categories -> Nullable<Array<Text>>,
        code -> Nullable<Text>,
        sensitivity_level -> Nullable<Int4>,
        sync_status -> Nullable<Text>,
        is_batch -> Nullable<Bool>,
        account_profile_id -> Nullable<Text>,
        device_name -> Nullable<Text>,
        browser_name -> Nullable<Text>,
        operating_system -> Nullable<Text>,
        authentication_method -> Nullable<Text>,
        location -> Nullable<Text>,
        ip_address -> Nullable<Text>,
        session_started -> Nullable<Timestamp>,
        remark -> Nullable<Text>,

        user_role_id -> Nullable<Text>,
        user_account_id -> Nullable<Text>,
        user_is_root_user -> Nullable<Bool>,
        token -> Nullable<Text>,
        cookie_path -> Nullable<Text>,
        cookie_expire -> Nullable<Text>,
        cookie_http_only -> Nullable<Bool>,
        cookie_original_max_age -> Nullable<Int8>,
        origin_url -> Nullable<Text>,
        origin_host -> Nullable<Text>,
        origin_user_agent -> Nullable<Text>,
        valid_pass_key -> Nullable<Text>,
        role_permission -> Nullable<Text>,
        field_permission -> Nullable<Text>,
        record_permission -> Nullable<Text>,
        expire -> Nullable<Timestamp>,
        application_accessed -> Nullable<Text>,
        last_accessed -> Nullable<Timestamptz>,

    }
}

table! {
    signed_in_activity(id) {
        id -> Nullable<Text>,
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
        timestamp -> Nullable<Timestamp>,
        tags -> Nullable<Array<Text>>,
        categories -> Nullable<Array<Text>>,
        code -> Nullable<Text>,
        sensitivity_level -> Nullable<Int4>,
        sync_status -> Nullable<Text>,
        is_batch -> Nullable<Bool>,
        account_profile_id -> Nullable<Text>,
        device_name -> Nullable<Text>,
        browser_name -> Nullable<Text>,
        operating_system -> Nullable<Text>,
        authentication_method -> Nullable<Text>,
        location -> Nullable<Text>,
        ip_address -> Nullable<Text>,
        session_started -> Nullable<Timestamp>,
        remark -> Nullable<Text>,
        session_id -> Nullable<Text>,

    }
}

//System tables

table! {
    external_contacts (id) {
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
        categories -> Nullable<Array<Text>>,
        code -> Nullable<Text>,
        id -> Nullable<Text>,
        timestamp -> Nullable<Timestamp>,
        sensitivity_level -> Nullable<Int4>,
        sync_status -> Nullable<Text>,
        is_batch -> Nullable<Bool>,

        first_name -> Nullable<Text>,
        last_name -> Nullable<Text>,
        email -> Nullable<Text>,
    }
}

table! {
    counters (entity) {
        entity -> Text,
        default_code -> Integer,
        prefix -> Text,
        counter -> Integer,
        digits_number -> Integer,
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

table! {
    stream_queue (id) {
        id -> Text,
        name -> Text,
        created_at -> Nullable<Timestamptz>,
        last_accessed -> Nullable<Timestamptz>,
    }
}

table! {
    stream_queue_items (id) {
        id -> Text,
        queue_name -> Text,
        content -> Jsonb,
        timestamp -> Nullable<Timestamptz>,
    }
}

table! {
    transactions (id) {
        id -> Text,
        timestamp -> Text,
        status -> Text,
        expiry -> BigInt,
    }
}

table! {
    organizations(id) {
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
        categories -> Nullable<Array<Text>>,
        code -> Nullable<Text>,
        id -> Nullable<Text>,
        timestamp -> Nullable<Timestamp>,
        sensitivity_level -> Nullable<Int4>,
        sync_status -> Nullable<Text>,
        is_batch -> Nullable<Bool>,

        parent_organization_id -> Nullable<Text>,
        name -> Nullable<Text>,
        organization_level -> Nullable<Int4>,
        root_organization_id -> Nullable<Text>,
        path_level -> Nullable<Array<Text>>,
    }
}

table! {
    organization_contacts(id) {
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
        categories -> Nullable<Array<Text>>,
        code -> Nullable<Text>,
        id -> Nullable<Text>,
        timestamp -> Nullable<Timestamp>,
        sensitivity_level -> Nullable<Int4>,
        sync_status -> Nullable<Text>,
        is_batch -> Nullable<Bool>,

        contact_id -> Nullable<Text>,
    }
}

table! {
    organization_accounts(id) {
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
        categories -> Nullable<Array<Text>>,
        code -> Nullable<Text>,
        id -> Nullable<Text>,
        timestamp -> Nullable<Timestamp>,
        sensitivity_level -> Nullable<Int4>,
        sync_status -> Nullable<Text>,
        is_batch -> Nullable<Bool>,

        organization_contact_id -> Nullable<Text>,
        account_id -> Nullable<Text>,
        account_secret -> Nullable<Text>,
        role_id -> Nullable<Text>,
        contact_id -> Nullable<Text>,
        device_id -> Nullable<Text>,
    }
}

table! {
    account_organizations (id) {
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
        categories -> Nullable<Array<Text>>,
        code -> Nullable<Text>,
        id -> Nullable<Text>,
        timestamp -> Nullable<Timestamp>,
        sensitivity_level -> Nullable<Int4>,
        sync_status -> Nullable<Text>,
        is_batch -> Nullable<Bool>,

        contact_id -> Nullable<Text>,
        email -> Nullable<Text>,
        account_id -> Nullable<Text>,
        role_id -> Nullable<Text>,
        account_organization_status -> Nullable<Text>,
        is_invited -> Nullable<Bool>,
        device_id -> Nullable<Text>,
    }
}

table! {
    account_profiles (id) {
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
        categories -> Nullable<Array<Text>>,
        code -> Nullable<Text>,
        id -> Nullable<Text>,
        timestamp -> Nullable<Timestamp>,
        sensitivity_level -> Nullable<Int4>,
        sync_status -> Nullable<Text>,
        is_batch -> Nullable<Bool>,

        first_name -> Nullable<Text>,
        last_name -> Nullable<Text>,
        email -> Nullable<Text>,
        account_id -> Nullable<Text>,
        date_of_birth -> Nullable<Timestamp>,
        middle_name -> Nullable<Text>,
        auth_preference -> Nullable<Text>,
    }
}

table! {
    accounts (id) {
        id -> Nullable<Text>,
        categories -> Nullable<Array<Text>>,
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
        timestamp -> Nullable<Timestamptz>,
        tags -> Nullable<Array<Text>>,
        sensitivity_level -> Nullable<Int4>,
        sync_status -> Nullable<Text>,
        is_batch -> Nullable<Bool>,
        account_id -> Nullable<Text>,
        image_url -> Nullable<Text>,
        account_secret -> Nullable<Text>,
        account_status -> Nullable<Text>,
        is_new_user -> Nullable<Bool>,

    }
}

table! {
    organization_domains(id) {
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
        categories -> Nullable<Array<Text>>,
        code -> Nullable<Text>,
        id -> Nullable<Text>,
        timestamp -> Nullable<Timestamp>,
        sensitivity_level -> Nullable<Int4>,
        sync_status -> Nullable<Text>,
        is_batch -> Nullable<Bool>,

        domain_name -> Nullable<Text>,
    }
}

joinable!(account_organizations -> accounts (account_id));
joinable!(stream_queue_items -> stream_queue (queue_name));
allow_tables_to_appear_in_same_query!(accounts, account_organizations);
allow_tables_to_appear_in_same_query!(stream_queue, stream_queue_items);
//application

table! {
    addresses (id) {
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
        categories -> Nullable<Array<Text>>,
        code -> Nullable<Text>,
        timestamp -> Nullable<Timestamp>,
         sensitivity_level -> Nullable<Int4>,
        sync_status -> Nullable<Text>,
        is_batch -> Nullable<Bool>,

        id -> Nullable<Text>,
        address -> Nullable<Text>,
        address_line_one -> Nullable<Text>,
        address_line_two -> Nullable<Text>,
        latitude -> Nullable<Float4>,
        longitude -> Nullable<Float4>,
        place_id -> Nullable<Text>,
        street_number -> Nullable<Text>,
        street -> Nullable<Text>,
        region -> Nullable<Text>,
        region_code -> Nullable<Text>,
        country_code -> Nullable<Text>,
        postal_code -> Nullable<Text>,
        country -> Nullable<Text>,
        state -> Nullable<Text>,
        city -> Nullable<Text>,
    }
}
table! {
    samples (id) {
        id -> Nullable<Text>,
        categories -> Nullable<Array<Text>>,
        code -> Nullable<Text>,
        tombstone -> Nullable<Integer>,
        status -> Nullable<Text>,
        previous_status -> Nullable<Text>,
        version -> Nullable<Integer>,
        created_date -> Nullable<Text>,
        created_time -> Nullable<Text>,
        updated_date -> Nullable<Text>,
        updated_time -> Nullable<Text>,
        organization_id -> Nullable<Text>,
        created_by -> Nullable<Text>,
        updated_by -> Nullable<Text>,
        deleted_by -> Nullable<Text>,
        requested_by -> Nullable<Text>,
        timestamp -> Nullable<Timestamp>,
        sensitivity_level -> Nullable<Integer>,
        sync_status -> Nullable<Text>,
        is_batch -> Nullable<Bool>,
        name -> Nullable<Text>,
        sample_text -> Nullable<Text>,
    }
}

table! {
    devices (id) {
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
        categories -> Nullable<Array<Text>>,
        code -> Nullable<Text>,
        id -> Nullable<Text>,
        timestamp -> Nullable<Timestamp>,
        sensitivity_level -> Nullable<Int4>,
        sync_status -> Nullable<Text>,
        is_batch -> Nullable<Bool>,

        model -> Nullable<Text>,
        address_id -> Nullable<Text>,
        instance_name -> Nullable<Text>,
        is_connection_established -> Nullable<Bool>,
        system_id -> Nullable<Text>,
        device_version -> Nullable<Text>,
        last_heartbeat -> Nullable<Text>,
        is_monitoring_enabled -> Nullable<Bool>,
        is_remote_access_enabled -> Nullable<Bool>,
        ip_address -> Nullable<Inet>,
        device_status -> Nullable<Text>,
        device_gui_protocol -> Nullable<Text>,
    }
}

table! {
    postgres_channels(id) {
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
        categories -> Nullable<Array<Text>>,
        code -> Nullable<Text>,
        timestamp -> Nullable<Timestamp>,
        id -> Nullable<Text>,
        sensitivity_level -> Nullable<Int4>,
        sync_status -> Nullable<Text>,
        is_batch -> Nullable<Bool>,

        channel_name -> Nullable<Text>,
        function -> Nullable<Text>,
    }
}

table! {
    contacts(id) {
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
        categories -> Nullable<Array<Text>>,
        code -> Nullable<Text>,
        id -> Nullable<Text>,
        timestamp -> Nullable<Timestamp>,
        sensitivity_level -> Nullable<Int4>,
        sync_status -> Nullable<Text>,
        is_batch -> Nullable<Bool>,

        first_name -> Nullable<Text>,
        middle_name -> Nullable<Text>,
        last_name -> Nullable<Text>,
        date_of_birth -> Nullable<Text>,
        account_id -> Nullable<Text>
    }
}

table! {
    contact_phone_numbers(id) {
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
        categories -> Nullable<Array<Text>>,
        code -> Nullable<Text>,
        id -> Nullable<Text>,
        timestamp -> Nullable<Timestamp>,
        sensitivity_level -> Nullable<Int4>,
        sync_status -> Nullable<Text>,
        is_batch -> Nullable<Bool>,

        contact_id -> Nullable<Text>,
        phone_number_raw -> Nullable<Text>,
    }
}

table! {
    contact_emails(id) {
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
        categories -> Nullable<Array<Text>>,
        code -> Nullable<Text>,
        id -> Nullable<Text>,
        timestamp -> Nullable<Timestamp>,
        sensitivity_level -> Nullable<Int4>,
        sync_status -> Nullable<Text>,
        is_batch -> Nullable<Bool>,

        contact_id -> Nullable<Text>,
        email -> Nullable<Text>,
        is_primary -> Nullable<Bool>
    }
}

table! {
    files(id) {
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
        timestamp -> Nullable<Timestamp>,
        tags -> Nullable<Array<Text>>,
        categories -> Nullable<Array<Text>>,
        code -> Nullable<Text>,
        id -> Nullable<Text>,
        sensitivity_level -> Nullable<Int4>,
        sync_status -> Nullable<Text>,
        is_batch -> Nullable<Bool>,
        image_url -> Nullable<Text>,
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
        version_id -> Nullable<Text>,
        download_path -> Nullable<Text>,
        presigned_url -> Nullable<Text>,
        presigned_url_expire -> Nullable<Int4>,
}
}

table! {
    test_hypertable(timestamp) {
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
        timestamp -> Nullable<Timestamptz>,
        tags -> Nullable<Array<Text>>,
        categories -> Nullable<Array<Text>>,
        code -> Nullable<Text>,
        id -> Nullable<Text>,
        sensitivity_level -> Nullable<Int4>,
        sync_status -> Nullable<Text>,
        is_batch -> Nullable<Bool>,
        hypertable_timestamp -> Nullable<Text>,
        sensor_id -> Nullable<Text>,
        temperature -> Nullable<Int4>,
        humidity -> Nullable<Int4>,
        location -> Nullable<Text>,
    }
}

table! {
    account_phone_numbers(id) {
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
        timestamp -> Nullable<Timestamp>,
        tags -> Nullable<Array<Text>>,
        categories -> Nullable<Array<Text>>,
        code -> Nullable<Text>,
        id -> Nullable<Text>,
        sensitivity_level -> Nullable<Int4>,
        sync_status -> Nullable<Text>,
        is_batch -> Nullable<Bool>,
        account_profile_id -> Nullable<Text>,
        raw_phone_number -> Nullable<Text>,
        is_primary -> Nullable<Bool>,
        iso_code -> Nullable<Text>,
        country_code -> Nullable<Text>,

}
}

table! {
    account_signatures(id) {
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
        timestamp -> Nullable<Timestamp>,
        tags -> Nullable<Array<Text>>,
        categories -> Nullable<Array<Text>>,
        code -> Nullable<Text>,
        id -> Nullable<Text>,
        sensitivity_level -> Nullable<Int4>,
        sync_status -> Nullable<Text>,
        is_batch -> Nullable<Bool>,
        account_profile_id -> Nullable<Text>,
        name -> Nullable<Text>,
        signature -> Nullable<Text>,
    }
}
