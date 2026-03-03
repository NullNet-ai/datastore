//! Store API client: login once, then create records with Authorization: Bearer <token>.

use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde_json::Value;

use crate::config::Config;

#[derive(Clone)]
pub struct StoreClient {
    client: reqwest::Client,
    base_url: String,
    account_id: String,
    account_secret: String,
    app_id: String,
    root_account_id: Option<String>,
    root_account_secret: Option<String>,
}

impl StoreClient {
    pub fn new(config: &Config) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let client = reqwest::Client::builder().build()?;
        Ok(Self {
            client,
            base_url: config.migrate_to_store_url.clone(),
            account_id: config.store_account_id.clone(),
            account_secret: config.store_account_secret.clone(),
            app_id: config.app_id.clone(),
            root_account_id: config.store_root_account_id.clone(),
            root_account_secret: config.store_root_account_secret.clone(),
        })
    }

    /// Login with the normal account and return the Bearer token.
    /// Use this token for create requests.
    pub async fn login(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!(
            "{}/api/organizations/auth?app_id={}",
            self.base_url, self.app_id
        );
        let body = serde_json::json!({
            "data": {
                "account_id": self.account_id,
                "account_secret": self.account_secret
            }
        });

        let res = self
            .client
            .post(&url)
            .header(CONTENT_TYPE, "application/json")
            .json(&body)
            .send()
            .await?;

        let status = res.status();
        let text = res.text().await?;
        if !status.is_success() {
            return Err(format!("login failed {}: {}", status, text).into());
        }

        let json: Value = serde_json::from_str(&text)
            .map_err(|e| format!("login response not json: {} body: {}", e, text))?;
        let token = json
            .get("token")
            .and_then(|t| t.as_str())
            .ok_or_else(|| format!("login response missing token: {}", text))?;
        Ok(token.to_string())
    }

    /// Login with the root account (if configured) and return a Bearer token.
    /// Falls back to normal login() if no separate root credentials are provided.
    pub async fn login_root(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        if let (Some(root_id), Some(root_secret)) = (
            self.root_account_id.as_ref(),
            self.root_account_secret.as_ref(),
        ) {
            let url = format!("{}/api/organizations/auth?is_root=true", self.base_url);
            let body = serde_json::json!({
                "data": {
                    "account_id": root_id,
                    "account_secret": root_secret
                }
            });

            let res = self
                .client
                .post(&url)
                .header(CONTENT_TYPE, "application/json")
                .json(&body)
                .send()
                .await?;

            let status = res.status();
            let text = res.text().await?;
            if !status.is_success() {
                return Err(format!("root login failed {}: {}", status, text).into());
            }

            let json: Value = serde_json::from_str(&text)
                .map_err(|e| format!("root login response not json: {} body: {}", e, text))?;
            let token = json
                .get("token")
                .and_then(|t| t.as_str())
                .ok_or_else(|| format!("root login response missing token: {}", text))?;
            Ok(token.to_string())
        } else {
            // No explicit root creds; reuse normal login. Patch phase will get 403 if root is required.
            eprintln!(
                "Warning: MIGRATE_STORE_ROOT_ACCOUNT_ID / MIGRATE_STORE_ROOT_ACCOUNT_SECRET not set; \
                 using normal account for root login — patch phase may fail with 403 Forbidden."
            );
            self.login().await
        }
    }

    /// POST one record to the store create route. Uses Bearer token so the session is not re-created.
    pub async fn create_record(
        &self,
        table: &str,
        record: &Value,
        token: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("{}/api/store/{}?pluck=id,code", self.base_url, table);

        let auth_value = format!("Bearer {}", token);
        let res = self
            .client
            .post(&url)
            .header(AUTHORIZATION, auth_value)
            .header(CONTENT_TYPE, "application/json")
            .json(record)
            .send()
            .await?;

        let status = res.status();
        if !status.is_success() {
            let text = res.text().await.unwrap_or_default();
            return Err(format!("create failed {}: {}", status, text).into());
        }
        Ok(())
    }

    /// PATCH one record by id (e.g. to restore circular FK columns after first pass).
    pub async fn patch_record(
        &self,
        table: &str,
        record_id: &str,
        patch_body: &Value,
        token: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("{}/api/store/root/{}/{}", self.base_url, table, record_id);

        let auth_value = format!("Bearer {}", token);
        let res = self
            .client
            .patch(&url)
            .header(AUTHORIZATION, auth_value)
            .header(CONTENT_TYPE, "application/json")
            .json(patch_body)
            .send()
            .await?;

        let status = res.status();
        if !status.is_success() {
            let text = res.text().await.unwrap_or_default();
            return Err(format!("patch failed {}: {}", status, text).into());
        }
        Ok(())
    }
}
