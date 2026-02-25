//! gRPC server implementation for CodeService.

use crate::generated::code_service_server::{CodeService, CodeServiceServer};
use crate::generated::{GetCodeRequest, GetCodeResponse, InitCountersRequest, InitCountersResponse};
use crate::redis_code;
use deadpool_redis::Pool;
use tonic::{Request, Response, Status};

pub struct CodeServiceImpl {
    pool: Pool,
}

impl CodeServiceImpl {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    pub fn into_service(self) -> CodeServiceServer<Self> {
        CodeServiceServer::new(self)
    }
}

#[tonic::async_trait]
impl CodeService for CodeServiceImpl {
    async fn get_code(
        &self,
        request: Request<GetCodeRequest>,
    ) -> Result<Response<GetCodeResponse>, Status> {
        let req = request.into_inner();
        let database = if req.database.is_empty() {
            "default"
        } else {
            req.database.as_str()
        };
        let table = req.table.as_str();
        if table.is_empty() {
            return Err(Status::invalid_argument("table is required"));
        }
        match redis_code::get_next_code(&self.pool, database, table).await {
            Ok(code) => Ok(Response::new(GetCodeResponse { code })),
            Err(redis_code::CodeError::ConfigMissing { .. }) => Err(Status::failed_precondition(
                format!("Config missing for {}:{}. Run InitCounters first.", database, table),
            )),
            Err(redis_code::CodeError::Redis(e)) => {
                log::error!("Redis error in get_code: {}", e);
                Err(Status::internal("Redis error"))
            }
            Err(redis_code::CodeError::Pool(e)) => {
                log::error!("Pool error in get_code: {}", e);
                Err(Status::internal("Connection pool error"))
            }
        }
    }

    async fn init_counters(
        &self,
        request: Request<InitCountersRequest>,
    ) -> Result<Response<InitCountersResponse>, Status> {
        let req = request.into_inner();
        let database = if req.database.is_empty() {
            "default"
        } else {
            req.database.as_str()
        };
        let entities: Vec<(String, String, i32, i32)> = req
            .counters
            .into_iter()
            .map(|c| {
                (
                    c.entity,
                    c.prefix,
                    c.default_code,
                    c.digits_number,
                )
            })
            .collect();
        if entities.is_empty() {
            return Ok(Response::new(InitCountersResponse {
                success: true,
                message: String::new(),
            }));
        }
        match redis_code::init_counters(&self.pool, database, &entities).await {
            Ok(()) => Ok(Response::new(InitCountersResponse {
                success: true,
                message: String::new(),
            })),
            Err(e) => {
                log::error!("init_counters error: {}", e);
                Ok(Response::new(InitCountersResponse {
                    success: false,
                    message: e.to_string(),
                }))
            }
        }
    }
}
