//! Destinations resource: outbound HTTP/SQS/etc. targets.

use crate::resources::_base::{put_opt, vars};
use crate::types::{Delivery, Destination, HealthConfig, ListResult, OAuth2Config, RetryPolicy};
use crate::HivehookError;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

const FRAGMENT: &str = "id name url signingSecret status type typeConfig timeoutMs rateLimitRps retryPolicy { maxAttempts initialDelay maxDelay backoffFactor } headers authType oauth2Config { tokenUrl clientId clientSecret scopes audience } mtlsCert mtlsKey deliveryMode pollApiKeyPrefix pollApiKey ordered blockedDeliveryId healthScore disabledReason healthConfig { windowHours disableBelow } outputFormat createdAt";

const POLL_FRAGMENT: &str = "id eventId subscriptionId destinationId status attempts maxAttempts nextAttemptAt createdAt";

/// Options for the `list` method on the destination service.
#[non_exhaustive]
#[derive(Debug, Default, Clone)]
pub struct ListDestinationsOptions {
    /// Filter by status.
    pub status: Option<String>,
    /// Free-text search.
    pub search: Option<String>,
    /// Offset-based page size.
    pub limit: Option<i32>,
    /// Offset-based page offset.
    pub offset: Option<i32>,
    /// Cursor for cursor-based pagination.
    pub after: Option<String>,
    /// Page size for cursor-based pagination.
    pub first: Option<i32>,
}

/// Input shape for `create` on the destination service.
#[non_exhaustive]
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDestinationInput {
    /// Human-readable name.
    pub name: String,
    /// Target URL.
    pub url: String,
    /// Destination type (`HTTP`, `SQS`, ...).
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<String>,
    /// Type-specific configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_config: Option<Value>,
    /// Timeout per attempt, in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<i32>,
    /// Per-destination RPS limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_rps: Option<i32>,
    /// Static headers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, Value>>,
    /// Retry policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_policy: Option<RetryPolicy>,
    /// Outbound auth type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    /// OAuth2 configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth2_config: Option<OAuth2Config>,
    /// mTLS client certificate (PEM).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtls_cert: Option<String>,
    /// mTLS client private key (PEM).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtls_key: Option<String>,
    /// Delivery mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_mode: Option<String>,
    /// Whether to enforce ordered delivery.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordered: Option<bool>,
    /// Auto-disable thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_config: Option<HealthConfig>,
    /// Output format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
}

/// Input shape for `update` on the destination service.
#[non_exhaustive]
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateDestinationInput {
    /// New name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// New URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// New status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// New type.
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<String>,
    /// New type config.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_config: Option<Value>,
    /// New timeout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<i32>,
    /// New RPS limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_rps: Option<i32>,
    /// New headers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, Value>>,
    /// New retry policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_policy: Option<RetryPolicy>,
    /// New auth type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    /// New OAuth2 configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth2_config: Option<OAuth2Config>,
    /// New mTLS certificate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtls_cert: Option<String>,
    /// New mTLS private key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtls_key: Option<String>,
    /// New delivery mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_mode: Option<String>,
    /// New ordered toggle.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordered: Option<bool>,
    /// New health-config thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_config: Option<HealthConfig>,
    /// New output format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
}

#[derive(Deserialize)]
struct ListData {
    destinations: ListResult<Destination>,
}

#[derive(Deserialize)]
struct GetData {
    destination: Option<Destination>,
}

#[derive(Deserialize)]
struct CreateData {
    #[serde(rename = "createDestination")]
    create_destination: Destination,
}

#[derive(Deserialize)]
struct UpdateData {
    #[serde(rename = "updateDestination")]
    update_destination: Destination,
}

#[derive(Deserialize)]
struct DeleteData {
    #[serde(rename = "deleteDestination")]
    delete_destination: bool,
}

#[derive(Deserialize)]
struct RotateData {
    #[serde(rename = "rotateDestinationSecret")]
    rotate_destination_secret: Destination,
}

#[derive(Deserialize)]
struct PollData {
    #[serde(rename = "pollDeliveries")]
    poll_deliveries: ListResult<Delivery>,
}

#[derive(Deserialize)]
struct AckData {
    #[serde(rename = "ackDeliveries")]
    ack_deliveries: i32,
}

#[derive(Deserialize)]
struct RegeneratePollKeyData {
    #[serde(rename = "regeneratePollApiKey")]
    regenerate_poll_api_key: Destination,
}

#[derive(Deserialize)]
struct SkipDlqData {
    #[serde(rename = "skipDLQEntry")]
    skip_dlq_entry: bool,
}
#[cfg(feature = "blocking")]
/// Blocking variant of the destination service.
pub struct DestinationService<'a> {
    pub(crate) transport: &'a crate::transport::BlockingGraphQLTransport,
}

#[cfg(feature = "blocking")]
impl<'a> DestinationService<'a> {
    /// List destinations.
    pub fn list(
        &self,
        options: ListDestinationsOptions,
    ) -> Result<ListResult<Destination>, HivehookError> {
        let query = format!(
            r#"query($status: DestinationStatus, $search: String, $limit: Int, $offset: Int, $after: String, $first: Int) {{
                destinations(status: $status, search: $search, limit: $limit, offset: $offset, after: $after, first: $first) {{
                    nodes {{ {FRAGMENT} }}
                    pageInfo {{ total limit offset endCursor hasNextPage }}
                }}
            }}"#
        );
        let mut v = vars();
        put_opt(&mut v, "status", options.status);
        put_opt(&mut v, "search", options.search);
        put_opt(&mut v, "limit", options.limit);
        put_opt(&mut v, "offset", options.offset);
        put_opt(&mut v, "after", options.after);
        put_opt(&mut v, "first", options.first);
        let data: ListData = self.transport.execute(&query, Some(v))?;
        Ok(data.destinations)
    }

    /// Get a destination by ID.
    pub fn get(&self, id: &str) -> Result<Option<Destination>, HivehookError> {
        let query = format!("query($id: UUID!) {{ destination(id: $id) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: GetData = self.transport.execute(&query, Some(v))?;
        Ok(data.destination)
    }

    /// Create a new destination.
    pub fn create(
        &self,
        input: CreateDestinationInput,
    ) -> Result<Destination, HivehookError> {
        let query = format!("mutation($input: CreateDestinationInput!) {{ createDestination(input: $input) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: CreateData = self.transport.execute(&query, Some(v))?;
        Ok(data.create_destination)
    }

    /// Update an existing destination.
    pub fn update(
        &self,
        id: &str,
        input: UpdateDestinationInput,
    ) -> Result<Destination, HivehookError> {
        let query = format!("mutation($id: UUID!, $input: UpdateDestinationInput!) {{ updateDestination(id: $id, input: $input) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: UpdateData = self.transport.execute(&query, Some(v))?;
        Ok(data.update_destination)
    }

    /// Delete a destination.
    pub fn delete(&self, id: &str) -> Result<bool, HivehookError> {
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: DeleteData = self.transport.execute(
            "mutation($id: UUID!) { deleteDestination(id: $id) }",
            Some(v),
        )?;
        Ok(data.delete_destination)
    }

    /// Rotate the signing secret.
    pub fn rotate_secret(&self, id: &str) -> Result<Destination, HivehookError> {
        let query = format!("mutation($id: UUID!) {{ rotateDestinationSecret(id: $id) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: RotateData = self.transport.execute(&query, Some(v))?;
        Ok(data.rotate_destination_secret)
    }

    /// Poll the destination's outstanding deliveries.
    pub fn poll_deliveries(
        &self,
        destination_id: &str,
        cursor: Option<String>,
        limit: Option<i32>,
    ) -> Result<ListResult<Delivery>, HivehookError> {
        let query = format!(
            r#"query($destinationId: UUID!, $cursor: String, $limit: Int) {{
                pollDeliveries(destinationId: $destinationId, cursor: $cursor, limit: $limit) {{
                    nodes {{ {POLL_FRAGMENT} }}
                    pageInfo {{ total limit offset endCursor hasNextPage }}
                }}
            }}"#
        );
        let mut v = vars();
        v.insert("destinationId".into(), Value::String(destination_id.into()));
        put_opt(&mut v, "cursor", cursor);
        put_opt(&mut v, "limit", limit);
        let data: PollData = self.transport.execute(&query, Some(v))?;
        Ok(data.poll_deliveries)
    }

    /// Acknowledge polled deliveries.
    pub fn ack_deliveries(
        &self,
        destination_id: &str,
        delivery_ids: &[String],
    ) -> Result<i32, HivehookError> {
        let mut v = vars();
        v.insert("destinationId".into(), Value::String(destination_id.into()));
        v.insert("deliveryIds".into(), serde_json::to_value(delivery_ids)?);
        let data: AckData = self.transport.execute(
            "mutation($destinationId: UUID!, $deliveryIds: [UUID!]!) { ackDeliveries(destinationId: $destinationId, deliveryIds: $deliveryIds) }",
            Some(v),
        )?;
        Ok(data.ack_deliveries)
    }

    /// Regenerate the poll API key.
    pub fn regenerate_poll_api_key(
        &self,
        destination_id: &str,
    ) -> Result<Destination, HivehookError> {
        let query = format!("mutation($destinationId: UUID!) {{ regeneratePollApiKey(destinationId: $destinationId) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("destinationId".into(), Value::String(destination_id.into()));
        let data: RegeneratePollKeyData = self.transport.execute(&query, Some(v))?;
        Ok(data.regenerate_poll_api_key)
    }

    /// Skip an inbound DLQ entry (no replay).
    pub fn skip_dlq_entry(&self, id: &str) -> Result<bool, HivehookError> {
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: SkipDlqData = self
            .transport
            .execute("mutation($id: UUID!) { skipDLQEntry(id: $id) }", Some(v))?;
        Ok(data.skip_dlq_entry)
    }
}

/// Async variant of the destination service.
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub struct AsyncDestinationService<'a> {
    pub(crate) transport: &'a crate::transport::AsyncGraphQLTransport,
}

#[cfg(feature = "async")]
impl<'a> AsyncDestinationService<'a> {
    /// List destinations.
    pub async fn list(
        &self,
        options: ListDestinationsOptions,
    ) -> Result<ListResult<Destination>, HivehookError> {
        let query = format!(
            r#"query($status: DestinationStatus, $search: String, $limit: Int, $offset: Int, $after: String, $first: Int) {{
                destinations(status: $status, search: $search, limit: $limit, offset: $offset, after: $after, first: $first) {{
                    nodes {{ {FRAGMENT} }}
                    pageInfo {{ total limit offset endCursor hasNextPage }}
                }}
            }}"#
        );
        let mut v = vars();
        put_opt(&mut v, "status", options.status);
        put_opt(&mut v, "search", options.search);
        put_opt(&mut v, "limit", options.limit);
        put_opt(&mut v, "offset", options.offset);
        put_opt(&mut v, "after", options.after);
        put_opt(&mut v, "first", options.first);
        let data: ListData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.destinations)
    }

    /// Get a destination by ID.
    pub async fn get(&self, id: &str) -> Result<Option<Destination>, HivehookError> {
        let query = format!("query($id: UUID!) {{ destination(id: $id) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: GetData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.destination)
    }

    /// Create a new destination.
    pub async fn create(
        &self,
        input: CreateDestinationInput,
    ) -> Result<Destination, HivehookError> {
        let query = format!("mutation($input: CreateDestinationInput!) {{ createDestination(input: $input) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: CreateData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.create_destination)
    }

    /// Update an existing destination.
    pub async fn update(
        &self,
        id: &str,
        input: UpdateDestinationInput,
    ) -> Result<Destination, HivehookError> {
        let query = format!("mutation($id: UUID!, $input: UpdateDestinationInput!) {{ updateDestination(id: $id, input: $input) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: UpdateData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.update_destination)
    }

    /// Delete a destination.
    pub async fn delete(&self, id: &str) -> Result<bool, HivehookError> {
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: DeleteData = self
            .transport
            .execute(
                "mutation($id: UUID!) { deleteDestination(id: $id) }",
                Some(v),
            )
            .await?;
        Ok(data.delete_destination)
    }

    /// Rotate the signing secret.
    pub async fn rotate_secret(&self, id: &str) -> Result<Destination, HivehookError> {
        let query = format!("mutation($id: UUID!) {{ rotateDestinationSecret(id: $id) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: RotateData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.rotate_destination_secret)
    }

    /// Poll the destination's outstanding deliveries.
    pub async fn poll_deliveries(
        &self,
        destination_id: &str,
        cursor: Option<String>,
        limit: Option<i32>,
    ) -> Result<ListResult<Delivery>, HivehookError> {
        let query = format!(
            r#"query($destinationId: UUID!, $cursor: String, $limit: Int) {{
                pollDeliveries(destinationId: $destinationId, cursor: $cursor, limit: $limit) {{
                    nodes {{ {POLL_FRAGMENT} }}
                    pageInfo {{ total limit offset endCursor hasNextPage }}
                }}
            }}"#
        );
        let mut v = vars();
        v.insert("destinationId".into(), Value::String(destination_id.into()));
        put_opt(&mut v, "cursor", cursor);
        put_opt(&mut v, "limit", limit);
        let data: PollData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.poll_deliveries)
    }

    /// Acknowledge polled deliveries.
    pub async fn ack_deliveries(
        &self,
        destination_id: &str,
        delivery_ids: &[String],
    ) -> Result<i32, HivehookError> {
        let mut v = vars();
        v.insert("destinationId".into(), Value::String(destination_id.into()));
        v.insert("deliveryIds".into(), serde_json::to_value(delivery_ids)?);
        let data: AckData = self.transport.execute(
            "mutation($destinationId: UUID!, $deliveryIds: [UUID!]!) { ackDeliveries(destinationId: $destinationId, deliveryIds: $deliveryIds) }",
            Some(v),
        ).await?;
        Ok(data.ack_deliveries)
    }

    /// Regenerate the poll API key.
    pub async fn regenerate_poll_api_key(
        &self,
        destination_id: &str,
    ) -> Result<Destination, HivehookError> {
        let query = format!("mutation($destinationId: UUID!) {{ regeneratePollApiKey(destinationId: $destinationId) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("destinationId".into(), Value::String(destination_id.into()));
        let data: RegeneratePollKeyData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.regenerate_poll_api_key)
    }

    /// Skip an inbound DLQ entry (no replay).
    pub async fn skip_dlq_entry(&self, id: &str) -> Result<bool, HivehookError> {
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: SkipDlqData = self
            .transport
            .execute("mutation($id: UUID!) { skipDLQEntry(id: $id) }", Some(v))
            .await?;
        Ok(data.skip_dlq_entry)
    }
}
