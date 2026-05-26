//! API-key management.

use crate::errors::HivehookError;
use crate::resources::_base::{put_opt, vars};
#[cfg(feature = "blocking")]
use crate::transport::BlockingGraphQLTransport;
use crate::types::{ApiKey, ApiKeyWithSecret, ListResult};
use serde::{Deserialize, Serialize};
use serde_json::Value;

const FRAGMENT: &str =
    "id name keyPrefix scopes sourceIds createdAt expiresAt revokedAt lastUsedAt";

/// Options for [`ApiKeyService::list`].
#[non_exhaustive]
#[derive(Debug, Default, Clone)]
pub struct ListApiKeysOptions {
    /// Free-text search.
    pub search: Option<String>,
    /// Offset-based page size.
    pub limit: Option<i32>,
    /// Offset-based page offset.
    pub offset: Option<i32>,
}

/// Input shape for `ApiKeyService::create`.
#[non_exhaustive]
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateApiKeyInput {
    /// Human-readable name.
    pub name: String,
    /// Granted scopes.
    pub scopes: Vec<String>,
    /// Optional list of source IDs to scope this key to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ids: Option<Vec<String>>,
    /// Optional expiry timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

#[derive(Deserialize)]
struct ListData {
    #[serde(rename = "apiKeys")]
    api_keys: ListResult<ApiKey>,
}

#[derive(Deserialize)]
struct GetData {
    #[serde(rename = "apiKey")]
    api_key: Option<ApiKey>,
}

#[derive(Deserialize)]
struct CreateData {
    #[serde(rename = "createAPIKey")]
    create_api_key: ApiKeyWithSecret,
}

#[derive(Deserialize)]
struct RevokeData {
    #[serde(rename = "revokeAPIKey")]
    revoke_api_key: bool,
}

#[cfg(feature = "blocking")]
/// Service for managing [`ApiKey`] resources.
pub struct ApiKeyService<'a> {
    pub(crate) transport: &'a BlockingGraphQLTransport,
}

#[cfg(feature = "blocking")]
impl<'a> ApiKeyService<'a> {
    /// List API keys.
    pub fn list(&self, options: ListApiKeysOptions) -> Result<ListResult<ApiKey>, HivehookError> {
        let query = format!(
            r#"query($search: String, $limit: Int, $offset: Int) {{
                apiKeys(search: $search, limit: $limit, offset: $offset) {{
                    nodes {{ {FRAGMENT} }}
                    pageInfo {{ total limit offset endCursor hasNextPage }}
                }}
            }}"#
        );
        let mut v = vars();
        put_opt(&mut v, "search", options.search);
        put_opt(&mut v, "limit", options.limit);
        put_opt(&mut v, "offset", options.offset);
        let data: ListData = self.transport.execute(&query, Some(v))?;
        Ok(data.api_keys)
    }

    /// Get an API key by ID.
    pub fn get(&self, id: &str) -> Result<Option<ApiKey>, HivehookError> {
        let query = format!("query($id: UUID!) {{ apiKey(id: $id) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: GetData = self.transport.execute(&query, Some(v))?;
        Ok(data.api_key)
    }

    /// Create a new API key. The returned [`ApiKeyWithSecret`] contains the
    /// one-time raw key; store it immediately as it cannot be retrieved later.
    pub fn create(&self, input: CreateApiKeyInput) -> Result<ApiKeyWithSecret, HivehookError> {
        let query = format!(
            "mutation($input: CreateAPIKeyInput!) {{ createAPIKey(input: $input) {{ apiKey {{ {FRAGMENT} }} rawKey }} }}"
        );
        let mut v = vars();
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: CreateData = self.transport.execute(&query, Some(v))?;
        Ok(data.create_api_key)
    }

    /// Revoke an API key.
    pub fn revoke(&self, id: &str) -> Result<bool, HivehookError> {
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: RevokeData = self
            .transport
            .execute("mutation($id: UUID!) { revokeAPIKey(id: $id) }", Some(v))?;
        Ok(data.revoke_api_key)
    }
}

/// Async variant of the API-key service.
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub struct AsyncApiKeyService<'a> {
    pub(crate) transport: &'a crate::transport::AsyncGraphQLTransport,
}

#[cfg(feature = "async")]
impl<'a> AsyncApiKeyService<'a> {
    /// List API keys.
    pub async fn list(
        &self,
        options: ListApiKeysOptions,
    ) -> Result<ListResult<ApiKey>, HivehookError> {
        let query = format!(
            r#"query($search: String, $limit: Int, $offset: Int) {{
                apiKeys(search: $search, limit: $limit, offset: $offset) {{
                    nodes {{ {FRAGMENT} }}
                    pageInfo {{ total limit offset endCursor hasNextPage }}
                }}
            }}"#
        );
        let mut v = vars();
        put_opt(&mut v, "search", options.search);
        put_opt(&mut v, "limit", options.limit);
        put_opt(&mut v, "offset", options.offset);
        let data: ListData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.api_keys)
    }

    /// Get an API key by ID.
    pub async fn get(&self, id: &str) -> Result<Option<ApiKey>, HivehookError> {
        let query = format!("query($id: UUID!) {{ apiKey(id: $id) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: GetData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.api_key)
    }

    /// Create a new API key. The returned [`ApiKeyWithSecret`] contains the
    /// one-time raw key; store it immediately as it cannot be retrieved later.
    pub async fn create(
        &self,
        input: CreateApiKeyInput,
    ) -> Result<ApiKeyWithSecret, HivehookError> {
        let query = format!(
            "mutation($input: CreateAPIKeyInput!) {{ createAPIKey(input: $input) {{ apiKey {{ {FRAGMENT} }} rawKey }} }}"
        );
        let mut v = vars();
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: CreateData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.create_api_key)
    }

    /// Revoke an API key.
    pub async fn revoke(&self, id: &str) -> Result<bool, HivehookError> {
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: RevokeData = self
            .transport
            .execute("mutation($id: UUID!) { revokeAPIKey(id: $id) }", Some(v))
            .await?;
        Ok(data.revoke_api_key)
    }
}
