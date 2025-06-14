use crate::db;
use crate::models::account_model::AccountModel;
use crate::models::account_profile_model::AccountProfileModel;
use crate::models::contact_email_model::ContactEmailModel;
use crate::models::contact_model::ContactModel;
use crate::models::counter_model::CounterModel;
use crate::models::device_model::DeviceModel;
use crate::models::organization_account_model::OrganizationAccountModel;
use crate::models::organization_contact_model::OrganizationContactModel;
use crate::models::organization_model::OrganizationModel;
use crate::organizations::structs::AccountType;
use crate::organizations::structs::Register;
use crate::schema::schema::accounts;
use crate::schema::schema::counters;
use crate::schema::schema::organizations;
use crate::sync::sync_service;
use argon2::password_hash::{rand_core::OsRng, SaltString};
use argon2::{Argon2, PasswordHasher};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel_async::RunQueryDsl;
use ipnetwork::IpNetwork;
use serde_json::json;
use std::env;
use ulid::Ulid;
use tokio_postgres::types::ToSql;
use serde_json::{Value, Map};
use crate::utils::utils;
use crate::organizations::auth_service;
use crate::controllers::store_controller::ApiError;
use actix_web::http::StatusCode;


pub fn get_defaults() -> (String, String, String, String, String, String, bool, String) {
    let default_organization_id = env::var("DEFAULT_ORGANIZATION_ID")
        .unwrap_or_else(|_| "01JBHKXHYSKPP247HZZWHA3JCT".to_string());
    let default_organization_name =
        env::var("DEFAULT_ORGANIZATION_NAME").unwrap_or_else(|_| "global-organization".to_string());
    let default_organization_admin_email = env::var("DEFAULT_ORGANIZATION_ADMIN_EMAIL")
        .unwrap_or_else(|_| "admin@dnamicro.com".to_string());
    let default_organization_admin_password = env::var("DEFAULT_ORGANIZATION_ADMIN_PASSWORD")
        .unwrap_or_else(|_| "ch@ng3m3Pl3@s3!!".to_string());
    let default_device_id =
        env::var("DEFAULT_DEVICE_ID").unwrap_or_else(|_| "system_device".to_string());
    let default_device_secret =
        env::var("DEFAULT_DEVICE_SECRET").unwrap_or_else(|_| "ch@ng3m3Pl3@s3!!".to_string());
    let debug = env::var("DEBUG")
        .unwrap_or_else(|_| "false".to_string())
        .to_lowercase()
        == "true";
    let super_admin_id =
        env::var("SUPER_ADMIN_ID").unwrap_or_else(|_| "01JCSAG79KQ1WM0F9B47Q700P1".to_string());

    (
        default_organization_id,
        default_organization_name,
        default_organization_admin_email,
        default_organization_admin_password,
        default_device_id,
        default_device_secret,
        debug,
        super_admin_id,
    )
}

pub async fn register(
    params: &Register,
    is_request: Option<bool>,
    account_organization_id: Option<String>,
) -> Result<String, ApiError> {
    // Changed return type to use diesel::result::Error
    let mut conn = db::get_async_connection().await;

    let is_request = is_request.unwrap_or(false);
    let (_, _, _, _, _, _, _, super_admin_id) = get_defaults();

    let account_type = params.account_type.clone().unwrap_or(AccountType::Contact);
    let account_id = &params.account_id;
    let team_organization_name = params.organization_name.as_ref();
    let account_secret = &params.account_secret;
    let is_new_user = params.is_new_user.unwrap_or(true);
    let first_name = &params.first_name;
    let last_name = &params.last_name;
    let is_invited = params.is_invited.unwrap_or(false);
    let role_id = params.role_id.clone().unwrap_or_else(|| String::new());
    let account_organization_status = params.account_organization_status.clone();
    let account_organization_categories = params.account_organization_categories.clone();
    let account_organization_id = params.account_organization_id.clone().unwrap_or_default();
    let contact_categories = params.contact_categories.clone();
    let device_categories = params.device_categories.clone();
    let responsible_account_organization_id = params.responsible_account_organization_id.clone();

    let mut personal_organization_id: Option<String> = None;

     let now = chrono::Utc::now();
     let formatted_date = now.format("%Y-%m-%d").to_string(); // Format date
     let formatted_time = now.format("%H:%M:%S").to_string(); // Format time in 24-hour format
     
     let _account_id = if is_request {
        Ulid::new().to_string()
     } else {
         super_admin_id
     };     
     let is_contact_account = account_type == AccountType::Contact;
 

    let existing_account = accounts::table
        .filter(
            accounts::account_id
                .eq(account_id)
                .and(accounts::tombstone.eq(0)),
        )
        .first::<AccountModel>(&mut conn)
        .await
        .optional()?;

    // Query for organizations counter
    let organizations_counter = counters::table
        .filter(counters::entity.eq("organizations"))
        .first::<CounterModel>(&mut conn)
        .await
        .optional()?;

    let contacts_counter = counters::table
        .filter(counters::entity.eq("contacts"))
        .first::<CounterModel>(&mut conn)
        .await
        .optional()?;

    // Query for account_organizations counter
    let account_organizations_counter = counters::table
        .filter(counters::entity.eq("account_organizations"))
        .first::<CounterModel>(&mut conn)
        .await
        .optional()?;

        if existing_account.is_some() {
            // Return a DieselError with a custom message
            return Err(DieselError::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                Box::new("Account with the same email already exist.".to_string()),
            ).into());
        } else {
            //create a personal organization
            let personal_organization_id = create_new_organization(
                "Personal Organization".to_string(),
                vec!["Personal".to_string()],
                None,
                if organizations_counter.is_some() {
                    match utils::generate_code("organizations").await {
                        Ok(code) => code,
                        Err(_) => None,
                    }
                } else {
                    None
                },
                responsible_account_organization_id.clone(),
            ).await?;

            //now create an account

             create_account(
                _account_id, 
                account_id.to_string(), 
                account_secret.to_string(), 
                personal_organization_id, 
                is_new_user, 
                formatted_date, 
                formatted_time, 
                first_name.to_string(), 
                last_name.to_string(), 
                if !is_contact_account {
                    Some("Inactive".to_string())
                } else {
                    None
                }, 
                if !is_contact_account {
                    Some("Draft".to_string())
                } else {
                    None
                }, 
                responsible_account_organization_id.clone(), 
            ).await?;
        }

        if (is_contact_account && (!is_invited || !is_request)) {
            // Your existing code here
        } else if (is_contact_account && is_invited) {
            // Handle invited contact accounts
        }

    Ok("".to_string()) // Changed to return Ok with the string
}


pub async fn create_new_organization(
    organization_name: String,
    categories: Vec<String>,
    organization_id: Option<String>,
    code: Option<String>,
    responsible_account_organization_id: Option<String>,
) -> Result<String, DieselError> {
    let now = chrono::Utc::now();
    let formatted_date = now.format("%Y-%m-%d").to_string(); // Format date
    let formatted_time = now.format("%H:%M:%S").to_string(); // Format time in 24-hour format
    
    let id = organization_id.unwrap_or_else(|| Ulid::new().to_string());
    
    // Create organization object
    let mut organization = Map::new();
    organization.insert("id".to_string(), Value::String(id.clone()));
    organization.insert("name".to_string(), Value::String(organization_name.clone()));
    organization.insert("categories".to_string(), Value::Array(
        categories.iter().map(|c| Value::String(c.clone())).collect()
    ));
    organization.insert("organization_id".to_string(), Value::String(id.clone()));
    organization.insert("parent_organization_id".to_string(), Value::Null);
    organization.insert("tombstone".to_string(), Value::Number(serde_json::Number::from(0)));
    organization.insert("status".to_string(), Value::String("Active".to_string()));
    organization.insert("created_date".to_string(), Value::String(formatted_date.clone()));
    organization.insert("created_time".to_string(), Value::String(formatted_time.clone()));
    organization.insert("updated_date".to_string(), Value::String(formatted_date));
    organization.insert("updated_time".to_string(), Value::String(formatted_time));
    
    if let Some(code_value) = code {
        organization.insert("code".to_string(), Value::String(code_value));
    }
    
    if let Some(responsible_id) = responsible_account_organization_id {
        organization.insert("created_by".to_string(), Value::String(responsible_id));
    }
    
    sync_service::insert(&"organizations".to_string(), Value::Object(organization)).await?;
    
    
    Ok(id)
}


pub async fn create_account(
    id: String,
    account_id: String,
    account_secret: String,
    personal_organization_id: String,
    is_new_user: bool,
    formatted_date: String,
    formatted_time: String,
    first_name: String,
    last_name: String,
    account_status: Option<String>,
    status: Option<String>,
    responsible_account_organization_id: Option<String>,
) -> Result<(), ApiError> {
    
    // Hash the password
    let hashed_password = match auth_service::password_hash(&account_secret).await {
        Ok(hash) => hash,
        Err(e) => {
            log::error!("Failed to hash password: {}", e);
            return Err(ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to hash password: {}", e)
            ).into());
        }
    };
    // Create account with personal organization using AccountModel
    let account = AccountModel {
        id: Some(id.clone()),
        account_id: Some(account_id.clone()),
        account_secret: Some(hashed_password),
        organization_id: Some(personal_organization_id.clone()),
        account_status: Some(account_status.unwrap_or_else(|| "Active".to_string())),
        is_new_user: Some(is_new_user),
        tombstone: Some(0),
        status: Some(status.unwrap_or_else(|| "Active".to_string())),
        created_date: Some(formatted_date.clone()),
        created_time: Some(formatted_time.clone()),
        updated_date: Some(formatted_date.clone()),
        updated_time: Some(formatted_time.clone()),
        created_by: responsible_account_organization_id.clone(),
        ..Default::default()
    };
    
    // Convert model to JSON and insert into database
    let account_json = serde_json::to_value(&account)
    .map_err(|e| ApiError::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Failed to serialize account: {}", e)
    ))?;
    
    // Insert account into database
    sync_service::insert(&"accounts".to_string(), account_json).await?;
    
    // Create account profile using AccountProfileModel
    let account_profile = AccountProfileModel {
        id: Some(Ulid::new().to_string()),
        first_name: Some(first_name),
        last_name: Some(last_name),
        email: Some(account_id.clone()),
        account_id: Some(id.clone()),
        organization_id: Some(personal_organization_id),
        tombstone: Some(0),
        status: Some("Active".to_string()),
        created_date: Some(formatted_date.clone()),
        created_time: Some(formatted_time.clone()),
        updated_date: Some(formatted_date),
        updated_time: Some(formatted_time),
        created_by: responsible_account_organization_id,
        ..Default::default()
    };
    
    // Convert model to JSON and insert into database
    let profile_json = serde_json::to_value(&account_profile)
    .map_err(|e| ApiError::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Failed to serialize account profile: {}", e)
    ))?;
    
    // Insert account profile into database
    sync_service::insert(&"account_profiles".to_string(), profile_json).await?;
    
    log::info!("Created Account: {}, email: {}", id, account_id);
    
    Ok(())
}