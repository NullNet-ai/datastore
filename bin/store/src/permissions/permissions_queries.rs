use crate::utils::constructor_service;
use diesel::prelude::*;
use diesel::sql_types::*;
use diesel::QueryableByName;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(QueryableByName, Debug, Serialize, Deserialize)]
pub struct PermissionQueryResult {
    #[diesel(sql_type = Nullable<Text>)]
    pub permission_id: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub entity_field_id: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub account_organization_id: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub id: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub role_permission_id: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub record_id: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub record_entity: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub role: Option<String>,
    #[diesel(sql_type = Nullable<Integer>)]
    pub sensitivity_level: Option<i32>,
    #[diesel(sql_type = Nullable<Text>)]
    pub entity: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub field: Option<String>,
    #[diesel(sql_type = Nullable<Bool>)]
    pub is_encryptable: Option<bool>,
    #[diesel(sql_type = Nullable<Bool>)]
    pub is_system_field: Option<bool>,
    #[diesel(sql_type = Nullable<Bool>)]
    pub is_searchable: Option<bool>,
    #[diesel(sql_type = Nullable<Bool>)]
    pub is_allowed_to_return: Option<bool>,
    #[diesel(sql_type = Nullable<Bool>)]
    pub sensitive: Option<bool>,
    #[diesel(sql_type = Nullable<Bool>)]
    pub read: Option<bool>,
    #[diesel(sql_type = Nullable<Bool>)]
    pub write: Option<bool>,
    #[diesel(sql_type = Nullable<Bool>)]
    pub encrypt: Option<bool>,
    #[diesel(sql_type = Nullable<Bool>)]
    pub decrypt: Option<bool>,
    #[diesel(sql_type = Nullable<Bool>)]
    pub required: Option<bool>,
    #[diesel(sql_type = Nullable<Bool>)]
    pub archive: Option<bool>,
    #[diesel(sql_type = Nullable<Bool>)]
    pub delete: Option<bool>,
}

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
         data_permissions.role_permission_id as role_permission_id, 
         record_permissions.record_id as record_id, 
         record_permissions.record_entity as record_entity, 
         ur.role as role, 
         ur.sensitivity_level as sensitivity_level, 
         entities.name as entity, 
         fields.name as field, 
         CASE WHEN entity_fields.is_encryptable = TRUE THEN 
         entity_fields.is_encryptable ELSE scf.is_encryptable END as is_encryptable, 
         scf.is_system_field as is_system_field, 
         scf.is_searchable as is_searchable, 
         scf.is_allowed_to_return as is_allowed_to_return, 
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
         LEFT JOIN role_permissions ON data_permissions.role_permission_id = role_permissions.id 
         LEFT JOIN permissions as ip ON role_permissions.permission_id = ip.id 
         LEFT JOIN record_permissions ON data_permissions.record_permission_id = record_permissions.id 
         LEFT JOIN permissions as rp ON record_permissions.permission_id = rp.id 
         LEFT JOIN account_organizations ON account_organizations.id = data_permissions.account_organization_id 
         LEFT JOIN user_roles as ur ON account_organizations.role_id = ur.id 
         LEFT JOIN system_config_fields as scf ON fields.id = scf.field_id 
         WHERE (( ur.sensitivity_level >= {}) OR data_permissions.account_organization_id = '{}') 
         {}",
        sensitivity_level,
        account_organization_id,
        constructor_service::construct_permission_select_where_clause(tables, main_fields)
    );

    query
}

#[derive(QueryableByName, Debug, Serialize, Deserialize)]
pub struct ValidPassKeyResult {
    #[diesel(sql_type = Text)]
    pub id: String,
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

#[derive(QueryableByName, Debug, Serialize, Deserialize)]
pub struct GroupByFieldRecordPermissionsResult {
    #[diesel(sql_type = Nullable<Text>)]
    pub role: Option<String>,
    #[diesel(sql_type = Text)]
    pub entity: String,
    #[diesel(sql_type = BigInt)]
    pub total_fields: i64,
    #[diesel(sql_type = BigInt)]
    pub total_fields_with_write: i64,
    #[diesel(sql_type = Bool)]
    pub sensitive: bool,
    #[diesel(sql_type = Bool)]
    pub read: bool,
    #[diesel(sql_type = Bool)]
    pub write: bool,
    #[diesel(sql_type = Bool)]
    pub encrypt: bool,
    #[diesel(sql_type = Bool)]
    pub decrypt: bool,
    #[diesel(sql_type = Bool)]
    pub required: bool,
    #[diesel(sql_type = Bool)]
    pub archive: bool,
    #[diesel(sql_type = Bool)]
    pub delete: bool,
}

pub fn get_group_by_field_record_permissions(table: &str, role_id: &str) -> String {
    let query = format!(
        " 
           SELECT 
           JSON_AGG(ur.role)->1 AS role, 
           entities.name as entity, 
           count(fields.name) AS total_fields, 
           COUNT(*) FILTER ( 
             WHERE ( 
                   CASE 
                     WHEN data_permissions.role_permission_id IS NOT NULL 
                       THEN ip.write 
                       ELSE p.write 
                   END 
             IS TRUE) 
           ) as total_fields_with_write, 
           CASE 
             WHEN COUNT(*) FILTER ( 
                 WHERE ( 
                   CASE 
                     WHEN data_permissions.role_permission_id IS NOT NULL 
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
                     WHEN data_permissions.role_permission_id IS NOT NULL 
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
                     WHEN data_permissions.role_permission_id IS NOT NULL 
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
                     WHEN data_permissions.role_permission_id IS NOT NULL 
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
                     WHEN data_permissions.role_permission_id IS NOT NULL 
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
                     WHEN data_permissions.role_permission_id IS NOT NULL 
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
                     WHEN data_permissions.role_permission_id IS NOT NULL 
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
                     WHEN data_permissions.role_permission_id IS NOT NULL 
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
           LEFT JOIN role_permissions ON data_permissions.role_permission_id = role_permissions.id 
           LEFT JOIN permissions as ip ON role_permissions.permission_id = ip.id 
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
#[derive(QueryableByName, Debug, Serialize, Deserialize)]
pub struct RolePermissionResult {
    #[diesel(sql_type = Nullable<Text>)]
    pub pid: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub role: Option<String>,
    #[diesel(sql_type = Nullable<Integer>)]
    pub sensitivity_level: Option<i32>,
    #[diesel(sql_type = Nullable<Bool>)]
    pub sensitive: Option<bool>,
    #[diesel(sql_type = Nullable<Bool>)]
    pub read: Option<bool>,
    #[diesel(sql_type = Nullable<Bool>)]
    pub write: Option<bool>,
    #[diesel(sql_type = Nullable<Bool>)]
    pub encrypt: Option<bool>,
    #[diesel(sql_type = Nullable<Bool>)]
    pub decrypt: Option<bool>,
    #[diesel(sql_type = Nullable<Bool>)]
    pub required: Option<bool>,
    #[diesel(sql_type = Nullable<Bool>)]
    pub archive: Option<bool>,
    #[diesel(sql_type = Nullable<Bool>)]
    pub delete: Option<bool>,
}

pub fn get_role_permissions_query(role_id: &str) -> String {
    let query = format!(
        " 
         SELECT 
           p.id as pid,  
           user_roles.role as role, 
           user_roles.sensitivity_level as sensitivity_level, 
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
         LEFT JOIN user_roles on role_permissions.role_id = user_roles.id 
         WHERE user_roles.id = '{}' 
         ",
        role_id
    );

    query
}

/// Enum representing the different types of permission queries
pub enum PermissionQueryType {
    Permissions,
    ValidPassKeys,
    GroupByFieldRecordPermissions,
    RolePermissions,
}

/// Execute a permission query based on the query type and parameters
pub async fn execute_permission_query<'a, C>(
    conn: &mut C,
    query_type: PermissionQueryType,
    params: PermissionQueryParams,
) -> Result<PermissionQueryOutput, Box<dyn Error>>
where
    C: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + 'a,
{
    match query_type {
        PermissionQueryType::Permissions => {
            if let PermissionQueryParams::DataPermissions {
                tables,
                main_fields,
                sensitivity_level,
                account_organization_id,
            } = params
            {
                let query = get_permissions_query(
                    &tables,
                    &main_fields,
                    sensitivity_level,
                    &account_organization_id,
                );
                let results = diesel::dsl::sql_query(query)
                    .load::<PermissionQueryResult>(conn)
                    .await?;
                Ok(PermissionQueryOutput::Permissions(results))
            } else {
                Err("Invalid parameters for Permissions query".into())
            }
        }
        PermissionQueryType::ValidPassKeys => {
            if let PermissionQueryParams::ValidPassKeys {
                organization_id,
                table,
                pgp_sym_key,
            } = params
            {
                let query = get_valid_pass_keys_query(&organization_id, &table, &pgp_sym_key);
                let results = diesel::dsl::sql_query(query)
                    .load::<ValidPassKeyResult>(conn)
                    .await?;
                Ok(PermissionQueryOutput::ValidPassKeys(results))
            } else {
                Err("Invalid parameters for ValidPassKeys query".into())
            }
        }
        PermissionQueryType::GroupByFieldRecordPermissions => {
            if let PermissionQueryParams::GroupByFieldRecordPermissions { table, role_id } = params
            {
                let query = get_group_by_field_record_permissions(&table, &role_id);
                let results = diesel::dsl::sql_query(query)
                    .load::<GroupByFieldRecordPermissionsResult>(conn)
                    .await?;
                Ok(PermissionQueryOutput::GroupByFieldRecordPermission(results))
            } else {
                Err("Invalid parameters for GroupByFieldRecordPermission query".into())
            }
        }
        PermissionQueryType::RolePermissions => {
            if let PermissionQueryParams::RolePermissions { role_id } = params {
                let query = get_role_permissions_query(&role_id);
                let results = diesel::dsl::sql_query(query)
                    .load::<RolePermissionResult>(conn)
                    .await?;
                Ok(PermissionQueryOutput::RolePermissions(results))
            } else {
                Err("Invalid parameters for RolePermissions query".into())
            }
        }
    }
}

/// Parameters for different permission queries
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PermissionQueryParams {
    DataPermissions {
        tables: Vec<String>,
        main_fields: Vec<String>,
        sensitivity_level: u32,
        account_organization_id: String,
    },
    ValidPassKeys {
        organization_id: String,
        table: String,
        pgp_sym_key: String,
    },
    GroupByFieldRecordPermissions {
        table: String,
        role_id: String,
    },
    RolePermissions {
        role_id: String,
    },
}

impl Default for PermissionQueryParams {
    fn default() -> Self {
        Self::DataPermissions {
            tables: Vec::new(),
            main_fields: Vec::new(),
            sensitivity_level: 0,
            account_organization_id: String::new(),
        }
    }
}

/// Output types for different permission queries
pub enum PermissionQueryOutput {
    Permissions(Vec<PermissionQueryResult>),
    ValidPassKeys(Vec<ValidPassKeyResult>),
    GroupByFieldRecordPermission(Vec<GroupByFieldRecordPermissionsResult>),
    RolePermissions(Vec<RolePermissionResult>),
}
