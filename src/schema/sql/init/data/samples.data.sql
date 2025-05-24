DO $$
DECLARE
    name TEXT := 'samples';
    record_id TEXT;
    encryption_key_id TEXT;
    organization_id TEXT := '01JBHKXHYSKPP247HZZWHA3JCT';
    record_email TEXT := 'admin@dnamicro.com';
    pgp_sym_key TEXT := 'dummy_secret_key';
    record_limit INT := 10000;
BEGIN
    RAISE NOTICE 'name = %', name;
    FOR i IN 1..record_limit LOOP
        record_id := uuid_generate_v4()::TEXT;
        
        -- Only show progress every 1000 records to reduce log noise
        IF i % 1000 = 0 THEN
            RAISE NOTICE 'Progress: % of % records (% percent)', 
                i, 
                record_limit, 
                ROUND((i::float / record_limit::float) * 100);
        END IF;
        RAISE NOTICE 'record_id = %', record_id;
        encryption_key_id := encode(digest(organization_id || '_' || name || '_' || pgp_sym_key, 'sha1'), 'hex');
        RAISE NOTICE 'encryption_key_id: %', encryption_key_id;
        -- insert encryption_keys
        INSERT INTO encryption_keys (id, organization_id, entity, created_by) VALUES (
            encryption_key_id,
            safe_encrypt(organization_id, pgp_sym_key),
            safe_encrypt(name, pgp_sym_key),
            record_email
        ) ON CONFLICT (id) DO NOTHING;

        INSERT INTO samples (id, name, sample_text, categories, status, organization_id) VALUES (
            record_id,
            name || i,
            'sample_text ' || i,
            jsonb_build_array('category' || i, 'subcategory' || i),
            CASE WHEN i % 2 = 0 THEN 'Active' ELSE 'InActive' END,
            organization_id
        );
    END LOOP;
END 
$$;