//! Deliveries resource: per-destination delivery attempts for inbound events.

use crate::resources::_base::{put_opt, vars};
use crate::types::{Delivery, ListResult};
use crate::HivehookError;
use serde::Deserialize;
use serde_json::Value;

const FRAGMENT: &str = "id eventId subscriptionId destinationId status attempts maxAttempts nextAttemptAt createdAt";

const DETAIL_FRAGMENT: &str = "id eventId subscriptionId destinationId status attempts maxAttempts nextAttemptAt createdAt deliveryAttempts { id deliveryId attemptNumber responseStatus responseBody error durationMs attemptedAt }";

/// Options for the `list` method on the delivery service.
#[non_exhaustive]
#[derive(Debug, Default, Clone)]
pub struct ListDeliveriesOptions {
    /// Filter by event ID.
    pub event_id: Option<String>,
    /// Filter by destination ID.
    pub destination_id: Option<String>,
    /// Filter by subscription ID.
    pub subscription_id: Option<String>,
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
    deliveries: ListResult<Delivery>,
}

#[derive(Deserialize)]
struct GetData {
    delivery: Option<Delivery>,
}
#[cfg(feature = "blocking")]
/// Blocking variant of the delivery service.
pub struct DeliveryService<'a> {
    pub(crate) transport: &'a crate::transport::BlockingGraphQLTransport,
}

#[cfg(feature = "blocking")]
impl<'a> DeliveryService<'a> {
    /// List deliveries.
    pub fn list(
        &self,
        options: ListDeliveriesOptions,
    ) -> Result<ListResult<Delivery>, HivehookError> {
        let query = format!(
            r#"query($eventId: UUID, $destinationId: UUID, $subscriptionId: UUID, $status: DeliveryStatus, $search: String, $limit: Int, $offset: Int, $after: String, $first: Int) {{
                deliveries(eventId: $eventId, destinationId: $destinationId, subscriptionId: $subscriptionId, status: $status, search: $search, limit: $limit, offset: $offset, after: $after, first: $first) {{
                    nodes {{ {FRAGMENT} }}
                    pageInfo {{ total limit offset endCursor hasNextPage }}
                }}
            }}"#
        );
        let mut v = vars();
        put_opt(&mut v, "eventId", options.event_id);
        put_opt(&mut v, "destinationId", options.destination_id);
        put_opt(&mut v, "subscriptionId", options.subscription_id);
        put_opt(&mut v, "status", options.status);
        put_opt(&mut v, "search", options.search);
        put_opt(&mut v, "limit", options.limit);
        put_opt(&mut v, "offset", options.offset);
        put_opt(&mut v, "after", options.after);
        put_opt(&mut v, "first", options.first);
        let data: ListData = self.transport.execute(&query, Some(v))?;
        Ok(data.deliveries)
    }

    /// Get a delivery by ID, including its detailed attempts.
    pub fn get(&self, id: &str) -> Result<Option<Delivery>, HivehookError> {
        let query = format!("query($id: UUID!) {{ delivery(id: $id) {{ {DETAIL_FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: GetData = self.transport.execute(&query, Some(v))?;
        Ok(data.delivery)
    }
}

/// Async variant of the delivery service.
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub struct AsyncDeliveryService<'a> {
    pub(crate) transport: &'a crate::transport::AsyncGraphQLTransport,
}

#[cfg(feature = "async")]
impl<'a> AsyncDeliveryService<'a> {
    /// List deliveries.
    pub async fn list(
        &self,
        options: ListDeliveriesOptions,
    ) -> Result<ListResult<Delivery>, HivehookError> {
        let query = format!(
            r#"query($eventId: UUID, $destinationId: UUID, $subscriptionId: UUID, $status: DeliveryStatus, $search: String, $limit: Int, $offset: Int, $after: String, $first: Int) {{
                deliveries(eventId: $eventId, destinationId: $destinationId, subscriptionId: $subscriptionId, status: $status, search: $search, limit: $limit, offset: $offset, after: $after, first: $first) {{
                    nodes {{ {FRAGMENT} }}
                    pageInfo {{ total limit offset endCursor hasNextPage }}
                }}
            }}"#
        );
        let mut v = vars();
        put_opt(&mut v, "eventId", options.event_id);
        put_opt(&mut v, "destinationId", options.destination_id);
        put_opt(&mut v, "subscriptionId", options.subscription_id);
        put_opt(&mut v, "status", options.status);
        put_opt(&mut v, "search", options.search);
        put_opt(&mut v, "limit", options.limit);
        put_opt(&mut v, "offset", options.offset);
        put_opt(&mut v, "after", options.after);
        put_opt(&mut v, "first", options.first);
        let data: ListData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.deliveries)
    }

    /// Get a delivery by ID, including its detailed attempts.
    pub async fn get(&self, id: &str) -> Result<Option<Delivery>, HivehookError> {
        let query = format!("query($id: UUID!) {{ delivery(id: $id) {{ {DETAIL_FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: GetData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.delivery)
    }
}
