//! Typed entity definitions returned by the Hivehook GraphQL API.
//!
//! Every struct mirrors the canonical shape defined in `sdk-python/hivehook/types.py`
//! and uses `#[serde(rename_all = "camelCase")]` because the wire format is camelCase.
//! Optional/nullable GraphQL fields are modelled as `Option<T>`. Timestamps are kept
//! as `String` for now to avoid pulling in a date/time dependency; callers can parse
//! them with `chrono`/`time` if needed.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Pagination
// ---------------------------------------------------------------------------

/// Page metadata returned alongside list connections.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    /// Total number of records matching the query.
    #[serde(default)]
    pub total: i64,
    /// Page size used for this response.
    #[serde(default)]
    pub limit: i64,
    /// Offset used for this response.
    #[serde(default)]
    pub offset: i64,
    /// Cursor pointing at the next page, when cursor pagination is used.
    #[serde(default)]
    pub end_cursor: Option<String>,
    /// Whether more records are available after this page.
    #[serde(default)]
    pub has_next_page: bool,
}

/// A generic GraphQL connection containing nodes and page metadata.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListResult<T> {
    /// The records on this page.
    pub nodes: Vec<T>,
    /// Page metadata describing total count, cursors, etc.
    pub page_info: PageInfo,
}

// ---------------------------------------------------------------------------
// Shared sub-structs
// ---------------------------------------------------------------------------

/// Retry behaviour applied to a destination/endpoint after a failed delivery.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RetryPolicy {
    /// Maximum number of attempts before giving up.
    #[serde(default)]
    pub max_attempts: i32,
    /// Initial delay between attempts, expressed as a Go duration string.
    #[serde(default)]
    pub initial_delay: String,
    /// Maximum delay between attempts.
    #[serde(default)]
    pub max_delay: String,
    /// Multiplicative backoff factor applied between attempts.
    #[serde(default)]
    pub backoff_factor: f64,
}

/// A single body-match clause used in subscription/endpoint filtering.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BodyMatchRule {
    /// JSON pointer / dotted path into the event body.
    #[serde(default)]
    pub path: String,
    /// Comparison value.
    #[serde(default)]
    pub value: String,
    /// Operator (`eq`, `neq`, `contains`, ...).
    #[serde(default)]
    pub operator: String,
}

/// A recursive filter rule supporting boolean groups.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterRule {
    /// Operator, e.g. `eq`, `and`, `or`.
    #[serde(default)]
    pub operator: String,
    /// Optional JSON path target.
    #[serde(default)]
    pub path: Option<String>,
    /// Optional comparison value (any JSON value).
    #[serde(default)]
    pub value: Option<Value>,
    /// Optional nested rules for boolean groups.
    #[serde(default)]
    pub rules: Option<Vec<FilterRule>>,
}

/// Filter configuration attached to a subscription or endpoint.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterConfig {
    /// Allow-list of event types.
    #[serde(default)]
    pub event_types: Option<Vec<String>>,
    /// Allow-list of regular expressions matching event types.
    #[serde(default)]
    pub regex: Option<Vec<String>>,
    /// Body-match clauses.
    #[serde(default)]
    pub body_match: Option<Vec<BodyMatchRule>>,
    /// Top-level recursive rules.
    #[serde(default)]
    pub rules: Option<Vec<FilterRule>>,
}

/// Transformation configuration applied before delivery.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransformConfig {
    /// Whether the payload should be wrapped in a Hivehook envelope.
    #[serde(default)]
    pub envelope: bool,
    /// Additional headers to merge into the outbound request.
    #[serde(default)]
    pub headers: Option<HashMap<String, Value>>,
}

/// Custom response config for an ingest source.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseConfig {
    /// HTTP status code returned by ingest.
    #[serde(default)]
    pub status_code: i32,
    /// Static body returned by ingest.
    #[serde(default)]
    pub body: String,
    /// Content-Type header returned by ingest.
    #[serde(default)]
    pub content_type: String,
}

/// Deduplication policy applied to inbound events.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DedupConfig {
    /// Deduplication strategy (e.g. `header`, `hash`).
    #[serde(default)]
    pub strategy: String,
    /// Fields involved in the deduplication key.
    #[serde(default)]
    pub fields: Option<Vec<String>>,
    /// Window over which duplicates are suppressed.
    #[serde(default)]
    pub window: Option<String>,
}

/// OAuth2 client-credentials configuration.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OAuth2Config {
    /// Token endpoint URL.
    #[serde(default)]
    pub token_url: String,
    /// Client ID.
    #[serde(default)]
    pub client_id: String,
    /// Client secret (stored encrypted server-side).
    #[serde(default)]
    pub client_secret: String,
    /// Requested scopes.
    #[serde(default)]
    pub scopes: Vec<String>,
    /// Audience for the token.
    #[serde(default)]
    pub audience: String,
}

/// Auto-disable behaviour for unhealthy destinations.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthConfig {
    /// Rolling window in hours.
    #[serde(default)]
    pub window_hours: i32,
    /// Disable the destination if the success rate falls below this value.
    #[serde(default)]
    pub disable_below: f64,
}

/// Email alert channel configuration.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EmailAlertConfig {
    /// Recipient addresses.
    #[serde(default)]
    pub to: Vec<String>,
    /// Optional subject template.
    #[serde(default)]
    pub subject_template: Option<String>,
}

/// Slack alert channel configuration.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SlackAlertConfig {
    /// Slack incoming webhook URL.
    #[serde(default)]
    pub webhook_url: String,
    /// Optional override channel.
    #[serde(default)]
    pub channel: Option<String>,
}

/// OpenTelemetry exporter configuration attached to an organization.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OtlpConfig {
    /// Collector endpoint.
    #[serde(default)]
    pub endpoint: String,
    /// Extra headers sent to the collector.
    #[serde(default)]
    pub headers: Option<HashMap<String, String>>,
    /// Whether to allow insecure connections.
    #[serde(default)]
    pub insecure: bool,
    /// Sampling rate between 0.0 and 1.0.
    #[serde(default)]
    pub sample_rate: f64,
}

// ---------------------------------------------------------------------------
// Inbound entities
// ---------------------------------------------------------------------------

/// A webhook source that receives inbound events.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    /// Source identifier.
    #[serde(default)]
    pub id: String,
    /// Human-readable name.
    #[serde(default)]
    pub name: String,
    /// URL slug used for the ingest endpoint.
    #[serde(default)]
    pub slug: String,
    /// Provider type (e.g. `stripe`, `github`, `generic`).
    #[serde(default)]
    pub provider_type: String,
    /// Provider-specific verification configuration.
    #[serde(default)]
    pub verify_config: Option<Value>,
    /// Source status (`active`, `paused`, ...).
    #[serde(default)]
    pub status: String,
    /// Per-source RPS limit (0 = unlimited).
    #[serde(default)]
    pub rate_limit_rps: i32,
    /// Whether spike protection is enabled.
    #[serde(default)]
    pub spike_protection: bool,
    /// Hard ingest RPS ceiling.
    #[serde(default)]
    pub max_ingest_rps: i32,
    /// Broker-specific configuration.
    #[serde(default)]
    pub broker_config: Option<Value>,
    /// Custom HTTP response configuration.
    #[serde(default)]
    pub response_config: Option<ResponseConfig>,
    /// Deduplication configuration.
    #[serde(default)]
    pub dedup_config: Option<DedupConfig>,
    /// Creation timestamp (RFC3339).
    #[serde(default)]
    pub created_at: String,
}

/// An outbound destination that receives delivered events.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Destination {
    /// Destination identifier.
    #[serde(default)]
    pub id: String,
    /// Human-readable name.
    #[serde(default)]
    pub name: String,
    /// Target URL.
    #[serde(default)]
    pub url: String,
    /// Active signing secret used to sign outbound requests.
    #[serde(default)]
    pub signing_secret: String,
    /// Destination status.
    #[serde(default)]
    pub status: String,
    /// Destination type (`HTTP`, `SQS`, etc.).
    #[serde(default, rename = "type")]
    pub type_: String,
    /// Type-specific configuration blob.
    #[serde(default)]
    pub type_config: Option<Value>,
    /// Timeout per delivery attempt, in milliseconds.
    #[serde(default)]
    pub timeout_ms: i32,
    /// Per-destination RPS limit.
    #[serde(default)]
    pub rate_limit_rps: i32,
    /// Retry policy.
    #[serde(default)]
    pub retry_policy: Option<RetryPolicy>,
    /// Static headers added to every outbound request.
    #[serde(default)]
    pub headers: Option<HashMap<String, Value>>,
    /// Outbound auth type (`none`, `bearer`, `basic`, `oauth2`, `mtls`).
    #[serde(default)]
    pub auth_type: String,
    /// OAuth2 configuration when `auth_type == "oauth2"`.
    #[serde(default)]
    pub oauth2_config: Option<OAuth2Config>,
    /// mTLS client certificate (PEM).
    #[serde(default)]
    pub mtls_cert: String,
    /// mTLS client private key (PEM).
    #[serde(default)]
    pub mtls_key: String,
    /// Delivery mode (`push`, `poll`).
    #[serde(default)]
    pub delivery_mode: String,
    /// Public prefix of the poll API key.
    #[serde(default)]
    pub poll_api_key_prefix: String,
    /// Full poll API key (only present immediately after rotation).
    #[serde(default)]
    pub poll_api_key: String,
    /// Whether ordered (one-in-flight) delivery is enabled.
    #[serde(default)]
    pub ordered: bool,
    /// ID of the delivery currently blocking the queue, if any.
    #[serde(default)]
    pub blocked_delivery_id: Option<String>,
    /// Rolling success rate score, 0.0..1.0.
    #[serde(default = "default_health_score")]
    pub health_score: f64,
    /// Reason the destination was auto-disabled, if applicable.
    #[serde(default)]
    pub disabled_reason: Option<String>,
    /// Auto-disable thresholds.
    #[serde(default)]
    pub health_config: Option<HealthConfig>,
    /// Output payload format.
    #[serde(default = "default_output_format")]
    pub output_format: String,
    /// Creation timestamp.
    #[serde(default)]
    pub created_at: String,
}

fn default_health_score() -> f64 {
    1.0
}

fn default_output_format() -> String {
    "default".to_string()
}

/// Wiring between a source and destination.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Subscription {
    /// Subscription identifier.
    #[serde(default)]
    pub id: String,
    /// Human-readable name.
    #[serde(default)]
    pub name: String,
    /// Source identifier.
    #[serde(default)]
    pub source_id: String,
    /// Destination identifier.
    #[serde(default)]
    pub destination_id: String,
    /// Optional filter configuration.
    #[serde(default)]
    pub filter_config: Option<FilterConfig>,
    /// Optional transformation configuration.
    #[serde(default)]
    pub transform_config: Option<TransformConfig>,
    /// Whether the subscription is enabled.
    #[serde(default)]
    pub enabled: bool,
    /// Creation timestamp.
    #[serde(default)]
    pub created_at: String,
}

/// A received inbound event.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    /// Event identifier.
    #[serde(default)]
    pub id: String,
    /// Originating source.
    #[serde(default)]
    pub source_id: String,
    /// Idempotency key extracted from the inbound request.
    #[serde(default)]
    pub idempotency_key: String,
    /// Event type as classified by the provider.
    #[serde(default)]
    pub event_type: String,
    /// Captured headers.
    #[serde(default)]
    pub headers: Option<HashMap<String, Value>>,
    /// Raw request body.
    #[serde(default)]
    pub raw_body: Option<String>,
    /// Processing status.
    #[serde(default)]
    pub status: String,
    /// Receipt timestamp.
    #[serde(default)]
    pub received_at: String,
}

/// A single delivery attempt.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryAttempt {
    /// Attempt identifier.
    #[serde(default)]
    pub id: String,
    /// Parent delivery identifier.
    #[serde(default)]
    pub delivery_id: String,
    /// 1-indexed attempt number.
    #[serde(default)]
    pub attempt_number: i32,
    /// HTTP status code returned by the destination.
    #[serde(default)]
    pub response_status: i32,
    /// Response body (truncated).
    #[serde(default)]
    pub response_body: String,
    /// Transport error, if any.
    #[serde(default)]
    pub error: String,
    /// Wall-clock duration of the attempt.
    #[serde(default)]
    pub duration_ms: i32,
    /// Attempt timestamp.
    #[serde(default)]
    pub attempted_at: String,
}

/// A delivery of an event to a destination.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Delivery {
    /// Delivery identifier.
    #[serde(default)]
    pub id: String,
    /// Originating event identifier.
    #[serde(default)]
    pub event_id: String,
    /// Subscription that produced this delivery.
    #[serde(default)]
    pub subscription_id: String,
    /// Target destination.
    #[serde(default)]
    pub destination_id: String,
    /// Current delivery status.
    #[serde(default)]
    pub status: String,
    /// Number of attempts made so far.
    #[serde(default)]
    pub attempts: i32,
    /// Maximum allowed attempts.
    #[serde(default)]
    pub max_attempts: i32,
    /// Next scheduled attempt timestamp, if pending.
    #[serde(default)]
    pub next_attempt_at: Option<String>,
    /// Creation timestamp.
    #[serde(default)]
    pub created_at: String,
    /// Detailed list of attempts (only populated when explicitly requested).
    #[serde(default)]
    pub delivery_attempts: Option<Vec<DeliveryAttempt>>,
}

/// A dead-letter queue entry for an inbound delivery.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DlqEntry {
    /// DLQ entry identifier.
    #[serde(default)]
    pub id: String,
    /// Failed delivery identifier.
    #[serde(default)]
    pub delivery_id: String,
    /// Originating event identifier.
    #[serde(default)]
    pub event_id: String,
    /// Last error captured before the entry was dead-lettered.
    #[serde(default)]
    pub last_error: String,
    /// Timestamp the entry was replayed, if it was.
    #[serde(default)]
    pub replayed_at: Option<String>,
    /// Creation timestamp.
    #[serde(default)]
    pub created_at: String,
}

/// An API key issued for accessing the Hivehook API.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKey {
    /// API key identifier.
    #[serde(default)]
    pub id: String,
    /// Human-readable name.
    #[serde(default)]
    pub name: String,
    /// Public key prefix (never the full key after creation).
    #[serde(default)]
    pub key_prefix: String,
    /// Granted scopes.
    #[serde(default)]
    pub scopes: Vec<String>,
    /// Sources this key is scoped to (empty == all).
    #[serde(default)]
    pub source_ids: Vec<String>,
    /// Creation timestamp.
    #[serde(default)]
    pub created_at: String,
    /// Optional expiry timestamp.
    #[serde(default)]
    pub expires_at: Option<String>,
    /// Revocation timestamp, if revoked.
    #[serde(default)]
    pub revoked_at: Option<String>,
    /// Last-used timestamp.
    #[serde(default)]
    pub last_used_at: Option<String>,
}

/// Wrapper returned by `apiKeys.create()` containing the one-time raw key.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeyWithSecret {
    /// The persistent API key metadata.
    pub api_key: ApiKey,
    /// The raw API key string (only available at creation time).
    pub raw_key: String,
}

/// An alert rule firing on system events.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AlertRule {
    /// Alert rule identifier.
    #[serde(default)]
    pub id: String,
    /// Human-readable name.
    #[serde(default)]
    pub name: String,
    /// Type of condition that triggers the alert.
    #[serde(default)]
    pub condition_type: String,
    /// Numeric threshold.
    #[serde(default)]
    pub threshold: i32,
    /// Webhook URL for the `WEBHOOK` channel.
    #[serde(default)]
    pub webhook_url: String,
    /// Output channel (`WEBHOOK`, `EMAIL`, `SLACK`).
    #[serde(default = "default_alert_channel")]
    pub channel: String,
    /// Email channel configuration.
    #[serde(default)]
    pub email_config: Option<EmailAlertConfig>,
    /// Slack channel configuration.
    #[serde(default)]
    pub slack_config: Option<SlackAlertConfig>,
    /// Cooldown between firings, as a Go duration.
    #[serde(default)]
    pub cooldown: String,
    /// Whether the rule is enabled.
    #[serde(default)]
    pub enabled: bool,
    /// Creation timestamp.
    #[serde(default)]
    pub created_at: String,
}

fn default_alert_channel() -> String {
    "WEBHOOK".to_string()
}

/// A user-supplied bookmark on an event.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Bookmark {
    /// Bookmark identifier.
    #[serde(default)]
    pub id: String,
    /// Bookmarked event identifier.
    #[serde(default)]
    pub event_id: String,
    /// Bookmark name.
    #[serde(default)]
    pub name: String,
    /// Free-form notes.
    #[serde(default)]
    pub notes: String,
    /// Creation timestamp.
    #[serde(default)]
    pub created_at: String,
}

/// JSON-schema for an event type, used for validation and docs.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventTypeSchema {
    /// Schema identifier.
    #[serde(default)]
    pub id: String,
    /// The event type this schema describes.
    #[serde(default)]
    pub event_type: String,
    /// Free-form description.
    #[serde(default)]
    pub description: String,
    /// JSON Schema document.
    #[serde(default)]
    pub schema: Option<Value>,
    /// Example payload.
    #[serde(default)]
    pub example: Option<Value>,
    /// Creation timestamp.
    #[serde(default)]
    pub created_at: String,
    /// Update timestamp.
    #[serde(default)]
    pub updated_at: String,
}

// ---------------------------------------------------------------------------
// Outbound entities
// ---------------------------------------------------------------------------

/// An outbound application that emits messages.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Application {
    /// Application identifier.
    #[serde(default)]
    pub id: String,
    /// Human-readable name.
    #[serde(default)]
    pub name: String,
    /// Caller-supplied unique identifier.
    #[serde(default)]
    pub uid: String,
    /// Creation timestamp.
    #[serde(default)]
    pub created_at: String,
}

/// An outbound endpoint (the customer's HTTP receiver).
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Endpoint {
    /// Endpoint identifier.
    #[serde(default)]
    pub id: String,
    /// Owning application.
    #[serde(default)]
    pub application_id: String,
    /// Target URL.
    #[serde(default)]
    pub url: String,
    /// Signing secret used to sign payloads.
    #[serde(default)]
    pub signing_secret: String,
    /// Optional filter configuration.
    #[serde(default)]
    pub filter_config: Option<FilterConfig>,
    /// Endpoint status.
    #[serde(default)]
    pub status: String,
    /// Endpoint type (`HTTP`, etc.).
    #[serde(default, rename = "type")]
    pub type_: String,
    /// Type-specific configuration.
    #[serde(default)]
    pub type_config: Option<Value>,
    /// Per-endpoint RPS limit.
    #[serde(default)]
    pub rate_limit_rps: i32,
    /// Timeout per attempt, in milliseconds.
    #[serde(default)]
    pub timeout_ms: i32,
    /// Retry policy.
    #[serde(default)]
    pub retry_policy: Option<RetryPolicy>,
    /// Static headers.
    #[serde(default)]
    pub headers: Option<HashMap<String, Value>>,
    /// Outbound auth type.
    #[serde(default)]
    pub auth_type: String,
    /// OAuth2 configuration.
    #[serde(default)]
    pub oauth2_config: Option<OAuth2Config>,
    /// mTLS certificate (PEM).
    #[serde(default)]
    pub mtls_cert: String,
    /// mTLS private key (PEM).
    #[serde(default)]
    pub mtls_key: String,
    /// Delivery mode (`push`, `poll`).
    #[serde(default)]
    pub delivery_mode: String,
    /// Public prefix of the poll API key.
    #[serde(default)]
    pub poll_api_key_prefix: String,
    /// Full poll API key (only present immediately after rotation).
    #[serde(default)]
    pub poll_api_key: String,
    /// Whether ordered delivery is enabled.
    #[serde(default)]
    pub ordered: bool,
    /// ID of the delivery currently blocking the queue, if any.
    #[serde(default)]
    pub blocked_delivery_id: Option<String>,
    /// Rolling success rate score.
    #[serde(default = "default_health_score")]
    pub health_score: f64,
    /// Reason the endpoint was auto-disabled, if applicable.
    #[serde(default)]
    pub disabled_reason: Option<String>,
    /// Auto-disable thresholds.
    #[serde(default)]
    pub health_config: Option<HealthConfig>,
    /// Output payload format.
    #[serde(default = "default_output_format")]
    pub output_format: String,
    /// Creation timestamp.
    #[serde(default)]
    pub created_at: String,
}

/// An outbound message queued for delivery.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    /// Message identifier.
    #[serde(default)]
    pub id: String,
    /// Owning application.
    #[serde(default)]
    pub application_id: String,
    /// Event type identifier.
    #[serde(default)]
    pub event_type: String,
    /// Base64-encoded payload (when present).
    #[serde(default)]
    pub payload: Option<String>,
    /// Idempotency key.
    #[serde(default)]
    pub idempotency_key: String,
    /// Processing status.
    #[serde(default)]
    pub status: String,
    /// Creation timestamp.
    #[serde(default)]
    pub created_at: String,
}

/// A single outbound delivery attempt.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OutboundDeliveryAttempt {
    /// Attempt identifier.
    #[serde(default)]
    pub id: String,
    /// Parent delivery identifier.
    #[serde(default)]
    pub delivery_id: String,
    /// 1-indexed attempt number.
    #[serde(default)]
    pub attempt_number: i32,
    /// HTTP status code from the endpoint.
    #[serde(default)]
    pub response_status: i32,
    /// Truncated response body.
    #[serde(default)]
    pub response_body: String,
    /// Transport error, if any.
    #[serde(default)]
    pub error: String,
    /// Attempt duration.
    #[serde(default)]
    pub duration_ms: i32,
    /// Attempt timestamp.
    #[serde(default)]
    pub attempted_at: String,
}

/// A delivery of a message to an endpoint.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OutboundDelivery {
    /// Delivery identifier.
    #[serde(default)]
    pub id: String,
    /// Originating message identifier.
    #[serde(default)]
    pub message_id: String,
    /// Target endpoint.
    #[serde(default)]
    pub endpoint_id: String,
    /// Current status.
    #[serde(default)]
    pub status: String,
    /// Attempts made so far.
    #[serde(default)]
    pub attempts: i32,
    /// Maximum allowed attempts.
    #[serde(default)]
    pub max_attempts: i32,
    /// Next scheduled attempt timestamp.
    #[serde(default)]
    pub next_attempt_at: Option<String>,
    /// Creation timestamp.
    #[serde(default)]
    pub created_at: String,
    /// Detailed list of attempts.
    #[serde(default)]
    pub delivery_attempts: Option<Vec<OutboundDeliveryAttempt>>,
}

/// A dead-letter queue entry for an outbound delivery.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OutboundDlqEntry {
    /// DLQ entry identifier.
    #[serde(default)]
    pub id: String,
    /// Failed delivery identifier.
    #[serde(default)]
    pub delivery_id: String,
    /// Originating message identifier.
    #[serde(default)]
    pub message_id: String,
    /// Last error.
    #[serde(default)]
    pub last_error: String,
    /// Timestamp the entry was replayed, if it was.
    #[serde(default)]
    pub replayed_at: Option<String>,
    /// Creation timestamp.
    #[serde(default)]
    pub created_at: String,
}

// ---------------------------------------------------------------------------
// System / management entities
// ---------------------------------------------------------------------------

/// System health and counters.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemStatus {
    /// Overall status string (`healthy`, etc.).
    #[serde(default)]
    pub status: String,
    /// Size of the inbound DLQ.
    #[serde(default)]
    pub dlq_size: i64,
    /// Size of the outbound DLQ.
    #[serde(default)]
    pub outbound_dlq_size: i64,
    /// Queue depth across destinations.
    #[serde(default)]
    pub queue_depth: i64,
    /// Workers currently processing a delivery.
    #[serde(default)]
    pub active_workers: i32,
    /// Configured worker pool size.
    #[serde(default)]
    pub total_workers: i32,
    /// Uptime in seconds.
    #[serde(default)]
    pub uptime: i64,
    /// Build version.
    #[serde(default)]
    pub version: String,
    /// Total number of sources.
    #[serde(default)]
    pub sources_total: i64,
    /// Total number of destinations.
    #[serde(default)]
    pub destinations_total: i64,
    /// Total number of subscriptions.
    #[serde(default)]
    pub subscriptions_total: i64,
    /// Total number of events received.
    #[serde(default)]
    pub events_total: i64,
    /// Total number of failed events.
    #[serde(default)]
    pub events_failed: i64,
    /// Total deliveries.
    #[serde(default)]
    pub deliveries_total: i64,
    /// Pending deliveries.
    #[serde(default)]
    pub deliveries_pending: i64,
    /// Delivered deliveries.
    #[serde(default)]
    pub deliveries_delivered: i64,
    /// Total outbound messages.
    #[serde(default)]
    pub messages_total: i64,
    /// Total outbound deliveries.
    #[serde(default)]
    pub outbound_deliveries_total: i64,
    /// Pending outbound deliveries.
    #[serde(default)]
    pub outbound_deliveries_pending: i64,
    /// Failed outbound deliveries.
    #[serde(default)]
    pub outbound_deliveries_failed: i64,
}

/// Result of a DLQ replay-all operation.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplayResult {
    /// Number of deliveries enqueued for replay.
    #[serde(default)]
    pub deliveries: i64,
}

/// Result of a DLQ purge operation.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PurgeResult {
    /// Number of entries purged.
    #[serde(default)]
    pub purged: i64,
}

/// A custom transformation script applied to events.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Transformation {
    /// Transformation identifier.
    #[serde(default)]
    pub id: String,
    /// Human-readable name.
    #[serde(default)]
    pub name: String,
    /// Free-form description.
    #[serde(default)]
    pub description: String,
    /// Script source.
    #[serde(default)]
    pub code: String,
    /// Whether the transformation is enabled.
    #[serde(default)]
    pub enabled: bool,
    /// Whether failures fall through (vs. dropping the event).
    #[serde(default)]
    pub fail_open: bool,
    /// Execution timeout in milliseconds.
    #[serde(default)]
    pub timeout_ms: i32,
    /// Creation timestamp.
    #[serde(default)]
    pub created_at: String,
    /// Update timestamp.
    #[serde(default)]
    pub updated_at: String,
}

/// Result of testing a transformation against a sample payload.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransformTestResult {
    /// Whether the transformation executed successfully.
    #[serde(default)]
    pub success: bool,
    /// Resulting payload, if successful.
    #[serde(default)]
    pub output: Option<Value>,
    /// Error message, if not successful.
    #[serde(default)]
    pub error: String,
    /// Execution duration in milliseconds.
    #[serde(default)]
    pub duration_ms: i32,
}

/// A short-lived token granting access to the management portal.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PortalToken {
    /// Opaque token value.
    #[serde(default)]
    pub token: String,
    /// Token expiry timestamp.
    #[serde(default)]
    pub expires_at: String,
}

/// An organization (tenant) within Hivehook.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
    /// Organization identifier.
    #[serde(default)]
    pub id: String,
    /// Display name.
    #[serde(default)]
    pub name: String,
    /// URL-safe slug.
    #[serde(default)]
    pub slug: String,
    /// Whether SSO is enabled.
    #[serde(default)]
    pub sso_enabled: bool,
    /// SSO provider name, if any.
    #[serde(default)]
    pub sso_provider: Option<String>,
    /// Event retention in days.
    #[serde(default)]
    pub retention_events: i32,
    /// Message retention in days.
    #[serde(default)]
    pub retention_messages: i32,
    /// OpenTelemetry configuration.
    #[serde(default)]
    pub otlp_config: Option<OtlpConfig>,
    /// Creation timestamp.
    #[serde(default)]
    pub created_at: String,
    /// Update timestamp.
    #[serde(default)]
    pub updated_at: String,
}

/// A user within an organization.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    /// User identifier.
    #[serde(default)]
    pub id: String,
    /// Owning organization.
    #[serde(default)]
    pub organization_id: String,
    /// Email address.
    #[serde(default)]
    pub email: String,
    /// Display name.
    #[serde(default)]
    pub name: String,
    /// Role (`admin`, `member`, ...).
    #[serde(default)]
    pub role: String,
    /// Last-login timestamp.
    #[serde(default)]
    pub last_login_at: Option<String>,
    /// Creation timestamp.
    #[serde(default)]
    pub created_at: String,
    /// Update timestamp.
    #[serde(default)]
    pub updated_at: String,
}

/// An audit log entry recording a state change in the system.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditLog {
    /// Audit entry identifier.
    #[serde(default)]
    pub id: String,
    /// Type of actor (`user`, `api_key`, `system`).
    #[serde(default)]
    pub actor_type: String,
    /// Actor identifier.
    #[serde(default)]
    pub actor_id: String,
    /// Cached actor display name.
    #[serde(default)]
    pub actor_name: String,
    /// Action performed.
    #[serde(default)]
    pub action: String,
    /// Resource type affected.
    #[serde(default)]
    pub resource_type: String,
    /// Resource identifier affected.
    #[serde(default)]
    pub resource_id: String,
    /// Owning organization.
    #[serde(default)]
    pub org_id: String,
    /// Source IP address of the actor.
    #[serde(default)]
    pub ip_address: String,
    /// User-agent string.
    #[serde(default)]
    pub user_agent: String,
    /// Free-form structured details payload.
    #[serde(default)]
    pub details: Option<Value>,
    /// Creation timestamp.
    #[serde(default)]
    pub created_at: String,
}

// ---------------------------------------------------------------------------
// Streams
// ---------------------------------------------------------------------------

/// A persisted stream of events emitted by an application.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Stream {
    /// Stream identifier.
    #[serde(default)]
    pub id: String,
    /// Owning application.
    #[serde(default)]
    pub application_id: String,
    /// Stream name.
    #[serde(default)]
    pub name: String,
    /// Status (`active`, `paused`, ...).
    #[serde(default)]
    pub status: String,
    /// Retention in days.
    #[serde(default)]
    pub retention_days: i32,
    /// Creation timestamp.
    #[serde(default)]
    pub created_at: String,
}

/// A consumer cursor over a stream.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamConsumer {
    /// Consumer identifier.
    #[serde(default)]
    pub id: String,
    /// Owning stream.
    #[serde(default)]
    pub stream_id: String,
    /// Consumer name.
    #[serde(default)]
    pub name: String,
    /// Last acknowledged sequence number.
    #[serde(default)]
    pub cursor_sequence: i64,
    /// Creation timestamp.
    #[serde(default)]
    pub created_at: String,
    /// Update timestamp.
    #[serde(default)]
    pub updated_at: String,
}

/// A managed sink that forwards stream events to an external system.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamSink {
    /// Sink identifier.
    #[serde(default)]
    pub id: String,
    /// Owning stream.
    #[serde(default)]
    pub stream_id: String,
    /// Sink name.
    #[serde(default)]
    pub name: String,
    /// Sink type (`s3`, `kafka`, ...).
    #[serde(default)]
    pub sink_type: String,
    /// Sink-specific configuration.
    #[serde(default)]
    pub config: HashMap<String, Value>,
    /// Batch size.
    #[serde(default)]
    pub batch_size: i32,
    /// Flush interval as a Go duration.
    #[serde(default)]
    pub flush_interval: String,
    /// Last-acknowledged sequence number.
    #[serde(default)]
    pub cursor_sequence: i64,
    /// Sink status.
    #[serde(default)]
    pub status: String,
    /// Last flush timestamp.
    #[serde(default)]
    pub last_flushed_at: Option<String>,
    /// Creation timestamp.
    #[serde(default)]
    pub created_at: String,
}

/// A single message persisted in a Stream's log.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamEntry {
    /// Entry identifier.
    #[serde(default)]
    pub id: String,
    /// Owning stream.
    #[serde(default)]
    pub stream_id: String,
    /// Monotonically increasing position in the stream.
    #[serde(default)]
    pub sequence: i64,
    /// Source Message ID, when the entry originated from a published Message.
    #[serde(default)]
    pub message_id: Option<String>,
    /// Event type carried by the entry.
    #[serde(default)]
    pub event_type: String,
    /// Base64-encoded payload.
    #[serde(default)]
    pub payload: String,
    /// Creation timestamp.
    #[serde(default)]
    pub created_at: String,
}

/// A meta-event webhook configuration: forwards events about events
/// (delivery.failed, source.created, etc.) to an external receiver.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaEventConfig {
    /// Config identifier.
    #[serde(default)]
    pub id: String,
    /// Human-readable name.
    #[serde(default)]
    pub name: String,
    /// Receiver URL.
    #[serde(default)]
    pub url: String,
    /// HMAC signing secret returned by the server.
    #[serde(default)]
    pub signing_secret: String,
    /// Meta-event types this config subscribes to.
    #[serde(default)]
    pub event_types: Vec<String>,
    /// Whether the config is currently active.
    #[serde(default)]
    pub enabled: bool,
    /// Creation timestamp.
    #[serde(default)]
    pub created_at: String,
}
