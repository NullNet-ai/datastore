use diesel::sql_query;
use diesel::sql_types::Text;
use diesel::QueryableByName;
use diesel_async::RunQueryDsl;
use log::{debug, info};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

const _SCHEMA_RELATED_FIELD_DEPTH: usize = 3;

#[derive(Debug, Clone)]
pub struct GenerateSchemaOptions {
    pub include_crdt_tables: Vec<String>,
    pub exclude_formatting_fields: Vec<String>,
}

impl Default for GenerateSchemaOptions {
    fn default() -> Self {
        Self {
            include_crdt_tables: Vec::new(),
            exclude_formatting_fields: Vec::new(),
        }
    }
}

impl GenerateSchemaOptions {
    /// Check if a field should be excluded from formatting based on the options
    #[allow(dead_code)]
    pub fn should_exclude_field(&self, field_name: &str) -> bool {
        self.exclude_formatting_fields
            .contains(&field_name.to_string())
    }

    /// Check if a table is in the CRDT tables list
    #[allow(dead_code)]
    pub fn is_crdt_table(&self, table_name: &str) -> bool {
        self.include_crdt_tables.contains(&table_name.to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForeignKeyInfo {
    pub source_table: String,
    pub column: String,
    pub referenced_table: String,
}

#[allow(dead_code)]
pub struct GenerateSchemaService {
    db_pool: &'static crate::database::db::AsyncDbPool,
    redis_client: redis::Client,
}

impl GenerateSchemaService {
    #[allow(dead_code)]
    pub fn new(redis_client: redis::Client) -> Self {
        Self {
            db_pool: crate::database::db::get_async_pool(),
            redis_client,
        }
    }

    #[allow(dead_code)]
    pub async fn generate_schema(
        &self,
        options: GenerateSchemaOptions,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("Generating application schema.");

        // Get all table names from the database
        let tables = self.get_all_tables().await?;

        // Process each table
        for table in tables {
            self.process_table(&table, &options).await?;
        }

        Ok(())
    }

    #[allow(dead_code)]
    async fn get_all_tables(
        &self,
    ) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
        let mut conn = self.db_pool.get().await?;

        let query = r#"
            SELECT table_name 
            FROM information_schema.tables 
            WHERE table_schema = 'public' 
            AND table_type = 'BASE TABLE'
        "#;

        let results = sql_query(query).load::<TableNameRow>(&mut conn).await?;

        Ok(results.into_iter().map(|row| row.table_name).collect())
    }

    #[allow(dead_code)]
    async fn extract_foreign_keys(
        &self,
        table_name: &str,
    ) -> Result<Vec<ForeignKeyInfo>, Box<dyn std::error::Error + Send + Sync>> {
        let mut conn = self.db_pool.get().await?;

        let query = format!(
            r#"
            SELECT
                tc.table_name AS source_table,
                kcu.column_name AS column,
                ccu.table_name AS referenced_table
            FROM
                information_schema.table_constraints AS tc
                JOIN information_schema.key_column_usage AS kcu
                    ON tc.constraint_name = kcu.constraint_name
                    AND tc.table_schema = kcu.table_schema
                JOIN information_schema.constraint_column_usage AS ccu
                    ON ccu.constraint_name = tc.constraint_name
                    AND ccu.table_schema = tc.table_schema
            WHERE
                tc.constraint_type = 'FOREIGN KEY'
                AND tc.table_name = '{}'
                AND tc.table_schema = 'public'
        "#,
            table_name
        );

        let results = sql_query(&query).load::<ForeignKeyRow>(&mut conn).await?;

        Ok(results
            .into_iter()
            .map(|row| ForeignKeyInfo {
                source_table: row.source_table,
                column: row.column,
                referenced_table: row.referenced_table,
            })
            .collect())
    }

    #[allow(dead_code)]
    fn format_table_fields_sync(
        &self,
        table: &str,
        parent_field_name: &str,
        _exclude_formatting_fields: &[String],
        depth: usize,
    ) -> Vec<String> {
        if depth > _SCHEMA_RELATED_FIELD_DEPTH {
            return Vec::new();
        }

        let mut current_parent = parent_field_name.to_string();
        if current_parent.is_empty() {
            current_parent = pluralizer::pluralize(table, 1, false);
        }

        // For now, return a simple implementation without recursion
        // In a real implementation, this would need to be async and handle foreign keys
        vec![format!("{}.{}", current_parent, table)]
    }

    #[allow(dead_code)]
    async fn get_table_schema(
        &self,
        table: &str,
    ) -> Result<HashMap<String, String>, Box<dyn std::error::Error + Send + Sync>> {
        let mut conn = self.db_pool.get().await?;

        let query = format!(
            r#"
            SELECT column_name, data_type
            FROM information_schema.columns
            WHERE table_name = '{}' AND table_schema = 'public'
            ORDER BY ordinal_position
        "#,
            table
        );

        let results = sql_query(&query).load::<ColumnInfoRow>(&mut conn).await?;

        let schema: HashMap<String, String> = results
            .into_iter()
            .map(|row| (row.column_name, row.data_type))
            .collect();

        Ok(schema)
    }

    #[allow(dead_code)]
    async fn process_table(
        &self,
        table: &str,
        options: &GenerateSchemaOptions,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut redis_conn = self.redis_client.get_async_connection().await?;

        // Get table schema
        let table_schema = self.get_table_schema(table).await?;

        // Format fields with related fields (simplified for now)
        let formatted_fields =
            self.format_table_fields_sync(table, "", &options.exclude_formatting_fields, 0);

        let schema_data = json!({
            "table_name": table,
            "column": serde_json::to_string(&table_schema)?,
            "constraint": serde_json::to_string(&self.extract_foreign_keys(table).await?)?,
            "index": serde_json::to_string(&self.get_table_indexes(table).await?)?,
            "formatted_with_related_fields": serde_json::to_string(&formatted_fields)?,
        });

        let hash_key = format!("schema:{}", table);

        // Save to Redis as hash
        for (key, value) in schema_data.as_object().unwrap() {
            redis_conn
                .hset::<_, _, _, ()>(&hash_key, key, value.as_str().unwrap_or(""))
                .await?;
        }

        debug!("Successfully saved {} schema to Redis", table);

        Ok(())
    }

    #[allow(dead_code)]
    async fn get_table_indexes(
        &self,
        table: &str,
    ) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
        let mut conn = self.db_pool.get().await?;

        let query = format!(
            r#"
            SELECT indexname
            FROM pg_indexes
            WHERE tablename = '{}' AND schemaname = 'public'
        "#,
            table
        );

        let results = sql_query(&query).load::<IndexRow>(&mut conn).await?;

        Ok(results.into_iter().map(|row| row.indexname).collect())
    }
}

// Diesel query result types
#[derive(Debug, QueryableByName)]
#[allow(dead_code)]
struct TableNameRow {
    #[diesel(sql_type = Text)]
    table_name: String,
}

#[derive(Debug, QueryableByName)]
#[allow(dead_code)]
struct ForeignKeyRow {
    #[diesel(sql_type = Text)]
    source_table: String,
    #[diesel(sql_type = Text)]
    column: String,
    #[diesel(sql_type = Text)]
    referenced_table: String,
}

#[derive(Debug, QueryableByName)]
#[allow(dead_code)]
struct ColumnInfoRow {
    #[diesel(sql_type = Text)]
    column_name: String,
    #[diesel(sql_type = Text)]
    data_type: String,
}

#[derive(Debug, QueryableByName)]
#[allow(dead_code)]
struct IndexRow {
    #[diesel(sql_type = Text)]
    indexname: String,
}
