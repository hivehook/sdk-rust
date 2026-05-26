//! Outbound endpoint management.

use crate::errors::HivehookError;
use crate::resources::_base::{put_opt, vars};
#[cfg(feature = "blocking")]
use crate::transport::BlockingGraphQLTransport;
use crate::types::{
    Endpoint, FilterConfig, HealthConfig, ListResult, OAuth2Config, OutboundDelivery, RetryPolicy,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

const FRAGMENT: &str = "id applicationId url signingSecret status type typeConfig rateLimitRps timeoutMs retryPolicy { maxAttempts initialDelay maxDelay backoffFactor } filterConfig { eventTypes regex bodyMatch { path value operator } rules { path operator value rules { path operator value } } } headers authType oauth2Config { tokenUrl clientId clientSecret scopes audience } mtlsCert mtlsKey deliveryMode pollApiKeyPrefix pollApiKey ordered blockedDeliveryId healthScore disabledReason healthConfig { windowHours disableBelow } outputFormat createdAt";

const POLL_FRAGMENT: &str = "id messageId endpointId status attempts maxAttempts nextAttemptAt createdAt";

/// Options for [`EndpointService::list`].
#[non_exhaustive]
#[derive(Debug, Default, Clone)]
pub struct ListEndpointsOptions {
    /// Filter by application ID.
    pub application_id: Option<String>,
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

/// Input shape for `EndpointService::create`.
#[non_exhaustive]
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateEndpointInput {
    /// Owning application.
    pub application_id: String,
    /// Target URL.
    pub url: String,
    /// Endpoint type.
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<String>,
    /// Type-specific configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_config: Option<Value>,
    /// Timeout per attempt, in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<i32>,
    /// Per-endpoint RPS limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_rps: Option<i32>,
    /// Static headers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, Value>>,
    /// Filter configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_config: Option<FilterConfig>,
    /// Retry policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_policy: Option<RetryPolicy>,
    /// Outbound auth type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    /// OAuth2 configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth2_config: Option<OAuth2Config>,
    /// mTLS certificate (PEM).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtls_cert: Option<String>,
    /// mTLS private key (PEM).
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

/// Input shape for `EndpointService::update`.
#[non_exhaustive]
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateEndpointInput {
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
    /// New filter config.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_config: Option<FilterConfig>,
    /// New retry policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_policy: Option<RetryPolicy>,
    /// New auth type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    /// New OAuth2 config.
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
    /// New auto-disable thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_config: Option<HealthConfig>,
    /// New output format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
}

#[derive(Deserialize)]
struct ListData {
    endpoints: ListResult<Endpoint>,
}

#[derive(Deserialize)]
struct GetData {
    endpoint: Option<Endpoint>,
}

#[derive(Deserialize)]
struct CreateData {
    #[serde(rename = "createEndpoint")]
    create_endpoint: Endpoint,
}

#[derive(Deserialize)]
struct UpdateData {
    #[serde(rename = "updateEndpoint")]
    update_endpoint: Endpoint,
}

#[derive(Deserialize)]
struct DeleteData {
    #[serde(rename = "deleteEndpoint")]
    delete_endpoint: bool,
}

#[derive(Deserialize)]
struct RotateData {
    #[serde(rename = "rotateEndpointSecret")]
    rotate_endpoint_secret: Endpoint,
}

#[derive(Deserialize)]
struct PollData {
    #[serde(rename = "pollOutboundDeliveries")]
    poll_outbound_deliveries: ListResult<OutboundDelivery>,
}

#[derive(Deserialize)]
struct AckData {
    #[serde(rename = "ackOutboundDeliveries")]
    ack_outbound_deliveries: i32,
}

#[derive(Deserialize)]
struct RegenerateData {
    #[serde(rename = "regenerateOutboundPollApiKey")]
    regenerate_outbound_poll_api_key: Endpoint,
}

#[derive(Deserialize)]
struct SkipData {
    #[serde(rename = "skipOutboundDlqEntry")]
    skip_outbound_dlq_entry: bool,
}

#[cfg(feature = "blocking")]
/// Service for managing [`Endpoint`] resources.
pub struct EndpointService<'a> {
    pub(crate) transport: &'a BlockingGraphQLTransport,
}

#[cfg(feature = "blocking")]
impl<'a> EndpointService<'a> {
    /// List endpoints.
    pub fn list(
        &self,
        options: ListEndpointsOptions,
    ) -> Result<ListResult<Endpoint>, HivehookError> {
        let query = format!(
            r#"query($applicationId: UUID, $status: EndpointStatus, $search: String, $limit: Int, $offset: Int, $after: String, $first: Int) {{
                endpoints(applicationId: $applicationId, status: $status, search: $search, limit: $limit, offset: $offset, after: $after, first: $first) {{
                    nodes {{ {FRAGMENT} }}
                    pageInfo {{ total limit offset endCursor hasNextPage }}
                }}
            }}"#
        );
        let mut v = vars();
        put_opt(&mut v, "applicationId", options.application_id);
        put_opt(&mut v, "status", options.status);
        put_opt(&mut v, "search", options.search);
        put_opt(&mut v, "limit", options.limit);
        put_opt(&mut v, "offset", options.offset);
        put_opt(&mut v, "after", options.after);
        put_opt(&mut v, "first", options.first);
        let data: ListData = self.transport.execute(&query, Some(v))?;
        Ok(data.endpoints)
    }

    /// Get an endpoint by ID.
    pub fn get(&self, id: &str) -> Result<Option<Endpoint>, HivehookError> {
        let query = format!("query($id: UUID!) {{ endpoint(id: $id) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: GetData = self.transport.execute(&query, Some(v))?;
        Ok(data.endpoint)
    }

    /// Create a new endpoint.
    pub fn create(&self, input: CreateEndpointInput) -> Result<Endpoint, HivehookError> {
        let query = format!("mutation($input: CreateEndpointInput!) {{ createEndpoint(input: $input) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: CreateData = self.transport.execute(&query, Some(v))?;
        Ok(data.create_endpoint)
    }

    /// Update an endpoint.
    pub fn update(
        &self,
        id: &str,
        input: UpdateEndpointInput,
    ) -> Result<Endpoint, HivehookError> {
        let query = format!("mutation($id: UUID!, $input: UpdateEndpointInput!) {{ updateEndpoint(id: $id, input: $input) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: UpdateData = self.transport.execute(&query, Some(v))?;
        Ok(data.update_endpoint)
    }

    /// Delete an endpoint.
    pub fn delete(&self, id: &str) -> Result<bool, HivehookError> {
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: DeleteData = self
            .transport
            .execute("mutation($id: UUID!) { deleteEndpoint(id: $id) }", Some(v))?;
        Ok(data.delete_endpoint)
    }

    /// Rotate the endpoint's signing secret.
    pub fn rotate_secret(&self, id: &str) -> Result<Endpoint, HivehookError> {
        let query = format!("mutation($id: UUID!) {{ rotateEndpointSecret(id: $id) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: RotateData = self.transport.execute(&query, Some(v))?;
        Ok(data.rotate_endpoint_secret)
    }

    /// Poll the endpoint for outstanding outbound deliveries.
    pub fn poll_deliveries(
        &self,
        endpoint_id: &str,
        cursor: Option<String>,
        limit: Option<i32>,
    ) -> Result<ListResult<OutboundDelivery>, HivehookError> {
        let query = format!(
            r#"query($endpointId: UUID!, $cursor: String, $limit: Int) {{
                pollOutboundDeliveries(endpointId: $endpointId, cursor: $cursor, limit: $limit) {{
                    nodes {{ {POLL_FRAGMENT} }}
                    pageInfo {{ total limit offset endCursor hasNextPage }}
                }}
            }}"#
        );
        let mut v = vars();
        v.insert("endpointId".into(), Value::String(endpoint_id.into()));
        put_opt(&mut v, "cursor", cursor);
        put_opt(&mut v, "limit", limit);
        let data: PollData = self.transport.execute(&query, Some(v))?;
        Ok(data.poll_outbound_deliveries)
    }

    /// Acknowledge polled deliveries. Returns the number of acknowledged
    /// deliveries.
    pub fn ack_deliveries(
        &self,
        endpoint_id: &str,
        delivery_ids: &[String],
    ) -> Result<i32, HivehookError> {
        let mut v = vars();
        v.insert("endpointId".into(), Value::String(endpoint_id.into()));
        v.insert("deliveryIds".into(), serde_json::to_value(delivery_ids)?);
        let data: AckData = self.transport.execute(
            "mutation($endpointId: UUID!, $deliveryIds: [UUID!]!) { ackOutboundDeliveries(endpointId: $endpointId, deliveryIds: $deliveryIds) }",
            Some(v),
        )?;
        Ok(data.ack_outbound_deliveries)
    }

    /// Regenerate the poll API key for this endpoint.
    pub fn regenerate_poll_api_key(&self, endpoint_id: &str) -> Result<Endpoint, HivehookError> {
        let query = format!("mutation($endpointId: UUID!) {{ regenerateOutboundPollApiKey(endpointId: $endpointId) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("endpointId".into(), Value::String(endpoint_id.into()));
        let data: RegenerateData = self.transport.execute(&query, Some(v))?;
        Ok(data.regenerate_outbound_poll_api_key)
    }

    /// Skip an outbound DLQ entry without replay.
    pub fn skip_outbound_dlq_entry(&self, id: &str) -> Result<bool, HivehookError> {
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: SkipData = self.transport.execute(
            "mutation($id: UUID!) { skipOutboundDlqEntry(id: $id) }",
            Some(v),
        )?;
        Ok(data.skip_outbound_dlq_entry)
    }
}

/// Async variant of the endpoint service.
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub struct AsyncEndpointService<'a> {
    pub(crate) transport: &'a crate::transport::AsyncGraphQLTransport,
}

#[cfg(feature = "async")]
impl<'a> AsyncEndpointService<'a> {
    /// List endpoints.
    pub async fn list(
        &self,
        options: ListEndpointsOptions,
    ) -> Result<ListResult<Endpoint>, HivehookError> {
        let query = format!(
            r#"query($applicationId: UUID, $status: EndpointStatus, $search: String, $limit: Int, $offset: Int, $after: String, $first: Int) {{
                endpoints(applicationId: $applicationId, status: $status, search: $search, limit: $limit, offset: $offset, after: $after, first: $first) {{
                    nodes {{ {FRAGMENT} }}
                    pageInfo {{ total limit offset endCursor hasNextPage }}
                }}
            }}"#
        );
        let mut v = vars();
        put_opt(&mut v, "applicationId", options.application_id);
        put_opt(&mut v, "status", options.status);
        put_opt(&mut v, "search", options.search);
        put_opt(&mut v, "limit", options.limit);
        put_opt(&mut v, "offset", options.offset);
        put_opt(&mut v, "after", options.after);
        put_opt(&mut v, "first", options.first);
        let data: ListData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.endpoints)
    }

    /// Get an endpoint by ID.
    pub async fn get(&self, id: &str) -> Result<Option<Endpoint>, HivehookError> {
        let query = format!("query($id: UUID!) {{ endpoint(id: $id) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: GetData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.endpoint)
    }

    /// Create a new endpoint.
    pub async fn create(&self, input: CreateEndpointInput) -> Result<Endpoint, HivehookError> {
        let query = format!("mutation($input: CreateEndpointInput!) {{ createEndpoint(input: $input) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: CreateData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.create_endpoint)
    }

    /// Update an endpoint.
    pub async fn update(
        &self,
        id: &str,
        input: UpdateEndpointInput,
    ) -> Result<Endpoint, HivehookError> {
        let query = format!("mutation($id: UUID!, $input: UpdateEndpointInput!) {{ updateEndpoint(id: $id, input: $input) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: UpdateData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.update_endpoint)
    }

    /// Delete an endpoint.
    pub async fn delete(&self, id: &str) -> Result<bool, HivehookError> {
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: DeleteData = self
            .transport
            .execute("mutation($id: UUID!) { deleteEndpoint(id: $id) }", Some(v))
            .await?;
        Ok(data.delete_endpoint)
    }

    /// Rotate the endpoint's signing secret.
    pub async fn rotate_secret(&self, id: &str) -> Result<Endpoint, HivehookError> {
        let query = format!("mutation($id: UUID!) {{ rotateEndpointSecret(id: $id) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: RotateData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.rotate_endpoint_secret)
    }

    /// Poll the endpoint for outstanding outbound deliveries.
    pub async fn poll_deliveries(
        &self,
        endpoint_id: &str,
        cursor: Option<String>,
        limit: Option<i32>,
    ) -> Result<ListResult<OutboundDelivery>, HivehookError> {
        let query = format!(
            r#"query($endpointId: UUID!, $cursor: String, $limit: Int) {{
                pollOutboundDeliveries(endpointId: $endpointId, cursor: $cursor, limit: $limit) {{
                    nodes {{ {POLL_FRAGMENT} }}
                    pageInfo {{ total limit offset endCursor hasNextPage }}
                }}
            }}"#
        );
        let mut v = vars();
        v.insert("endpointId".into(), Value::String(endpoint_id.into()));
        put_opt(&mut v, "cursor", cursor);
        put_opt(&mut v, "limit", limit);
        let data: PollData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.poll_outbound_deliveries)
    }

    /// Acknowledge polled deliveries. Returns the number of acknowledged
    /// deliveries.
    pub async fn ack_deliveries(
        &self,
        endpoint_id: &str,
        delivery_ids: &[String],
    ) -> Result<i32, HivehookError> {
        let mut v = vars();
        v.insert("endpointId".into(), Value::String(endpoint_id.into()));
        v.insert("deliveryIds".into(), serde_json::to_value(delivery_ids)?);
        let data: AckData = self.transport.execute(
            "mutation($endpointId: UUID!, $deliveryIds: [UUID!]!) { ackOutboundDeliveries(endpointId: $endpointId, deliveryIds: $deliveryIds) }",
            Some(v),
        ).await?;
        Ok(data.ack_outbound_deliveries)
    }

    /// Regenerate the poll API key for this endpoint.
    pub async fn regenerate_poll_api_key(
        &self,
        endpoint_id: &str,
    ) -> Result<Endpoint, HivehookError> {
        let query = format!("mutation($endpointId: UUID!) {{ regenerateOutboundPollApiKey(endpointId: $endpointId) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("endpointId".into(), Value::String(endpoint_id.into()));
        let data: RegenerateData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.regenerate_outbound_poll_api_key)
    }

    /// Skip an outbound DLQ entry without replay.
    pub async fn skip_outbound_dlq_entry(&self, id: &str) -> Result<bool, HivehookError> {
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: SkipData = self
            .transport
            .execute(
                "mutation($id: UUID!) { skipOutboundDlqEntry(id: $id) }",
                Some(v),
            )
            .await?;
        Ok(data.skip_outbound_dlq_entry)
    }
}
