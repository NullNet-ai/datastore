use crate::utils::constructor_service;

pub fn get_permissions_query(
    tables: &[String],
    main_fields: &[String],
    sensitivity_level: u32,
    account_organization_id: &String,
) -> String {
    let query = format!(
        " 
         SELECT 
         p.id as permission_id, 
         data_permissions.entity_field_id as entity_field_id, 
         data_permissions.account_organization_id as account_organization_id, 
         data_permissions.id as id, 
         data_permissions.inherited_permission_id as inherited_permission_id, 
         p.record_id as record_id, 
         p.record_entity as record_entity, 
         ur.role as role, 
         ur.level as level, 
         entities.name as entity, 
         fields.name as field, 
         fields.is_encryptable as is_encryptable, 
         CASE WHEN ip.sensitive IS NOT NULL THEN ip.sensitive 
                 ELSE p.sensitive END as sensitive, 
         CASE WHEN ip.read IS NOT NULL THEN ip.read 
                 ELSE p.read END as read, 
         CASE WHEN ip.write IS NOT NULL THEN ip.write 
                 ELSE p.write END as write, 
         CASE WHEN ip.encrypt IS NOT NULL THEN ip.encrypt 
                 ELSE p.encrypt END as encrypt, 
         CASE WHEN ip.decrypt IS NOT NULL THEN ip.decrypt 
                 ELSE p.decrypt END as decrypt, 
         CASE WHEN ip.required IS NOT NULL THEN ip.required 
                 ELSE p.required END as required, 
         CASE WHEN ip.archive IS NOT NULL THEN ip.archive 
                 ELSE p.archive END as archive, 
         CASE WHEN ip.delete IS NOT NULL THEN ip.delete 
                 ELSE p.delete END as delete 
         FROM data_permissions 
         LEFT JOIN entity_fields ON data_permissions.entity_field_id = entity_fields.id 
         LEFT JOIN fields ON entity_fields.field_id = fields.id 
         LEFT JOIN entities ON entity_fields.entity_id = entities.id 
         LEFT JOIN permissions as p ON data_permissions.permission_id = p.id 
         LEFT JOIN permissions as ip ON data_permissions.inherited_permission_id = ip.id 
         LEFT JOIN account_organizations ON account_organizations.id = data_permissions.account_organization_id 
         LEFT JOIN user_roles as ur ON account_organizations.role_id = ur.id 
         WHERE (( ur.level >= {}) OR data_permissions.account_organization_id = '{}') 
         {}",
        sensitivity_level,
        account_organization_id,
        constructor_service::construct_permission_select_where_clause(tables, main_fields)
    );

    query
}

pub fn get_valid_pass_keys_query(organization_id: &str, table: &str, pgp_sym_key: &str) -> String {
    let query = format!(
        " 
         SELECT id FROM encryption_keys WHERE safe_decrypt(organization_id::TEXT,'{}') = '{}' AND safe_decrypt(entity::TEXT,'{}') = '{}' 
         ",
        pgp_sym_key,
        organization_id,
        pgp_sym_key,
        table
    );

    query
}

pub fn get_record_valid_pass_keys_query(table: &str, role_id: &str) -> String {
    let query = format!(
        " 
           SELECT 
           JSON_AGG(ur.role)->1 AS role, 
           entities.name as entity, 
           count(fields.name) AS total_fields, 
           COUNT(*) FILTER ( 
             WHERE ( 
                   CASE 
                     WHEN data_permissions.inherited_permission_id IS NOT NULL 
                       THEN ip.write 
                       ELSE p.write 
                   END 
             IS TRUE) 
           ) as total_fields_with_write, 
           CASE 
             WHEN COUNT(*) FILTER ( 
                 WHERE ( 
                   CASE 
                     WHEN data_permissions.inherited_permission_id IS NOT NULL 
                       THEN ip.sensitive 
                       ELSE p.sensitive 
                   END 
                 IS TRUE) 
               ) != count(fields.name) 
               THEN false 
               ELSE true 
           END AS sensitive, 
           CASE 
             WHEN COUNT(*) FILTER ( 
                 WHERE ( 
                   CASE 
                     WHEN data_permissions.inherited_permission_id IS NOT NULL 
                       THEN ip.read 
                       ELSE p.read 
                   END 
                 IS TRUE) 
               ) != count(fields.name) 
               THEN false 
               ELSE true 
           END AS read, 
           CASE 
             WHEN COUNT(*) FILTER ( 
                 WHERE ( 
                   CASE 
                     WHEN data_permissions.inherited_permission_id IS NOT NULL 
                       THEN ip.write 
                       ELSE p.write 
                   END 
                 IS TRUE) 
               ) != count(fields.name) 
               THEN false 
               ELSE true 
           END AS write, 
           CASE 
             WHEN COUNT(*) FILTER ( 
                 WHERE ( 
                   CASE 
                     WHEN data_permissions.inherited_permission_id IS NOT NULL 
                       THEN ip.encrypt 
                       ELSE p.encrypt 
                   END 
                 IS TRUE) 
               ) != count(fields.name) 
               THEN false 
               ELSE true 
           END AS encrypt, 
           CASE 
             WHEN COUNT(*) FILTER ( 
                 WHERE ( 
                   CASE 
                     WHEN data_permissions.inherited_permission_id IS NOT NULL 
                       THEN ip.decrypt 
                       ELSE p.decrypt 
                   END 
                 IS TRUE) 
               ) != count(fields.name) 
               THEN false 
               ELSE true 
           END AS decrypt, 
           CASE 
             WHEN COUNT(*) FILTER ( 
                 WHERE ( 
                   CASE 
                     WHEN data_permissions.inherited_permission_id IS NOT NULL 
                       THEN ip.required 
                       ELSE p.required 
                   END 
                 IS TRUE) 
               ) != count(fields.name) 
               THEN false 
               ELSE true 
           END AS required, 
           CASE 
             WHEN COUNT(*) FILTER ( 
                 WHERE ( 
                   CASE 
                     WHEN data_permissions.inherited_permission_id IS NOT NULL 
                       THEN ip.archive 
                       ELSE p.archive 
                   END 
                 IS TRUE) 
               ) != count(fields.name) 
               THEN false 
               ELSE true 
           END AS archive, 
           CASE 
             WHEN COUNT(*) FILTER ( 
                 WHERE ( 
                   CASE 
                     WHEN data_permissions.inherited_permission_id IS NOT NULL 
                       THEN ip.delete 
                       ELSE p.delete 
                   END 
                 IS TRUE) 
               ) != count(fields.name) 
               THEN false 
               ELSE true 
           END AS delete 
           FROM data_permissions 
           LEFT JOIN entity_fields ON data_permissions.entity_field_id = entity_fields.id 
           LEFT JOIN fields ON entity_fields.field_id = fields.id 
           LEFT JOIN entities ON entity_fields.entity_id = entities.id 
           LEFT JOIN permissions as p ON data_permissions.permission_id = p.id 
           LEFT JOIN permissions as ip ON data_permissions.inherited_permission_id = ip.id 
           LEFT JOIN account_organizations ON account_organizations.id = data_permissions.account_organization_id 
           LEFT JOIN user_roles as ur ON account_organizations.role_id = ur.id 
           WHERE ur.id = '{}' AND entities.name = '{}' 
           GROUP BY entities.name; 
         ",
        role_id,
        table
    );

    query
}

pub fn get_role_permissions_query(role_id: &str) -> String {
    let query = format!(
        " 
         SELECT 
           p.id as pid,  
           role_permissions.role_name as role, 
           user_roles.level as level, 
           p.sensitive as sensitive, 
           p.read as read, 
           p.write as write, 
           p.encrypt as encrypt, 
           p.decrypt as decrypt, 
           p.required as required, 
           p.archive as archive, 
           p.delete as delete 
         FROM role_permissions 
         LEFT JOIN permissions as p on role_permissions.permission_id = p.id 
         LEFT JOIN data_permissions on role_permissions.permission_id = data_permissions.permission_id 
         LEFT JOIN user_roles on role_permissions.role_name = user_roles.role 
         WHERE user_roles.id = '{}' 
         ",
        role_id
    );

    query
}
