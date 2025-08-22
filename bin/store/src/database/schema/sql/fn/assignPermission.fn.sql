DROP FUNCTION IF EXISTS assignPermission;
CREATE OR REPLACE FUNCTION assignPermission()
RETURNS TRIGGER AS $$
    DECLARE
        entity_name TEXT;
        result TEXT;
        row RECORD;
        account_organization_record_id TEXT;
        _role_permission_id TEXT;
        user_role_name TEXT;
        _permission_id TEXT;
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
        -- Loop over each row in the new_entity_fields transition table
        FOR row IN 
            SELECT 
                nef.id AS entity_field_id,
                nef.created_by AS created_by
            FROM new_entity_fields nef
        LOOP
            -- do something with each row
            RAISE NOTICE 'creating permissions for: %', row.entity_field_id;
            SELECT 
                account_organizations.id,
                user_roles.role
            INTO 
                account_organization_record_id,
                user_role_name
            FROM account_organizations 
            LEFT JOIN user_roles ON account_organizations.role_id = user_roles.id
            WHERE email = row.created_by;

            _permission_id := uuid_generate_v4()::TEXT;
            RAISE NOTICE '_permission_id: %', _permission_id;

            -- create permissions per field
            INSERT INTO permissions (id, read, write, encrypt, decrypt, required, sensitive, archive, delete, created_by) 
            SELECT _permission_id, read, write, encrypt, decrypt, required, sensitive, archive, delete, row.created_by
            WHERE NOT EXISTS (SELECT 1 FROM permissions WHERE id = _permission_id);

            SELECT role_permissions.id INTO _role_permission_id 
            FROM role_permissions 
            LEFT JOIN user_roles ON role_permissions.role_id = user_roles.id
            WHERE user_roles.role = user_role_name;

            -- Check if arr_permissions has elements before accessing
            IF account_organization_record_id IS NOT NULL THEN
                INSERT INTO data_permissions (id, entity_field_id, permission_id, account_organization_id, created_by, role_permission_id) 
                SELECT uuid_generate_v4()::text, row.entity_field_id, _permission_id, account_organization_record_id, row.created_by, _role_permission_id
                WHERE NOT EXISTS (SELECT 1 FROM data_permissions WHERE entity_field_id = row.entity_field_id);
            END IF;

        END LOOP;

        RETURN NULL;
    END;
$$ LANGUAGE plpgsql;
