DO $$
DECLARE
    name TEXT := 'user_roles';
    record_id TEXT;
    encryption_key_id TEXT;
    organization_id TEXT := '01JBHKXHYSKPP247HZZWHA3JCT';
    record_email TEXT := 'superadmin@dnamicro.com';
    record_emails TEXT[] := ARRAY['superadmin@dnamicro.com', 'dbadmin@dnamicro.com', 'admin@dnamicro.com', 'member@dnamicro.com'];
    pgp_sym_key TEXT := 'dummy_secret_key';
    roles TEXT[] := ARRAY['root','Super Admin', 'DB Admin', 'Guest', 'Admin', 'Member'];
    record_limit INT := array_length(roles, 1);
    _permission_id TEXT;
    _role_permission_id TEXT;
    lpermissions BOOLEAN[];
    role_level INTEGER := 1000;
    re TEXT;
    account_organization_record_id TEXT;
BEGIN
    --RAISE NOTICE 'name = %', name;
    -- Get the account organization ID
    SELECT id INTO account_organization_record_id 
    FROM account_organizations 
    WHERE email = record_email;
    FOR i IN 1..record_limit LOOP
        record_id := TRIM(REGEXP_REPLACE(LOWER(roles[i]), '\s+', '_', 'g'));
        _permission_id := uuid_generate_v4()::TEXT;
        -- Only show progress every 1000 records to reduce log noise
        IF i % 1000 = 0 THEN
            RAISE NOTICE 'Progress: % of % records (% percent)', 
                i, 
                record_limit, 
                ROUND((i::float / record_limit::float) * 100);
        END IF;
        
        FOREACH re IN ARRAY record_emails LOOP
            BEGIN
                _role_permission_id := uuid_generate_v4()::TEXT;
                --RAISE NOTICE 'record_id = %', record_id;
                encryption_key_id := encode(digest(organization_id || '_' || name || '_' || pgp_sym_key, 'sha1'), 'hex');
                --RAISE NOTICE 'encryption_key_id: %', encryption_key_id;
                -- insert encryption_keys
                INSERT INTO encryption_keys (id, organization_id, entity, created_by) VALUES (
                    encryption_key_id,
                    safe_encrypt(organization_id, pgp_sym_key),
                    safe_encrypt(name, pgp_sym_key),
                    re
                ) ON CONFLICT (id) DO NOTHING;

                -- [ read, write, encrypt, decrypt, required, sensitive, archive, delete ]
                IF (roles[i] = 'Super Admin') THEN
                    -- [ read=true, write=true, encrypt=true, decrypt=true, required=false, sensitive=false, archive=true, delete=true ]
                    lpermissions := ARRAY[true, true, true, true, false, false, true, true];
                    role_level = 0;
                ELSIF (roles[i] = 'DB Admin') THEN
                    -- [ read=true, write=true, encrypt=true, decrypt=true, required=false, sensitive=false, archive=true, delete=true ]
                    lpermissions := ARRAY[true, true, true, true, false, false, true, true];
                    role_level = 0;
                ELSIF (roles[i] = 'Guest') THEN
                    -- [ read=true, write=false, encrypt=false, decrypt=false, required=false, sensitive=false, archive=false, delete=false ]
                    lpermissions := ARRAY[true, false, false, false, false, false, false, false];
                    role_level = 1000;
                ELSIF (roles[i] = 'Admin') THEN
                    -- [ read=true, write=true, encrypt=true, decrypt=true, required=false, sensitive=false, archive=true, delete=true ]
                    lpermissions := ARRAY[true, true, true, true, false, false, true, true];
                    role_level = 1;
                ELSIF (roles[i] = 'Member') THEN
                    -- [ read=true, write=true, encrypt=false, decrypt=false, required=false, sensitive=false, archive=false, delete=false ]
                    lpermissions := ARRAY[true, true, false, false, false, false, false, false];
                    role_level = 500;
                END IF;
            
                 

                INSERT INTO user_roles (id, role, entity, status, organization_id, sensitivity_level, created_by) VALUES (
                    record_id,
                    roles[i],
                    'Contact',
                    'Active', 
                    organization_id,
                    role_level,
                    account_organization_record_id
                ) ON CONFLICT (role) DO NOTHING;

                INSERT INTO permissions (id, read, write, encrypt, decrypt, required, sensitive, archive, delete, created_by) 
                SELECT _permission_id, lpermissions[1], lpermissions[2], lpermissions[3], lpermissions[4], lpermissions[5], lpermissions[6], lpermissions[7], lpermissions[8], re
                WHERE NOT EXISTS (SELECT 1 FROM permissions WHERE id = _permission_id);

                INSERT INTO role_permissions (id, role_id, permission_id, created_by)
                SELECT _role_permission_id, record_id, _permission_id, re
                WHERE NOT EXISTS (SELECT 1 FROM role_permissions WHERE id = _role_permission_id);

                WITH data_perm AS (
                    SELECT dp.id
                    FROM data_permissions dp
                    LEFT JOIN account_organizations on account_organizations.id = dp.account_organization_id
                    LEFT JOIN user_roles as ur ON account_organizations.role_id = ur.id
                    WHERE ur.role = roles[i]
                )
                UPDATE data_permissions 
                SET role_permission_id = _role_permission_id
                FROM data_perm
                WHERE data_permissions.id = data_perm.id;
            END;
        END LOOP;
        
    END LOOP;
END 
$$;