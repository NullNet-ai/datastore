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
    encrypted BOOLEAN, 
    decrypted BOOLEAN, 
    required BOOLEAN, 
    created_by TEXT
);
CREATE EXTENSION IF NOT EXISTS "pgcrypto";
DO $$
DECLARE
    entity_record_id TEXT := uuid_generate_v4()::TEXT;
    organization_record_id TEXT := uuid_generate_v4()::TEXT;
    record_email TEXT := 'admin@dnamicro.com';
    main_entity TEXT := 'data_permissions';
    fields field_type[] := ARRAY[
        ROW('Id_id_text', 'Id', 'id', 'text', record_email)::field_type,
        ROW('EntityFieldId_entity_id_text', 'Entity Field Id', 'entity_field_id', 'text', record_email)::field_type,
        ROW('PermissionId_permission_id_text', 'Permission Id', 'permission_id', 'text', record_email)::field_type,
        ROW('InheritedPermissionId_permission_id_text', 'Inherited Permission Id', 'inherited_permission_id', 'text', record_email)::field_type,
        ROW('AccountOrganizationId_account_organization_id_text', 'Account Organization Id', 'account_organization_id', 'text',record_email)::field_type,
        ROW('Version_version_serial', 'Version', 'version', 'serial', record_email)::field_type,
        ROW('CreatedBy_created_by_text', 'Created By', 'created_by', 'text', record_email)::field_type,
        ROW('UpdatedBy_updated_by_text', 'Updated By', 'updated_by', 'text', record_email)::field_type,
        ROW('DeletedBy_deleted_by_text', 'Deleted By', 'deleted_by', 'text', record_email)::field_type,
        ROW('Timestamp_timestamp_text', 'Timestamp', 'timestamp', 'text', record_email)::field_type
    ];
    arr_permissions permission_type[] := ARRAY[
        ROW('0b023cd7-1471-4980-902e-b67f28e2c370', true, true, true, true, true, record_email)::permission_type,
        ROW('26958631-a9a0-46de-ab71-442f9c970e26', false, true, true, true, true, record_email)::permission_type
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
        INSERT INTO permissions (id, read, write, encrypted, decrypted, required, created_by) 
        SELECT arr_permission.id, arr_permission.read, arr_permission.write, arr_permission.encrypted, arr_permission.decrypted, arr_permission.required, record_email
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