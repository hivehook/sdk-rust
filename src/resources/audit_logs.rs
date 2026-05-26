//! Audit log service.

use crate::errors::HivehookError;
use crate::resources::_base::{put_opt, vars};
#[cfg(feature = "blocking")]
use crate::transport::BlockingGraphQLTransport;
use crate::types::{AuditLog, ListResult};
use serde::Deserialize;
use serde_json::Value;

const FRAGMENT: &str = "id actorType actorId actorName action resourceType resourceId orgId ipAddress userAgent details createdAt";

/// Options for [`AuditLogService::list`].
#[non_exhaustive]
#[derive(Debug, Default, Clone)]
pub struct ListAuditLogsOptions {
    /// Filter by actor type.
    pub actor_type: Option<String>,
    /// Filter by resource type.
    pub resource_type: Option<String>,
    /// Filter by resource ID.
    pub resource_id: Option<String>,
    /// Filter by action.
    pub action: Option<String>,
    /// Filter by lower bound on `createdAt` (RFC3339).
    pub since: Option<String>,
    /// Filter by upper bound on `createdAt` (RFC3339).
    pub until: Option<String>,
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
    #[serde(rename = "auditLogs")]
    audit_logs: ListResult<AuditLog>,
}

#[derive(Deserialize)]
struct GetData {
    #[serde(rename = "auditLog")]
    audit_log: Option<AuditLog>,
}

#[cfg(feature = "blocking")]
/// Service for reading [`AuditLog`] entries.
pub struct AuditLogService<'a> {
    pub(crate) transport: &'a BlockingGraphQLTransport,
}

#[cfg(feature = "blocking")]
impl<'a> AuditLogService<'a> {
    /// List audit log entries.
    pub fn list(
        &self,
        options: ListAuditLogsOptions,
    ) -> Result<ListResult<AuditLog>, HivehookError> {
        let query = format!(
            r#"query($actorType: String, $resourceType: String, $resourceId: UUID, $action: String, $since: Time, $until: Time, $search: String, $limit: Int, $offset: Int, $after: String, $first: Int) {{
                auditLogs(actorType: $actorType, resourceType: $resourceType, resourceId: $resourceId, action: $action, since: $since, until: $until, search: $search, limit: $limit, offset: $offset, after: $after, first: $first) {{
                    nodes {{ {FRAGMENT} }}
                    pageInfo {{ total limit offset endCursor hasNextPage }}
                }}
            }}"#
        );
        let mut v = vars();
        put_opt(&mut v, "actorType", options.actor_type);
        put_opt(&mut v, "resourceType", options.resource_type);
        put_opt(&mut v, "resourceId", options.resource_id);
        put_opt(&mut v, "action", options.action);
        put_opt(&mut v, "since", options.since);
        put_opt(&mut v, "until", options.until);
        put_opt(&mut v, "search", options.search);
        put_opt(&mut v, "limit", options.limit);
        put_opt(&mut v, "offset", options.offset);
        put_opt(&mut v, "after", options.after);
        put_opt(&mut v, "first", options.first);
        let data: ListData = self.transport.execute(&query, Some(v))?;
        Ok(data.audit_logs)
    }

    /// Get a single audit log entry.
    pub fn get(&self, id: &str) -> Result<Option<AuditLog>, HivehookError> {
        let query = format!("query($id: UUID!) {{ auditLog(id: $id) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: GetData = self.transport.execute(&query, Some(v))?;
        Ok(data.audit_log)
    }
}

/// Async variant of the audit-log service.
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub struct AsyncAuditLogService<'a> {
    pub(crate) transport: &'a crate::transport::AsyncGraphQLTransport,
}

#[cfg(feature = "async")]
impl<'a> AsyncAuditLogService<'a> {
    /// List audit log entries.
    pub async fn list(
        &self,
        options: ListAuditLogsOptions,
    ) -> Result<ListResult<AuditLog>, HivehookError> {
        let query = format!(
            r#"query($actorType: String, $resourceType: String, $resourceId: UUID, $action: String, $since: Time, $until: Time, $search: String, $limit: Int, $offset: Int, $after: String, $first: Int) {{
                auditLogs(actorType: $actorType, resourceType: $resourceType, resourceId: $resourceId, action: $action, since: $since, until: $until, search: $search, limit: $limit, offset: $offset, after: $after, first: $first) {{
                    nodes {{ {FRAGMENT} }}
                    pageInfo {{ total limit offset endCursor hasNextPage }}
                }}
            }}"#
        );
        let mut v = vars();
        put_opt(&mut v, "actorType", options.actor_type);
        put_opt(&mut v, "resourceType", options.resource_type);
        put_opt(&mut v, "resourceId", options.resource_id);
        put_opt(&mut v, "action", options.action);
        put_opt(&mut v, "since", options.since);
        put_opt(&mut v, "until", options.until);
        put_opt(&mut v, "search", options.search);
        put_opt(&mut v, "limit", options.limit);
        put_opt(&mut v, "offset", options.offset);
        put_opt(&mut v, "after", options.after);
        put_opt(&mut v, "first", options.first);
        let data: ListData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.audit_logs)
    }

    /// Get a single audit log entry.
    pub async fn get(&self, id: &str) -> Result<Option<AuditLog>, HivehookError> {
        let query = format!("query($id: UUID!) {{ auditLog(id: $id) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: GetData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.audit_log)
    }
}
