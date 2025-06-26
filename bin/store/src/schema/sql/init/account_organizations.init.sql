DO $$
DECLARE
-- TODO - define if system field and allow return!!!
    entity_record_id TEXT := uuid_generate_v4()::TEXT;
    organization_record_id TEXT := uuid_generate_v4()::TEXT;
    record_email TEXT := 'superadmin@dnamicro.com';
    record_emails TEXT[] := ARRAY['superadmin@dnamicro.com', 'dbadmin@dnamicro.com', 'admin@dnamicro.com', 'member@dnamicro.com'];
    main_entity TEXT := 'account_organizations';
    fields field_type[] := ARRAY[
        -- system fields
        ROW('Id_id_text', 'Id', 'id', 'text', record_email, false, true, false,'','',to_jsonb(ARRAY['primaryKey']))::field_type,
        ROW('Categories_categories_jsonb', 'Categories', 'categories', 'jsonb', record_email, false, true, false,'[]','',to_jsonb(ARRAY['']))::field_type,
        ROW('Code_code_text', 'Code', 'code', 'text', record_email, false, true, false,'','',to_jsonb(ARRAY['']))::field_type,
        ROW('Tombstone_tombstone_integer', 'Tombstone', 'tombstone', 'integer', record_email, false, true, false,'0','',to_jsonb(ARRAY['']))::field_type,
        ROW('Status_status_text', 'Status', 'status', 'text', record_email, false, true, false,'Active','',to_jsonb(ARRAY['']))::field_type,
        ROW('PreviousStatus_previous_status_text', 'PreviousStatus', 'previous_status', 'text', record_email, false, true, false,'','',to_jsonb(ARRAY['']))::field_type,
        ROW('Version_version_integer', 'Version', 'version', 'integer', record_email, false, true, false,'1','',to_jsonb(ARRAY['']))::field_type,
        ROW('CreatedDate_created_date_text', 'CreatedDate', 'created_date', 'text', record_email, false, true, false,'','',to_jsonb(ARRAY['']))::field_type,
        ROW('CreatedTime_created_time_text', 'CreatedTime', 'created_time', 'text', record_email, false, true, false,'','',to_jsonb(ARRAY['']))::field_type,
        ROW('UpdatedDate_updated_date_text', 'UpdatedDate', 'updated_date', 'text', record_email, false, true, false,'','',to_jsonb(ARRAY['']))::field_type,
        ROW('UpdatedTime_updated_time_text', 'UpdatedTime', 'updated_time', 'text', record_email, false, true, false,'','',to_jsonb(ARRAY['']))::field_type,
        ROW('OrganizationId_organization_id_text', 'OrganizationId', 'organization_id', 'text', record_email, false, true, false,'','organizations',to_jsonb(ARRAY['']))::field_type,
        ROW('CreatedBy_created_by_text', 'CreatedBy', 'created_by', 'text', record_email, false, true, false,'','account_organizations',to_jsonb(ARRAY['']))::field_type,
        ROW('UpdatedBy_updated_by_text', 'UpdatedBy', 'updated_by', 'text', record_email, false, true, false,'','account_organizations',to_jsonb(ARRAY['']))::field_type,
        ROW('DeletedBy_deleted_by_text', 'DeletedBy', 'deleted_by', 'text', record_email, false, true, false,'','account_organizations',to_jsonb(ARRAY['']))::field_type,
        ROW('RequestedBy_requested_by_text', 'RequestedBy', 'requested_by', 'text', record_email, false, true, false,'','account_organizations',to_jsonb(ARRAY['']))::field_type,
        ROW('Timestamp_timestamp_timestamp_with_time_zone', 'Timestamp', 'timestamp', 'timestamp_with_time_zone', record_email, false, true, false,'','',to_jsonb(ARRAY['notNull','defaultNow']))::field_type,
        ROW('Tags_tags_jsonb', 'Tags', 'tags', 'jsonb', record_email, false, true, false,'[]','',to_jsonb(ARRAY['']))::field_type,
        ROW('ImageUrl_image_url_varchar', 'ImageUrl', 'image_url', 'varchar', record_email, false, true, false,'','',to_jsonb(ARRAY['']))::field_type,
        -- main fields
        ROW('ContactId_contact_id_text', 'ContactId', 'contact_id', 'text', record_email, false, false, false,'','contacts',to_jsonb(ARRAY['']))::field_type,
        ROW('Email_email_text', 'Email', 'email', 'text', record_email, false, false, false,'','',to_jsonb(ARRAY['']))::field_type,
        ROW('AccountId_account_id_text', 'AccountId', 'account_id', 'text', record_email, false, false, false,'','accounts',to_jsonb(ARRAY['']))::field_type,
        ROW('RoleId_role_id_text', 'RoleId', 'role_id', 'text', record_email, false, false, false,'','user_roles',to_jsonb(ARRAY['']))::field_type,
        ROW('AccountOrganizationStatus_account_organization_status_text', 'AccountOrganizationStatus', 'account_organization_status', 'text', record_email, false, false, false,'','',to_jsonb(ARRAY['']))::field_type,
        ROW('IsInvited_is_invited_boolean', 'IsInvited', 'is_invited', 'boolean', record_email, false, false, false,'false','',to_jsonb(ARRAY['']))::field_type,
        ROW('DeviceId_device_id_text', 'DeviceId', 'device_id', 'text', record_email, false, false, false,'','devices',to_jsonb(ARRAY['']))::field_type
    ];
    arr_permission permission_type;
    permission_record_email TEXT;
BEGIN
    --RAISE NOTICE 'main_entity = %', main_entity;
    -- entities
    INSERT INTO entities (id, name, organization_id, created_by)
    SELECT entity_record_id, main_entity, organization_record_id, record_email
    WHERE NOT EXISTS (SELECT 1 FROM entities WHERE id = entity_record_id);

    -- loop permission record emails
    FOREACH permission_record_email IN ARRAY record_emails LOOP
        DECLARE
             field field_type;
            _permission_id TEXT;
        BEGIN
            -- loop fields
            FOREACH field IN ARRAY fields LOOP
                INSERT INTO fields (id, label, name, field_type, created_by, is_encryptable, is_system_field, _default, reference_to, constraints) 
                SELECT field.id, field.label, field.name, field.field_type, field.created_by, field.is_encryptable, field.is_system_field, field._default, field.reference_to, field.constraints
                WHERE NOT EXISTS (SELECT 1 FROM fields WHERE id = field.id);

                DECLARE
                    entity_field_record_id TEXT := uuid_generate_v4()::TEXT;
                    account_organization_record_id TEXT;
                    -- guest permissions ( default )
                    read BOOLEAN := true;
                    write BOOLEAN := false;
                    encrypt BOOLEAN := false;
                    decrypt BOOLEAN := false;
                    required BOOLEAN := false;
                    sensitive BOOLEAN := false;
                    archive BOOLEAN := false;
                    delete BOOLEAN := false;
                BEGIN
                    -- entity fields
                    INSERT INTO entity_fields (id, entity_id, field_id, created_by) 
                    SELECT entity_field_record_id, entity_record_id, field.id, permission_record_email
                    WHERE NOT EXISTS (SELECT 1 FROM entity_fields WHERE id = entity_field_record_id);

                    -- Get the account organization ID
                    SELECT id INTO account_organization_record_id 
                    FROM account_organizations 
                    WHERE email = permission_record_email;

                    _permission_id := uuid_generate_v4()::TEXT;

                    --RAISE NOTICE 'account_organization_record_id = %', account_organization_record_id;
                    --RAISE NOTICE 'record_email = %', record_email;
                    --RAISE NOTICE 'permission_id = %', permission_id;
                    -- create permissions per field
                    INSERT INTO permissions (id, read, write, encrypt, decrypt, required, sensitive, archive, delete, created_by) 
                    SELECT _permission_id, read, write, encrypt, decrypt, required, sensitive, archive, delete, permission_record_email
                    WHERE NOT EXISTS (SELECT 1 FROM permissions WHERE id = _permission_id);

                    -- Check if arr_permissions has elements before accessing
                    IF account_organization_record_id IS NOT NULL THEN
                        INSERT INTO data_permissions (id, entity_field_id, permission_id, account_organization_id, created_by) 
                        SELECT uuid_generate_v4()::text, entity_field_record_id, _permission_id, account_organization_record_id, permission_record_email
                        WHERE NOT EXISTS (SELECT 1 FROM data_permissions WHERE entity_field_id = entity_field_record_id);
                    END IF;
                END;
            END LOOP;
        END;
    END LOOP;

END
$$;