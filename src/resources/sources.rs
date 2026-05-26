//! Sources resource: inbound webhook endpoints.

use crate::resources::_base::{put_opt, vars};
use crate::types::{ListResult, Source};
use crate::HivehookError;
use serde::Deserialize;
use serde_json::Value;

const FRAGMENT: &str = "id name slug providerType verifyConfig status rateLimitRps spikeProtection maxIngestRps brokerConfig responseConfig { statusCode body contentType } dedupConfig { strategy fields window } createdAt";

/// Optional filters for the `list` method on the source service.
#[non_exhaustive]
#[derive(Debug, Default, Clone)]
pub struct ListSourcesOptions {
    /// Filter by source status.
    pub status: Option<String>,
    /// Filter by provider type.
    pub provider_type: Option<String>,
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

/// Input shape for the `create` method on the source service.
#[non_exhaustive]
#[derive(Debug, Default, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSourceInput {
    /// Human-readable name.
    pub name: String,
    /// URL-safe slug for the ingest endpoint.
    pub slug: String,
    /// Provider type identifier.
    pub provider_type: String,
    /// Provider-specific verification configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verify_config: Option<Value>,
    /// Whether to enable spike protection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spike_protection: Option<bool>,
    /// Hard ingest RPS ceiling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_ingest_rps: Option<i32>,
}

/// Input shape for the `update` method on the source service.
#[non_exhaustive]
#[derive(Debug, Default, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSourceInput {
    /// New name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// New slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    /// New status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// New verification config.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verify_config: Option<Value>,
    /// New per-source RPS limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_rps: Option<i32>,
    /// New spike-protection toggle.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spike_protection: Option<bool>,
    /// New hard ingest RPS ceiling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_ingest_rps: Option<i32>,
}

#[derive(Deserialize)]
struct ListData {
    sources: ListResult<Source>,
}

#[derive(Deserialize)]
struct GetData {
    source: Option<Source>,
}

#[derive(Deserialize)]
struct CreateData {
    #[serde(rename = "createSource")]
    create_source: Source,
}

#[derive(Deserialize)]
struct UpdateData {
    #[serde(rename = "updateSource")]
    update_source: Source,
}

#[derive(Deserialize)]
struct DeleteData {
    #[serde(rename = "deleteSource")]
    delete_source: bool,
}

#[derive(Deserialize)]
struct RotateData {
    #[serde(rename = "rotateSourceSecret")]
    rotate_source_secret: Source,
}

#[derive(Deserialize)]
struct ClearSecondaryData {
    #[serde(rename = "clearSourceSecondarySecret")]
    clear_source_secondary_secret: Source,
}



#[cfg(feature = "blocking")]
/// Blocking variant of the source service.
pub struct SourceService<'a> {
    pub(crate) transport: &'a crate::transport::BlockingGraphQLTransport,
}

#[cfg(feature = "blocking")]
impl<'a> SourceService<'a> {
    /// List sources with optional filters.
    pub fn list(
        &self,
        options: ListSourcesOptions,
    ) -> Result<ListResult<Source>, HivehookError> {
        let query = format!(
            r#"query($status: SourceStatus, $providerType: String, $search: String, $limit: Int, $offset: Int, $after: String, $first: Int) {{
                sources(status: $status, providerType: $providerType, search: $search, limit: $limit, offset: $offset, after: $after, first: $first) {{
                    nodes {{ {FRAGMENT} }}
                    pageInfo {{ total limit offset endCursor hasNextPage }}
                }}
            }}"#
        );
        let mut v = vars();
        put_opt(&mut v, "status", options.status);
        put_opt(&mut v, "providerType", options.provider_type);
        put_opt(&mut v, "search", options.search);
        put_opt(&mut v, "limit", options.limit);
        put_opt(&mut v, "offset", options.offset);
        put_opt(&mut v, "after", options.after);
        put_opt(&mut v, "first", options.first);
        let data: ListData = self.transport.execute(&query, Some(v))?;
        Ok(data.sources)
    }

    /// Get a source by ID. Returns `None` if no source matches.
    pub fn get(&self, id: &str) -> Result<Option<Source>, HivehookError> {
        let query = format!("query($id: UUID!) {{ source(id: $id) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: GetData = self.transport.execute(&query, Some(v))?;
        Ok(data.source)
    }

    /// Create a new source.
    pub fn create(&self, input: CreateSourceInput) -> Result<Source, HivehookError> {
        let query = format!("mutation($input: CreateSourceInput!) {{ createSource(input: $input) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: CreateData = self.transport.execute(&query, Some(v))?;
        Ok(data.create_source)
    }

    /// Update an existing source.
    pub fn update(
        &self,
        id: &str,
        input: UpdateSourceInput,
    ) -> Result<Source, HivehookError> {
        let query = format!("mutation($id: UUID!, $input: UpdateSourceInput!) {{ updateSource(id: $id, input: $input) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: UpdateData = self.transport.execute(&query, Some(v))?;
        Ok(data.update_source)
    }

    /// Delete a source. Returns `true` if the source was removed.
    pub fn delete(&self, id: &str) -> Result<bool, HivehookError> {
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: DeleteData = self
            .transport
            .execute("mutation($id: UUID!) { deleteSource(id: $id) }", Some(v))?;
        Ok(data.delete_source)
    }

    /// Rotate the signing secret for a source.
    pub fn rotate_secret(&self, id: &str) -> Result<Source, HivehookError> {
        let query = format!("mutation($id: UUID!) {{ rotateSourceSecret(id: $id) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: RotateData = self.transport.execute(&query, Some(v))?;
        Ok(data.rotate_source_secret)
    }

    /// Clear the secondary signing secret previously installed during rotation.
    pub fn clear_secondary_secret(&self, id: &str) -> Result<Source, HivehookError> {
        let query = format!("mutation($id: UUID!) {{ clearSourceSecondarySecret(id: $id) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: ClearSecondaryData = self.transport.execute(&query, Some(v))?;
        Ok(data.clear_source_secondary_secret)
    }
}

/// Async variant of the source service.
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub struct AsyncSourceService<'a> {
    pub(crate) transport: &'a crate::transport::AsyncGraphQLTransport,
}

#[cfg(feature = "async")]
impl<'a> AsyncSourceService<'a> {
    /// List sources with optional filters.
    pub async fn list(
        &self,
        options: ListSourcesOptions,
    ) -> Result<ListResult<Source>, HivehookError> {
        let query = format!(
            r#"query($status: SourceStatus, $providerType: String, $search: String, $limit: Int, $offset: Int, $after: String, $first: Int) {{
                sources(status: $status, providerType: $providerType, search: $search, limit: $limit, offset: $offset, after: $after, first: $first) {{
                    nodes {{ {FRAGMENT} }}
                    pageInfo {{ total limit offset endCursor hasNextPage }}
                }}
            }}"#
        );
        let mut v = vars();
        put_opt(&mut v, "status", options.status);
        put_opt(&mut v, "providerType", options.provider_type);
        put_opt(&mut v, "search", options.search);
        put_opt(&mut v, "limit", options.limit);
        put_opt(&mut v, "offset", options.offset);
        put_opt(&mut v, "after", options.after);
        put_opt(&mut v, "first", options.first);
        let data: ListData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.sources)
    }

    /// Get a source by ID. Returns `None` if no source matches.
    pub async fn get(&self, id: &str) -> Result<Option<Source>, HivehookError> {
        let query = format!("query($id: UUID!) {{ source(id: $id) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: GetData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.source)
    }

    /// Create a new source.
    pub async fn create(&self, input: CreateSourceInput) -> Result<Source, HivehookError> {
        let query = format!("mutation($input: CreateSourceInput!) {{ createSource(input: $input) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: CreateData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.create_source)
    }

    /// Update an existing source.
    pub async fn update(
        &self,
        id: &str,
        input: UpdateSourceInput,
    ) -> Result<Source, HivehookError> {
        let query = format!("mutation($id: UUID!, $input: UpdateSourceInput!) {{ updateSource(id: $id, input: $input) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: UpdateData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.update_source)
    }

    /// Delete a source. Returns `true` if the source was removed.
    pub async fn delete(&self, id: &str) -> Result<bool, HivehookError> {
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: DeleteData = self
            .transport
            .execute("mutation($id: UUID!) { deleteSource(id: $id) }", Some(v))
            .await?;
        Ok(data.delete_source)
    }

    /// Rotate the signing secret for a source.
    pub async fn rotate_secret(&self, id: &str) -> Result<Source, HivehookError> {
        let query = format!("mutation($id: UUID!) {{ rotateSourceSecret(id: $id) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: RotateData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.rotate_source_secret)
    }

    /// Clear the secondary signing secret previously installed during rotation.
    pub async fn clear_secondary_secret(&self, id: &str) -> Result<Source, HivehookError> {
        let query = format!("mutation($id: UUID!) {{ clearSourceSecondarySecret(id: $id) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: ClearSecondaryData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.clear_source_secondary_secret)
    }
}
