use crate::db::{self, AsyncDbPooledConnection};
use tokio::sync::Semaphore;
use std::sync::Arc;

/// Connection limiter for flush operations to prevent thundering herd problems
pub struct FlushConnectionLimiter {
    semaphore: Arc<Semaphore>,
    #[allow(dead_code)]
    max_concurrent: usize,
}

impl FlushConnectionLimiter {
    pub fn new(max_concurrent: usize) -> Arc<Self> {
        Arc::new(Self {
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
            max_concurrent,
        })
    }
    
    pub async fn acquire_flush_connection(&self) -> Result<(FlushPermit, AsyncDbPooledConnection), Box<dyn std::error::Error + Send + Sync>> {
        let permit = self.semaphore.clone().acquire_owned().await
            .map_err(|e| format!("Failed to acquire flush permit: {}", e))?;
        
        let conn = db::get_async_connection().await;
        
        Ok((FlushPermit { _permit: permit }, conn))
    }
    

    #[allow(dead_code)]
    pub fn is_at_capacity(&self) -> bool {
        self.semaphore.available_permits() == 0
    }
}

/// RAII guard for flush permit
pub struct FlushPermit {
    _permit: tokio::sync::OwnedSemaphorePermit,
}

impl Drop for FlushPermit {
    fn drop(&mut self) {}
}

static FLUSH_LIMITER: std::sync::OnceLock<Arc<FlushConnectionLimiter>> = std::sync::OnceLock::new();

pub fn get_flush_limiter() -> Arc<FlushConnectionLimiter> {
    FLUSH_LIMITER.get_or_init(|| {
        let max_concurrent = std::env::var("MAX_CONCURRENT_FLUSHES")
            .ok()
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(10);
        
        FlushConnectionLimiter::new(max_concurrent)
    }).clone()
}