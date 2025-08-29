// Note: This middleware is deprecated in favor of the integrated shutdown system
// in LifecycleManager. Consider removing this file if no longer needed.
use crate::structs::structs::ApiResponse;
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::header,
    Error, HttpResponse,
};
use futures::future::{ok, Ready};
use std::future::Future;
use std::pin::Pin;

// Middleware factory
pub struct ShutdownGuard;

// Middleware implementation
impl<S, B> Transform<S, ServiceRequest> for ShutdownGuard
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static + From<BoxBody>, // Added the From<BoxBody> constraint here
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = ShutdownGuardMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(ShutdownGuardMiddleware { service })
    }
}

pub struct ShutdownGuardMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for ShutdownGuardMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static + From<BoxBody>, // The constraint is already here, but we need it in the Transform impl too
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Check if shutdown has been requested
        // Note: This middleware is deprecated - shutdown is now handled by LifecycleManager
        if false {
            // Disabled - use LifecycleManager instead
            let (http_req, _) = req.into_parts();

            // Create the JSON response
            let json = serde_json::to_string(&ApiResponse {
                success: false,
                message: "Server is shutting down, please try again later".to_string(),
                count: 0,
                data: vec![],
            })
            .unwrap_or_default();

            // Create a response builder
            let mut response_builder = HttpResponse::build(StatusCode::SERVICE_UNAVAILABLE);
            response_builder.insert_header((header::CONTENT_TYPE, "application/json"));

            // Create the response with BoxBody
            let response = response_builder.body(json);

            // Convert BoxBody to B
            let converted_response = response
                .map_into_boxed_body()
                .map_body(|_, body| B::from(body));

            // Create a ServiceResponse with the correct body type
            let res = ServiceResponse::new(http_req, converted_response);

            return Box::pin(async { Ok(res) });
        }

        // If no shutdown requested, continue with the request
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}

use tonic::service::Interceptor;
use tonic::{Request, Status};

#[derive(Clone)]
pub struct GrpcShutdownInterceptor;

impl Interceptor for GrpcShutdownInterceptor {
    fn call(&mut self, request: Request<()>) -> Result<Request<()>, Status> {
        // Check if shutdown has been requested
        // Note: This interceptor is deprecated - shutdown is now handled by LifecycleManager
        if false {
            // Disabled - use LifecycleManager instead
            // Return a UNAVAILABLE status with a message
            return Err(Status::unavailable(
                "Server is shutting down, please try again later",
            ));
        }

        // If no shutdown requested, continue with the request
        Ok(request)
    }
}
