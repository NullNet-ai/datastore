use crate::db;
use crate::db::DbPooledConnection;
use crate::models::sync_endpoint_model::SyncEndpoint;
use crate::schema::schema::sync_endpoints;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PostOpts {
    pub url: String,
    pub username: Option<String>,
    pub password: Option<String>,
}

pub fn get_all_sync_endpoints(conn: &mut DbPooledConnection) -> Result<Vec<SyncEndpoint>, DieselError> {
    let endpoints = sync_endpoints::table
        .load::<SyncEndpoint>(conn)?;
    
    if endpoints.is_empty() {
        return Err(DieselError::NotFound);
    }
    
    Ok(endpoints)
}

pub fn create_endpoint(conn: &mut DbPooledConnection, endpoint: SyncEndpoint) -> Result<serde_json::Value, DieselError> {
    // Log the endpoint data (similar to console.log in the TypeScript version)
    log::info!("@schema.sync_endpoints {:?}", endpoint);
    
    diesel::insert_into(sync_endpoints::table)
        .values(&endpoint)
        .on_conflict(sync_endpoints::id)
        .do_update()
        .set(&endpoint)
        .execute(conn)?;
    
    // Return a JSON response with message: "ok"
    Ok(serde_json::json!({
        "message": "ok"
    }))
}

pub fn get_active_sync_endpoints(conn: &mut DbPooledConnection) -> Result<Vec<PostOpts>, DieselError> {
    let endpoints = sync_endpoints::table
        .filter(sync_endpoints::status.eq("Active"))
        .select((
            sync_endpoints::url,
            sync_endpoints::username,
            sync_endpoints::password,
        ))
        .load::<(String, String, String)>(conn)?;
    
    let result: Vec<PostOpts> = endpoints
        .into_iter()
        .map(|(url, username, password)| PostOpts {
            url,
            username: Some(username),
            password: Some(password),
        })
        .collect();
    
    Ok(result)
}