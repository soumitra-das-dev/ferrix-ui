//! # ferrix-network
//!
//! Trait-abstracted HTTP client honoring Android trust store, proxies, and security policies.

use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum NetworkError {
    #[error("HTTP request failed: {0}")]
    RequestFailed(String),
    #[error("Failed to parse JSON response: {0}")]
    InvalidJson(String),
    #[error("Request timed out")]
    Timeout,
    #[error("Network security policy violation (e.g. cleartext traffic not permitted)")]
    SecurityPolicyViolation,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
}

#[derive(Clone, Debug, PartialEq)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
}

impl HttpRequest {
    pub fn get(url: impl Into<String>) -> Self {
        Self {
            method: HttpMethod::Get,
            url: url.into(),
            headers: HashMap::new(),
            body: None,
        }
    }

    pub fn post(url: impl Into<String>, body: Vec<u8>) -> Self {
        Self {
            method: HttpMethod::Post,
            url: url.into(),
            headers: HashMap::new(),
            body: Some(body),
        }
    }

    pub fn with_header(mut self, key: impl Into<String>, val: impl Into<String>) -> Self {
        self.headers.insert(key.into(), val.into());
        self
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl HttpResponse {
    pub fn new(status: u16, body: Vec<u8>) -> Self {
        Self {
            status,
            headers: HashMap::new(),
            body,
        }
    }

    pub fn json<T: DeserializeOwned>(&self) -> Result<T, NetworkError> {
        serde_json::from_slice(&self.body).map_err(|e| NetworkError::InvalidJson(e.to_string()))
    }
}

/// Abstract HTTP client interface.
pub trait HttpClient: Send + Sync {
    fn execute(&self, req: HttpRequest) -> Result<HttpResponse, NetworkError>;
}

/// In-memory mock HTTP client for deterministic testing and offline demo execution.
#[derive(Clone, Default)]
pub struct MockHttpClient {
    routes: Arc<Mutex<HashMap<String, HttpResponse>>>,
}

impl MockHttpClient {
    pub fn new() -> Self {
        Self {
            routes: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn register(&self, url: impl Into<String>, response: HttpResponse) {
        self.routes.lock().unwrap().insert(url.into(), response);
    }
}

impl HttpClient for MockHttpClient {
    fn execute(&self, req: HttpRequest) -> Result<HttpResponse, NetworkError> {
        let routes = self.routes.lock().unwrap();
        if let Some(resp) = routes.get(&req.url) {
            Ok(resp.clone())
        } else {
            Err(NetworkError::RequestFailed(format!(
                "404 Not Found: {}",
                req.url
            )))
        }
    }
}

/// Query status state machine for `use_query`.
#[derive(Clone, Debug, PartialEq)]
pub enum QueryStatus<T> {
    Loading,
    Success(T),
    Error(String),
}

/// Mutation status state machine for `use_mutation`.
#[derive(Clone, Debug, PartialEq)]
pub enum MutationStatus<T> {
    Idle,
    Loading,
    Success(T),
    Error(String),
}
