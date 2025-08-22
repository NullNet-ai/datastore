use crate::database::schema::schema::sync_endpoints;
use crate::generated::models::sync_endpoint_model::SyncEndpointModel;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel_async::AsyncPgConnection;
use diesel_async::RunQueryDsl;

use super::transport::transport_driver::PostOpts;

#[allow(warnings)]
pub async fn get_all_sync_endpoints(
    conn: &mut AsyncPgConnection,
) -> Result<Vec<SyncEndpointModel>, DieselError> {
    let endpoints = sync_endpoints::table
        .load::<SyncEndpointModel>(conn)
        .await?;

    if endpoints.is_empty() {
        return Err(DieselError::NotFound);
    }

    Ok(endpoints)
}

pub async fn create_endpoint(
    conn: &mut AsyncPgConnection,
    endpoint: &SyncEndpointModel,
) -> Result<serde_json::Value, DieselError> {
    // Log the endpoint data (similar to console.log in the TypeScript version)
    log::info!("@schema.sync_endpoints {:?}", endpoint);

    diesel::insert_into(sync_endpoints::table)
        .values(endpoint)
        .on_conflict(sync_endpoints::id)
        .do_update()
        .set(endpoint)
        .execute(conn)
        .await?;

    // Return a JSON response with message: "ok"
    Ok(serde_json::json!({
        "message": "ok"
    }))
}

pub async fn get_sync_endpoints(
    conn: &mut AsyncPgConnection,
) -> Result<Vec<PostOpts>, DieselError> {
    let endpoints = sync_endpoints::table
        .filter(sync_endpoints::status.eq("Active"))
        .select((
            sync_endpoints::url,
            sync_endpoints::username,
            sync_endpoints::password,
        ))
        .load::<(String, String, String)>(conn)
        .await?;

    let result: Vec<PostOpts> = endpoints
        .into_iter()
        .map(|(url, username, password)| PostOpts {
            url,
            username: username,
            password: password,
        })
        .collect();

    Ok(result)
}
