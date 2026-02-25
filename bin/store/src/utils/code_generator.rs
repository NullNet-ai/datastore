//! Code generation via counter-service (gRPC + Redis). Requires CODE_SERVICE_GRPC_URL.

use diesel::result::Error as DieselError;
use std::env;
use std::io;
use tonic::transport::Channel;
use tonic::transport::Endpoint;

static CODE_SERVICE_CLIENT: tokio::sync::OnceCell<counter_service::CodeServiceClient<Channel>> =
    tokio::sync::OnceCell::const_new();

fn code_service_err(msg: impl Into<String>) -> DieselError {
    DieselError::QueryBuilderError(Box::new(io::Error::new(
        io::ErrorKind::Other,
        msg.into(),
    )))
}

fn code_service_url() -> Option<String> {
    env::var("CODE_SERVICE_GRPC_URL").ok().filter(|s| !s.is_empty())
}

/// Database name from DATABASE_URL (e.g. "connectivo" from postgres://admin:admin@localhost:5433/connectivo).
/// Used as the "database" (project) key when calling the counter service. Falls back to "default" if unset or unparseable.
pub fn database_name_from_env() -> String {
    let url = match env::var("DATABASE_URL").ok().filter(|s| !s.is_empty()) {
        Some(u) => u,
        None => return "default".to_string(),
    };
    let path = url.split('?').next().unwrap_or(&url);
    path.rsplit('/')
        .next()
        .filter(|s| !s.is_empty())
        .unwrap_or("default")
        .to_string()
}

async fn get_client() -> Result<&'static counter_service::CodeServiceClient<Channel>, DieselError> {
    if let Some(existing) = CODE_SERVICE_CLIENT.get() {
        return Ok(existing);
    }
    let url = code_service_url()
        .ok_or_else(|| code_service_err("CODE_SERVICE_GRPC_URL not set"))?;
    // gRPC requires a scheme (e.g. http:// or https://)
    let url_with_scheme = if url.contains("://") {
        url
    } else {
        format!("http://{}", url)
    };
    let endpoint = Endpoint::from_shared(url_with_scheme)
        .map_err(|e| code_service_err(format!("CODE_SERVICE_GRPC_URL invalid: {}", e)))?;
    let client = counter_service::CodeServiceClient::connect(endpoint)
        .await
        .map_err(|e| {
            log::error!("Code service connect error: {}", e);
            code_service_err(e.to_string())
        })?;
    let _ = CODE_SERVICE_CLIENT.set(client);
    CODE_SERVICE_CLIENT
        .get()
        .ok_or_else(|| code_service_err("Code service client not initialized"))
}

/// Generate next unique code for the table via counter-service.
pub async fn generate_code(
    table: &str,
    _prefix_param: &str,
    _default_code_param: i32,
) -> Result<String, DieselError> {
    let client = get_client().await?;
    let req = counter_service::GetCodeRequest {
        database: database_name_from_env(),
        table: table.to_string(),
    };
    let res = client.clone().get_code(req).await.map_err(|e| {
        log::error!("Code service get_code error: {}", e);
        code_service_err(e.to_string())
    })?;
    Ok(res.into_inner().code)
}

/// Generate next unique code for the entity via counter-service.
pub async fn generate_code_optional(entity: &str) -> Result<Option<String>, DieselError> {
    let client = get_client().await?;
    let req = counter_service::GetCodeRequest {
        database: database_name_from_env(),
        table: entity.to_string(),
    };
    let res = client.clone().get_code(req).await.map_err(|e| {
        log::error!("Code service get_code error: {}", e);
        code_service_err(e.to_string())
    })?;
    Ok(Some(res.into_inner().code))
}

/// Initialize counters in the code service.
pub async fn init_counters(
    database: &str,
    counters: &[(String, String, i32, i32)], // (entity, prefix, default_code, digits_number)
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if counters.is_empty() {
        return Ok(());
    }
    let client = get_client().await.map_err(|e| e.to_string())?;
    let configs: Vec<counter_service::CounterConfig> = counters
        .iter()
        .map(|(entity, prefix, default_code, digits_number)| counter_service::CounterConfig {
            entity: entity.clone(),
            prefix: prefix.clone(),
            default_code: *default_code,
            digits_number: *digits_number,
        })
        .collect();
    let req = counter_service::InitCountersRequest {
        database: database.to_string(),
        counters: configs,
    };
    let res = client.clone().init_counters(req).await.map_err(|e| e.to_string())?;
    let inner = res.into_inner();
    if !inner.success {
        return Err(inner.message.into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn init_counters_empty_list_returns_ok() {
        let r = init_counters("default", &[]).await;
        assert!(r.is_ok());
    }
}
