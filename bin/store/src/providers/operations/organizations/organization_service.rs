use crate::config::core::EnvConfig;
use crate::controllers::store_controller::ApiError;
use crate::database::db;
use crate::generated::models::account_model::AccountModel;
use crate::generated::models::account_organization_model::AccountOrganizationModel;
use crate::generated::models::account_profile_model::AccountProfileModel;
use crate::generated::models::contact_email_model::ContactEmailModel;
use crate::generated::models::contact_model::ContactModel;
use crate::generated::models::counter_model::CounterModel;
use crate::generated::models::device_model::DeviceModel;
use crate::generated::models::organization_account_model::OrganizationAccountModel;
use crate::generated::models::organization_model::OrganizationModel;
use crate::generated::schema::account_organizations;
use crate::generated::schema::accounts;
use crate::generated::schema::counters;
use crate::generated::schema::organizations;
use crate::providers::operations::auth::auth_service;
use crate::providers::operations::organizations::structs::AccountType;
use crate::providers::operations::organizations::structs::Register;
use crate::providers::operations::sync::sync_service;
use crate::structs::organizations_structs::VerifyPasswordParams;
use crate::utils::helpers;
use actix_web::http::StatusCode;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel_async::RunQueryDsl;
use serde_json::json;
use serde_json::Value;
use ulid::Ulid;

pub fn get_defaults() -> (
    String,
    String,
    String,
    String,
    String,
    String,
    bool,
    String,
    String,
) {
    let config = EnvConfig::default();
    let default_organization_id = config.default_organization_id;
    let default_organization_name = config.default_organization_name;
    let default_organization_admin_email = config.default_organization_admin_email;
    let default_organization_admin_password = config.default_organization_admin_password;
    let default_device_id = config.default_device_id;
    let default_device_secret = config.default_device_secret;
    let debug = config.debug;
    let super_admin_id = config.super_admin_id;
    let system_device_ulid = config.system_device_ulid;
    (
        default_organization_id,
        default_organization_name,
        default_organization_admin_email,
        default_organization_admin_password,
        default_device_id,
        default_device_secret,
        debug,
        super_admin_id,
        system_device_ulid,
    )
}
#[allow(warnings)]
pub async fn register(
    params: &Register,
    is_request: Option<bool>,
    _account_organization_id: Option<String>,
) -> Result<Value, ApiError> {
    // Changed return type to use diesel::result::Error
    let mut conn = db::get_async_connection().await;

    let is_request = is_request.unwrap_or(false);
    let (_, _, _, _, _, _, _, super_admin_id, system_device_ulid) = get_defaults();

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

    let _personal_organization_id: Option<String> = None;

    let now = chrono::Utc::now();
    let formatted_date = now.format("%Y-%m-%d").to_string(); // Format date
    let formatted_time = now.format("%H:%M:%S").to_string(); // Format time in 24-hour format

    let mut _account_id = if is_request {
        Ulid::new().to_string()
    } else {
        if account_type == AccountType::Contact {
            super_admin_id.clone()
        } else {
            system_device_ulid.clone()
        }
    };
    let is_contact_account = account_type == AccountType::Contact;
    let mut team_organization_id: Option<String> = None;

    let query_result = accounts::table
        .filter(
            accounts::account_id
                .eq(account_id)
                .and(accounts::tombstone.eq(0)),
        )
        .first::<AccountModel>(&mut conn)
        .await
        .optional();

    let existing_account = query_result?;

    // Query for organizations counter
    let organizations_counter = counters::table
        .filter(counters::entity.eq("organizations"))
        .first::<CounterModel>(&mut conn)
        .await
        .optional()?;

    let _contacts_counter = counters::table
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

    if let Some(existing_account) = existing_account {
        let account_id_value = existing_account.id.ok_or_else(|| {
            ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Account exists but has no ID.",
            )
        })?;
        _account_id = account_id_value.clone();

        // Check if organization is already existing
        let organization_id = params.organization_id.clone().unwrap_or_default();
        let existing_team_org = organizations::table
            .filter(
                organizations::id
                    .eq(&organization_id)
                    .and(organizations::tombstone.eq(0)),
            )
            .first::<OrganizationModel>(&mut conn)
            .await
            .optional()?;

        if let Some(existing_team_org) = existing_team_org {
            // Check if assigned to an account_organization
            let existing_account_org = account_organizations::table
                .filter(
                    account_organizations::organization_id
                        .eq(&existing_team_org.id.clone().unwrap_or_default())
                        .and(account_organizations::account_id.eq(_account_id.clone())),
                )
                .first::<AccountOrganizationModel>(&mut conn)
                .await
                .optional()?;

            if let Some(existing_account_org) = existing_account_org {
                // if yes, do not create anything
                log::warn!(
                    "Account {} already exists in organization {}, assigned to account_organization {}",
                    account_id,
                    existing_team_org.id.clone().unwrap_or_default(),
                    existing_account_org.id.clone().unwrap_or_default()
                );

                return Ok(json!({
                    "organization_id": team_organization_id,
                    "account_organization_id": existing_account_org.id,
                    "account_id": _account_id,
                    "email": account_id,
                    "contact_id": existing_account_org.contact_id,
                    "device_id": existing_account_org.device_id,
                }));
            } else {
                // assign to account_organization
                team_organization_id = Some(existing_team_org.id.clone().unwrap_or_default());
            }
        }
    } else {
        //create a personal organization
        let personal_organization_id = create_new_organization(
            "Personal Organization".to_string(),
            vec!["Personal".to_string()],
            None,
            if organizations_counter.is_some() {
                match helpers::generate_code("organizations").await {
                    Ok(code) => code,
                    Err(_) => None,
                }
            } else {
                None
            },
            responsible_account_organization_id.clone(),
        )
        .await?;

        //now create an account

        create_account(
            _account_id.clone(),
            account_id.to_string(),
            account_secret.to_string(),
            personal_organization_id,
            is_new_user,
            formatted_date.clone(),
            formatted_time.clone(),
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
            Some(true),
        )
        .await?;
    }

    team_organization_id = Some(
        create_new_organization(
            team_organization_name
                .clone()
                .unwrap_or(&String::new())
                .to_string(),
            vec!["Team".to_string()],
            params.organization_id.clone(),
            if organizations_counter.is_some() {
                helpers::generate_code("organizations").await?
            } else {
                None
            },
            responsible_account_organization_id.clone(),
        )
        .await?,
    );

    if is_contact_account && (!is_invited || !is_request) {
        match async {
                // Never use empty string for contact id (SUPER_ADMIN_ID may be unset in .env)
                let user_id = if is_request {
                    Ulid::new().to_string()
                } else if super_admin_id.trim().is_empty() {
                    Ulid::new().to_string()
                } else {
                    super_admin_id.clone()
                };

                let team_organization_id = team_organization_id.ok_or_else(|| {
                    ApiError::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Team organization ID is required but not available.",
                    )
                })?;
                // Create contact using ContactModel
                let contact = ContactModel {
                    id: Some(user_id.clone()),
                    organization_id: Some(team_organization_id.clone()),
                    first_name: Some(first_name.clone()),
                    last_name: Some(last_name.clone()),
                    categories: Some(contact_categories.clone().unwrap_or_else(|| vec!["Contact".to_string()])),
                    account_id: Some(_account_id.clone()),
                    code: if organizations_counter.is_some() {
                        helpers::generate_code("organizations").await?
                    } else {
                        None
                    },
                    tombstone: Some(0),
                    status: Some("Active".to_string()),
                    created_date: Some(formatted_date.clone()),
                    created_time: Some(formatted_time.clone()),
                    updated_date: Some(formatted_date.clone()),
                    updated_time: Some(formatted_time.clone()),
                    created_by: responsible_account_organization_id.clone(),
                    ..Default::default()
                };

                // Convert model to JSON and insert into database
                let contact_json = serde_json::to_value(&contact)
                    .map_err(|e| ApiError::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Failed to serialize contact: {}", e)
                    ))?;

                sync_service::insert(&"contacts".to_string(), contact_json).await?;

                // Create Contact Email using ContactEmailModel
                let contact_email = ContactEmailModel {
                    id: Some(Ulid::new().to_string()),
                    contact_id: Some(user_id.clone()),
                    email: Some(account_id.clone()),
                    is_primary: Some(true),
                    organization_id: Some(team_organization_id.clone()),
                    tombstone: Some(0),
                    status: Some("Active".to_string()),
                    created_date: Some(formatted_date.clone()),
                    created_time: Some(formatted_time.clone()),
                    updated_date: Some(formatted_date.clone()),
                    updated_time: Some(formatted_time.clone()),
                    created_by: responsible_account_organization_id.clone(),
                    ..Default::default()
                };

                // Convert model to JSON and insert into database
                let contact_email_json = serde_json::to_value(&contact_email)
                    .map_err(|e| ApiError::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Failed to serialize contact email: {}", e)
                    ))?;

                sync_service::insert(&"contact_emails".to_string(), contact_email_json).await?;

                // Create Account Organization using AccountOrganizationModel
                let account_organization = AccountOrganizationModel {
                    id: Some(Ulid::new().to_string()),
                    email: Some(account_id.clone()),
                    categories: Some(account_organization_categories.clone().unwrap_or_else(|| vec!["Internal User".to_string()])),
                    account_id: Some(_account_id.clone()),
                    organization_id: Some(team_organization_id.clone()),
                    contact_id: Some(user_id.clone()),
                    account_organization_status: Some(account_organization_status.clone().unwrap_or_else(|| "Active".to_string())),
                    role_id: Some(role_id.clone()),
                    is_invited: Some(is_invited),
                    code: if account_organizations_counter.is_some() {
                        helpers::generate_code("account_organizations").await?
                    } else {
                        None
                    },
                    tombstone: Some(0),
                    status: Some("Active".to_string()),
                    created_date: Some(formatted_date.clone()),
                    created_time: Some(formatted_time.clone()),
                    updated_date: Some(formatted_date.clone()),
                    updated_time: Some(formatted_time.clone()),
                    created_by: responsible_account_organization_id.clone(),
                    ..Default::default()
                };

                // Convert model to JSON and insert into database
                let account_organization_json = serde_json::to_value(&account_organization)
                    .map_err(|e| ApiError::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Failed to serialize account organization: {}", e)
                    ))?;

                sync_service::insert(&"account_organizations".to_string(), account_organization_json.clone()).await?;

                log::info!(
                    "Signed up Account ({}) {} with email: {} successfully linked to Team Organization {}",
                    account_type,
                    _account_id,
                    account_id,
                    team_organization_id
                );

                Ok::<_, ApiError>(json!({
                    "organization_id": team_organization_id,
                    "account_organization_id": account_organization.id.unwrap_or_default(),
                    "account_id": _account_id,
                    "email": account_id,
                    "contact_id": user_id,
                }))
            }.await {
                Ok(result) => Ok(result),
                Err(error) => {
                    log::error!(
                        "Failed to sign up an account ({}) {} to team organization {}: {}",
                        account_type,
                        _account_id,
                        params.organization_id.clone().unwrap_or_else(|| "unknown".to_string()),
                        error
                    );
                    Ok(().into())
                }
            }
    } else {
        // Handle invited contact accounts
        match async
        {
            if is_invited && account_organization_id.is_empty() {
                return Err(ApiError::new(
                    StatusCode::BAD_REQUEST,
                    "Account Organization ID is required if Account is invited".to_string(),
                ));
            }

            let mut device_id: Option<String> = Some(_account_id.clone());
            let mut device_code: Option<String> = None;

            // Create Device if not a contact account
            if !is_contact_account {
                let devices_counter = counters::table
                    .filter(counters::entity.eq("devices"))
                    .first::<CounterModel>(&mut conn)
                    .await
                    .optional()?;
                    // Use new ULID if it's a request, otherwise use system_device_ulid
                    let device_id_value = device_id.ok_or_else(|| {
                        ApiError::new(
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "Device ID is required but not available.",
                        )
                    })?;
                let code = if devices_counter.is_some() {
                    helpers::generate_code("devices").await?
                } else {
                    None
                };

                // Create device using DeviceModel
                let device = DeviceModel {
                    id: Some(device_id_value.clone()),
                    organization_id: params.organization_id.clone(),
                    categories: device_categories.clone(),
                    code: code.clone(),
                    tombstone: Some(0),
                    status: Some(if is_request { "Draft" } else { "Active" }.to_string()),
                    created_date: Some(formatted_date.clone()),
                    created_time: Some(formatted_time.clone()),
                    updated_date: Some(formatted_date.clone()),
                    updated_time: Some(formatted_time.clone()),
                    created_by: responsible_account_organization_id.clone(),
                    ..Default::default()
                };

                // Convert model to JSON and insert into database
                let device_json = serde_json::to_value(&device)
                    .map_err(|e| ApiError::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Failed to serialize device: {}", e)
                    ))?;

                sync_service::insert(&"devices".to_string(), device_json).await?;
                device_id = Some(device_id_value);
                device_code = code;
            }

            // Link the account and organization on account_organizations
            let account_org_id = if account_organization_id.is_empty() {
                Ulid::new().to_string()
            } else {
                account_organization_id.clone()
            };

            // Create AccountOrganizationModel
            let account_organization = AccountOrganizationModel {
                id: Some(account_org_id.clone()),
                email: Some(account_id.clone()),
                account_id: Some(_account_id.clone()),
                organization_id: team_organization_id.clone(),
                categories: account_organization_categories.clone(),
                is_invited: Some(is_invited),
                tombstone: Some(0),
                created_date: Some(formatted_date.clone()),
                created_time: Some(formatted_time.clone()),
                updated_date: Some(formatted_date.clone()),
                updated_time: Some(formatted_time.clone()),
                created_by: responsible_account_organization_id.clone(),
                device_id: if !is_contact_account { device_id.clone() } else { None },
                // Conditional fields for non-contact accounts
                account_organization_status: if !is_contact_account && is_request {
                    Some("Inactive".to_string())
                } else {
                    account_organization_status.clone()
                },
                status: if !is_contact_account && is_request {
                    Some("Draft".to_string())
                } else {
                    Some("Active".to_string())
                },
                ..Default::default()
            };

            // Convert model to JSON
            let account_organization_json = serde_json::to_value(&account_organization)
                .map_err(|e| ApiError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to serialize account organization: {}", e)
                ))?;

            // Insert or update account organization
            let _created_account_organization = if account_organization_id.is_empty() {
                sync_service::insert(&"account_organizations".to_string(), account_organization_json).await?
            } else {
                sync_service::update(&"account_organizations".to_string(), account_organization_json, &account_organization_id).await?
            };

            log::info!(
                "Invited Account ({}) {} with email: {} successfully linked to Team Organization {}",
                account_type,
                _account_id,
                account_id,
                params.organization_id.clone().unwrap_or_else(|| "unknown".to_string())
            );

            // Create response JSON
            let mut result = json!({
                "organization_id": params.organization_id.clone().unwrap_or_default(),
                "account_organization_id": account_org_id,
                "account_id": _account_id,
                "email": account_id
            });

            // Add device information if available
            if let Some(dev_id) = device_id {
                if let Some(obj) = result.as_object() {
                    let mut obj = obj.clone();
                    obj.insert("device_id".to_string(), Value::String(dev_id));

                    if let Some(code) = device_code {
                        obj.insert("device_code".to_string(), Value::String(code));
                    }

                    result = Value::Object(obj);
                } else {
                    return Err(ApiError::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Expected JSON object but got different type when adding device information",
                    ));
                }
            }

            Ok(result)
            }.await {
                Ok(result) => Ok(result),
                Err(error) => {
                    log::error!(
                        "Failed to sign up an account ({}) {} to team organization {}: {}",
                        account_type,
                        _account_id,
                        params.organization_id.clone().unwrap_or_else(|| "unknown".to_string()),
                        error
                    );
                    // Return empty JSON value on error
                    Ok(().into())
                }
            }
    }
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
    let organization = OrganizationModel {
        id: Some(id.clone()),
        name: Some(organization_name.clone()),
        categories: Some(categories.clone()),
        organization_id: Some(id.clone()),
        parent_organization_id: None,
        tombstone: Some(0),
        status: Some("Active".to_string()),
        created_date: Some(formatted_date.clone()),
        created_time: Some(formatted_time.clone()),
        updated_date: Some(formatted_date),
        updated_time: Some(formatted_time),
        code: code, // This will be None if code is None
        created_by: responsible_account_organization_id, // This will be None if responsible_id is None
        // Set default values for other required fields
        ..Default::default()
    };

    // Convert the model to a JSON Value for the sync_service
    let organization_value = serde_json::to_value(organization).map_err(|e| {
        DieselError::DatabaseError(
            diesel::result::DatabaseErrorKind::SerializationFailure,
            Box::new(format!("Failed to serialize organization: {}", e)),
        )
    })?;

    sync_service::insert(&"organizations".to_string(), organization_value).await?;

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
    mut create_profile: Option<bool>,
) -> Result<(), ApiError> {
    // Hash the password
    if !create_profile.is_some() {
        create_profile = Some(true);
    }
    let hashed_password = match auth_service::password_hash(&account_secret).await {
        Ok(hash) => hash,
        Err(e) => {
            log::error!("Failed to hash password: {}", e);
            return Err(ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to hash password: {}", e),
            )
            .into());
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
    let account_json = serde_json::to_value(&account).map_err(|e| {
        ApiError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to serialize account: {}", e),
        )
    })?;

    // Insert account into database
    sync_service::insert(&"accounts".to_string(), account_json).await?;

    // Create account profile using AccountProfileModel

    if create_profile.unwrap_or(true) {
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
        let profile_json = serde_json::to_value(&account_profile).map_err(|e| {
            ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to serialize account profile: {}", e),
            )
        })?;

        // Insert account profile into database
        sync_service::insert(&"account_profiles".to_string(), profile_json).await?;
    }

    log::info!("Created Account: {}, email: {}", id, account_id);

    Ok(())
}

pub async fn initialize(data: Option<Register>) -> Result<(), ApiError> {
    let (
        default_organization_id,
        default_organization_name,
        default_organization_admin_email,
        default_organization_admin_password,
        _,
        _,
        _,
        _,
        _,
    ) = get_defaults();

    // Create default register data if not provided
    let data = data.unwrap_or_else(|| Register {
        organization_id: Some(default_organization_id.clone()),
        organization_name: Some(default_organization_name.clone()),
        account_id: default_organization_admin_email.clone(),
        account_secret: default_organization_admin_password.clone(),
        first_name: "Super".to_string(),
        last_name: "Admin".to_string(),
        account_type: Some(AccountType::Contact),
        // Initialize other fields with None/default values
        id: None,
        name: None,
        contact_id: None,
        email: None,
        password: None,
        parent_organization_id: None,
        code: None,
        categories: None,
        account_status: None,
        is_new_user: None,
        is_invited: None,
        role_id: None,
        account_organization_status: None,
        account_organization_categories: None,
        account_organization_id: None,
        contact_categories: None,
        device_categories: None,
        responsible_account_organization_id: None,
    });

    // If default organization ID is not set, return early
    if default_organization_id.is_empty() {
        log::error!("Default organization ID is not set. Please configure DEFAULT_ORGANIZATION_ID environment variable.");
        return Ok(());
    }
    // Register the organization and admin account
    register(&data, None, None).await?;

    Ok(())
}
#[allow(warnings)]
pub async fn initialize_device() -> Result<(), ApiError> {
    let (
        default_organization_id,
        _,
        _,
        _,
        default_device_id,
        default_device_secret,
        _,
        _,
        system_device_ulid,
    ) = get_defaults();

    let now = chrono::Utc::now();
    let formatted_date = now.format("%Y-%m-%d").to_string();
    let formatted_time = now.format("%H:%M:%S").to_string();

    // Create device using DeviceModel
    let device = DeviceModel {
        id: Some(system_device_ulid.clone()),
        categories: Some(vec!["Device".to_string()]),
        created_date: Some(formatted_date.clone()),
        created_time: Some(formatted_time.clone()),
        updated_date: Some(formatted_date.clone()),
        updated_time: Some(formatted_time.clone()),
        organization_id: Some(default_organization_id.clone()),
        tombstone: Some(0),
        status: Some("Active".to_string()),
        ..Default::default()
    };

    // Convert model to JSON and insert into database
    log::info!("Creating Device: {}", system_device_ulid);
    let device_json = serde_json::to_value(&device).map_err(|e| {
        ApiError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to serialize device: {}", e),
        )
    })?;

    sync_service::insert(&"devices".to_string(), device_json).await?;
    log::info!("Created Device: {}", system_device_ulid);

    // Create organization account for the device
    let hashed_password = auth_service::password_hash(&default_device_secret)
        .await
        .map_err(|e| {
            ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to hash password: {}", e),
            )
        })?;

    let organization_account = OrganizationAccountModel {
        id: Some(system_device_ulid.clone()),
        organization_id: Some(default_organization_id.clone()),
        device_id: Some(system_device_ulid.clone()),
        account_id: Some(default_device_id.clone()),
        account_secret: Some(hashed_password),
        tombstone: Some(0),
        categories: Some(vec!["Device".to_string()]),
        status: Some("Active".to_string()),
        created_date: Some(formatted_date.clone()),
        created_time: Some(formatted_time.clone()),
        updated_date: Some(formatted_date.clone()),
        updated_time: Some(formatted_time.clone()),
        ..Default::default()
    };

    log::info!(
        "Creating Organization Account for device: {}",
        system_device_ulid
    );
    let organization_account_json = serde_json::to_value(&organization_account).map_err(|e| {
        ApiError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to serialize organization account: {}", e),
        )
    })?;

    sync_service::insert(
        &"organization_accounts".to_string(),
        organization_account_json,
    )
    .await?;

    Ok(())
}

pub async fn verify_password(
    params: VerifyPasswordParams,
) -> Result<VerifyPasswordParams, ApiError> {
    let mut conn = db::get_async_connection().await;
    let account_id = params.account_id;
    let password = params.password;
    let account = accounts::table
        .filter(accounts::id.eq(account_id).and(accounts::tombstone.eq(0)))
        .first::<AccountModel>(&mut conn)
        .await
        .optional()
        .map_err(|e| {
            ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
        })?;

    match account {
        Some(account_model) => {
            let account_secret = account_model.account_secret.as_deref().unwrap_or_default();
            let is_valid = auth_service::password_verify(account_secret, &password).await?;
            if is_valid {
                Ok(VerifyPasswordParams {
                    account_id: account_model.id.unwrap_or_default(),
                    password,
                })
            } else {
                Err(ApiError::new(
                    StatusCode::UNAUTHORIZED,
                    "Invalid password".to_string(),
                ))
            }
        }
        None => Err(ApiError::new(
            StatusCode::NOT_FOUND,
            "Account not found".to_string(),
        )),
    }
}
