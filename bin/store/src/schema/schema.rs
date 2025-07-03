use diesel::{allow_tables_to_appear_in_same_query, joinable, table};

//permissions

table! {
    data_permissions(id) {
        id -> Nullable<Text>,
        entity_field_id -> Nullable<Text>,
        inherited_permission_id -> Nullable<Text>,
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
        version -> Nullable<Int4>,
        created_by -> Nullable<Text>,
        updated_by -> Nullable<Text>,
        deleted_by -> Nullable<Text>,
        timestamp -> Nullable<Text>,
        tombstone -> Nullable<Int4>,

        role_name -> Nullable<Text>,
        permission_id -> Nullable<Text>,
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
        sensitivity_level -> Nullable<Int4>,
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
    sessions(sid) {
        sid -> Text,
        sess -> Jsonb,
        expire -> Timestamp,
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

        first_name -> Nullable<Text>,
        last_name -> Nullable<Text>,
        email -> Nullable<Text>,
        account_id -> Nullable<Text>,
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
        timestamp -> Nullable<Text>,
        tags -> Nullable<Array<Text>>,
        sensitivity_level -> Nullable<Int4>,

        account_id -> Nullable<Text>,
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

        domain_name -> Nullable<Text>,
    }
}

joinable!(account_organizations -> accounts (account_id));
allow_tables_to_appear_in_same_query!(accounts, account_organizations);
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
    app_firewalls (id) {
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

        id -> Nullable<Text>,
        app_id -> Nullable<Text>,
        firewall -> Nullable<Text>,
    }
}

table! {
    appguard_logs (id) {
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

        id -> Nullable<Text>,
        level -> Nullable<Text>,
        message -> Nullable<Text>,
    }
}

table! {
    temp_appguard_logs (id) {
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

        id -> Nullable<Text>,
        level -> Nullable<Text>,
        message -> Nullable<Text>,
    }
}

table! {
    device_aliases (id) {
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

        id -> Nullable<Text>,
        #[sql_name = "type"]
        alias_type -> Nullable<Text>,
        name -> Nullable<Text>,
        value -> Nullable<Text>,
        description -> Nullable<Text>,
        device_alias_status -> Nullable<Text>,
    }
}

table! {
    temp_device_aliases (id) {
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

        id -> Nullable<Text>,
        #[sql_name = "type"]
        alias_type -> Nullable<Text>,
        name -> Nullable<Text>,
        value -> Nullable<Text>,
        description -> Nullable<Text>,
        device_alias_status -> Nullable<Text>,
    }
}

table! {
    device_configurations (id) {
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

        id -> Nullable<Text>,
        device_id -> Nullable<Text>,
        digest -> Nullable<Text>,
        hostname -> Nullable<Text>,
        raw_content -> Nullable<Text>,
        config_version -> Nullable<Int4>,
    }
}

table! {
    device_interface_addresses (id) {
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

        id -> Nullable<Text>,
        device_interface_id -> Nullable<Text>,
        address -> Nullable<Inet>,
    }
}

table! {
    temp_device_interface_addresses (id) {
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

        id -> Nullable<Text>,
        device_interface_id -> Nullable<Text>,
        address -> Nullable<Inet>,
    }
}

table! {
    device_interfaces(id) {
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

        id -> Nullable<Text>,
        device_configuration_id -> Nullable<Text>,
        name -> Nullable<Text>,
        device -> Nullable<Text>,
    }
}

table! {
    temp_device_interfaces(id) {
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

        id -> Nullable<Text>,
        device_configuration_id -> Nullable<Text>,
        name -> Nullable<Text>,
        device -> Nullable<Text>,
    }
}

table! {
    device_remote_access_sessions(id) {
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

        id -> Nullable<Text>,
        device_id -> Nullable<Text>,
        remote_access_type -> Nullable<Text>,
        remote_access_session -> Nullable<Text>,
        remote_access_status -> Nullable<Text>,
        remote_access_category -> Nullable<Text>,
    }
}

table! {
    temp_device_remote_access_sessions(id) {
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

        id -> Nullable<Text>,
        device_id -> Nullable<Text>,
        remote_access_type -> Nullable<Text>,
        remote_access_session -> Nullable<Text>,
        remote_access_status -> Nullable<Text>,
        remote_access_category -> Nullable<Text>,
    }
}

table! {
    device_rules(id) {
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

        id -> Nullable<Text>,
        device_configuration_id -> Nullable<Text>,
        disabled -> Nullable<Bool>,
        #[sql_name = "type"]
        rule_type -> Nullable<Text>,
        policy -> Nullable<Text>,
        protocol -> Nullable<Text>,
        source_port -> Nullable<Text>,
        source_addr -> Nullable<Text>,
        source_type -> Nullable<Text>,
        destination_port -> Nullable<Text>,
        destination_addr -> Nullable<Text>,
        description -> Nullable<Text>,
        device_rule_status -> Nullable<Text>,
        interface -> Nullable<Text>,
        order -> Nullable<Int4>,
        destination_inversed -> Nullable<Bool>,
        destination_type -> Nullable<Text>,
        source_inversed -> Nullable<Bool>,
    }
}

table! {
    temp_device_rules(id) {
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

        id -> Nullable<Text>,
        device_configuration_id -> Nullable<Text>,
        disabled -> Nullable<Bool>,
        #[sql_name = "type"]
        rule_type -> Nullable<Text>,
        policy -> Nullable<Text>,
        protocol -> Nullable<Text>,
        source_port -> Nullable<Text>,
        source_addr -> Nullable<Text>,
        source_type -> Nullable<Text>,
        destination_port -> Nullable<Text>,
        destination_addr -> Nullable<Text>,
        description -> Nullable<Text>,
        device_rule_status -> Nullable<Text>,
        interface -> Nullable<Text>,
        order -> Nullable<Int4>,
        destination_inversed -> Nullable<Bool>,
        destination_type -> Nullable<Text>,
        source_inversed -> Nullable<Bool>,
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
        categories -> Nullable<Array<Text>>,
        code -> Nullable<Text>,
         sensitivity_level -> Nullable<Int4>,


        id -> Nullable<Text>,
        timestamp -> Nullable<Timestamp>,
        hypertable_timestamp -> Nullable<Text>,
        interface_name -> Nullable<Text>,
        device_id -> Nullable<Text>,
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
    temp_packets (id) {
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
        categories -> Nullable<Array<Text>>,
        code -> Nullable<Text>,
        id -> Nullable<Text>,
        timestamp -> Nullable<Timestamp>,
        sensitivity_level -> Nullable<Int4>,

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
    temp_connections (id, timestamp) {
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
    device_ssh_keys (id) {
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

       
        public_key -> Nullable<Text>,
        private_key -> Nullable<Text>,
        passphrase -> Nullable<Text>,
        device_id -> Nullable<Text>,
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
    ip_infos(id) {
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

        country -> Nullable<Text>,
        asn -> Nullable<Text>,
        org -> Nullable<Text>,
        continent_code -> Nullable<Text>,
        city -> Nullable<Text>,
        region -> Nullable<Text>,
        postal -> Nullable<Text>,
        timezone -> Nullable<Text>,
        blacklist -> Nullable<Bool>,
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

        channel_name -> Nullable<Text>,
        function -> Nullable<Text>,
    }
}

table! {
    resolutions(id) {
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

        resolution_type -> Nullable<Text>
    }
}

table! {
    wallguard_logs(id, timestamp) {
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

        level -> Nullable<Text>,
        message -> Nullable<Text>,
        hypertable_timestamp -> Nullable<Text>,
    }
}

table! {
    temp_wallguard_logs(id, timestamp) {
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

        level -> Nullable<Text>,
        message -> Nullable<Text>,
        hypertable_timestamp -> Nullable<Text>,
    }
}

table! {
    device_group_settings (id) {
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

        name -> Nullable<Text>,
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

        contact_id -> Nullable<Text>,
        email -> Nullable<Text>,
        is_primary -> Nullable<Bool>
    }
}
