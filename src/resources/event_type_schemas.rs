//! Event-type schema management.

use crate::errors::HivehookError;
use crate::resources::_base::{put_opt, vars};
#[cfg(feature = "blocking")]
use crate::transport::BlockingGraphQLTransport;
use crate::types::{EventTypeSchema, ListResult};
use serde::{Deserialize, Serialize};
use serde_json::Value;

const FRAGMENT: &str = "id eventType description schema example createdAt updatedAt";

/// Options for [`EventTypeSchemaService::list`].
#[non_exhaustive]
#[derive(Debug, Default, Clone)]
pub struct ListEventTypeSchemasOptions {
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

/// Input shape for `EventTypeSchemaService::create`.
#[non_exhaustive]
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateEventTypeSchemaInput {
    /// Event type identifier.
    pub event_type: String,
    /// Free-form description.
    pub description: String,
    /// JSON Schema document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Value>,
    /// Example payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<Value>,
}

/// Input shape for `EventTypeSchemaService::update`.
#[non_exhaustive]
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateEventTypeSchemaInput {
    /// New event type identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    /// New description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// New schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Value>,
    /// New example.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<Value>,
}

#[derive(Deserialize)]
struct ListData {
    #[serde(rename = "eventTypeSchemas")]
    event_type_schemas: ListResult<EventTypeSchema>,
}

#[derive(Deserialize)]
struct GetData {
    #[serde(rename = "eventTypeSchema")]
    event_type_schema: Option<EventTypeSchema>,
}

#[derive(Deserialize)]
struct CreateData {
    #[serde(rename = "createEventTypeSchema")]
    create_event_type_schema: EventTypeSchema,
}

#[derive(Deserialize)]
struct UpdateData {
    #[serde(rename = "updateEventTypeSchema")]
    update_event_type_schema: EventTypeSchema,
}

#[derive(Deserialize)]
struct DeleteData {
    #[serde(rename = "deleteEventTypeSchema")]
    delete_event_type_schema: bool,
}

#[cfg(feature = "blocking")]
/// Service for managing [`EventTypeSchema`] resources.
pub struct EventTypeSchemaService<'a> {
    pub(crate) transport: &'a BlockingGraphQLTransport,
}

#[cfg(feature = "blocking")]
impl<'a> EventTypeSchemaService<'a> {
    /// List schemas.
    pub fn list(
        &self,
        options: ListEventTypeSchemasOptions,
    ) -> Result<ListResult<EventTypeSchema>, HivehookError> {
        let query = format!(
            r#"query($search: String, $limit: Int, $offset: Int, $after: String, $first: Int) {{
                eventTypeSchemas(search: $search, limit: $limit, offset: $offset, after: $after, first: $first) {{
                    nodes {{ {FRAGMENT} }}
                    pageInfo {{ total limit offset endCursor hasNextPage }}
                }}
            }}"#
        );
        let mut v = vars();
        put_opt(&mut v, "search", options.search);
        put_opt(&mut v, "limit", options.limit);
        put_opt(&mut v, "offset", options.offset);
        put_opt(&mut v, "after", options.after);
        put_opt(&mut v, "first", options.first);
        let data: ListData = self.transport.execute(&query, Some(v))?;
        Ok(data.event_type_schemas)
    }

    /// Get a schema by ID.
    pub fn get(&self, id: &str) -> Result<Option<EventTypeSchema>, HivehookError> {
        let query = format!("query($id: UUID!) {{ eventTypeSchema(id: $id) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: GetData = self.transport.execute(&query, Some(v))?;
        Ok(data.event_type_schema)
    }

    /// Create a new schema.
    pub fn create(
        &self,
        input: CreateEventTypeSchemaInput,
    ) -> Result<EventTypeSchema, HivehookError> {
        let query = format!("mutation($input: CreateEventTypeSchemaInput!) {{ createEventTypeSchema(input: $input) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: CreateData = self.transport.execute(&query, Some(v))?;
        Ok(data.create_event_type_schema)
    }

    /// Update an existing schema.
    pub fn update(
        &self,
        id: &str,
        input: UpdateEventTypeSchemaInput,
    ) -> Result<EventTypeSchema, HivehookError> {
        let query = format!("mutation($id: UUID!, $input: UpdateEventTypeSchemaInput!) {{ updateEventTypeSchema(id: $id, input: $input) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: UpdateData = self.transport.execute(&query, Some(v))?;
        Ok(data.update_event_type_schema)
    }

    /// Delete a schema.
    pub fn delete(&self, id: &str) -> Result<bool, HivehookError> {
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: DeleteData = self.transport.execute(
            "mutation($id: UUID!) { deleteEventTypeSchema(id: $id) }",
            Some(v),
        )?;
        Ok(data.delete_event_type_schema)
    }
}

/// Async variant of the event-type schema service.
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub struct AsyncEventTypeSchemaService<'a> {
    pub(crate) transport: &'a crate::transport::AsyncGraphQLTransport,
}

#[cfg(feature = "async")]
impl<'a> AsyncEventTypeSchemaService<'a> {
    /// List schemas.
    pub async fn list(
        &self,
        options: ListEventTypeSchemasOptions,
    ) -> Result<ListResult<EventTypeSchema>, HivehookError> {
        let query = format!(
            r#"query($search: String, $limit: Int, $offset: Int, $after: String, $first: Int) {{
                eventTypeSchemas(search: $search, limit: $limit, offset: $offset, after: $after, first: $first) {{
                    nodes {{ {FRAGMENT} }}
                    pageInfo {{ total limit offset endCursor hasNextPage }}
                }}
            }}"#
        );
        let mut v = vars();
        put_opt(&mut v, "search", options.search);
        put_opt(&mut v, "limit", options.limit);
        put_opt(&mut v, "offset", options.offset);
        put_opt(&mut v, "after", options.after);
        put_opt(&mut v, "first", options.first);
        let data: ListData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.event_type_schemas)
    }

    /// Get a schema by ID.
    pub async fn get(&self, id: &str) -> Result<Option<EventTypeSchema>, HivehookError> {
        let query = format!("query($id: UUID!) {{ eventTypeSchema(id: $id) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: GetData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.event_type_schema)
    }

    /// Create a new schema.
    pub async fn create(
        &self,
        input: CreateEventTypeSchemaInput,
    ) -> Result<EventTypeSchema, HivehookError> {
        let query = format!("mutation($input: CreateEventTypeSchemaInput!) {{ createEventTypeSchema(input: $input) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: CreateData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.create_event_type_schema)
    }

    /// Update an existing schema.
    pub async fn update(
        &self,
        id: &str,
        input: UpdateEventTypeSchemaInput,
    ) -> Result<EventTypeSchema, HivehookError> {
        let query = format!("mutation($id: UUID!, $input: UpdateEventTypeSchemaInput!) {{ updateEventTypeSchema(id: $id, input: $input) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: UpdateData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.update_event_type_schema)
    }

    /// Delete a schema.
    pub async fn delete(&self, id: &str) -> Result<bool, HivehookError> {
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: DeleteData = self
            .transport
            .execute(
                "mutation($id: UUID!) { deleteEventTypeSchema(id: $id) }",
                Some(v),
            )
            .await?;
        Ok(data.delete_event_type_schema)
    }
}
