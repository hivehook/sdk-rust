//! Stream sinks: forward stream events to external systems.

use crate::errors::HivehookError;
use crate::resources::_base::{put_opt, vars};
#[cfg(feature = "blocking")]
use crate::transport::BlockingGraphQLTransport;
use crate::types::{ListResult, StreamSink};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

const FRAGMENT: &str = "id streamId name sinkType config batchSize flushInterval cursorSequence status lastFlushedAt createdAt";

/// Options for [`StreamSinkService::list`].
#[non_exhaustive]
#[derive(Debug, Default, Clone)]
pub struct ListStreamSinksOptions {
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

/// Input shape for `StreamSinkService::create`.
#[non_exhaustive]
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateStreamSinkInput {
    /// Owning stream.
    pub stream_id: String,
    /// Sink name.
    pub name: String,
    /// Sink type.
    pub sink_type: String,
    /// Sink-specific configuration.
    pub config: HashMap<String, Value>,
    /// Optional batch size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<i32>,
    /// Optional flush interval (Go duration).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flush_interval: Option<String>,
}

/// Input shape for `StreamSinkService::update`.
#[non_exhaustive]
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateStreamSinkInput {
    /// New name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// New config.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<HashMap<String, Value>>,
    /// New batch size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<i32>,
    /// New flush interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flush_interval: Option<String>,
    /// New status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Deserialize)]
struct ListData {
    #[serde(rename = "streamSinks")]
    stream_sinks: ListResult<StreamSink>,
}

#[derive(Deserialize)]
struct GetData {
    #[serde(rename = "streamSink")]
    stream_sink: Option<StreamSink>,
}

#[derive(Deserialize)]
struct CreateData {
    #[serde(rename = "createStreamSink")]
    create_stream_sink: StreamSink,
}

#[derive(Deserialize)]
struct UpdateData {
    #[serde(rename = "updateStreamSink")]
    update_stream_sink: StreamSink,
}

#[derive(Deserialize)]
struct DeleteData {
    #[serde(rename = "deleteStreamSink")]
    delete_stream_sink: bool,
}

#[cfg(feature = "blocking")]
/// Service for managing [`StreamSink`] resources.
pub struct StreamSinkService<'a> {
    pub(crate) transport: &'a BlockingGraphQLTransport,
}

#[cfg(feature = "blocking")]
impl<'a> StreamSinkService<'a> {
    /// List sinks attached to `stream_id`.
    pub fn list(
        &self,
        stream_id: &str,
        options: ListStreamSinksOptions,
    ) -> Result<ListResult<StreamSink>, HivehookError> {
        let query = format!(
            r#"query($streamId: UUID!, $status: SinkStatus, $search: String, $limit: Int, $offset: Int, $after: String, $first: Int) {{
                streamSinks(streamId: $streamId, status: $status, search: $search, limit: $limit, offset: $offset, after: $after, first: $first) {{
                    nodes {{ {FRAGMENT} }}
                    pageInfo {{ total limit offset endCursor hasNextPage }}
                }}
            }}"#
        );
        let mut v = vars();
        v.insert("streamId".into(), Value::String(stream_id.into()));
        put_opt(&mut v, "status", options.status);
        put_opt(&mut v, "search", options.search);
        put_opt(&mut v, "limit", options.limit);
        put_opt(&mut v, "offset", options.offset);
        put_opt(&mut v, "after", options.after);
        put_opt(&mut v, "first", options.first);
        let data: ListData = self.transport.execute(&query, Some(v))?;
        Ok(data.stream_sinks)
    }

    /// Get a sink by ID.
    pub fn get(&self, id: &str) -> Result<Option<StreamSink>, HivehookError> {
        let query = format!("query($id: UUID!) {{ streamSink(id: $id) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: GetData = self.transport.execute(&query, Some(v))?;
        Ok(data.stream_sink)
    }

    /// Create a new sink.
    pub fn create(&self, input: CreateStreamSinkInput) -> Result<StreamSink, HivehookError> {
        let query = format!("mutation($input: CreateStreamSinkInput!) {{ createStreamSink(input: $input) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: CreateData = self.transport.execute(&query, Some(v))?;
        Ok(data.create_stream_sink)
    }

    /// Update a sink.
    pub fn update(
        &self,
        id: &str,
        input: UpdateStreamSinkInput,
    ) -> Result<StreamSink, HivehookError> {
        let query = format!("mutation($id: UUID!, $input: UpdateStreamSinkInput!) {{ updateStreamSink(id: $id, input: $input) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: UpdateData = self.transport.execute(&query, Some(v))?;
        Ok(data.update_stream_sink)
    }

    /// Delete a sink.
    pub fn delete(&self, id: &str) -> Result<bool, HivehookError> {
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: DeleteData = self.transport.execute(
            "mutation($id: UUID!) { deleteStreamSink(id: $id) }",
            Some(v),
        )?;
        Ok(data.delete_stream_sink)
    }
}

/// Async variant of the stream-sink service.
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub struct AsyncStreamSinkService<'a> {
    pub(crate) transport: &'a crate::transport::AsyncGraphQLTransport,
}

#[cfg(feature = "async")]
impl<'a> AsyncStreamSinkService<'a> {
    /// List sinks attached to `stream_id`.
    pub async fn list(
        &self,
        stream_id: &str,
        options: ListStreamSinksOptions,
    ) -> Result<ListResult<StreamSink>, HivehookError> {
        let query = format!(
            r#"query($streamId: UUID!, $status: SinkStatus, $search: String, $limit: Int, $offset: Int, $after: String, $first: Int) {{
                streamSinks(streamId: $streamId, status: $status, search: $search, limit: $limit, offset: $offset, after: $after, first: $first) {{
                    nodes {{ {FRAGMENT} }}
                    pageInfo {{ total limit offset endCursor hasNextPage }}
                }}
            }}"#
        );
        let mut v = vars();
        v.insert("streamId".into(), Value::String(stream_id.into()));
        put_opt(&mut v, "status", options.status);
        put_opt(&mut v, "search", options.search);
        put_opt(&mut v, "limit", options.limit);
        put_opt(&mut v, "offset", options.offset);
        put_opt(&mut v, "after", options.after);
        put_opt(&mut v, "first", options.first);
        let data: ListData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.stream_sinks)
    }

    /// Get a sink by ID.
    pub async fn get(&self, id: &str) -> Result<Option<StreamSink>, HivehookError> {
        let query = format!("query($id: UUID!) {{ streamSink(id: $id) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: GetData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.stream_sink)
    }

    /// Create a new sink.
    pub async fn create(&self, input: CreateStreamSinkInput) -> Result<StreamSink, HivehookError> {
        let query = format!("mutation($input: CreateStreamSinkInput!) {{ createStreamSink(input: $input) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: CreateData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.create_stream_sink)
    }

    /// Update a sink.
    pub async fn update(
        &self,
        id: &str,
        input: UpdateStreamSinkInput,
    ) -> Result<StreamSink, HivehookError> {
        let query = format!("mutation($id: UUID!, $input: UpdateStreamSinkInput!) {{ updateStreamSink(id: $id, input: $input) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: UpdateData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.update_stream_sink)
    }

    /// Delete a sink.
    pub async fn delete(&self, id: &str) -> Result<bool, HivehookError> {
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: DeleteData = self
            .transport
            .execute(
                "mutation($id: UUID!) { deleteStreamSink(id: $id) }",
                Some(v),
            )
            .await?;
        Ok(data.delete_stream_sink)
    }
}
