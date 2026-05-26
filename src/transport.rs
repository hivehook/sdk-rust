//! GraphQL transport used by every resource service.
//!
//! Serializes the query plus variables, attaches the bearer token, decodes the
//! `data` envelope into a caller-supplied type via Serde, and surfaces GraphQL
//! errors as [`HivehookError`].

use crate::errors::HivehookError;
use serde::de::DeserializeOwned;
use serde_json::{json, Map, Value};
use std::time::Duration;

/// Default maximum number of retry attempts after the initial request.
pub(crate) const DEFAULT_MAX_RETRIES: u32 = 2;

fn graphql_url(base_url: &str) -> String {
    format!("{}/graphql", base_url.trim_end_matches('/'))
}

fn extract_message_from_body(body: &str) -> String {
    match serde_json::from_str::<Value>(body) {
        Ok(json) => json
            .get("errors")
            .and_then(|e| e.as_array())
            .and_then(|a| a.first())
            .and_then(|e| e.get("message"))
            .and_then(|m| m.as_str())
            .map(String::from)
            .or_else(|| {
                json.get("message")
                    .and_then(|m| m.as_str())
                    .map(String::from)
            })
            .unwrap_or_else(|| body.to_string()),
        Err(_) => body.to_string(),
    }
}

fn parse_retry_after(header: Option<&str>) -> Option<Duration> {
    let raw = header?.trim();
    if let Ok(secs) = raw.parse::<u64>() {
        return Some(Duration::from_secs(secs));
    }
    // HTTP-date form is not parsed; only delta-seconds are honored here.
    None
}

fn classify_http_error(status: u16, retry_after: Option<&str>, body: &str) -> HivehookError {
    let message = extract_message_from_body(body);
    match status {
        401 => HivehookError::Auth(if message.is_empty() {
            "unauthorized".into()
        } else {
            message
        }),
        429 => HivehookError::RateLimit {
            retry_after: parse_retry_after(retry_after),
            message,
        },
        500..=599 => HivehookError::ServerError { status, message },
        _ => HivehookError::Api {
            message,
            status_code: Some(status),
        },
    }
}

fn decode_data(response_body: &str) -> Result<Map<String, Value>, HivehookError> {
    let json: Value = serde_json::from_str(response_body)?;

    if let Some(errors) = json.get("errors").and_then(|e| e.as_array()) {
        if let Some(err) = errors.first() {
            let message = err
                .get("message")
                .and_then(|m| m.as_str())
                .unwrap_or("unknown error")
                .to_string();
            let extensions = err.get("extensions").cloned();
            let code = err
                .get("extensions")
                .and_then(|e| e.get("code"))
                .and_then(|c| c.as_str());

            return Err(match code {
                Some("NOT_FOUND") => HivehookError::NotFound {
                    message,
                    extensions,
                },
                Some("CONFLICT") => HivehookError::Conflict {
                    message,
                    extensions,
                },
                Some("VALIDATION") => HivehookError::Validation {
                    message,
                    extensions,
                },
                _ => HivehookError::Api {
                    message,
                    status_code: None,
                },
            });
        }
    }

    json.get("data")
        .and_then(|d| d.as_object())
        .cloned()
        .ok_or_else(|| HivehookError::Api {
            message: "empty response data".into(),
            status_code: Some(500),
        })
}

fn build_body(query: &str, variables: Option<Map<String, Value>>) -> Value {
    let mut body = json!({ "query": query });
    if let Some(vars) = variables {
        body["variables"] = Value::Object(vars);
    }
    body
}

/// Returns `true` if the error is something a retry might recover from.
fn is_retryable(err: &HivehookError) -> bool {
    match err {
        HivehookError::RateLimit { .. } => true,
        HivehookError::ServerError { .. } => true,
        HivehookError::Http(e) => e.is_timeout() || e.is_connect() || e.is_request(),
        _ => false,
    }
}

/// Returns the recommended sleep duration before the next retry attempt.
fn retry_delay(err: &HivehookError, attempt: u32) -> Duration {
    if let HivehookError::RateLimit {
        retry_after: Some(d),
        ..
    } = err
    {
        return *d;
    }
    // Exponential backoff: 100ms, 200ms, 400ms ...
    let base_ms = 100u64.saturating_mul(1u64 << attempt.min(6));
    Duration::from_millis(base_ms)
}

/// Blocking HTTP transport for the Hivehook GraphQL API.
///
/// Available when the `blocking` feature is enabled (on by default).
#[cfg(feature = "blocking")]
#[derive(Clone)]
pub struct BlockingGraphQLTransport {
    client: reqwest::blocking::Client,
    graphql_url: String,
    api_key: Option<String>,
    max_retries: u32,
}

#[cfg(feature = "blocking")]
impl std::fmt::Debug for BlockingGraphQLTransport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BlockingGraphQLTransport")
            .field("graphql_url", &self.graphql_url)
            .field(
                "api_key",
                &self.api_key.as_ref().map(|_| "[REDACTED]"),
            )
            .field("max_retries", &self.max_retries)
            .finish()
    }
}

#[cfg(feature = "blocking")]
impl BlockingGraphQLTransport {
    /// Construct a new transport pointing at `base_url`. The `/graphql` suffix
    /// is appended automatically.
    pub fn new(base_url: &str, api_key: Option<String>) -> Result<Self, HivehookError> {
        Self::with_options(base_url, api_key, None, DEFAULT_MAX_RETRIES)
    }

    /// Construct a transport with a custom request timeout and max-retries.
    pub fn with_options(
        base_url: &str,
        api_key: Option<String>,
        timeout: Option<Duration>,
        max_retries: u32,
    ) -> Result<Self, HivehookError> {
        let mut builder = reqwest::blocking::Client::builder()
            .user_agent(concat!("hivehook-rust/", env!("CARGO_PKG_VERSION")));
        if let Some(t) = timeout {
            builder = builder.timeout(t);
        }
        Ok(Self {
            client: builder.build()?,
            graphql_url: graphql_url(base_url),
            api_key,
            max_retries,
        })
    }

    /// Execute a GraphQL operation and decode the `data` payload into `T`.
    pub fn execute<T: DeserializeOwned>(
        &self,
        query: &str,
        variables: Option<Map<String, Value>>,
    ) -> Result<T, HivehookError> {
        let data = self.execute_raw(query, variables)?;
        serde_json::from_value(Value::Object(data)).map_err(HivehookError::from)
    }

    /// Execute a GraphQL operation and return the raw `data` object.
    pub fn execute_raw(
        &self,
        query: &str,
        variables: Option<Map<String, Value>>,
    ) -> Result<Map<String, Value>, HivehookError> {
        let body = build_body(query, variables);
        let mut attempt: u32 = 0;
        loop {
            let result = self.send_once(&body);
            match result {
                Ok(v) => return Ok(v),
                Err(err) => {
                    if attempt < self.max_retries && is_retryable(&err) {
                        std::thread::sleep(retry_delay(&err, attempt));
                        attempt += 1;
                        continue;
                    }
                    return Err(err);
                }
            }
        }
    }

    fn send_once(&self, body: &Value) -> Result<Map<String, Value>, HivehookError> {
        let mut req = self
            .client
            .post(&self.graphql_url)
            .header("Content-Type", "application/json");

        if let Some(ref key) = self.api_key {
            req = req.header("Authorization", format!("Bearer {}", key));
        }

        let response = req.json(body).send()?;
        let status = response.status().as_u16();
        let retry_after = response
            .headers()
            .get("retry-after")
            .and_then(|h| h.to_str().ok())
            .map(String::from);
        let response_body = response.text()?;

        if status >= 400 {
            return Err(classify_http_error(
                status,
                retry_after.as_deref(),
                &response_body,
            ));
        }

        decode_data(&response_body)
    }
}

/// Async HTTP transport for the Hivehook GraphQL API.
///
/// Mirrors the public surface of [`BlockingGraphQLTransport`] but uses
/// [`reqwest::Client`] internally so callers can `.await` requests. Available
/// when the `async` feature is enabled (on by default).
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
#[derive(Clone)]
pub struct AsyncGraphQLTransport {
    client: reqwest::Client,
    graphql_url: String,
    api_key: Option<String>,
    max_retries: u32,
}

#[cfg(feature = "async")]
impl std::fmt::Debug for AsyncGraphQLTransport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AsyncGraphQLTransport")
            .field("graphql_url", &self.graphql_url)
            .field(
                "api_key",
                &self.api_key.as_ref().map(|_| "[REDACTED]"),
            )
            .field("max_retries", &self.max_retries)
            .finish()
    }
}

#[cfg(feature = "async")]
impl AsyncGraphQLTransport {
    /// Construct a new async transport pointing at `base_url`. The `/graphql`
    /// suffix is appended automatically.
    pub fn new(base_url: &str, api_key: Option<String>) -> Result<Self, HivehookError> {
        Self::with_options(base_url, api_key, None, DEFAULT_MAX_RETRIES)
    }

    /// Construct an async transport with a custom request timeout and
    /// max-retries.
    pub fn with_options(
        base_url: &str,
        api_key: Option<String>,
        timeout: Option<Duration>,
        max_retries: u32,
    ) -> Result<Self, HivehookError> {
        let mut builder = reqwest::Client::builder()
            .user_agent(concat!("hivehook-rust/", env!("CARGO_PKG_VERSION")));
        if let Some(t) = timeout {
            builder = builder.timeout(t);
        }
        Ok(Self {
            client: builder.build()?,
            graphql_url: graphql_url(base_url),
            api_key,
            max_retries,
        })
    }

    /// Execute a GraphQL operation and decode the `data` payload into `T`.
    pub async fn execute<T: DeserializeOwned>(
        &self,
        query: &str,
        variables: Option<Map<String, Value>>,
    ) -> Result<T, HivehookError> {
        let data = self.execute_raw(query, variables).await?;
        serde_json::from_value(Value::Object(data)).map_err(HivehookError::from)
    }

    /// Execute a GraphQL operation and return the raw `data` object.
    pub async fn execute_raw(
        &self,
        query: &str,
        variables: Option<Map<String, Value>>,
    ) -> Result<Map<String, Value>, HivehookError> {
        let body = build_body(query, variables);
        let mut attempt: u32 = 0;
        loop {
            let result = self.send_once(&body).await;
            match result {
                Ok(v) => return Ok(v),
                Err(err) => {
                    if attempt < self.max_retries && is_retryable(&err) {
                        tokio::time::sleep(retry_delay(&err, attempt)).await;
                        attempt += 1;
                        continue;
                    }
                    return Err(err);
                }
            }
        }
    }

    async fn send_once(&self, body: &Value) -> Result<Map<String, Value>, HivehookError> {
        let mut req = self
            .client
            .post(&self.graphql_url)
            .header("Content-Type", "application/json");

        if let Some(ref key) = self.api_key {
            req = req.header("Authorization", format!("Bearer {}", key));
        }

        let response = req.json(body).send().await?;
        let status = response.status().as_u16();
        let retry_after = response
            .headers()
            .get("retry-after")
            .and_then(|h| h.to_str().ok())
            .map(String::from);
        let response_body = response.text().await?;

        if status >= 400 {
            return Err(classify_http_error(
                status,
                retry_after.as_deref(),
                &response_body,
            ));
        }

        decode_data(&response_body)
    }
}
