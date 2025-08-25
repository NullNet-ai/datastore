use crate::controllers::store_controller::ApiError;
use crate::database::db;
use crate::generated::schema;
use crate::generated::models::account_model::AccountModel;
use crate::generated::models::account_organization_model::AccountOrganizationModel;
use crate::generated::models::account_profile_model::AccountProfileModel;
use crate::generated::models::counter_model::CounterModel;
use crate::generated::models::organization_model::OrganizationModel;
use crate::init::structs::InitializerParams;
use actix_web::http::StatusCode;
use chrono::Utc;
use diesel::prelude::*;
use diesel_async::{AsyncConnection, RunQueryDsl};
use std::env;

pub struct RootAccountInitializer;

impl RootAccountInitializer {
    pub fn new() -> Self {
        RootAccountInitializer
    }

    pub async fn initialize(&self, params: Option<InitializerParams>) -> Result<(), ApiError> {
        if let Some(params) = &params {
            params.entity.clone()
        } else {
            return Err(ApiError::new(
                StatusCode::BAD_REQUEST,
                "Indicate entity for Root Account Configuration".to_string(),
            ));
        };

        // Define constants for root account
        let root_account_id = "01JM3GTWCHR3CM2NP85C0Q2KN1".to_string();
        let personal_organization_id = "01JSN4XA2C3A7RHN3MNZZJGBR3".to_string();
        let account_id = "root".to_string();
        let account_secret =
            env::var("ROOT_ACCOUNT_PASSWORD").unwrap_or_else(|_| "pl3@s3ch@ng3m3!!".to_string());

        let mut conn = db::get_async_connection().await;

        // Check if root account already exists
        let existing_root = schema::account_organizations::table
            .filter(schema::account_organizations::dsl::id.eq(&root_account_id))
            .first::<AccountOrganizationModel>(&mut conn)
            .await
            .optional()?;

        let existing_root_org = schema::organizations::table
            .filter(schema::organizations::dsl::id.eq(&personal_organization_id))
            .first::<OrganizationModel>(&mut conn)
            .await
            .optional()?;

        if existing_root.is_some() || existing_root_org.is_some() {
            log::warn!("Root Account already existing.");
            return Ok(());
        }

        // Get counters for code generation
        let organizations_counter = schema::counters::table
            .filter(schema::counters::dsl::entity.eq("organizations"))
            .first::<CounterModel>(&mut conn)
            .await
            .optional()?;

        let account_organizations_counter = schema::counters::table
            .filter(schema::counters::dsl::entity.eq("account_organizations"))
            .first::<CounterModel>(&mut conn)
            .await
            .optional()?;

        // Generate code function
        let generate_root_account_code = |counter: &CounterModel| -> String {
            let root_count = 0;
            let prefix = &counter.prefix;
            let default_code = counter.default_code;
            let mut digits_number = counter.digits_number;

            let get_digit = |num: i32| -> usize { num.to_string().len() };

            if digits_number > 0 {
                digits_number = digits_number - get_digit(root_count) as i32;
                let zero_digits = if digits_number > 0 {
                    "0".repeat(digits_number as usize)
                } else {
                    "".to_string()
                };
                format!("{}{}{}", prefix, zero_digits, root_count)
            } else {
                format!("{}{}", prefix, default_code + root_count)
            }
        };

        // Get current date and time
        let now = Utc::now();
        let formatted_date = now.format("%Y-%m-%d").to_string();
        let formatted_time = now.format("%H:%M:%S").to_string();

        // Create system fields
        let system_fields = (
            0,                      // tombstone
            "Active".to_string(),   // status
            formatted_date.clone(), // created_date
            formatted_time.clone(), // created_time
            formatted_date.clone(), // updated_date
            formatted_time.clone(), // updated_time
        );

        // Hash the password
        let hashed_password =
            match crate::providers::operations::auth::auth_service::password_hash(&account_secret)
                .await
            {
                Ok(hash) => hash,
                Err(e) => {
                    return Err(ApiError::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Failed to hash password: {}", e),
                    ));
                }
            };

        // Create personal organization
        let mut personal_organization = OrganizationModel {
            id: Some(personal_organization_id.clone()),
            name: Some("Root Personal Organization".to_string()),
            categories: Some(vec!["Root".to_string(), "Personal".to_string()]),
            organization_id: Some(personal_organization_id.clone()),
            tombstone: Some(system_fields.0),
            status: Some(system_fields.1.clone()),
            created_date: Some(system_fields.2.clone()),
            created_time: Some(system_fields.3.clone()),
            updated_date: Some(system_fields.4.clone()),
            updated_time: Some(system_fields.5.clone()),
            ..Default::default()
        };

        // Add code if counter exists
        if let Some(counter) = &organizations_counter {
            personal_organization.code = Some(generate_root_account_code(counter));
        }

        // Create root account
        let root_account = AccountModel {
            id: Some(root_account_id.clone()),
            categories: Some(vec!["Root".to_string()]),
            account_id: Some(account_id.clone()),
            account_secret: Some(hashed_password),
            organization_id: Some(personal_organization_id.clone()),
            account_status: Some("Active".to_string()),
            tombstone: Some(system_fields.0),
            status: Some(system_fields.1.clone()),
            created_date: Some(system_fields.2.clone()),
            created_time: Some(system_fields.3.clone()),
            updated_date: Some(system_fields.4.clone()),
            updated_time: Some(system_fields.5.clone()),
            ..Default::default()
        };

        // Create root account profile
        let root_account_profile = AccountProfileModel {
            id: Some(root_account_id.clone()),
            email: Some(account_id.clone()),
            account_id: Some(root_account_id.clone()),
            organization_id: Some(personal_organization_id.clone()),
            tombstone: Some(system_fields.0),
            status: Some(system_fields.1.clone()),
            created_date: Some(system_fields.2.clone()),
            created_time: Some(system_fields.3.clone()),
            updated_date: Some(system_fields.4.clone()),
            updated_time: Some(system_fields.5.clone()),
            ..Default::default()
        };

        // Create root account organization
        let mut root_account_organization = AccountOrganizationModel {
            id: Some(root_account_id.clone()),
            email: Some(account_id.clone()),
            categories: Some(vec!["Root".to_string()]),
            account_id: Some(root_account_id.clone()),
            organization_id: Some(personal_organization_id.clone()),
            account_organization_status: Some("Active".to_string()),
            tombstone: Some(system_fields.0),
            status: Some(system_fields.1.clone()),
            created_date: Some(system_fields.2.clone()),
            created_time: Some(system_fields.3.clone()),
            updated_date: Some(system_fields.4.clone()),
            updated_time: Some(system_fields.5.clone()),
            ..Default::default()
        };

        // Add code if counter exists
        if let Some(counter) = &account_organizations_counter {
            root_account_organization.code = Some(generate_root_account_code(counter));
        }

        // Start a transaction
        let result = conn
            .transaction::<_, ApiError, _>(|conn| {
                Box::pin(async move {
                    // Insert personal organization
                    diesel::insert_into(schema::organizations::table)
                        .values(&personal_organization)
                        .execute(conn)
                        .await
                        .map_err(|e| {
                            ApiError::new(
                                StatusCode::INTERNAL_SERVER_ERROR,
                                format!("Failed to insert personal organization: {}", e),
                            )
                        })?;

                    // Insert root account
                    diesel::insert_into(schema::accounts::table)
                        .values(&root_account)
                        .execute(conn)
                        .await
                        .map_err(|e| {
                            ApiError::new(
                                StatusCode::INTERNAL_SERVER_ERROR,
                                format!("Failed to insert root account: {}", e),
                            )
                        })?;

                    // Insert root account profile
                    diesel::insert_into(schema::account_profiles::table)
                        .values(&root_account_profile)
                        .execute(conn)
                        .await
                        .map_err(|e| {
                            ApiError::new(
                                StatusCode::INTERNAL_SERVER_ERROR,
                                format!("Failed to insert root account profile: {}", e),
                            )
                        })?;

                    // Insert root account organization
                    let result = diesel::insert_into(schema::account_organizations::table)
                        .values(&root_account_organization)
                        .returning((
                            schema::account_organizations::id,
                            schema::account_organizations::email,
                            schema::account_organizations::categories,
                            schema::account_organizations::status,
                        ))
                        .get_result::<(
                            Option<String>,
                            Option<String>,
                            Option<Vec<String>>,
                            Option<String>,
                        )>(conn)
                        .await
                        .map_err(|e| {
                            ApiError::new(
                                StatusCode::INTERNAL_SERVER_ERROR,
                                format!("Failed to insert root account organization: {}", e),
                            )
                        })?;

                    log::debug!("Root Account created: {:?}", result);
                    Ok(())
                })
            })
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}

// Create a singleton instance
lazy_static::lazy_static! {
    pub static ref ROOT_ACCOUNT_INITIALIZER: RootAccountInitializer = RootAccountInitializer::new();
}

// Helper function to get the initializer instance
pub fn get_root_account_initializer() -> &'static RootAccountInitializer {
    &ROOT_ACCOUNT_INITIALIZER
}
