use crate::auth::auth_service;
use crate::controllers::store_controller::ApiError;
use crate::db;
use crate::models::account_model::AccountModel;
use crate::organizations::structs::LoginResponse;
use crate::schema::schema::accounts;
use actix_web::http::StatusCode;
use chrono::{Duration, Utc};
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::Text;
use diesel::QueryableByName;
use diesel_async::RunQueryDsl;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::sync::Mutex;

#[derive(QueryableByName, Debug)]
struct JsonResult {
    #[diesel(sql_type = diesel::sql_types::Text)]
    json_result: String,
}

#[derive(Debug, QueryableByName, Queryable)]
pub struct AccountWithOrg {
    #[diesel(sql_type = diesel::sql_types::Integer)]
    pub account_organization_id: i32,
    #[diesel(embed)]
    pub account: AccountModel,
}

// Simple in-memory cache implementation for tokens
static TOKEN_CACHE: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub async fn auth(
    account_id: &str,
    account_secret: &str,
    organization_id: &str,
) -> Result<LoginResponse, ApiError> {
    // Get database connection
    let mut conn = db::get_async_connection().await;

    let account_json = get_account_with_org(account_id).await?;
    // If no accounts found in the first query, try the fallback query
    let (account, account_organization_id) = if !account_json.is_object()
        || account_json.as_object().unwrap().is_empty()
    {
        // Fallback query directly to accounts table
        log::debug!("No accounts found in account_organizations, trying direct accounts query");

        let direct_accounts = accounts::table
            .filter(accounts::tombstone.eq(0))
            .filter(accounts::status.eq("Active"))
            .filter(accounts::account_id.eq(account_id))
            .load::<AccountModel>(&mut conn)
            .await
            .map_err(|e| {
                ApiError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Accounts query error: {}", e),
                )
            })?;

        if !direct_accounts.is_empty() {
            // Use the account from the fallback query
            (Some(direct_accounts[0].clone()), None)
        } else {
            // No accounts found in either query
            log::debug!("No accounts found for the provided credentials");
            (None, None)
        }
    } else {
        // Extract account_organization_id from JSON
        let account_organization_id = account_json["account_organization_id"]
            .as_str()
            .map(|s| s.to_string());

        // Deserialize the account data into AccountModel
        let account: Option<AccountModel> = serde_json::from_value(account_json["account"].clone())
            .map(Some)
            .map_err(|e| {
                ApiError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Account deserialization error: {}", e),
                )
            })?;

        (account, account_organization_id)
    };

    // If no account found in either query, return None
    let account =
        account.ok_or_else(|| ApiError::new(StatusCode::NOT_FOUND, "Account not found"))?;

    // Verify the password
    if let Some(stored_hash) = &account.account_secret {
        let is_valid = auth_service::password_verify(stored_hash, account_secret).await?;

        if !is_valid {
            return Err(ApiError::new(
                StatusCode::UNAUTHORIZED,
                "Invalid credentials",
            ));
        }

        // Get the signed in account with all related data
        let signed_in_account;

        if account_organization_id.is_some() {
            let account_organization_id = account_organization_id.unwrap();

            // Create your filters array based on your requirements
            let filters = vec!["ao.tombstone = 0", "ao.status = 'Active'"];

            // Call the function to get the account with profile and organization data
            signed_in_account =
                match get_account_with_profile_and_org(&account_organization_id, &filters).await {
                    Ok(account) => account,
                    Err(err) => {
                        log::error!("Error fetching account with profile and org: {}", err);
                        serde_json::json!({})
                    }
                };
        } else {
            // Call the function to get the account with profile and organization data by account_id
            signed_in_account =
                match get_account_with_profile_and_org_by_account_id(account_id).await {
                    Ok(account) => account,
                    Err(err) => {
                        log::error!(
                            "Error fetching account with profile and org by account_id: {}",
                            err
                        );
                        serde_json::json!({})
                    }
                };
        };

        // Create token value with the signed in account
        let token_value = json!({
            "account": signed_in_account,
            "signed_in_account": signed_in_account
        });

        // Generate JWT token
        let new_token = sign(&token_value).await?;

        // Cache the token
        let jwt_expires_in = env::var("JWT_EXPIRES_IN").unwrap_or_else(|_| "24h".to_string());
        let expiration_ms = time_string_to_ms(&jwt_expires_in);
        let mut cache = TOKEN_CACHE.lock().unwrap();
        cache.insert(new_token.clone(), token_value.to_string());

        return Ok(LoginResponse {
            message: "Authenticated".to_string(),
            token: Some(new_token),
        });
    }

    // Password verification failed
    Err(ApiError::new(
        StatusCode::UNAUTHORIZED,
        "Invalid credentials",
    ))
}

pub async fn root_auth(
    account_id: &str,
    account_secret: &str,
    previously_logged_in: Option<&str>,
) -> Result<LoginResponse, ApiError> {
    // Get database connection
    let mut conn = db::get_async_connection().await;

    // Query to find the root account

    // Build the SQL query to get account with profile and organization data
    let result = sql_query(
        "
        SELECT json_build_object(
            'is_root_account', true,
            'account', json_build_object(
                'id', a.id,
                'account_id', a.account_id,
                'account_secret', a.account_secret,
                'status', a.status
            ),
            'profile', CASE WHEN ap.id IS NOT NULL THEN json_build_object(
                'id', ap.id,
                'first_name', ap.first_name,
                'last_name', ap.last_name,
                'email', ap.email,
                'account_id', ap.account_id,
                'categories', ap.categories,
                'code', ap.code,
                'status', ap.status,
                'organization_id', ap.organization_id
            ) ELSE NULL END,
            'organization', CASE WHEN o.id IS NOT NULL THEN json_build_object(
                'id', o.id,
                'name', o.name,
                'code', o.code,
                'categories', o.categories,
                'status', o.status,
                'organization_id', o.organization_id,
                'parent_organization_id', o.parent_organization_id
            ) ELSE NULL END,
            'id', a.id,
            'account_id', a.account_id,
            'organization_id', ao.organization_id,
            'account_organization_id', ao.id,
            'account_status', ao.account_organization_status,
            'role_id', ao.role_id
        ) as json_result
        FROM account_organizations ao
        LEFT JOIN accounts a ON a.id = ao.account_id
        LEFT JOIN account_profiles ap ON ap.account_id = a.id
        LEFT JOIN organizations o ON o.id = ao.organization_id
        WHERE ao.tombstone = 0
        AND ao.email = $1
        AND ao.status = 'Active'
        AND ao.categories @> ARRAY['Root']
        LIMIT 1
        ",
    )
    .bind::<Text, _>(account_id)
    .get_result::<JsonResult>(&mut conn)
    .await;

    // Process the result
    let account_organization = match result {
        Ok(json_result) => {
            // Parse the JSON string into a serde_json::Value
            let value: serde_json::Value =
                serde_json::from_str(&json_result.json_result).map_err(|e| {
                    ApiError::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("JSON parsing error: {}", e),
                    )
                })?;

            // Add empty contact and device objects
            let mut value_obj = value.as_object().unwrap().clone();
            value_obj.insert("contact".to_string(), json!({}));
            value_obj.insert("device".to_string(), json!({}));

            serde_json::Value::Object(value_obj)
        }
        Err(diesel::result::Error::NotFound) => {
            // Return an empty JSON object if no results found
            if let Some(debug) = std::env::var("DEBUG").ok() {
                if debug == "true" {
                    log::error!("Root account not found");
                }
            }
            json!({})
        }
        Err(e) => {
            if let Some(debug) = std::env::var("DEBUG").ok() {
                if debug == "true" {
                    log::error!("{}", e);
                }
            }
            return Err(ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database query error: {}", e),
            ));
        }
    };

    // Check if account exists
    if account_organization.is_object() && account_organization.get("account").is_none() {
        return Err(ApiError::new(
            StatusCode::NOT_FOUND,
            "Root Account not found",
        ));
    }

    // Extract account data
    let account = account_organization["account"].clone();

    // Check if password is provided
    if account_secret.is_empty() {
        return Err(ApiError::new(StatusCode::FORBIDDEN, "Password is required"));
    }

    // Verify password
    let verified = auth_service::password_verify(
        account["account_secret"].as_str().unwrap_or_default(),
        account_secret,
    )
    .await?;

    if !verified {
        return Ok(LoginResponse {
            message: "Invalid Root Credentials".to_string(),
            token: None,
        });
    }

    // Create token value
    let mut account_org_clone = account_organization.clone();
    if let Some(obj) = account_org_clone.as_object_mut() {
        obj.remove("account");
    }

    let token_value = json!({
        "account": account_org_clone,
        "previously_logged_in": previously_logged_in
    });

    // Generate JWT token
    let new_token = sign(&token_value).await?;

    // Cache the token
    let jwt_expires_in = env::var("JWT_EXPIRES_IN").unwrap_or_else(|_| "24h".to_string());
    let expiration_ms = time_string_to_ms(&jwt_expires_in);
    let mut cache = TOKEN_CACHE.lock().unwrap();
    cache.insert(new_token.clone(), token_value.to_string());

    Ok(LoginResponse {
        message: "Authenticated".to_string(),
        token: Some(new_token),
    })
}

pub fn clear_cache(token: &str) -> bool {
    let mut cache = TOKEN_CACHE.lock().unwrap();
    cache.remove(token).is_some()
}

pub fn time_string_to_ms(time_str: &str) -> Result<u64, Box<dyn std::error::Error>> {
    // Format: 1d 2h 30m 45s
    if let Some(captures) =
        regex::Regex::new(r"^((?:\d+)d\s*)?((?:\d+)h\s*)?((?:\d+)m\s*)?((?:\d+)s\s*)?$")
            .unwrap()
            .captures(time_str)
    {
        let days = captures.get(1).map_or(0, |m| {
            m.as_str()
                .trim_end_matches('d')
                .trim()
                .parse::<u64>()
                .unwrap_or(0)
        });
        let hours = captures.get(2).map_or(0, |m| {
            m.as_str()
                .trim_end_matches('h')
                .trim()
                .parse::<u64>()
                .unwrap_or(0)
        });
        let minutes = captures.get(3).map_or(0, |m| {
            m.as_str()
                .trim_end_matches('m')
                .trim()
                .parse::<u64>()
                .unwrap_or(0)
        });
        let seconds = captures.get(4).map_or(0, |m| {
            m.as_str()
                .trim_end_matches('s')
                .trim()
                .parse::<u64>()
                .unwrap_or(0)
        });

        let total_ms = days * 24 * 60 * 60 * 1000
            + hours * 60 * 60 * 1000
            + minutes * 60 * 1000
            + seconds * 1000;
        return Ok(total_ms);
    }

    // Format: HH:mm:ss
    if let Some(captures) = regex::Regex::new(r"^(\d{1,2}):(\d{2}):(\d{2})$")
        .unwrap()
        .captures(time_str)
    {
        let hours = captures
            .get(1)
            .map_or(0, |m| m.as_str().parse::<u64>().unwrap_or(0));
        let minutes = captures
            .get(2)
            .map_or(0, |m| m.as_str().parse::<u64>().unwrap_or(0));
        let seconds = captures
            .get(3)
            .map_or(0, |m| m.as_str().parse::<u64>().unwrap_or(0));

        let total_ms = hours * 60 * 60 * 1000 + minutes * 60 * 1000 + seconds * 1000;
        return Ok(total_ms);
    }

    // Format: mm:ss
    if let Some(captures) = regex::Regex::new(r"^(\d{1,2}):(\d{2})$")
        .unwrap()
        .captures(time_str)
    {
        let minutes = captures
            .get(1)
            .map_or(0, |m| m.as_str().parse::<u64>().unwrap_or(0));
        let seconds = captures
            .get(2)
            .map_or(0, |m| m.as_str().parse::<u64>().unwrap_or(0));

        let total_ms = minutes * 60 * 1000 + seconds * 1000;
        return Ok(total_ms);
    }

    // If none of the formats match
    Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::InvalidInput,
        "Invalid time format",
    )))
}

pub async fn get_account_with_profile_and_org_by_account_id(
    account_id: &str,
) -> Result<serde_json::Value, ApiError> {
    // Get database connection
    let mut conn = db::get_async_connection().await;

    // Define a struct to hold the JSON result
    #[derive(QueryableByName, Debug)]
    struct JsonResult {
        #[diesel(sql_type = diesel::sql_types::Text)]
        json_result: String,
    }

    // Query the database using raw SQL that returns JSON
    let result = sql_query(
        "
        SELECT json_build_object(
            'id', a.id,
            'account_id', a.account_id,
            'organization_id', a.organization_id,
            'account_status', a.account_status,
            'profile', CASE WHEN ap.id IS NOT NULL THEN json_build_object(
                'id', ap.id,
                'first_name', ap.first_name,
                'last_name', ap.last_name,
                'email', ap.email,
                'account_id', ap.account_id,
                'categories', ap.categories,
                'code', ap.code,
                'status', ap.status,
                'organization_id', ap.organization_id
            ) ELSE NULL END,
            'organization', CASE WHEN o.id IS NOT NULL THEN json_build_object(
                'id', o.id,
                'name', o.name,
                'code', o.code,
                'categories', o.categories,
                'status', o.status,
                'organization_id', o.organization_id,
                'parent_organization_id', o.parent_organization_id
            ) ELSE NULL END,
            'contact', json_build_object(),
            'device', json_build_object(),
            'account_organization_id', NULL,
            'role_id', NULL
        ) as json_result
        FROM accounts a
        LEFT JOIN account_profiles ap ON ap.account_id = a.id
        LEFT JOIN organizations o ON o.id = a.organization_id
        WHERE a.tombstone = 0
          AND a.status = 'Active'
          AND a.account_id = $1
        LIMIT 1
    ",
    )
    .bind::<Text, _>(account_id)
    .get_result::<JsonResult>(&mut conn)
    .await;

    match result {
        Ok(json_result) => {
            // Parse the JSON string into a serde_json::Value
            let value: serde_json::Value =
                serde_json::from_str(&json_result.json_result).map_err(|e| {
                    ApiError::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("JSON parsing error: {}", e),
                    )
                })?;

            Ok(value)
        }
        Err(diesel::result::Error::NotFound) => {
            // Return an empty JSON object if no results found
            Ok(serde_json::json!({}))
        }
        Err(e) => {
            if let Some(debug) = std::env::var("DEBUG").ok() {
                if debug == "true" {
                    log::error!("{}", e);
                }
            }
            Err(ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database query error: {}", e),
            ))
        }
    }
}

pub async fn get_account_with_profile_and_org(
    account_organization_id: &str,
    filters: &[&str], // Pass your filters as an array of strings
) -> Result<serde_json::Value, ApiError> {
    // Get database connection
    let mut conn = db::get_async_connection().await;

    // Define a struct to hold the JSON result
    #[derive(QueryableByName, Debug)]
    struct JsonResult {
        #[diesel(sql_type = diesel::sql_types::Text)]
        json_result: String,
    }

    // Build the filter conditions
    let filter_conditions = filters.join(" AND ");
    let filter_clause = if !filter_conditions.is_empty() {
        format!("AND ({}) ", filter_conditions)
    } else {
        String::new()
    };

    // Query the database using raw SQL that returns JSON
    let query = format!(
        "SELECT json_build_object(
        'id', a.id,
        'account_id', a.account_id,
        'organization_id', ao.organization_id,
        'account_organization_id', ao.id,
        'account_status', ao.account_organization_status,
        'role_id', ao.role_id,
        'profile', CASE WHEN ap.id IS NOT NULL THEN json_build_object(
            'id', ap.id,
            'first_name', ap.first_name,
            'last_name', ap.last_name,
            'email', ap.email,
            'account_id', ap.account_id,
            'categories', ap.categories,
            'code', ap.code,
            'status', ap.status,
            'organization_id', ap.organization_id
        ) ELSE NULL END,
        'contact', CASE WHEN c.id IS NOT NULL THEN json_build_object(
            'id', c.id,
            'first_name', c.first_name,
            'last_name', c.last_name,
            'account_id', c.account_id,
            'code', c.code,
            'categories', c.categories,
            'status', c.status,
            'organization_id', c.organization_id,
            'date_of_birth', c.date_of_birth
        ) ELSE NULL END,
        'device', CASE WHEN d.id IS NOT NULL THEN json_build_object(
    'id', d.id,
    'code', d.code,
    'categories', d.categories,
    'status', d.status,
    'organization_id', d.organization_id,
    'timestamp', d.timestamp
) ELSE NULL END,
        'organization', CASE WHEN o.id IS NOT NULL THEN json_build_object(
            'id', o.id,
            'name', o.name,
            'code', o.code,
            'categories', o.categories,
            'status', o.status,
            'organization_id', o.organization_id,
            'parent_organization_id', o.parent_organization_id
        ) ELSE NULL END
    ) as json_result
    FROM account_organizations ao
    LEFT JOIN accounts a ON a.id = ao.account_id
    LEFT JOIN account_profiles ap ON ap.account_id = a.id
    LEFT JOIN contacts c ON c.id = ao.contact_id
    LEFT JOIN devices d ON d.id = ao.device_id
    LEFT JOIN organizations o ON o.id = ao.organization_id
    WHERE ao.id = $1 {}LIMIT 1",
        filter_clause
    );

    let result = sql_query(&query)
        .bind::<Text, _>(account_organization_id)
        .get_result::<JsonResult>(&mut conn)
        .await;

    match result {
        Ok(json_result) => {
            // Parse the JSON string into a serde_json::Value
            let value: serde_json::Value =
                serde_json::from_str(&json_result.json_result).map_err(|e| {
                    ApiError::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("JSON parsing error: {}", e),
                    )
                })?;

            Ok(value)
        }
        Err(diesel::result::Error::NotFound) => {
            // Return an empty JSON object if no results found
            Ok(serde_json::json!({}))
        }
        Err(e) => {
            if let Some(debug) = std::env::var("DEBUG").ok() {
                if debug == "true" {
                    log::error!("{}", e);
                }
            }
            Err(ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database query error: {}", e),
            ))
        }
    }
}

async fn get_account_with_org(account_id: &str) -> Result<serde_json::Value, ApiError> {
    // Get database connection
    let mut conn = db::get_async_connection().await;

    // Define a struct to hold the JSON result
    #[derive(QueryableByName, Debug)]
    struct JsonResult {
        #[diesel(sql_type = diesel::sql_types::Json)]
        json_result: serde_json::Value,
    }

    // Query the database using raw SQL that returns JSON
    let result = sql_query(
        "
        SELECT json_build_object(
            'account_organization_id', ao.id,
            'account', row_to_json(a.*)
        ) as json_result
        FROM account_organizations ao
        LEFT JOIN accounts a ON a.id = ao.account_id
        WHERE ao.tombstone = 0
          AND ao.status = 'Active'
          AND ao.email = $1
          AND ao.account_id IS NOT NULL
        LIMIT 1
    ",
    )
    .bind::<Text, _>(account_id)
    .get_result::<JsonResult>(&mut conn)
    .await;

    match result {
        Ok(json_result) => {
            // The json_result.json_result is already a serde_json::Value, no need to parse
            Ok(json_result.json_result)
        }
        Err(diesel::result::Error::NotFound) => {
            // Return an empty JSON object if no results found
            Ok(serde_json::json!({}))
        }
        Err(e) => {
            if let Some(debug) = std::env::var("DEBUG").ok() {
                if debug == "true" {
                    log::error!("{}", e);
                }
            }
            Err(ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!(
                    "Database query error in getting account with organization: {}",
                    e
                ),
            ))
        }
    }
}

async fn sign(token_value: &serde_json::Value) -> Result<String, Box<dyn std::error::Error>> {
    let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "Ch@ng3m3Pl3@s3!!".to_string());
    let jwt_expires_in = env::var("JWT_EXPIRES_IN").unwrap_or_else(|_| "24h".to_string());

    // Set token expiration using JWT_EXPIRES_IN
    let expiration_ms = time_string_to_ms(&jwt_expires_in).unwrap_or(24 * 60 * 60 * 1000); // Default to 24h
    let expiration = Utc::now() + Duration::milliseconds(expiration_ms as i64);
    let now = Utc::now();

    // Create a mutable clone of the token_value to add exp and iat
    let mut payload = token_value.clone();

    // Add exp and iat to the payload
    if let Some(obj) = payload.as_object_mut() {
        obj.insert("exp".to_string(), json!(expiration.timestamp() as usize));
        obj.insert("iat".to_string(), json!(now.timestamp() as usize));
    }

    // Encode the token with the full payload
    let token = encode(
        &Header::new(Algorithm::HS256),
        &payload,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )?;

    Ok(token)
}

pub async fn invalidate_token(token: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Remove token from cache
    let mut cache = TOKEN_CACHE.lock().unwrap();
    cache.remove(token);

    Ok(())
}
