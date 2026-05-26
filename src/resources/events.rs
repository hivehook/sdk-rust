//! Events resource: received inbound events.

use crate::resources::_base::{put_opt, vars};
use crate::types::{Event, ListResult};
use crate::HivehookError;
use serde::Deserialize;
use serde_json::Value;

const FRAGMENT: &str =
    "id sourceId idempotencyKey eventType headers rawBody status receivedAt";

/// Options for the `list` method on the event service.
#[non_exhaustive]
#[derive(Debug, Default, Clone)]
pub struct ListEventsOptions {
    /// Filter by source ID.
    pub source_id: Option<String>,
    /// Filter by event type.
    pub event_type: Option<String>,
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

#[derive(Deserialize)]
struct ListData {
    events: ListResult<Event>,
}

#[derive(Deserialize)]
struct GetData {
    event: Option<Event>,
}
#[cfg(feature = "blocking")]
/// Blocking variant of the event service.
pub struct EventService<'a> {
    pub(crate) transport: &'a crate::transport::BlockingGraphQLTransport,
}

#[cfg(feature = "blocking")]
impl<'a> EventService<'a> {
    /// List events.
    pub fn list(
        &self,
        options: ListEventsOptions,
    ) -> Result<ListResult<Event>, HivehookError> {
        let query = format!(
            r#"query($sourceId: UUID, $eventType: String, $status: EventStatus, $search: String, $limit: Int, $offset: Int, $after: String, $first: Int) {{
                events(sourceId: $sourceId, eventType: $eventType, status: $status, search: $search, limit: $limit, offset: $offset, after: $after, first: $first) {{
                    nodes {{ {FRAGMENT} }}
                    pageInfo {{ total limit offset endCursor hasNextPage }}
                }}
            }}"#
        );
        let mut v = vars();
        put_opt(&mut v, "sourceId", options.source_id);
        put_opt(&mut v, "eventType", options.event_type);
        put_opt(&mut v, "status", options.status);
        put_opt(&mut v, "search", options.search);
        put_opt(&mut v, "limit", options.limit);
        put_opt(&mut v, "offset", options.offset);
        put_opt(&mut v, "after", options.after);
        put_opt(&mut v, "first", options.first);
        let data: ListData = self.transport.execute(&query, Some(v))?;
        Ok(data.events)
    }

    /// Get an event by ID.
    pub fn get(&self, id: &str) -> Result<Option<Event>, HivehookError> {
        let query = format!("query($id: UUID!) {{ event(id: $id) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: GetData = self.transport.execute(&query, Some(v))?;
        Ok(data.event)
    }
}

/// Async variant of the event service.
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub struct AsyncEventService<'a> {
    pub(crate) transport: &'a crate::transport::AsyncGraphQLTransport,
}

#[cfg(feature = "async")]
impl<'a> AsyncEventService<'a> {
    /// List events.
    pub async fn list(
        &self,
        options: ListEventsOptions,
    ) -> Result<ListResult<Event>, HivehookError> {
        let query = format!(
            r#"query($sourceId: UUID, $eventType: String, $status: EventStatus, $search: String, $limit: Int, $offset: Int, $after: String, $first: Int) {{
                events(sourceId: $sourceId, eventType: $eventType, status: $status, search: $search, limit: $limit, offset: $offset, after: $after, first: $first) {{
                    nodes {{ {FRAGMENT} }}
                    pageInfo {{ total limit offset endCursor hasNextPage }}
                }}
            }}"#
        );
        let mut v = vars();
        put_opt(&mut v, "sourceId", options.source_id);
        put_opt(&mut v, "eventType", options.event_type);
        put_opt(&mut v, "status", options.status);
        put_opt(&mut v, "search", options.search);
        put_opt(&mut v, "limit", options.limit);
        put_opt(&mut v, "offset", options.offset);
        put_opt(&mut v, "after", options.after);
        put_opt(&mut v, "first", options.first);
        let data: ListData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.events)
    }

    /// Get an event by ID.
    pub async fn get(&self, id: &str) -> Result<Option<Event>, HivehookError> {
        let query = format!("query($id: UUID!) {{ event(id: $id) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: GetData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.event)
    }
}
