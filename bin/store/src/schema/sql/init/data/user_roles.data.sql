DO $$
DECLARE
    name TEXT := 'user_roles';
    record_id TEXT;
    encryption_key_id TEXT;
    organization_id TEXT := '01JBHKXHYSKPP247HZZWHA3JCT';
    record_email TEXT := 'admin@dnamicro.com';
    pgp_sym_key TEXT := 'dummy_secret_key';
    record_limit INT := 5;
    roles TEXT[] := ARRAY['Super Admin', 'DB Admin', 'QA Admin', 'PM Admin', 'Developer'];
    permission_id TEXT;
    lpermissions BOOLEAN[];
    role_level INTEGER := 1000;
BEGIN
    --RAISE NOTICE 'name = %', name;
    FOR i IN 1..record_limit LOOP
        record_id := uuid_generate_v4()::TEXT;
        permission_id := uuid_generate_v4()::TEXT;
        -- Only show progress every 1000 records to reduce log noise
        IF i % 1000 = 0 THEN
            RAISE NOTICE 'Progress: % of % records (% percent)', 
                i, 
                record_limit, 
                ROUND((i::float / record_limit::float) * 100);
        END IF;
        --RAISE NOTICE 'record_id = %', record_id;
        encryption_key_id := encode(digest(organization_id || '_' || name || '_' || pgp_sym_key, 'sha1'), 'hex');
        --RAISE NOTICE 'encryption_key_id: %', encryption_key_id;
        -- insert encryption_keys
        INSERT INTO encryption_keys (id, organization_id, entity, created_by) VALUES (
            encryption_key_id,
            safe_encrypt(organization_id, pgp_sym_key),
            safe_encrypt(name, pgp_sym_key),
            record_email
        ) ON CONFLICT (id) DO NOTHING;

        -- [ read, write, encrypt, decrypt, required, sensitive, archive, delete ]
        IF (roles[i] = 'Super Admin') THEN
            lpermissions := ARRAY[true, true, true, true, false, false, true, true];
            role_level = 0;
        ELSIF (roles[i] = 'DB Admin') THEN
            lpermissions := ARRAY[true, true, true, true, false, false, true, true];
            role_level = 1;
        ELSIF (roles[i] = 'QA Admin') THEN
            lpermissions := ARRAY[true, true, true, true, false, false, true, true];
            role_level = 10;
        ELSIF (roles[i] = 'PM Admin') THEN
            lpermissions := ARRAY[true, true, false, false, false, false, true, true];
            role_level = 10;
        ELSIF (roles[i] = 'Developer') THEN
            lpermissions := ARRAY[true, true, false, false, false, false, true, false];
            role_level = 9;
        END IF;
       
        INSERT INTO user_roles (id, role, entity, status, organization_id, level) VALUES (
            record_id,
            roles[i],
            'Contact',
            'Active', 
            organization_id,
            role_level
        ) ON CONFLICT (role) DO UPDATE SET level = role_level;

        INSERT INTO permissions (id, read, write, encrypt, decrypt, required, sensitive, archive, delete, created_by) 
            SELECT permission_id, lpermissions[1], lpermissions[2], lpermissions[3], lpermissions[4], lpermissions[5], lpermissions[6], lpermissions[7], lpermissions[8], record_email
            WHERE NOT EXISTS (SELECT 1 FROM permissions WHERE id = permission_id);

        INSERT INTO role_permissions (id, role_name, permission_id) VALUES (
            uuid_generate_v4()::TEXT,
            roles[i],
            permission_id
        ) ON CONFLICT (role_name) DO NOTHING;
    END LOOP;
END 
$$;