use actix_web::http::Method;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RequestType {
    Read,
    Write,
}

impl Default for RequestType {
    fn default() -> Self {
        RequestType::Read
    }
}

impl fmt::Display for RequestType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RequestType::Read => write!(f, "read"),
            RequestType::Write => write!(f, "write"),
        }
    }
}

#[derive(Debug)]
pub struct EndpointPattern {
    pub method: Method,
    pub path_pattern: Regex,
    pub request_type: RequestType,
}

lazy_static! {
    static ref ENDPOINT_PATTERNS: Vec<EndpointPattern> = vec![
        // Read patterns
        EndpointPattern {
            method: Method::GET,
            path_pattern: Regex::new(r"^/api/store/[^/]+/[^/]+$").unwrap(),
            request_type: RequestType::Read,
        },
        EndpointPattern {
            method: Method::GET,
            path_pattern: Regex::new(r"^/api/store/root/[^/]+/[^/]+$").unwrap(),
            request_type: RequestType::Read,
        },
        EndpointPattern {
            method: Method::POST,
            path_pattern: Regex::new(r"^/api/store/[^/]+/filter$").unwrap(),
            request_type: RequestType::Read,
        },
        EndpointPattern {
            method: Method::POST,
            path_pattern: Regex::new(r"^/api/store/root/[^/]+/filter$").unwrap(),
            request_type: RequestType::Read,
        },

        // Write patterns
        EndpointPattern {
            method: Method::POST,
            path_pattern: Regex::new(r"^/api/store/[^/]+$").unwrap(),
            request_type: RequestType::Write,
        },
        EndpointPattern {
            method: Method::PATCH,
            path_pattern: Regex::new(r"^/api/store/[^/]+/[^/]+$").unwrap(),
            request_type: RequestType::Write,
        },
        EndpointPattern {
            method: Method::DELETE,
            path_pattern: Regex::new(r"^/api/store/[^/]+/[^/]+$").unwrap(),
            request_type: RequestType::Write,
        },
        EndpointPattern {
            method: Method::POST,
            path_pattern: Regex::new(r"^/api/store/root/[^/]+$").unwrap(),
            request_type: RequestType::Write,
        },
        EndpointPattern {
            method: Method::PATCH,
            path_pattern: Regex::new(r"^/api/store/root/[^/]+/[^/]+$").unwrap(),
            request_type: RequestType::Write,
        },
        EndpointPattern {
            method: Method::DELETE,
            path_pattern: Regex::new(r"^/api/store/root/[^/]+/[^/]+$").unwrap(),
            request_type: RequestType::Write,
        },
        EndpointPattern {
            method: Method::POST,
            path_pattern: Regex::new(r"^/api/store/batch/[^/]+$").unwrap(),
            request_type: RequestType::Write,
        },
        EndpointPattern {
            method: Method::PATCH,
            path_pattern: Regex::new(r"^/api/store/batch/[^/]+$").unwrap(),
            request_type: RequestType::Write,
        },
        EndpointPattern {
            method: Method::DELETE,
            path_pattern: Regex::new(r"^/api/store/batch/[^/]+$").unwrap(),
            request_type: RequestType::Write,
        },
    ];
}

pub struct RequestTypeHandler;

impl RequestTypeHandler {
    pub fn get_request_type(method: &Method, url: &str) -> Result<RequestType, String> {
        for pattern in ENDPOINT_PATTERNS.iter() {
            if &pattern.method == method && pattern.path_pattern.is_match(url) {
                return Ok(pattern.request_type);
            }
        }

        Err(format!("Invalid Request Type: {}:{}", method, url))
    }
}
