DROP TYPE IF EXISTS field_type;
CREATE TYPE field_type AS (
    id TEXT,
   label TEXT,
  name TEXT,
  type TEXT,
  created_by TEXT
);

DROP TYPE IF EXISTS permission_type;
CREATE TYPE permission_type AS (
    id TEXT, 
    read BOOLEAN, 
    write BOOLEAN, 
    encrypt BOOLEAN, 
    decrypt BOOLEAN, 
    required BOOLEAN, 
    sensitive BOOLEAN, 
    archive BOOLEAN, 
    delete BOOLEAN, 
    created_by TEXT
);
CREATE EXTENSION IF NOT EXISTS "pgcrypto";
DO $$
DECLARE
    entity_record_id TEXT := uuid_generate_v4()::TEXT;
    organization_record_id TEXT := uuid_generate_v4()::TEXT;
    record_email TEXT := 'admin@dnamicro.com';
    main_entity TEXT := 'files';
    fields field_type[] := ARRAY[
        -- system fields
        ROW('Id_id_text', 'Id', 'id', 'text', record_email)::field_type,
        ROW('Categories_categories_text[]', 'Categories', 'categories', 'text[]', record_email)::field_type,
        ROW('Code_code_text', 'Code', 'code', 'text', record_email)::field_type,
        ROW('Tombstone_tombstone_integer', 'Tombstone', 'tombstone', 'integer', record_email)::field_type,
        ROW('Status_status_text', 'Status', 'status', 'text', record_email)::field_type,
        ROW('PreviousStatus_previous_status_text', 'PreviousStatus', 'previous_status', 'text', record_email)::field_type,
        ROW('Version_version_integer', 'Version', 'version', 'integer', record_email)::field_type,
        ROW('CreatedDate_created_date_text', 'CreatedDate', 'created_date', 'text', record_email)::field_type,
        ROW('CreatedTime_created_time_text', 'CreatedTime', 'created_time', 'text', record_email)::field_type,
        ROW('UpdatedDate_updated_date_text', 'UpdatedDate', 'updated_date', 'text', record_email)::field_type,
        ROW('UpdatedTime_updated_time_text', 'UpdatedTime', 'updated_time', 'text', record_email)::field_type,
        ROW('OrganizationId_organization_id_text', 'OrganizationId', 'organization_id', 'text', record_email)::field_type,
        ROW('CreatedBy_created_by_text', 'CreatedBy', 'created_by', 'text', record_email)::field_type,
        ROW('UpdatedBy_updated_by_text', 'UpdatedBy', 'updated_by', 'text', record_email)::field_type,
        ROW('DeletedBy_deleted_by_text', 'DeletedBy', 'deleted_by', 'text', record_email)::field_type,
        ROW('RequestedBy_requested_by_text', 'RequestedBy', 'requested_by', 'text', record_email)::field_type,
        ROW('Timestamp_timestamp_text', 'Timestamp', 'timestamp', 'text', record_email)::field_type,
        ROW('Tags_tags_text[]', 'Tags', 'tags', 'text[]', record_email)::field_type,
        ROW('ImageUrl_image_url_varchar', 'ImageUrl', 'image_url', 'varchar', record_email)::field_type,
        -- main fields
        ROW('Fieldname_fieldname_text', 'Fieldname', 'fieldname', 'text', record_email)::field_type,
        ROW('Originalname_originalname_text', 'Originalname', 'originalname', 'text', record_email)::field_type,
        ROW('Encoding_encoding_text', 'Encoding', 'encoding', 'text', record_email)::field_type,
        ROW('Mimetype_mimetype_text', 'Mimetype', 'mimetype', 'text', record_email)::field_type,
        ROW('Destination_destination_text', 'Destination', 'destination', 'text', record_email)::field_type,
        ROW('Filename_filename_text', 'Filename', 'filename', 'text', record_email)::field_type,
        ROW('Path_path_text', 'Path', 'path', 'text', record_email)::field_type,
        ROW('Size_size_integer', 'Size', 'size', 'integer', record_email)::field_type,
        ROW('UploadedBy_uploaded_by_text', 'UploadedBy', 'uploaded_by', 'text', record_email)::field_type,
        ROW('DownloadedBy_downloaded_by_text', 'DownloadedBy', 'downloaded_by', 'text', record_email)::field_type,
        ROW('Etag_etag_text', 'Etag', 'etag', 'text', record_email)::field_type,
        ROW('VersionId_versionId_text', 'VersionId', 'versionId', 'text', record_email)::field_type,
        ROW('DownloadPath_download_path_text', 'DownloadPath', 'download_path', 'text', record_email)::field_type,
        ROW('PresignedURL_presignedURL_text', 'PresignedURL', 'presignedURL', 'text', record_email)::field_type,
        ROW('PresignedURLExpires_presignedURLExpires_integer', 'PresignedURLExpires', 'presignedURLExpires', 'integer', record_email)::field_type

    ];
    arr_permissions permission_type[] := ARRAY[
        ROW('0b023cd7-1471-4980-902e-b67f28e2c370', true, true, true, true, true,true, true, true, record_email)::permission_type,
        ROW('26958631-a9a0-46de-ab71-442f9c970e26', false, true, true, true, true,false, true, true, record_email)::permission_type
    ];
    arr_permission permission_type;
    field field_type;
BEGIN
    -- entities
    INSERT INTO entities (id, name, organization_id, created_by)
    SELECT entity_record_id, main_entity, organization_record_id, record_email
    WHERE NOT EXISTS (SELECT 1 FROM entities WHERE id = entity_record_id);

    -- loop permissions
    FOREACH arr_permission IN ARRAY arr_permissions LOOP
        INSERT INTO permissions (id, read, write, encrypt, decrypt, required, sensitive, archive, delete, created_by) 
        SELECT arr_permission.id, arr_permission.read, arr_permission.write, arr_permission.encrypt, arr_permission.decrypt, arr_permission.required, arr_permission.sensitive, arr_permission.archive, arr_permission.delete, record_email
        WHERE NOT EXISTS (SELECT 1 FROM permissions WHERE id = arr_permission.id);
    END LOOP;

    -- loop fields
    FOREACH field IN ARRAY fields LOOP
        INSERT INTO fields (id, label, name, type, created_by) 
        SELECT field.id, field.label, field.name, field.type, field.created_by
        WHERE NOT EXISTS (SELECT 1 FROM fields WHERE id = field.id);

        DECLARE
            entity_field_record_id TEXT := uuid_generate_v4()::text;
            account_organization_record_id TEXT;
        BEGIN
            -- entity fields
            INSERT INTO entity_fields (id, entity_id, field_id, created_by) 
            SELECT entity_field_record_id, entity_record_id, field.id, record_email
            WHERE NOT EXISTS (SELECT 1 FROM entity_fields WHERE id = entity_field_record_id);

            -- Get the account organization ID
            SELECT id INTO account_organization_record_id 
            FROM account_organizations 
            WHERE email = record_email;

            RAISE NOTICE 'account_organization_record_id = %', account_organization_record_id;
            RAISE NOTICE 'record_email = %', record_email;

            -- Check if arr_permissions has elements before accessing
            IF account_organization_record_id IS NOT NULL THEN
                INSERT INTO data_permissions (id, entity_field_id, inherited_permission_id, account_organization_id, created_by) 
                SELECT uuid_generate_v4()::text, entity_field_record_id, arr_permissions[1].id, account_organization_record_id, record_email
                WHERE NOT EXISTS (SELECT 1 FROM data_permissions WHERE entity_field_id = entity_field_record_id);
            END IF;
        END;
    END LOOP;

END
$$;