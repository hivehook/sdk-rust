//! User management.

use crate::errors::HivehookError;
use crate::resources::_base::{put_opt, vars};
#[cfg(feature = "blocking")]
use crate::transport::BlockingGraphQLTransport;
use crate::types::{ListResult, User};
use serde::{Deserialize, Serialize};
use serde_json::Value;

const FRAGMENT: &str = "id organizationId email name role lastLoginAt createdAt updatedAt";

/// Options for [`UserService::list`].
#[non_exhaustive]
#[derive(Debug, Default, Clone)]
pub struct ListUsersOptions {
    /// Filter by organization.
    pub organization_id: Option<String>,
    /// Free-text search.
    pub search: Option<String>,
    /// Offset-based page size.
    pub limit: Option<i32>,
    /// Offset-based page offset.
    pub offset: Option<i32>,
}

/// Input shape for `UserService::invite`.
#[non_exhaustive]
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InviteUserInput {
    /// Recipient email.
    pub email: String,
    /// Optional display name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Optional role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

#[derive(Deserialize)]
struct ListData {
    users: ListResult<User>,
}

#[derive(Deserialize)]
struct MeData {
    me: Option<User>,
}

#[derive(Deserialize)]
struct InviteData {
    #[serde(rename = "inviteUser")]
    invite_user: User,
}

#[derive(Deserialize)]
struct RemoveData {
    #[serde(rename = "removeUser")]
    remove_user: bool,
}

#[derive(Deserialize)]
struct UpdateRoleData {
    #[serde(rename = "updateUserRole")]
    update_user_role: User,
}

#[cfg(feature = "blocking")]
/// Service for managing [`User`] resources.
pub struct UserService<'a> {
    pub(crate) transport: &'a BlockingGraphQLTransport,
}

#[cfg(feature = "blocking")]
impl<'a> UserService<'a> {
    /// List users.
    pub fn list(&self, options: ListUsersOptions) -> Result<ListResult<User>, HivehookError> {
        let query = format!(
            r#"query($organizationId: UUID, $search: String, $limit: Int, $offset: Int) {{
                users(organizationId: $organizationId, search: $search, limit: $limit, offset: $offset) {{
                    nodes {{ {FRAGMENT} }}
                    pageInfo {{ total limit offset endCursor hasNextPage }}
                }}
            }}"#
        );
        let mut v = vars();
        put_opt(&mut v, "organizationId", options.organization_id);
        put_opt(&mut v, "search", options.search);
        put_opt(&mut v, "limit", options.limit);
        put_opt(&mut v, "offset", options.offset);
        let data: ListData = self.transport.execute(&query, Some(v))?;
        Ok(data.users)
    }

    /// Return the currently-authenticated user.
    pub fn me(&self) -> Result<Option<User>, HivehookError> {
        let query = format!("query {{ me {{ {FRAGMENT} }} }}");
        let data: MeData = self.transport.execute(&query, None)?;
        Ok(data.me)
    }

    /// Invite a user to an organization.
    pub fn invite(
        &self,
        organization_id: &str,
        input: InviteUserInput,
    ) -> Result<User, HivehookError> {
        let query = format!("mutation($organizationId: UUID!, $input: InviteUserInput!) {{ inviteUser(organizationId: $organizationId, input: $input) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("organizationId".into(), Value::String(organization_id.into()));
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: InviteData = self.transport.execute(&query, Some(v))?;
        Ok(data.invite_user)
    }

    /// Remove a user.
    pub fn remove(&self, id: &str) -> Result<bool, HivehookError> {
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: RemoveData = self
            .transport
            .execute("mutation($id: UUID!) { removeUser(id: $id) }", Some(v))?;
        Ok(data.remove_user)
    }

    /// Update a user's role.
    pub fn update_role(&self, id: &str, role: &str) -> Result<User, HivehookError> {
        let query = format!("mutation($id: UUID!, $input: UpdateUserRoleInput!) {{ updateUserRole(id: $id, input: $input) {{ {FRAGMENT} }} }}");
        let mut input = serde_json::Map::new();
        input.insert("role".into(), Value::String(role.into()));
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        v.insert("input".into(), Value::Object(input));
        let data: UpdateRoleData = self.transport.execute(&query, Some(v))?;
        Ok(data.update_user_role)
    }
}

/// Async variant of the user service.
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub struct AsyncUserService<'a> {
    pub(crate) transport: &'a crate::transport::AsyncGraphQLTransport,
}

#[cfg(feature = "async")]
impl<'a> AsyncUserService<'a> {
    /// List users.
    pub async fn list(
        &self,
        options: ListUsersOptions,
    ) -> Result<ListResult<User>, HivehookError> {
        let query = format!(
            r#"query($organizationId: UUID, $search: String, $limit: Int, $offset: Int) {{
                users(organizationId: $organizationId, search: $search, limit: $limit, offset: $offset) {{
                    nodes {{ {FRAGMENT} }}
                    pageInfo {{ total limit offset endCursor hasNextPage }}
                }}
            }}"#
        );
        let mut v = vars();
        put_opt(&mut v, "organizationId", options.organization_id);
        put_opt(&mut v, "search", options.search);
        put_opt(&mut v, "limit", options.limit);
        put_opt(&mut v, "offset", options.offset);
        let data: ListData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.users)
    }

    /// Return the currently-authenticated user.
    pub async fn me(&self) -> Result<Option<User>, HivehookError> {
        let query = format!("query {{ me {{ {FRAGMENT} }} }}");
        let data: MeData = self.transport.execute(&query, None).await?;
        Ok(data.me)
    }

    /// Invite a user to an organization.
    pub async fn invite(
        &self,
        organization_id: &str,
        input: InviteUserInput,
    ) -> Result<User, HivehookError> {
        let query = format!("mutation($organizationId: UUID!, $input: InviteUserInput!) {{ inviteUser(organizationId: $organizationId, input: $input) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("organizationId".into(), Value::String(organization_id.into()));
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: InviteData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.invite_user)
    }

    /// Remove a user.
    pub async fn remove(&self, id: &str) -> Result<bool, HivehookError> {
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: RemoveData = self
            .transport
            .execute("mutation($id: UUID!) { removeUser(id: $id) }", Some(v))
            .await?;
        Ok(data.remove_user)
    }

    /// Update a user's role.
    pub async fn update_role(&self, id: &str, role: &str) -> Result<User, HivehookError> {
        let query = format!("mutation($id: UUID!, $input: UpdateUserRoleInput!) {{ updateUserRole(id: $id, input: $input) {{ {FRAGMENT} }} }}");
        let mut input = serde_json::Map::new();
        input.insert("role".into(), Value::String(role.into()));
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        v.insert("input".into(), Value::Object(input));
        let data: UpdateRoleData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.update_user_role)
    }
}
