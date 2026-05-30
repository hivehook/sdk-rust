//! Meta-event webhook configuration.

use crate::errors::HivehookError;
use crate::resources::_base::{put_opt, vars};
#[cfg(feature = "blocking")]
use crate::transport::BlockingGraphQLTransport;
use crate::types::{ListResult, MetaEventConfig};
use serde::{Deserialize, Serialize};
use serde_json::Value;

const FRAGMENT: &str = "id name url signingSecret eventTypes enabled createdAt";

/// Options for [`MetaEventConfigService::list`].
#[non_exhaustive]
#[derive(Debug, Default, Clone)]
pub struct ListMetaEventConfigsOptions {
    pub search: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
    pub after: Option<String>,
    pub first: Option<i32>,
}

#[non_exhaustive]
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMetaEventConfigInput {
    pub name: String,
    pub url: String,
    pub event_types: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[non_exhaustive]
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMetaEventConfigInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_types: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Deserialize)]
struct ListData {
    #[serde(rename = "metaEventConfigs")]
    meta_event_configs: ListResult<MetaEventConfig>,
}

#[derive(Deserialize)]
struct GetData {
    #[serde(rename = "metaEventConfig")]
    meta_event_config: Option<MetaEventConfig>,
}

#[derive(Deserialize)]
struct CreateData {
    #[serde(rename = "createMetaEventConfig")]
    create_meta_event_config: MetaEventConfig,
}

#[derive(Deserialize)]
struct UpdateData {
    #[serde(rename = "updateMetaEventConfig")]
    update_meta_event_config: MetaEventConfig,
}

#[derive(Deserialize)]
struct DeleteData {
    #[serde(rename = "deleteMetaEventConfig")]
    delete_meta_event_config: bool,
}

#[cfg(feature = "blocking")]
pub struct MetaEventConfigService<'a> {
    pub(crate) transport: &'a BlockingGraphQLTransport,
}

#[cfg(feature = "blocking")]
impl<'a> MetaEventConfigService<'a> {
    pub fn list(
        &self,
        options: ListMetaEventConfigsOptions,
    ) -> Result<ListResult<MetaEventConfig>, HivehookError> {
        let query = format!(
            r#"query($search: String, $limit: Int, $offset: Int, $after: String, $first: Int) {{
                metaEventConfigs(search: $search, limit: $limit, offset: $offset, after: $after, first: $first) {{
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
        Ok(data.meta_event_configs)
    }

    pub fn get(&self, id: &str) -> Result<Option<MetaEventConfig>, HivehookError> {
        let query = format!("query($id: UUID!) {{ metaEventConfig(id: $id) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: GetData = self.transport.execute(&query, Some(v))?;
        Ok(data.meta_event_config)
    }

    pub fn create(
        &self,
        input: CreateMetaEventConfigInput,
    ) -> Result<MetaEventConfig, HivehookError> {
        let query = format!(
            "mutation($input: CreateMetaEventConfigInput!) {{ createMetaEventConfig(input: $input) {{ {FRAGMENT} }} }}"
        );
        let mut v = vars();
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: CreateData = self.transport.execute(&query, Some(v))?;
        Ok(data.create_meta_event_config)
    }

    pub fn update(
        &self,
        id: &str,
        input: UpdateMetaEventConfigInput,
    ) -> Result<MetaEventConfig, HivehookError> {
        let query = format!(
            "mutation($id: UUID!, $input: UpdateMetaEventConfigInput!) {{ updateMetaEventConfig(id: $id, input: $input) {{ {FRAGMENT} }} }}"
        );
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: UpdateData = self.transport.execute(&query, Some(v))?;
        Ok(data.update_meta_event_config)
    }

    pub fn delete(&self, id: &str) -> Result<bool, HivehookError> {
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: DeleteData = self.transport.execute(
            "mutation($id: UUID!) { deleteMetaEventConfig(id: $id) }",
            Some(v),
        )?;
        Ok(data.delete_meta_event_config)
    }
}

#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub struct AsyncMetaEventConfigService<'a> {
    pub(crate) transport: &'a crate::transport::AsyncGraphQLTransport,
}

#[cfg(feature = "async")]
impl<'a> AsyncMetaEventConfigService<'a> {
    pub async fn list(
        &self,
        options: ListMetaEventConfigsOptions,
    ) -> Result<ListResult<MetaEventConfig>, HivehookError> {
        let query = format!(
            r#"query($search: String, $limit: Int, $offset: Int, $after: String, $first: Int) {{
                metaEventConfigs(search: $search, limit: $limit, offset: $offset, after: $after, first: $first) {{
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
        Ok(data.meta_event_configs)
    }

    pub async fn get(&self, id: &str) -> Result<Option<MetaEventConfig>, HivehookError> {
        let query = format!("query($id: UUID!) {{ metaEventConfig(id: $id) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: GetData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.meta_event_config)
    }

    pub async fn create(
        &self,
        input: CreateMetaEventConfigInput,
    ) -> Result<MetaEventConfig, HivehookError> {
        let query = format!(
            "mutation($input: CreateMetaEventConfigInput!) {{ createMetaEventConfig(input: $input) {{ {FRAGMENT} }} }}"
        );
        let mut v = vars();
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: CreateData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.create_meta_event_config)
    }

    pub async fn update(
        &self,
        id: &str,
        input: UpdateMetaEventConfigInput,
    ) -> Result<MetaEventConfig, HivehookError> {
        let query = format!(
            "mutation($id: UUID!, $input: UpdateMetaEventConfigInput!) {{ updateMetaEventConfig(id: $id, input: $input) {{ {FRAGMENT} }} }}"
        );
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: UpdateData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.update_meta_event_config)
    }

    pub async fn delete(&self, id: &str) -> Result<bool, HivehookError> {
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: DeleteData = self.transport.execute(
            "mutation($id: UUID!) { deleteMetaEventConfig(id: $id) }",
            Some(v),
        ).await?;
        Ok(data.delete_meta_event_config)
    }
}
