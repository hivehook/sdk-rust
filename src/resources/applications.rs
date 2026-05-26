//! Outbound application management.

use crate::errors::HivehookError;
use crate::resources::_base::{put_opt, vars};
#[cfg(feature = "blocking")]
use crate::transport::BlockingGraphQLTransport;
use crate::types::{Application, ListResult};
use serde::{Deserialize, Serialize};
use serde_json::Value;

const FRAGMENT: &str = "id name uid createdAt";

/// Options for [`ApplicationService::list`].
#[non_exhaustive]
#[derive(Debug, Default, Clone)]
pub struct ListApplicationsOptions {
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

/// Input shape for `ApplicationService::create`.
#[non_exhaustive]
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateApplicationInput {
    /// Human-readable name.
    pub name: String,
    /// Optional caller-supplied unique identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

/// Input shape for `ApplicationService::update`.
#[non_exhaustive]
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateApplicationInput {
    /// New name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// New UID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

#[derive(Deserialize)]
struct ListData {
    applications: ListResult<Application>,
}

#[derive(Deserialize)]
struct GetData {
    application: Option<Application>,
}

#[derive(Deserialize)]
struct CreateData {
    #[serde(rename = "createApplication")]
    create_application: Application,
}

#[derive(Deserialize)]
struct UpdateData {
    #[serde(rename = "updateApplication")]
    update_application: Application,
}

#[derive(Deserialize)]
struct DeleteData {
    #[serde(rename = "deleteApplication")]
    delete_application: bool,
}

#[cfg(feature = "blocking")]
/// Service for managing [`Application`] resources.
pub struct ApplicationService<'a> {
    pub(crate) transport: &'a BlockingGraphQLTransport,
}

#[cfg(feature = "blocking")]
impl<'a> ApplicationService<'a> {
    /// List applications.
    pub fn list(
        &self,
        options: ListApplicationsOptions,
    ) -> Result<ListResult<Application>, HivehookError> {
        let query = format!(
            r#"query($search: String, $limit: Int, $offset: Int, $after: String, $first: Int) {{
                applications(search: $search, limit: $limit, offset: $offset, after: $after, first: $first) {{
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
        Ok(data.applications)
    }

    /// Get an application by ID.
    pub fn get(&self, id: &str) -> Result<Option<Application>, HivehookError> {
        let query = format!("query($id: UUID!) {{ application(id: $id) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: GetData = self.transport.execute(&query, Some(v))?;
        Ok(data.application)
    }

    /// Create a new application.
    pub fn create(&self, input: CreateApplicationInput) -> Result<Application, HivehookError> {
        let query = format!(
            "mutation($input: CreateApplicationInput!) {{ createApplication(input: $input) {{ {FRAGMENT} }} }}"
        );
        let mut v = vars();
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: CreateData = self.transport.execute(&query, Some(v))?;
        Ok(data.create_application)
    }

    /// Update an application.
    pub fn update(
        &self,
        id: &str,
        input: UpdateApplicationInput,
    ) -> Result<Application, HivehookError> {
        let query = format!(
            "mutation($id: UUID!, $input: UpdateApplicationInput!) {{ updateApplication(id: $id, input: $input) {{ {FRAGMENT} }} }}"
        );
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: UpdateData = self.transport.execute(&query, Some(v))?;
        Ok(data.update_application)
    }

    /// Delete an application.
    pub fn delete(&self, id: &str) -> Result<bool, HivehookError> {
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: DeleteData = self.transport.execute(
            "mutation($id: UUID!) { deleteApplication(id: $id) }",
            Some(v),
        )?;
        Ok(data.delete_application)
    }
}

/// Async variant of the application service.
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub struct AsyncApplicationService<'a> {
    pub(crate) transport: &'a crate::transport::AsyncGraphQLTransport,
}

#[cfg(feature = "async")]
impl<'a> AsyncApplicationService<'a> {
    /// List applications.
    pub async fn list(
        &self,
        options: ListApplicationsOptions,
    ) -> Result<ListResult<Application>, HivehookError> {
        let query = format!(
            r#"query($search: String, $limit: Int, $offset: Int, $after: String, $first: Int) {{
                applications(search: $search, limit: $limit, offset: $offset, after: $after, first: $first) {{
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
        Ok(data.applications)
    }

    /// Get an application by ID.
    pub async fn get(&self, id: &str) -> Result<Option<Application>, HivehookError> {
        let query = format!("query($id: UUID!) {{ application(id: $id) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: GetData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.application)
    }

    /// Create a new application.
    pub async fn create(
        &self,
        input: CreateApplicationInput,
    ) -> Result<Application, HivehookError> {
        let query = format!(
            "mutation($input: CreateApplicationInput!) {{ createApplication(input: $input) {{ {FRAGMENT} }} }}"
        );
        let mut v = vars();
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: CreateData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.create_application)
    }

    /// Update an application.
    pub async fn update(
        &self,
        id: &str,
        input: UpdateApplicationInput,
    ) -> Result<Application, HivehookError> {
        let query = format!(
            "mutation($id: UUID!, $input: UpdateApplicationInput!) {{ updateApplication(id: $id, input: $input) {{ {FRAGMENT} }} }}"
        );
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: UpdateData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.update_application)
    }

    /// Delete an application.
    pub async fn delete(&self, id: &str) -> Result<bool, HivehookError> {
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: DeleteData = self
            .transport
            .execute(
                "mutation($id: UUID!) { deleteApplication(id: $id) }",
                Some(v),
            )
            .await?;
        Ok(data.delete_application)
    }
}
