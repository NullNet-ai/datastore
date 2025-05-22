use diesel::table;

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
        timestamp -> Timestamp,

        id -> Text,
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
        timestamp -> Timestamp,

        id -> Text,
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
        timestamp -> Timestamp,

        id -> Text,
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
        timestamp -> Timestamp,

        id -> Text,
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
        timestamp -> Timestamp,

        id -> Text,
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
        timestamp -> Timestamp,

        id -> Text,
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
        timestamp -> Timestamp,

        id -> Text,
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
        timestamp -> Timestamp,

        id -> Text,
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
        timestamp -> Timestamp,

        id -> Text,
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
        timestamp -> Timestamp,

        id -> Text,
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
        timestamp -> Timestamp,

        id -> Text,
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
        timestamp -> Timestamp,

        id -> Text,
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
        timestamp -> Timestamp,

        id -> Text,
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
        timestamp -> Timestamp,

        id -> Text,
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
        timestamp -> Timestamp,

        id -> Text,
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


        id -> Text,
        timestamp -> Timestamp,
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

        id -> Text,
        timestamp -> Timestamp,
        public_key -> Text,
        private_key -> Text,
        passphrase -> Text,
        device_id -> Text,
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
        id -> Text,
        timestamp -> Timestamp,

        model -> Nullable<Text>,
        address_id -> Nullable<Text>,
        instance_name -> Nullable<Text>,
        is_connection_established -> Nullable<Bool>,
        system_id -> Nullable<Text>,
        device_version -> Nullable<Text>,
        last_heartbeat -> Nullable<Text>, // Adjust if using Timestamp
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
        timestamp -> Timestamp,
        id -> Text,

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
        timestamp -> Timestamp,
        id -> Text,

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
        timestamp -> Timestamp,
        id -> Text,

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
        timestamp -> Timestamp,
        id -> Text,

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
        id -> Text,
        timestamp -> Timestamp,

        name -> Nullable<Text>,
    }
}

table! {
    counters (entity) {
        entity -> Text,
        default_code -> Integer,
        prefix -> Text,
        counter -> Integer,
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
        id -> Text,
        timestamp -> Timestamp,

        parent_organization_id -> Nullable<Text>,
        name -> Nullable<Text>
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
        id -> Text,
        timestamp -> Timestamp,

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
        id -> Text,
        timestamp -> Timestamp,

        organization_contact_id -> Nullable<Text>,
        account_id -> Nullable<Text>,
        account_secret -> Nullable<Text>,
        role_id -> Nullable<Text>,
        contact_id -> Nullable<Text>,
        device_id -> Nullable<Text>,
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
        id -> Text,
        timestamp -> Timestamp,

        first_name -> Nullable<Text>,
        middle_name -> Nullable<Text>,
        last_name -> Nullable<Text>,
        date_of_birth -> Nullable<Text>,
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
        id -> Text,
        timestamp -> Timestamp,

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
        id -> Text,
        timestamp -> Timestamp,

        contact_id -> Nullable<Text>,
        email -> Nullable<Text>,
    }
}
