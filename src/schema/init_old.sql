-- clean up
TRUNCATE TABLE fields CASCADE;
TRUNCATE TABLE entity_fields CASCADE;
TRUNCATE TABLE permissions CASCADE;
TRUNCATE TABLE entities CASCADE;
TRUNCATE TABLE data_permissions CASCADE;

-- entities
INSERT INTO entities (id, name, organization_id, created_by)
SELECT '01JTMGGDS25KQKZCCBSRZZQ8JY', 'data_permissions', '01JBHKXHYSKPP247HZZWHA3JBT','admin@dnamicro.com'
WHERE NOT EXISTS (SELECT 1 FROM entities WHERE id = '01JTMGGDS25KQKZCCBSRZZQ8JY');

-- fields for data_permisions
INSERT INTO fields (id, label, name, type, created_by) 
SELECT 'Id_id_text', 'Id', 'id', 'text', 'admin@dnamicro.com'
WHERE NOT EXISTS (SELECT 1 FROM fields WHERE id = 'Id_id_text');

-- entity fields
INSERT INTO entity_fields (id, entity_id, field_id, created_by) 
SELECT '01JTPWCN7GN1S3PMYM1QDFZNW9', '01JTMGGDS25KQKZCCBSRZZQ8JY', 'Id_id_text', 'admin@dnamicro.com'
WHERE NOT EXISTS (SELECT 1 FROM fields WHERE id = '01JTPWCN7GN1S3PMYM1QDFZNW9');

INSERT INTO fields (id, label, name, type, created_by) 
SELECT 'EntityFieldId_entity_id_text', 'Entity Field Id', 'entity_field_id', 'text', 'admin@dnamicro.com'
WHERE NOT EXISTS (SELECT 1 FROM fields WHERE id = 'EntityFieldId_entity_id_text');

-- entity fields
INSERT INTO entity_fields (id, entity_id, field_id, created_by) 
SELECT '01JTMJ2K8DGDXPG33FMS6PA0BA', '01JTMGGDS25KQKZCCBSRZZQ8JY', 'EntityFieldId_entity_id_text', 'admin@dnamicro.com'
WHERE NOT EXISTS (SELECT 1 FROM fields WHERE id = '01JTMJ2K8DGDXPG33FMS6PA0BA');

INSERT INTO fields (id, label, name, type, created_by) 
SELECT 'PermissionId_permission_id_text', 'Permission Id', 'permission_id', 'text', 'admin@dnamicro.com'
WHERE NOT EXISTS (SELECT 1 FROM fields WHERE id = 'PermissionId_permission_id_text');

-- entity fields
INSERT INTO entity_fields (id, entity_id, field_id, created_by) 
SELECT '01JTMJH6X8RN8C2XXV8Q92V5XJ', '01JTMGGDS25KQKZCCBSRZZQ8JY', 'PermissionId_permission_id_text', 'admin@dnamicro.com'
WHERE NOT EXISTS (SELECT 1 FROM fields WHERE id = '01JTMJH6X8RN8C2XXV8Q92V5XJ');

INSERT INTO fields (id, label, name, type, created_by) 
SELECT 'InheritedPermissionId_inherited_permission_id_text', 'Inherited Permission Id', 'inherited_permission_id', 'text', 'admin@dnamicro.com'
WHERE NOT EXISTS (SELECT 1 FROM fields WHERE id = 'InheritedPermissionId_inherited_permission_id_text');
-- entity fields
INSERT INTO entity_fields (id, entity_id, field_id, created_by) 
SELECT '01JTMJM07RZZJHBKST4N3WMHEV', '01JTMGGDS25KQKZCCBSRZZQ8JY', 'InheritedPermissionId_inherited_permission_id_text', 'admin@dnamicro.com'
WHERE NOT EXISTS (SELECT 1 FROM fields WHERE id = '01JTMJM07RZZJHBKST4N3WMHEV');

INSERT INTO fields (id, label, name, type, created_by) 
SELECT 'AccountOrganizationId_account_organization_id_text', 'Account Organization Id', 'account_organization_id', 'text', 'admin@dnamicro.com'
WHERE NOT EXISTS (SELECT 1 FROM fields WHERE id = 'AccountOrganizationId_account_organization_id_text');

-- entity fields
INSERT INTO entity_fields (id, entity_id, field_id, created_by) 
SELECT '01JTPVDQ4DPAN73YCMQK53VTT0', '01JTMGGDS25KQKZCCBSRZZQ8JY', 'AccountOrganizationId_account_organization_id_text', 'admin@dnamicro.com'
WHERE NOT EXISTS (SELECT 1 FROM fields WHERE id = '01JTPVDQ4DPAN73YCMQK53VTT0');

INSERT INTO fields (id, label, name, type, created_by) 
SELECT 'Version_version_serial', 'Version', 'version', 'serial', 'admin@dnamicro.com'
WHERE NOT EXISTS (SELECT 1 FROM fields WHERE id = 'Version_version_serial');

-- entity fields
INSERT INTO entity_fields (id, entity_id, field_id, created_by) 
SELECT '01JTMJN1D5V4XHZR43KG9ABP04', '01JTMGGDS25KQKZCCBSRZZQ8JY', 'Version_version_serial', 'admin@dnamicro.com'
WHERE NOT EXISTS (SELECT 1 FROM fields WHERE id = '01JTMJN1D5V4XHZR43KG9ABP04');

INSERT INTO fields (id, label, name, type, created_by) 
SELECT 'CreatedBy_created_by_text', 'Created By', 'created_by', 'text', 'admin@dnamicro.com'
WHERE NOT EXISTS (SELECT 1 FROM fields WHERE id = 'CreatedBy_created_by_text');

-- entity fields
INSERT INTO entity_fields (id, entity_id, field_id, created_by) 
SELECT '01JTPVQQH0PY44GPJ1TQVRYFC6', '01JTMGGDS25KQKZCCBSRZZQ8JY', 'CreatedBy_created_by_text', 'admin@dnamicro.com'
WHERE NOT EXISTS (SELECT 1 FROM fields WHERE id = '01JTPVQQH0PY44GPJ1TQVRYFC6');


INSERT INTO fields (id, label, name, type, created_by) 
SELECT 'UpdatedBy_updated_by_text', 'Updated By', 'updated_by', 'text', 'admin@dnamicro.com'
WHERE NOT EXISTS (SELECT 1 FROM fields WHERE id = 'UpdatedBy_updated_by_text');

-- entity fields
INSERT INTO entity_fields (id, entity_id, field_id, created_by) 
SELECT '01JTPVS2P99EJQTX5J3J8GY3G3', '01JTMGGDS25KQKZCCBSRZZQ8JY', 'UpdatedBy_updated_by_text', 'admin@dnamicro.com'
WHERE NOT EXISTS (SELECT 1 FROM fields WHERE id = '01JTPVS2P99EJQTX5J3J8GY3G3');

INSERT INTO fields (id, label, name, type, created_by) 
SELECT 'DeletedBy_deleted_by_text', 'Deleted By', 'deleted_by', 'text', 'admin@dnamicro.com'
WHERE NOT EXISTS (SELECT 1 FROM fields WHERE id = 'DeletedBy_deleted_by_text');

-- entity fields
INSERT INTO entity_fields (id, entity_id, field_id, created_by) 
SELECT '01JTPVTV3YGT50KR38PRB2ZF03', '01JTMGGDS25KQKZCCBSRZZQ8JY', 'DeletedBy_deleted_by_text', 'admin@dnamicro.com'
WHERE NOT EXISTS (SELECT 1 FROM fields WHERE id = '01JTPVTV3YGT50KR38PRB2ZF03');

INSERT INTO fields (id, label, name, type, created_by) 
SELECT 'Timestamp_timestamp_text', 'Timestamp', 'timestamp', 'text', 'admin@dnamicro.com'
WHERE NOT EXISTS (SELECT 1 FROM fields WHERE id = 'Timestamp_timestamp_text');

-- entity fields
INSERT INTO entity_fields (id, entity_id, field_id, created_by) 
SELECT '01JTPVX433TFKEDDXTKP2JRC3Q', '01JTMGGDS25KQKZCCBSRZZQ8JY', 'Timestamp_timestamp_text', 'admin@dnamicro.com'
WHERE NOT EXISTS (SELECT 1 FROM fields WHERE id = '01JTPVX433TFKEDDXTKP2JRC3Q');

INSERT INTO fields (id, label, name, type, created_by) 
SELECT 'Tombstone_tombstone_integer', 'Tombstone', 'tombstone', 'integer', 'admin@dnamicro.com'
WHERE NOT EXISTS (SELECT 1 FROM fields WHERE id = 'Tombstone_tombstone_integer');

-- entity fields
INSERT INTO entity_fields (id, entity_id, field_id, created_by) 
SELECT '01JTPXJ4ZV8PHDH44JKK4RKV91', '01JTMGGDS25KQKZCCBSRZZQ8JY', 'Tombstone_tombstone_integer', 'admin@dnamicro.com'
WHERE NOT EXISTS (SELECT 1 FROM fields WHERE id = '01JTPXJ4ZV8PHDH44JKK4RKV91');


-- permissions
INSERT INTO permissions (id, read, write, encrypted, decrypted, required, created_by) 
SELECT '01JTMGV92R9S4ESY1QKHHFGZ0V', true, true, true, true, true, 'admin@dnamicro.com'
WHERE NOT EXISTS (SELECT 1 FROM fields WHERE id = '01JTMGV92R9S4ESY1QKHHFGZ0V');

INSERT INTO permissions (id, read, write, encrypted, decrypted, required, created_by) 
SELECT '01JTQ5EAWX6VXRY4SDBF38SJTE', false, true, true, true, true, 'admin@dnamicro.com'
WHERE NOT EXISTS (SELECT 1 FROM fields WHERE id = '01JTQ5EAWX6VXRY4SDBF38SJTE');


DO $$
DECLARE
    account_organization_id TEXT;
BEGIN
-- Get the account organization ID
    SELECT id INTO account_organization_id 
    FROM account_organizations 
    WHERE email = 'admin@dnamicro.com';

-- -- If no record found, use a default value
--     IF account_organization_id IS NULL THEN
--         account_organization_id := '01JTPZ0FW9T6NPN5AAE6VAXMPK'; -- Default fallback ID
--     END IF;

-- data permissions - id
-- old permission 01JTMGV92R9S4ESY1QKHHFGZ0V
-- new permission 01JTQ5EAWX6VXRY4SDBF38SJTE
INSERT INTO data_permissions (id, entity_field_id, inherited_permission_id, account_organization_id, created_by) 
SELECT '01JTPW8M43MSPBTZ41GVJGVND2', '01JTPWCN7GN1S3PMYM1QDFZNW9', '01JTQ5EAWX6VXRY4SDBF38SJTE', account_organization_id, 'admin@dnamicro.com'
WHERE NOT EXISTS (SELECT 1 FROM fields WHERE id = '01JTPW8M43MSPBTZ41GVJGVND2');

INSERT INTO data_permissions (id, entity_field_id, inherited_permission_id, account_organization_id, created_by) 
SELECT '01JTPYSCEYVZR443PRAWZ9KKG8', '01JTMJ2K8DGDXPG33FMS6PA0BA', '01JTMGV92R9S4ESY1QKHHFGZ0V', account_organization_id, 'admin@dnamicro.com'
WHERE NOT EXISTS (SELECT 1 FROM fields WHERE id = '01JTPYSCEYVZR443PRAWZ9KKG8');

INSERT INTO data_permissions (id, entity_field_id, inherited_permission_id, account_organization_id, created_by) 
SELECT '01JTPYGSTYPFESWFZ0RP17H6X2', '01JTMJH6X8RN8C2XXV8Q92V5XJ', '01JTMGV92R9S4ESY1QKHHFGZ0V', account_organization_id, 'admin@dnamicro.com'
WHERE NOT EXISTS (SELECT 1 FROM fields WHERE id = '01JTPYGSTYPFESWFZ0RP17H6X2');

INSERT INTO data_permissions (id, entity_field_id, inherited_permission_id, account_organization_id, created_by) 
SELECT '01JTPYQK1Y69850SC08QVVGTYK', '01JTMJM07RZZJHBKST4N3WMHEV', '01JTMGV92R9S4ESY1QKHHFGZ0V', account_organization_id, 'admin@dnamicro.com'
WHERE NOT EXISTS (SELECT 1 FROM fields WHERE id = '01JTPYQK1Y69850SC08QVVGTYK');

INSERT INTO data_permissions (id, entity_field_id, inherited_permission_id, account_organization_id, created_by) 
SELECT '01JTPYRDE2XBRBDXRK17FW23WV', '01JTPVDQ4DPAN73YCMQK53VTT0', '01JTMGV92R9S4ESY1QKHHFGZ0V', account_organization_id, 'admin@dnamicro.com'
WHERE NOT EXISTS (SELECT 1 FROM fields WHERE id = '01JTPYRDE2XBRBDXRK17FW23WV');

INSERT INTO data_permissions (id, entity_field_id, inherited_permission_id, account_organization_id, created_by) 
SELECT '01JTPYV4F4M1RK1AJYW5J8BH96', '01JTMJN1D5V4XHZR43KG9ABP04', '01JTMGV92R9S4ESY1QKHHFGZ0V', account_organization_id, 'admin@dnamicro.com'
WHERE NOT EXISTS (SELECT 1 FROM fields WHERE id = '01JTPYV4F4M1RK1AJYW5J8BH96');

INSERT INTO data_permissions (id, entity_field_id, inherited_permission_id, account_organization_id, created_by) 
SELECT '01JTPYVR1BR5NJA71GW6JQP3B9', '01JTPVQQH0PY44GPJ1TQVRYFC6', '01JTMGV92R9S4ESY1QKHHFGZ0V', account_organization_id, 'admin@dnamicro.com'
WHERE NOT EXISTS (SELECT 1 FROM fields WHERE id = '01JTPYVR1BR5NJA71GW6JQP3B9');

INSERT INTO data_permissions (id, entity_field_id, inherited_permission_id, account_organization_id, created_by) 
SELECT '01JTPYW6ZBM3KCWV568VB8KF9W', '01JTPVS2P99EJQTX5J3J8GY3G3', '01JTMGV92R9S4ESY1QKHHFGZ0V', account_organization_id, 'admin@dnamicro.com'
WHERE NOT EXISTS (SELECT 1 FROM fields WHERE id = '01JTPYW6ZBM3KCWV568VB8KF9W');

INSERT INTO data_permissions (id, entity_field_id, inherited_permission_id, account_organization_id, created_by) 
SELECT '01JTPYX54QZ8W5K7A75X2P0MAM', '01JTPVTV3YGT50KR38PRB2ZF03', '01JTMGV92R9S4ESY1QKHHFGZ0V', account_organization_id, 'admin@dnamicro.com'
WHERE NOT EXISTS (SELECT 1 FROM fields WHERE id = '01JTPYX54QZ8W5K7A75X2P0MAM');

INSERT INTO data_permissions (id, entity_field_id, inherited_permission_id, account_organization_id, created_by) 
SELECT '01JTPYXQA1AZRCBGT9F6RBPA11', '01JTPVX433TFKEDDXTKP2JRC3Q', '01JTMGV92R9S4ESY1QKHHFGZ0V', account_organization_id, 'admin@dnamicro.com'
WHERE NOT EXISTS (SELECT 1 FROM fields WHERE id = '01JTPYXQA1AZRCBGT9F6RBPA11');

INSERT INTO data_permissions (id, entity_field_id, inherited_permission_id, account_organization_id, created_by) 
SELECT '01JTPYY0YG4CQ1KGGHDQG3YQ0T', '01JTPXJ4ZV8PHDH44JKK4RKV91', '01JTMGV92R9S4ESY1QKHHFGZ0V', account_organization_id, 'admin@dnamicro.com'
WHERE NOT EXISTS (SELECT 1 FROM fields WHERE id = '01JTPYY0YG4CQ1KGGHDQG3YQ0T');

END $$