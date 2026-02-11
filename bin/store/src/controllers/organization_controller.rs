use crate::generated::models::session_model::SessionModel;
use crate::middlewares::auth_middleware::extract_token;
use crate::middlewares::session_core::session_to_signed_in_activity;
use crate::providers::operations::auth::auth_service::verify;
use crate::providers::operations::organizations::auth_service::{auth, root_auth};
use crate::providers::operations::organizations::organization_service::{
    register, verify_password,
};
use crate::providers::operations::organizations::structs::Register;
use crate::providers::operations::sync::sync_service;
use crate::structs::core::ApiResponse;
use crate::structs::organizations_structs::VerifyPasswordParams;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthDto {
    pub data: AuthData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthData {
    pub account_id: Option<String>,
    pub account_secret: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub expiry_in_ms: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthByTokenDto {
    pub expiry_in_ms: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterDto {
    pub data: Register,
}

pub struct OrganizationsController;

impl OrganizationsController {
    pub async fn register(data: web::Json<RegisterDto>) -> impl Responder {
        if data.0.data.account_id.is_empty() || data.0.data.account_secret.is_empty() {
            return HttpResponse::BadRequest().json(serde_json::json!({"error": "Invalid Input"}));
        }

        let is_request = Some(true);

        match register(&data.0.data, is_request, None).await {
            Ok(result) => HttpResponse::Ok().json(result),
            Err(e) => {
                log::error!("Registration error: {}", e);
                HttpResponse::InternalServerError().json(serde_json::json!({"error": e}))
            }
        }
    }

    pub async fn reregister_existing_account(
        data: web::Json<RegisterDto>,
        id: web::Path<String>,
    ) -> impl Responder {
        if data.0.data.account_id.is_empty() || data.0.data.account_secret.is_empty() {
            return HttpResponse::BadRequest().json(serde_json::json!({"error": "Invalid Input"}));
        }

        let is_request = Some(true);
        let param = id.to_string();

        // Call the organization service register function with the id parameter
        match register(&data.0.data, is_request, Some(param)).await {
            Ok(result) => HttpResponse::Ok().json(result),
            Err(e) => {
                log::error!("Registration error: {}", e);
                HttpResponse::InternalServerError().json(serde_json::json!({"error": e}))
            }
        }
    }

    pub async fn auth(data: web::Json<AuthDto>, req: HttpRequest) -> impl Responder {
        let query_string = req.query_string();

        // Print auth extensions for debugging - access extensions only once

        // Get all extensions in a single borrow to avoid BorrowMutError
        let extensions = req.extensions();

        // Try to get session from extensions
        let session_option = extensions.get::<SessionModel>().cloned();

        // Print session information
        // Create signed_in_activity based on authentication result

        // Drop the extensions borrow before continuing
        drop(extensions);

        let session = match &session_option {
            Some(session) => session,
            None => {
                return HttpResponse::Unauthorized().json(ApiResponse {
                    success: false,
                    message: "Session doesn't exist in the login request".to_string(),
                    count: 0,
                    data: vec![],
                })
            }
        };

        // Extract session ID and handle error if it doesn't exist
        let session_id = match &session.id {
            Some(id) => id.clone(),
            None => {
                return HttpResponse::BadRequest().json(ApiResponse {
                    success: false,
                    message: "Session ID doesn't exist in the organization_controller".to_string(),
                    count: 0,
                    data: vec![],
                })
            }
        };

        let query_params: Vec<(String, String)> = query_string
            .split('&')
            .filter(|s| !s.is_empty())
            .filter_map(|s| {
                let parts: Vec<&str> = s.split('=').collect();
                if parts.len() == 2 {
                    Some((parts[0].to_string(), parts[1].to_string()))
                } else {
                    None
                }
            })
            .collect();

        // Find is_root and t parameters
        let is_root = query_params
            .iter()
            .find(|(k, _)| k == "is_root")
            .map(|(_, v)| v == "true")
            .unwrap_or(false);

        let _t = query_params
            .iter()
            .find(|(k, _)| k == "t")
            .map(|(_, v)| v.clone())
            .unwrap_or_default();

        // Get account_id and account_secret from the request body
        let account_id = data
            .data
            .account_id
            .clone()
            .unwrap_or_else(|| data.data.email.clone().unwrap_or_default());

        let account_secret = data
            .data
            .account_secret
            .clone()
            .unwrap_or_else(|| data.data.password.clone().unwrap_or_default());

        // Authenticate based on is_root parameter
        let result = if is_root {
            // Root authentication
            root_auth(
                &account_id,
                &account_secret,
                session_id.clone(),
                if !_t.is_empty() { Some(&_t) } else { None },
            )
            .await
            .map_err(|err| {
                log::error!("Root authentication error: {}", err);
                (err.message, None::<String>)
            })
        } else {
            // Regular authentication
            auth(
                &account_id,
                &account_secret,
                session_id.clone(),
                "", // Empty organization_id as it's not used in the auth function
            )
            .await
            .map_err(|err| {
                log::error!("Authentication error: {}", err);
                (err.message, None)
            })
        };

        // Extract account_organization_id from the result (available even if auth failed)
        let account_organization_id = match &result {
            Ok(login_response) => login_response.account_organization_id.clone(),
            Err(_) => None,
        };

        // Log the account_organization_id for debugging
        if let Some(ref ao_id) = account_organization_id {
            log::info!("Found account_organization_id: {}", ao_id);
        } else {
            log::info!("No account_organization_id found");
        }

        // Handle the authentication result and update session first
        let updated_session_option = match &result {
            Ok(login_response) => {
                if let Some(token) = &login_response.token {
                    // Successful authentication - update session with token
                    let updated = SessionModel {
                        token: Some(token.clone()),
                        origin_user_agent: req
                            .headers()
                            .get("user-agent")
                            .map(|v| v.to_str().unwrap_or_default().to_string()),
                        origin_host: Some(req.connection_info().host().to_string()),
                        origin_url: Some(req.path().to_string()),
                        user_role_id: Some(login_response.role_id.clone()),
                        user_is_root_user: Some(is_root),
                        user_account_id: Some(account_id),
                        account_organization_id: login_response.account_organization_id.clone(),
                        ..session.clone()
                    };
                    req.extensions_mut().insert(updated.clone());
                    Some(updated)
                } else {
                    // Failed authentication but we have account info - update session with account_organization_id
                    let updated = SessionModel {
                        origin_user_agent: req
                            .headers()
                            .get("user-agent")
                            .map(|v| v.to_str().unwrap_or_default().to_string()),
                        origin_host: Some(req.connection_info().host().to_string()),
                        origin_url: Some(req.path().to_string()),
                        account_organization_id: login_response.account_organization_id.clone(),
                        ..session.clone()
                    };
                    req.extensions_mut().insert(updated.clone());
                    Some(updated)
                }
            }
            Err(_) => session_option.clone(),
        };

        // Now use the updated session for signed_in_activity
        match &updated_session_option {
            Some(session) => {
                let (status, remarks) = match &result {
                    Ok(login_response) => {
                        if login_response.token.is_some() {
                            (Some("Success".to_string()), None)
                        } else {
                            (
                                Some("Failed".to_string()),
                                Some(login_response.message.clone()),
                            )
                        }
                    }
                    Err((message, _)) => (Some("Failed".to_string()), Some(message.clone())),
                };
                let signed_in_activity =
                    session_to_signed_in_activity(&session, status, remarks).await;

                // Convert to JSON and save to database using sync_service
                match serde_json::to_value(&signed_in_activity) {
                    Ok(activity_json) => {
                        if let Err(e) =
                            sync_service::insert(&"signed_in_activities".to_string(), activity_json)
                                .await
                        {
                            log::error!("Failed to save signed_in_activities: {}", e);
                        } else {
                            log::info!(
                                "Successfully saved signed_in_activities for session: {:?}",
                                session_id
                            );
                        }
                    }
                    Err(e) => {
                        log::error!("Failed to serialize signed_in_activity: {}", e);
                    }
                }
            }
            None => {
                log::error!("No session found in extensions");
            }
        }

        // Return the appropriate response
        match result {
            Ok(login_response) => {
                if let Some(token) = login_response.token {
                    // Set cookie and return token with sessionID
                    HttpResponse::Ok()
                        .cookie(
                            actix_web::cookie::Cookie::build("token", token.clone())
                                .path("/")
                                .finish(),
                        )
                        .json(serde_json::json!({
                            "token": token,
                            "sessionID": login_response.session_id.unwrap_or_else(|| session_id.clone()) 
                        }))
                } else {
                    // Authentication failed but no error was thrown
                    HttpResponse::Forbidden().json(serde_json::json!({
                        "message": login_response.message
                    }))
                }
            }
            Err((message, _)) => {
                // Authentication error
                HttpResponse::Forbidden().json(serde_json::json!({
                    "message": message
                }))
            }
        }
    }

    pub async fn logout(_req: HttpRequest, _token_header: Option<String>) -> impl Responder {
        // Empty implementation
        HttpResponse::Ok().finish()
    }

    pub async fn verify_token(
        req: HttpRequest,
        query: web::Query<std::collections::HashMap<String, String>>,
    ) -> impl Responder {
        // Extract token from Authorization header or query parameter
        let auth_header = req
            .headers()
            .get("authorization")
            .and_then(|h| h.to_str().ok());

        let token_from_header = extract_token(auth_header);
        let token_from_query = query.get("t").cloned();

        let token = token_from_query.or(token_from_header);

        match token {
            Some(t) => match verify(&t) {
                Ok(result) => {
                    let success_response = ApiResponse {
                        success: true,
                        message: "Token Verified".to_string(),
                        count: 1,
                        data: vec![serde_json::to_value(result).unwrap_or_default()],
                    };
                    HttpResponse::Ok().json(success_response)
                }
                Err(err) => {
                    let error_response = ApiResponse {
                        success: false,
                        message: format!("Token Verification Failed: {}", err),
                        count: 0,
                        data: vec![],
                    };
                    HttpResponse::BadRequest().json(error_response)
                }
            },
            None => {
                let error_response = ApiResponse {
                    success: false,
                    message: "Token Verification Failed: Missing token".to_string(),
                    count: 0,
                    data: vec![],
                };
                HttpResponse::BadRequest().json(error_response)
            }
        }
    }

    pub async fn verify_password(params: web::Json<VerifyPasswordParams>) -> impl Responder {
        let verify_request = params.0.clone();
        match verify_password(verify_request).await {
            Ok(result) => {
                let success_response = ApiResponse {
                    success: true,
                    message: "Password Verified".to_string(),
                    count: 1,
                    data: vec![serde_json::to_value(result).unwrap_or_default()],
                };
                HttpResponse::Ok().json(success_response)
            }
            Err(e) => {
                let error_response = ApiResponse {
                    success: false,
                    message: format!("Password Verification Failed: {:?}", e),
                    count: 0,
                    data: vec![],
                };
                HttpResponse::BadRequest().json(error_response)
            }
        }
    }

    pub async fn auth_by_token(
        data: web::Json<AuthByTokenDto>,
        req: HttpRequest,
    ) -> impl Responder {
        let query_string = req.query_string();

        // Get all extensions in a single borrow to avoid BorrowMutError
        let extensions = req.extensions();

        // Try to get session from extensions
        let session_option = extensions.get::<SessionModel>().cloned();

        // Drop the extensions borrow before continuing
        drop(extensions);

        let session = match &session_option {
            Some(session) => session,
            None => {
                return HttpResponse::Unauthorized().json(ApiResponse {
                    success: false,
                    message: "Session doesn't exist in the login request".to_string(),
                    count: 0,
                    data: vec![],
                })
            }
        };

        // Extract session ID and handle error if it doesn't exist
        let session_id = match &session.id {
            Some(id) => id.clone(),
            None => {
                return HttpResponse::BadRequest().json(ApiResponse {
                    success: false,
                    message: "Session ID doesn't exist in the organization_controller".to_string(),
                    count: 0,
                    data: vec![],
                })
            }
        };

        // Extract token from Authorization header or query parameter "t"
        let auth_header = req
            .headers()
            .get("authorization")
            .and_then(|h| h.to_str().ok());

        let token_from_header = auth_header.map(|header| {
            // Extract token from "Bearer <token>" format
            header.strip_prefix("Bearer ").unwrap_or(header).to_string()
        });

        // Parse query parameters to get token from "t" parameter
        let token_from_query = query_string
            .split('&')
            .filter(|s| !s.is_empty())
            .find_map(|s| {
                let parts: Vec<&str> = s.split('=').collect();
                if parts.len() == 2 && parts[0] == "t" {
                    Some(parts[1].to_string())
                } else {
                    None
                }
            });

        // Use header token if available, otherwise fall back to query parameter
        let token = match token_from_header.or(token_from_query) {
            Some(token) => token,
            None => {
                return HttpResponse::BadRequest().json(ApiResponse {
                    success: false,
                    message: "Missing authorization header or query parameter 't'".to_string(),
                    count: 0,
                    data: vec![],
                })
            }
        };

        // Parse query parameters for is_root
        let is_root = query_string
            .split('&')
            .filter(|s| !s.is_empty())
            .filter_map(|s| {
                let parts: Vec<&str> = s.split('=').collect();
                if parts.len() == 2 && parts[0] == "is_root" {
                    Some(parts[1] == "true")
                } else {
                    None
                }
            })
            .next()
            .unwrap_or(false);

        // Verify the token
        let verify_result = crate::providers::operations::auth::auth_service::verify(&token);

        match verify_result {
            Ok(claims) => {
                // Extract account information from claims
                let account_id = claims.account.account_id.clone();

                // Get account information using the existing auth service
                let (account, account_organization_id) = match crate::providers::operations::organizations::auth_service::get_account_info(&account_id).await {
                    Ok(info) => info,
                    Err(e) => {
                        log::error!("Failed to get account info: {}", e);
                        return HttpResponse::Forbidden().json(serde_json::json!({
                            "message": "Failed to retrieve account information"
                        }));
                    }
                };

                let account = match account {
                    Some(acc) => acc,
                    None => {
                        return HttpResponse::Forbidden().json(serde_json::json!({
                            "message": "Account not found"
                        }));
                    }
                };

                // Get the signed in account with all related data
                let mut signed_in_account;

                if let Some(ref account_organization_id) = account_organization_id {
                    // Create your filters array based on your requirements
                    let filters = vec!["ao.tombstone = 0", "ao.status = 'Active'"];

                    // Call the function to get the account with profile and organization data
                    signed_in_account =
                        match crate::providers::operations::organizations::auth_service::get_account_with_profile_and_org(&account_organization_id, &filters).await {
                            Ok(account) => account,
                            Err(err) => {
                                log::error!("Error fetching account with profile and org: {}", err);
                                serde_json::json!({})
                            }
                        };
                } else {
                    // Call the function to get the account with profile and organization data by account_id
                    signed_in_account =
                        match crate::providers::operations::organizations::auth_service::get_account_with_profile_and_org_by_account_id(&account_id).await {
                            Ok(account) => account,
                            Err(err) => {
                                log::error!("Error fetching account with profile and org by account_id: {}", err);
                                serde_json::json!({})
                            }
                        };
                }

                // Insert session_id as sessionID in the signed_in_account
                signed_in_account["sessionID"] = serde_json::json!(session_id);

                // Create token value with the signed in account
                let token_value = serde_json::json!({
                    "account": signed_in_account,
                    "sessionID": session_id,
                    "sensitivity_level": account.sensitivity_level,
                    "role_name": "".to_string(),
                    "signed_in_account": signed_in_account
                });

                // Generate new JWT token with custom expiry if provided
                let new_token = if let Some(custom_expiry_ms) = data.expiry_in_ms {
                    match crate::providers::operations::organizations::auth_service::sign_with_expiry(
                        &token_value,
                        custom_expiry_ms,
                    )
                    .await
                    {
                        Ok(t) => t,
                        Err(e) => {
                            log::error!("Failed to sign token with custom expiry: {}", e);
                            return HttpResponse::InternalServerError().json(serde_json::json!({
                                "message": "Failed to generate new token"
                            }));
                        }
                    }
                } else {
                    match crate::providers::operations::organizations::auth_service::sign(
                        &token_value,
                    )
                    .await
                    {
                        Ok(t) => t,
                        Err(e) => {
                            log::error!("Failed to sign token: {}", e);
                            return HttpResponse::InternalServerError().json(serde_json::json!({
                                "message": "Failed to generate new token"
                            }));
                        }
                    }
                };

                // Update session with new token
                let updated = SessionModel {
                    token: Some(new_token.clone()),
                    origin_user_agent: req
                        .headers()
                        .get("user-agent")
                        .map(|v| v.to_str().unwrap_or_default().to_string()),
                    origin_host: Some(req.connection_info().host().to_string()),
                    origin_url: Some(req.path().to_string()),
                    user_role_id: Some(
                        signed_in_account["role_id"]
                            .as_str()
                            .unwrap_or_default()
                            .to_string(),
                    ),
                    user_is_root_user: Some(is_root),
                    user_account_id: Some(account_id.clone()),
                    account_organization_id: account_organization_id.clone(),
                    ..session.clone()
                };
                req.extensions_mut().insert(updated.clone());

                // Log the signed in activity
                let signed_in_activity =
                    crate::middlewares::session_core::session_to_signed_in_activity(
                        &updated,
                        Some("Success".to_string()),
                        None,
                    )
                    .await;

                // Convert to JSON and save to database using sync_service
                match serde_json::to_value(&signed_in_activity) {
                    Ok(activity_json) => {
                        if let Err(e) = crate::providers::operations::sync::sync_service::insert(
                            &"signed_in_activities".to_string(),
                            activity_json,
                        )
                        .await
                        {
                            log::error!("Failed to save signed_in_activities: {}", e);
                        } else {
                            log::info!(
                                "Successfully saved signed_in_activities for session: {:?}",
                                session_id
                            );
                        }
                    }
                    Err(e) => {
                        log::error!("Failed to serialize signed_in_activity: {}", e);
                    }
                }

                // Return the new token and sessionID
                HttpResponse::Ok()
                    .cookie(
                        actix_web::cookie::Cookie::build("token", new_token.clone())
                            .path("/")
                            .finish(),
                    )
                    .json(serde_json::json!({
                        "token": new_token,
                        "sessionID": session_id
                    }))
            }
            Err(e) => {
                log::error!("Token verification failed: {}", e);
                HttpResponse::Forbidden().json(serde_json::json!({
                    "message": "Invalid token"
                }))
            }
        }
    }

    pub async fn refresh_token(req: HttpRequest) -> impl Responder {
        // Extract token from Authorization header
        let auth_header = req
            .headers()
            .get("authorization")
            .and_then(|h| h.to_str().ok());

        let token = match auth_header {
            Some(header) => {
                // Extract token from "Bearer <token>" format
                header.strip_prefix("Bearer ").unwrap_or(header).to_string()
            }
            None => {
                return HttpResponse::BadRequest().json(serde_json::json!({
                    "message": "Missing authorization header"
                }));
            }
        };

        // Verify the existing token
        let verify_result = crate::providers::operations::auth::auth_service::verify(&token);

        match verify_result {
            Ok(claims) => {
                // Extract account information from claims
                let account_id = claims.account.account_id.clone();

                // Get account information
                let (account, account_organization_id) = match crate::providers::operations::organizations::auth_service::get_account_info(&account_id).await {
                    Ok(info) => info,
                    Err(e) => {
                        log::error!("Failed to get account info: {}", e);
                        return HttpResponse::Forbidden().json(serde_json::json!({
                            "message": "Failed to retrieve account information"
                        }));
                    }
                };

                let account = match account {
                    Some(acc) => acc,
                    None => {
                        return HttpResponse::Forbidden().json(serde_json::json!({
                            "message": "Account not found"
                        }));
                    }
                };

                // Get the signed in account with all related data
                let signed_in_account = if let Some(ref account_organization_id) = account_organization_id {
                    // Create filters array
                    let filters = vec!["ao.tombstone = 0", "ao.status = 'Active'"];

                    // Call the function to get the account with profile and organization data
                    match crate::providers::operations::organizations::auth_service::get_account_with_profile_and_org(&account_organization_id, &filters).await {
                        Ok(account) => account,
                        Err(err) => {
                            log::error!("Error fetching account with profile and org: {}", err);
                            serde_json::json!({})
                        }
                    }
                } else {
                    // Call the function to get the account with profile and organization data by account_id
                    match crate::providers::operations::organizations::auth_service::get_account_with_profile_and_org_by_account_id(&account_id).await {
                        Ok(account) => account,
                        Err(err) => {
                            log::error!("Error fetching account with profile and org by account_id: {}", err);
                            serde_json::json!({})
                        }
                    }
                };

                // Create token value with the signed in account
                let token_value = serde_json::json!({
                    "account": signed_in_account,
                    "sensitivity_level": account.sensitivity_level,
                    "role_name": "",
                    "signed_in_account": signed_in_account
                });

                // Generate new JWT token
                let new_token = match crate::providers::operations::organizations::auth_service::sign(&token_value).await {
                    Ok(t) => t,
                    Err(e) => {
                        log::error!("Failed to sign token: {}", e);
                        return HttpResponse::InternalServerError().json(serde_json::json!({
                            "message": "Failed to generate new token"
                        }));
                    }
                };

                // Return the new token
                HttpResponse::Ok()
                    .cookie(
                        actix_web::cookie::Cookie::build("token", new_token.clone())
                            .path("/")
                            .finish(),
                    )
                    .json(serde_json::json!({
                        "token": new_token,
                        "message": "Token refreshed successfully"
                    }))
            }
            Err(e) => {
                log::error!("Token verification failed: {}", e);
                HttpResponse::Forbidden().json(serde_json::json!({
                    "message": "Invalid token"
                }))
            }
        }
    }
}
