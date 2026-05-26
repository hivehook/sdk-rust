//! Error type returned by every fallible Hivehook SDK call.

use std::time::Duration;
use thiserror::Error;

/// Errors that can be returned by SDK operations.
///
/// The enum is marked `#[non_exhaustive]` so that new variants can be added
/// without a breaking change. Always include a wildcard arm when matching.
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum HivehookError {
    /// Generic API-level error reported by the server.
    #[error("API error: {message}")]
    Api {
        /// Human-readable error message.
        message: String,
        /// HTTP status code, when available.
        status_code: Option<u16>,
    },

    /// The server returned a `NOT_FOUND` error code.
    #[error("not found: {message}")]
    NotFound {
        /// Human-readable error message.
        message: String,
        /// Raw `extensions` payload from the GraphQL error, when present.
        extensions: Option<serde_json::Value>,
    },

    /// The server returned a `CONFLICT` error code.
    #[error("conflict: {message}")]
    Conflict {
        /// Human-readable error message.
        message: String,
        /// Raw `extensions` payload from the GraphQL error, when present.
        extensions: Option<serde_json::Value>,
    },

    /// Authentication failed (missing/invalid API key).
    #[error("auth error: {0}")]
    Auth(String),

    /// The server reported a validation error on the request payload.
    #[error("validation error: {message}")]
    Validation {
        /// Human-readable error message.
        message: String,
        /// Raw `extensions` payload from the GraphQL error, when present.
        extensions: Option<serde_json::Value>,
    },

    /// HTTP 429: the server is rate-limiting the caller. If the server sent
    /// a `Retry-After` header it is parsed into [`retry_after`](Self::RateLimit).
    #[error("rate limited: {message}")]
    RateLimit {
        /// Suggested wait, parsed from the `Retry-After` header when present.
        retry_after: Option<Duration>,
        /// Human-readable error message.
        message: String,
    },

    /// HTTP 5xx server-side failure.
    #[error("server error {status}: {message}")]
    ServerError {
        /// HTTP status code returned by the server.
        status: u16,
        /// Human-readable error message.
        message: String,
    },

    /// Underlying HTTP/transport failure.
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// JSON serialization/deserialization failure.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}
