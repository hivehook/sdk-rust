//! Outbound delivery service.

use crate::errors::HivehookError;
use crate::resources::_base::{put_opt, vars};
#[cfg(feature = "blocking")]
use crate::transport::BlockingGraphQLTransport;
use crate::types::{ListResult, OutboundDelivery};
use serde::Deserialize;
use serde_json::Value;

const FRAGMENT: &str = "id messageId endpointId status attempts maxAttempts nextAttemptAt createdAt";
const DETAIL_FRAGMENT: &str = "id messageId endpointId status attempts maxAttempts nextAttemptAt createdAt deliveryAttempts { id deliveryId attemptNumber responseStatus responseBody error durationMs attemptedAt }";

/// Options for [`OutboundDeliveryService::list`].
#[non_exhaustive]
#[derive(Debug, Default, Clone)]
pub struct ListOutboundDeliveriesOptions {
    /// Filter by message ID.
    pub message_id: Option<String>,
    /// Filter by endpoint ID.
    pub endpoint_id: Option<String>,
    /// Filter by delivery status.
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

#[derive(Deserialize)]
struct ListData {
    #[serde(rename = "outboundDeliveries")]
    outbound_deliveries: ListResult<OutboundDelivery>,
}

#[derive(Deserialize)]
struct GetData {
    #[serde(rename = "outboundDelivery")]
    outbound_delivery: Option<OutboundDelivery>,
}

#[cfg(feature = "blocking")]
/// Service for reading [`OutboundDelivery`] resources.
pub struct OutboundDeliveryService<'a> {
    pub(crate) transport: &'a BlockingGraphQLTransport,
}

#[cfg(feature = "blocking")]
impl<'a> OutboundDeliveryService<'a> {
    /// List outbound deliveries.
    pub fn list(
        &self,
        options: ListOutboundDeliveriesOptions,
    ) -> Result<ListResult<OutboundDelivery>, HivehookError> {
        let query = format!(
            r#"query($messageId: UUID, $endpointId: UUID, $status: DeliveryStatus, $search: String, $limit: Int, $offset: Int, $after: String, $first: Int) {{
                outboundDeliveries(messageId: $messageId, endpointId: $endpointId, status: $status, search: $search, limit: $limit, offset: $offset, after: $after, first: $first) {{
                    nodes {{ {FRAGMENT} }}
                    pageInfo {{ total limit offset endCursor hasNextPage }}
                }}
            }}"#
        );
        let mut v = vars();
        put_opt(&mut v, "messageId", options.message_id);
        put_opt(&mut v, "endpointId", options.endpoint_id);
        put_opt(&mut v, "status", options.status);
        put_opt(&mut v, "search", options.search);
        put_opt(&mut v, "limit", options.limit);
        put_opt(&mut v, "offset", options.offset);
        put_opt(&mut v, "after", options.after);
        put_opt(&mut v, "first", options.first);
        let data: ListData = self.transport.execute(&query, Some(v))?;
        Ok(data.outbound_deliveries)
    }

    /// Get an outbound delivery by ID, including its detailed attempts.
    pub fn get(&self, id: &str) -> Result<Option<OutboundDelivery>, HivehookError> {
        let query = format!("query($id: UUID!) {{ outboundDelivery(id: $id) {{ {DETAIL_FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: GetData = self.transport.execute(&query, Some(v))?;
        Ok(data.outbound_delivery)
    }
}

/// Async variant of the outbound delivery service.
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub struct AsyncOutboundDeliveryService<'a> {
    pub(crate) transport: &'a crate::transport::AsyncGraphQLTransport,
}

#[cfg(feature = "async")]
impl<'a> AsyncOutboundDeliveryService<'a> {
    /// List outbound deliveries.
    pub async fn list(
        &self,
        options: ListOutboundDeliveriesOptions,
    ) -> Result<ListResult<OutboundDelivery>, HivehookError> {
        let query = format!(
            r#"query($messageId: UUID, $endpointId: UUID, $status: DeliveryStatus, $search: String, $limit: Int, $offset: Int, $after: String, $first: Int) {{
                outboundDeliveries(messageId: $messageId, endpointId: $endpointId, status: $status, search: $search, limit: $limit, offset: $offset, after: $after, first: $first) {{
                    nodes {{ {FRAGMENT} }}
                    pageInfo {{ total limit offset endCursor hasNextPage }}
                }}
            }}"#
        );
        let mut v = vars();
        put_opt(&mut v, "messageId", options.message_id);
        put_opt(&mut v, "endpointId", options.endpoint_id);
        put_opt(&mut v, "status", options.status);
        put_opt(&mut v, "search", options.search);
        put_opt(&mut v, "limit", options.limit);
        put_opt(&mut v, "offset", options.offset);
        put_opt(&mut v, "after", options.after);
        put_opt(&mut v, "first", options.first);
        let data: ListData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.outbound_deliveries)
    }

    /// Get an outbound delivery by ID, including its detailed attempts.
    pub async fn get(&self, id: &str) -> Result<Option<OutboundDelivery>, HivehookError> {
        let query = format!("query($id: UUID!) {{ outboundDelivery(id: $id) {{ {DETAIL_FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: GetData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.outbound_delivery)
    }
}
