use crate::schema::schema::products;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use serde_json::Value;

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = products)]
pub struct Product {
    pub id: String,
    pub sku: String,
    pub name: String,
    pub description: String,
    pub category_id: i32,
    pub price: String,
    pub currency: String,
    pub stock_quantity: i32,
    pub is_active: bool,
    pub is_featured: bool,
    pub weight_kg: Option<String>,
    pub dimensions: Option<Value>,
    pub tags: String,
    pub metadata: Option<Value>,
    pub image_urls: String,
    pub supplier_info: Option<Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: DateTime<Utc>,
    pub first_name: String,
}
