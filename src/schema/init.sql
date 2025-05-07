-- clean up
TRUNCATE TABLE fields CASCADE;
TRUNCATE TABLE entity_fields CASCADE;
TRUNCATE TABLE permissions CASCADE;
TRUNCATE TABLE entities CASCADE;

-- entities
INSERT INTO entities (id, name, organization_id, created_by)
SELECT '01JTMGGDS25KQKZCCBSRZZQ8JY', 'data_permissions', '01JBHKXHYSKPP247HZZWHA3JBT','admin@dnamicro.com'
WHERE NOT EXISTS (SELECT 1 FROM entities WHERE id = '01JTMGGDS25KQKZCCBSRZZQ8JY');

-- fields for data_permisions
INSERT INTO fields (id, label, name, type, created_by) 
SELECT 'EntityFieldId_entity_id_text', 'Entity Field Id', 'entity_field_id', 'text', 'admin@dnamicro.com'
WHERE NOT EXISTS (SELECT 1 FROM fields WHERE id = 'EntityId_entity_id_text');

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
SELECT 'Version_version_serial', 'Version', 'version', 'serial', 'admin@dnamicro.com'
WHERE NOT EXISTS (SELECT 1 FROM fields WHERE id = 'Version_version_serial');

-- entity fields
INSERT INTO entity_fields (id, entity_id, field_id, created_by) 
SELECT '01JTMJN1D5V4XHZR43KG9ABP04', '01JTMGGDS25KQKZCCBSRZZQ8JY', 'Version_version_serial', 'admin@dnamicro.com'
WHERE NOT EXISTS (SELECT 1 FROM fields WHERE id = '01JTMJN1D5V4XHZR43KG9ABP04');


-- permissions
INSERT INTO permissions (id, read, write, encrypted, decrypted, required, created_by) 
SELECT '01JTMGV92R9S4ESY1QKHHFGZ0V', true, true, true, true, true, 'admin@dnamicro.com'
WHERE NOT EXISTS (SELECT 1 FROM fields WHERE id = '01JTMGV92R9S4ESY1QKHHFGZ0V');