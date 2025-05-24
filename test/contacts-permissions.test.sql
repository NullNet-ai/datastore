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
-- old permission 01JTMGV92R9S4ESY1QKHHFGZ0V = read is true
-- new permission 01JTQ5EAWX6VXRY4SDBF38SJTE = read is false
-- for field id
DO $$
DECLARE
    entity_record_id TEXT := uuid_generate_v4()::TEXT;
    organization_record_id TEXT := uuid_generate_v4()::TEXT;
    record_email TEXT := 'admin@dnamicro.com';
    main_entity TEXT := 'contacts';
    fields field_type[] := ARRAY[
        ROW('Id_id_text', 'Id', 'id', 'text', record_email)::field_type,
        ROW('Firstname_first_name_text', 'First name', 'first_name', 'text', record_email)::field_type
    ];
    arr_permissions permission_type[] := ARRAY[
        ROW(uuid_generate_v4()::TEXT, true, true, true, true, true, record_email)::permission_type,
        ROW(uuid_generate_v4()::TEXT, false, true, true, true, true, record_email)::permission_type
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