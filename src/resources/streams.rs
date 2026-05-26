//! Streams resource.

use crate::errors::HivehookError;
use crate::resources::_base::{put_opt, vars};
#[cfg(feature = "blocking")]
use crate::transport::BlockingGraphQLTransport;
use crate::types::{ListResult, Stream};
use serde::{Deserialize, Serialize};
use serde_json::Value;

const FRAGMENT: &str = "id applicationId name status retentionDays createdAt";

/// Options for [`StreamService::list`].
#[non_exhaustive]
#[derive(Debug, Default, Clone)]
pub struct ListStreamsOptions {
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

/// Input shape for `StreamService::create`.
#[non_exhaustive]
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateStreamInput {
    /// Owning application.
    pub application_id: String,
    /// Stream name.
    pub name: String,
    /// Retention in days.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<i32>,
    /// Initial status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// Input shape for `StreamService::update`.
#[non_exhaustive]
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateStreamInput {
    /// New name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// New retention.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<i32>,
    /// New status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Deserialize)]
struct ListData {
    streams: ListResult<Stream>,
}

#[derive(Deserialize)]
struct GetData {
    stream: Option<Stream>,
}

#[derive(Deserialize)]
struct CreateData {
    #[serde(rename = "createStream")]
    create_stream: Stream,
}

#[derive(Deserialize)]
struct UpdateData {
    #[serde(rename = "updateStream")]
    update_stream: Stream,
}

#[derive(Deserialize)]
struct DeleteData {
    #[serde(rename = "deleteStream")]
    delete_stream: bool,
}

#[cfg(feature = "blocking")]
/// Service for managing [`Stream`] resources.
pub struct StreamService<'a> {
    pub(crate) transport: &'a BlockingGraphQLTransport,
}

#[cfg(feature = "blocking")]
impl<'a> StreamService<'a> {
    /// List streams.
    pub fn list(&self, options: ListStreamsOptions) -> Result<ListResult<Stream>, HivehookError> {
        let query = format!(
            r#"query($applicationId: UUID, $status: StreamStatus, $search: String, $limit: Int, $offset: Int, $after: String, $first: Int) {{
                streams(applicationId: $applicationId, status: $status, search: $search, limit: $limit, offset: $offset, after: $after, first: $first) {{
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
        Ok(data.streams)
    }

    /// Get a stream by ID.
    pub fn get(&self, id: &str) -> Result<Option<Stream>, HivehookError> {
        let query = format!("query($id: UUID!) {{ stream(id: $id) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: GetData = self.transport.execute(&query, Some(v))?;
        Ok(data.stream)
    }

    /// Create a new stream.
    pub fn create(&self, input: CreateStreamInput) -> Result<Stream, HivehookError> {
        let query = format!(
            "mutation($input: CreateStreamInput!) {{ createStream(input: $input) {{ {FRAGMENT} }} }}"
        );
        let mut v = vars();
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: CreateData = self.transport.execute(&query, Some(v))?;
        Ok(data.create_stream)
    }

    /// Update an existing stream.
    pub fn update(&self, id: &str, input: UpdateStreamInput) -> Result<Stream, HivehookError> {
        let query = format!(
            "mutation($id: UUID!, $input: UpdateStreamInput!) {{ updateStream(id: $id, input: $input) {{ {FRAGMENT} }} }}"
        );
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: UpdateData = self.transport.execute(&query, Some(v))?;
        Ok(data.update_stream)
    }

    /// Delete a stream.
    pub fn delete(&self, id: &str) -> Result<bool, HivehookError> {
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: DeleteData = self
            .transport
            .execute("mutation($id: UUID!) { deleteStream(id: $id) }", Some(v))?;
        Ok(data.delete_stream)
    }
}

/// Async variant of the stream service.
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub struct AsyncStreamService<'a> {
    pub(crate) transport: &'a crate::transport::AsyncGraphQLTransport,
}

#[cfg(feature = "async")]
impl<'a> AsyncStreamService<'a> {
    /// List streams.
    pub async fn list(
        &self,
        options: ListStreamsOptions,
    ) -> Result<ListResult<Stream>, HivehookError> {
        let query = format!(
            r#"query($applicationId: UUID, $status: StreamStatus, $search: String, $limit: Int, $offset: Int, $after: String, $first: Int) {{
                streams(applicationId: $applicationId, status: $status, search: $search, limit: $limit, offset: $offset, after: $after, first: $first) {{
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
        Ok(data.streams)
    }

    /// Get a stream by ID.
    pub async fn get(&self, id: &str) -> Result<Option<Stream>, HivehookError> {
        let query = format!("query($id: UUID!) {{ stream(id: $id) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: GetData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.stream)
    }

    /// Create a new stream.
    pub async fn create(&self, input: CreateStreamInput) -> Result<Stream, HivehookError> {
        let query = format!(
            "mutation($input: CreateStreamInput!) {{ createStream(input: $input) {{ {FRAGMENT} }} }}"
        );
        let mut v = vars();
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: CreateData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.create_stream)
    }

    /// Update an existing stream.
    pub async fn update(
        &self,
        id: &str,
        input: UpdateStreamInput,
    ) -> Result<Stream, HivehookError> {
        let query = format!(
            "mutation($id: UUID!, $input: UpdateStreamInput!) {{ updateStream(id: $id, input: $input) {{ {FRAGMENT} }} }}"
        );
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: UpdateData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.update_stream)
    }

    /// Delete a stream.
    pub async fn delete(&self, id: &str) -> Result<bool, HivehookError> {
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: DeleteData = self
            .transport
            .execute("mutation($id: UUID!) { deleteStream(id: $id) }", Some(v))
            .await?;
        Ok(data.delete_stream)
    }
}
