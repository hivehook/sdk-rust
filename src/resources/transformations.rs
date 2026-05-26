//! Transformations resource.

use crate::errors::HivehookError;
use crate::resources::_base::{put_opt, vars};
#[cfg(feature = "blocking")]
use crate::transport::BlockingGraphQLTransport;
use crate::types::{ListResult, Transformation, TransformTestResult};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

const FRAGMENT: &str = "id name description code enabled failOpen timeoutMs createdAt updatedAt";

/// Options for [`TransformationService::list`].
#[non_exhaustive]
#[derive(Debug, Default, Clone)]
pub struct ListTransformationsOptions {
    /// Filter by enabled state.
    pub enabled: Option<bool>,
    /// Free-text search.
    pub search: Option<String>,
    /// Cursor for cursor-based pagination.
    pub after: Option<String>,
    /// Page size for cursor-based pagination.
    pub first: Option<i32>,
}

/// Input shape for `TransformationService::create`.
#[non_exhaustive]
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTransformationInput {
    /// Human-readable name.
    pub name: String,
    /// Free-form description.
    pub description: String,
    /// Script source.
    pub code: String,
    /// Whether failures fall through.
    pub fail_open: bool,
    /// Execution timeout in milliseconds.
    pub timeout_ms: i32,
}

/// Input shape for `TransformationService::update`.
#[non_exhaustive]
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTransformationInput {
    /// New name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// New description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// New code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// New enabled state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// New fail-open toggle.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_open: Option<bool>,
    /// New timeout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<i32>,
}

/// Input shape for `TransformationService::test`.
#[non_exhaustive]
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TestTransformationInput {
    /// Script source.
    pub code: String,
    /// Sample payload.
    pub payload: Value,
    /// Optional event type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    /// Optional headers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, Value>>,
}

#[derive(Deserialize)]
struct ListData {
    transformations: ListResult<Transformation>,
}

#[derive(Deserialize)]
struct GetData {
    transformation: Option<Transformation>,
}

#[derive(Deserialize)]
struct CreateData {
    #[serde(rename = "createTransformation")]
    create_transformation: Transformation,
}

#[derive(Deserialize)]
struct UpdateData {
    #[serde(rename = "updateTransformation")]
    update_transformation: Transformation,
}

#[derive(Deserialize)]
struct DeleteData {
    #[serde(rename = "deleteTransformation")]
    delete_transformation: bool,
}

#[derive(Deserialize)]
struct TestData {
    #[serde(rename = "testTransformation")]
    test_transformation: TransformTestResult,
}

#[cfg(feature = "blocking")]
/// Service for managing [`Transformation`] resources.
pub struct TransformationService<'a> {
    pub(crate) transport: &'a BlockingGraphQLTransport,
}

#[cfg(feature = "blocking")]
impl<'a> TransformationService<'a> {
    /// List transformations.
    pub fn list(
        &self,
        options: ListTransformationsOptions,
    ) -> Result<ListResult<Transformation>, HivehookError> {
        let query = format!(
            r#"query($enabled: Boolean, $search: String, $after: String, $first: Int) {{
                transformations(enabled: $enabled, search: $search, after: $after, first: $first) {{
                    nodes {{ {FRAGMENT} }}
                    pageInfo {{ total limit offset endCursor hasNextPage }}
                }}
            }}"#
        );
        let mut v = vars();
        put_opt(&mut v, "enabled", options.enabled);
        put_opt(&mut v, "search", options.search);
        put_opt(&mut v, "after", options.after);
        put_opt(&mut v, "first", options.first);
        let data: ListData = self.transport.execute(&query, Some(v))?;
        Ok(data.transformations)
    }

    /// Get a transformation by ID.
    pub fn get(&self, id: &str) -> Result<Option<Transformation>, HivehookError> {
        let query = format!("query($id: UUID!) {{ transformation(id: $id) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: GetData = self.transport.execute(&query, Some(v))?;
        Ok(data.transformation)
    }

    /// Create a new transformation.
    pub fn create(
        &self,
        input: CreateTransformationInput,
    ) -> Result<Transformation, HivehookError> {
        let query = format!("mutation($input: CreateTransformationInput!) {{ createTransformation(input: $input) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: CreateData = self.transport.execute(&query, Some(v))?;
        Ok(data.create_transformation)
    }

    /// Update an existing transformation.
    pub fn update(
        &self,
        id: &str,
        input: UpdateTransformationInput,
    ) -> Result<Transformation, HivehookError> {
        let query = format!("mutation($id: UUID!, $input: UpdateTransformationInput!) {{ updateTransformation(id: $id, input: $input) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: UpdateData = self.transport.execute(&query, Some(v))?;
        Ok(data.update_transformation)
    }

    /// Delete a transformation.
    pub fn delete(&self, id: &str) -> Result<bool, HivehookError> {
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: DeleteData = self.transport.execute(
            "mutation($id: UUID!) { deleteTransformation(id: $id) }",
            Some(v),
        )?;
        Ok(data.delete_transformation)
    }

    /// Run a transformation against a sample payload without persisting it.
    pub fn test(
        &self,
        input: TestTransformationInput,
    ) -> Result<TransformTestResult, HivehookError> {
        let mut v = vars();
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: TestData = self.transport.execute(
            "mutation($input: TestTransformationInput!) { testTransformation(input: $input) { success output error durationMs } }",
            Some(v),
        )?;
        Ok(data.test_transformation)
    }
}

/// Async variant of the transformation service.
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub struct AsyncTransformationService<'a> {
    pub(crate) transport: &'a crate::transport::AsyncGraphQLTransport,
}

#[cfg(feature = "async")]
impl<'a> AsyncTransformationService<'a> {
    /// List transformations.
    pub async fn list(
        &self,
        options: ListTransformationsOptions,
    ) -> Result<ListResult<Transformation>, HivehookError> {
        let query = format!(
            r#"query($enabled: Boolean, $search: String, $after: String, $first: Int) {{
                transformations(enabled: $enabled, search: $search, after: $after, first: $first) {{
                    nodes {{ {FRAGMENT} }}
                    pageInfo {{ total limit offset endCursor hasNextPage }}
                }}
            }}"#
        );
        let mut v = vars();
        put_opt(&mut v, "enabled", options.enabled);
        put_opt(&mut v, "search", options.search);
        put_opt(&mut v, "after", options.after);
        put_opt(&mut v, "first", options.first);
        let data: ListData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.transformations)
    }

    /// Get a transformation by ID.
    pub async fn get(&self, id: &str) -> Result<Option<Transformation>, HivehookError> {
        let query = format!("query($id: UUID!) {{ transformation(id: $id) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: GetData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.transformation)
    }

    /// Create a new transformation.
    pub async fn create(
        &self,
        input: CreateTransformationInput,
    ) -> Result<Transformation, HivehookError> {
        let query = format!("mutation($input: CreateTransformationInput!) {{ createTransformation(input: $input) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: CreateData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.create_transformation)
    }

    /// Update an existing transformation.
    pub async fn update(
        &self,
        id: &str,
        input: UpdateTransformationInput,
    ) -> Result<Transformation, HivehookError> {
        let query = format!("mutation($id: UUID!, $input: UpdateTransformationInput!) {{ updateTransformation(id: $id, input: $input) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: UpdateData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.update_transformation)
    }

    /// Delete a transformation.
    pub async fn delete(&self, id: &str) -> Result<bool, HivehookError> {
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: DeleteData = self
            .transport
            .execute(
                "mutation($id: UUID!) { deleteTransformation(id: $id) }",
                Some(v),
            )
            .await?;
        Ok(data.delete_transformation)
    }

    /// Run a transformation against a sample payload without persisting it.
    pub async fn test(
        &self,
        input: TestTransformationInput,
    ) -> Result<TransformTestResult, HivehookError> {
        let mut v = vars();
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: TestData = self.transport.execute(
            "mutation($input: TestTransformationInput!) { testTransformation(input: $input) { success output error durationMs } }",
            Some(v),
        ).await?;
        Ok(data.test_transformation)
    }
}
