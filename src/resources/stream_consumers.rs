//! Stream consumer cursors.

use crate::errors::HivehookError;
use crate::resources::_base::{put_opt, vars};
#[cfg(feature = "blocking")]
use crate::transport::BlockingGraphQLTransport;
use crate::types::{ListResult, StreamConsumer};
use serde::Deserialize;
use serde_json::Value;

const FRAGMENT: &str = "id streamId name cursorSequence createdAt updatedAt";

/// Options for [`StreamConsumerService::list`].
#[non_exhaustive]
#[derive(Debug, Default, Clone)]
pub struct ListStreamConsumersOptions {
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
    #[serde(rename = "streamConsumers")]
    stream_consumers: ListResult<StreamConsumer>,
}

#[derive(Deserialize)]
struct GetData {
    #[serde(rename = "streamConsumer")]
    stream_consumer: Option<StreamConsumer>,
}

#[derive(Deserialize)]
struct CreateData {
    #[serde(rename = "createStreamConsumer")]
    create_stream_consumer: StreamConsumer,
}

#[derive(Deserialize)]
struct DeleteData {
    #[serde(rename = "deleteStreamConsumer")]
    delete_stream_consumer: bool,
}

#[derive(Deserialize)]
struct AdvanceData {
    #[serde(rename = "advanceConsumerCursor")]
    advance_consumer_cursor: StreamConsumer,
}

#[cfg(feature = "blocking")]
/// Service for managing [`StreamConsumer`] resources.
pub struct StreamConsumerService<'a> {
    pub(crate) transport: &'a BlockingGraphQLTransport,
}

#[cfg(feature = "blocking")]
impl<'a> StreamConsumerService<'a> {
    /// List consumers attached to `stream_id`.
    pub fn list(
        &self,
        stream_id: &str,
        options: ListStreamConsumersOptions,
    ) -> Result<ListResult<StreamConsumer>, HivehookError> {
        let query = format!(
            r#"query($streamId: UUID!, $search: String, $limit: Int, $offset: Int, $after: String, $first: Int) {{
                streamConsumers(streamId: $streamId, search: $search, limit: $limit, offset: $offset, after: $after, first: $first) {{
                    nodes {{ {FRAGMENT} }}
                    pageInfo {{ total limit offset endCursor hasNextPage }}
                }}
            }}"#
        );
        let mut v = vars();
        v.insert("streamId".into(), Value::String(stream_id.into()));
        put_opt(&mut v, "search", options.search);
        put_opt(&mut v, "limit", options.limit);
        put_opt(&mut v, "offset", options.offset);
        put_opt(&mut v, "after", options.after);
        put_opt(&mut v, "first", options.first);
        let data: ListData = self.transport.execute(&query, Some(v))?;
        Ok(data.stream_consumers)
    }

    /// Get a consumer by ID.
    pub fn get(&self, id: &str) -> Result<Option<StreamConsumer>, HivehookError> {
        let query = format!("query($id: UUID!) {{ streamConsumer(id: $id) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: GetData = self.transport.execute(&query, Some(v))?;
        Ok(data.stream_consumer)
    }

    /// Create a new consumer.
    pub fn create(&self, stream_id: &str, name: &str) -> Result<StreamConsumer, HivehookError> {
        let query = format!("mutation($input: CreateStreamConsumerInput!) {{ createStreamConsumer(input: $input) {{ {FRAGMENT} }} }}");
        let mut input = serde_json::Map::new();
        input.insert("streamId".into(), Value::String(stream_id.into()));
        input.insert("name".into(), Value::String(name.into()));
        let mut v = vars();
        v.insert("input".into(), Value::Object(input));
        let data: CreateData = self.transport.execute(&query, Some(v))?;
        Ok(data.create_stream_consumer)
    }

    /// Delete a consumer.
    pub fn delete(&self, id: &str) -> Result<bool, HivehookError> {
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: DeleteData = self.transport.execute(
            "mutation($id: UUID!) { deleteStreamConsumer(id: $id) }",
            Some(v),
        )?;
        Ok(data.delete_stream_consumer)
    }

    /// Advance the consumer's cursor to `sequence`.
    pub fn advance_cursor(
        &self,
        id: &str,
        sequence: i64,
    ) -> Result<StreamConsumer, HivehookError> {
        let query = format!("mutation($id: UUID!, $sequence: Int!) {{ advanceConsumerCursor(id: $id, sequence: $sequence) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        v.insert("sequence".into(), Value::Number(sequence.into()));
        let data: AdvanceData = self.transport.execute(&query, Some(v))?;
        Ok(data.advance_consumer_cursor)
    }
}

/// Async variant of the stream-consumer service.
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub struct AsyncStreamConsumerService<'a> {
    pub(crate) transport: &'a crate::transport::AsyncGraphQLTransport,
}

#[cfg(feature = "async")]
impl<'a> AsyncStreamConsumerService<'a> {
    /// List consumers attached to `stream_id`.
    pub async fn list(
        &self,
        stream_id: &str,
        options: ListStreamConsumersOptions,
    ) -> Result<ListResult<StreamConsumer>, HivehookError> {
        let query = format!(
            r#"query($streamId: UUID!, $search: String, $limit: Int, $offset: Int, $after: String, $first: Int) {{
                streamConsumers(streamId: $streamId, search: $search, limit: $limit, offset: $offset, after: $after, first: $first) {{
                    nodes {{ {FRAGMENT} }}
                    pageInfo {{ total limit offset endCursor hasNextPage }}
                }}
            }}"#
        );
        let mut v = vars();
        v.insert("streamId".into(), Value::String(stream_id.into()));
        put_opt(&mut v, "search", options.search);
        put_opt(&mut v, "limit", options.limit);
        put_opt(&mut v, "offset", options.offset);
        put_opt(&mut v, "after", options.after);
        put_opt(&mut v, "first", options.first);
        let data: ListData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.stream_consumers)
    }

    /// Get a consumer by ID.
    pub async fn get(&self, id: &str) -> Result<Option<StreamConsumer>, HivehookError> {
        let query = format!("query($id: UUID!) {{ streamConsumer(id: $id) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: GetData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.stream_consumer)
    }

    /// Create a new consumer.
    pub async fn create(
        &self,
        stream_id: &str,
        name: &str,
    ) -> Result<StreamConsumer, HivehookError> {
        let query = format!("mutation($input: CreateStreamConsumerInput!) {{ createStreamConsumer(input: $input) {{ {FRAGMENT} }} }}");
        let mut input = serde_json::Map::new();
        input.insert("streamId".into(), Value::String(stream_id.into()));
        input.insert("name".into(), Value::String(name.into()));
        let mut v = vars();
        v.insert("input".into(), Value::Object(input));
        let data: CreateData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.create_stream_consumer)
    }

    /// Delete a consumer.
    pub async fn delete(&self, id: &str) -> Result<bool, HivehookError> {
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: DeleteData = self
            .transport
            .execute(
                "mutation($id: UUID!) { deleteStreamConsumer(id: $id) }",
                Some(v),
            )
            .await?;
        Ok(data.delete_stream_consumer)
    }

    /// Advance the consumer's cursor to `sequence`.
    pub async fn advance_cursor(
        &self,
        id: &str,
        sequence: i64,
    ) -> Result<StreamConsumer, HivehookError> {
        let query = format!("mutation($id: UUID!, $sequence: Int!) {{ advanceConsumerCursor(id: $id, sequence: $sequence) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        v.insert("sequence".into(), Value::Number(sequence.into()));
        let data: AdvanceData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.advance_consumer_cursor)
    }
}
