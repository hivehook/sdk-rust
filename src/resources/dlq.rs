//! Inbound dead-letter queue service.

use crate::resources::_base::{put_opt, vars};
use crate::types::{DlqEntry, ListResult, PurgeResult, ReplayResult};
use crate::HivehookError;
use serde::Deserialize;
use serde_json::Value;

const FRAGMENT: &str = "id deliveryId eventId lastError replayedAt createdAt";

/// Options for the `list` method on the inbound DLQ service.
#[non_exhaustive]
#[derive(Debug, Default, Clone)]
pub struct ListDlqOptions {
    /// Filter by event ID.
    pub event_id: Option<String>,
    /// Filter by whether the entry has been replayed.
    pub replayed: Option<bool>,
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
    #[serde(rename = "dlqEntries")]
    dlq_entries: ListResult<DlqEntry>,
}

#[derive(Deserialize)]
struct ReplayData {
    #[serde(rename = "replayDLQEntry")]
    replay_dlq_entry: bool,
}

#[derive(Deserialize)]
struct ReplayAllData {
    #[serde(rename = "replayAllDLQ")]
    replay_all_dlq: ReplayResult,
}

#[derive(Deserialize)]
struct PurgeData {
    #[serde(rename = "purgeDLQ")]
    purge_dlq: PurgeResult,
}
#[cfg(feature = "blocking")]
/// Blocking variant of the inbound DLQ service.
#[allow(clippy::upper_case_acronyms)]
pub struct DLQService<'a> {
    pub(crate) transport: &'a crate::transport::BlockingGraphQLTransport,
}

#[cfg(feature = "blocking")]
impl<'a> DLQService<'a> {
    /// List DLQ entries.
    pub fn list(
        &self,
        options: ListDlqOptions,
    ) -> Result<ListResult<DlqEntry>, HivehookError> {
        let query = format!(
            r#"query($eventId: UUID, $replayed: Boolean, $search: String, $limit: Int, $offset: Int, $after: String, $first: Int) {{
                dlqEntries(eventId: $eventId, replayed: $replayed, search: $search, limit: $limit, offset: $offset, after: $after, first: $first) {{
                    nodes {{ {FRAGMENT} }}
                    pageInfo {{ total limit offset endCursor hasNextPage }}
                }}
            }}"#
        );
        let mut v = vars();
        put_opt(&mut v, "eventId", options.event_id);
        put_opt(&mut v, "replayed", options.replayed);
        put_opt(&mut v, "search", options.search);
        put_opt(&mut v, "limit", options.limit);
        put_opt(&mut v, "offset", options.offset);
        put_opt(&mut v, "after", options.after);
        put_opt(&mut v, "first", options.first);
        let data: ListData = self.transport.execute(&query, Some(v))?;
        Ok(data.dlq_entries)
    }

    /// Replay a single DLQ entry.
    pub fn replay(&self, id: &str) -> Result<bool, HivehookError> {
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: ReplayData = self
            .transport
            .execute("mutation($id: UUID!) { replayDLQEntry(id: $id) }", Some(v))?;
        Ok(data.replay_dlq_entry)
    }

    /// Replay every entry in the DLQ.
    pub fn replay_all(&self) -> Result<ReplayResult, HivehookError> {
        let data: ReplayAllData = self
            .transport
            .execute("mutation { replayAllDLQ { deliveries } }", None)?;
        Ok(data.replay_all_dlq)
    }

    /// Purge entries older than the supplied Go duration string. Pass
    /// `None` to purge everything.
    pub fn purge(&self, older_than: Option<&str>) -> Result<PurgeResult, HivehookError> {
        let mut v = vars();
        if let Some(o) = older_than {
            v.insert("olderThan".into(), Value::String(o.into()));
        }
        let data: PurgeData = self.transport.execute(
            "mutation($olderThan: String) { purgeDLQ(olderThan: $olderThan) { purged } }",
            Some(v),
        )?;
        Ok(data.purge_dlq)
    }
}

/// Async variant of the inbound DLQ service.
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
#[allow(clippy::upper_case_acronyms)]
pub struct AsyncDLQService<'a> {
    pub(crate) transport: &'a crate::transport::AsyncGraphQLTransport,
}

#[cfg(feature = "async")]
impl<'a> AsyncDLQService<'a> {
    /// List DLQ entries.
    pub async fn list(
        &self,
        options: ListDlqOptions,
    ) -> Result<ListResult<DlqEntry>, HivehookError> {
        let query = format!(
            r#"query($eventId: UUID, $replayed: Boolean, $search: String, $limit: Int, $offset: Int, $after: String, $first: Int) {{
                dlqEntries(eventId: $eventId, replayed: $replayed, search: $search, limit: $limit, offset: $offset, after: $after, first: $first) {{
                    nodes {{ {FRAGMENT} }}
                    pageInfo {{ total limit offset endCursor hasNextPage }}
                }}
            }}"#
        );
        let mut v = vars();
        put_opt(&mut v, "eventId", options.event_id);
        put_opt(&mut v, "replayed", options.replayed);
        put_opt(&mut v, "search", options.search);
        put_opt(&mut v, "limit", options.limit);
        put_opt(&mut v, "offset", options.offset);
        put_opt(&mut v, "after", options.after);
        put_opt(&mut v, "first", options.first);
        let data: ListData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.dlq_entries)
    }

    /// Replay a single DLQ entry.
    pub async fn replay(&self, id: &str) -> Result<bool, HivehookError> {
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: ReplayData = self
            .transport
            .execute("mutation($id: UUID!) { replayDLQEntry(id: $id) }", Some(v))
            .await?;
        Ok(data.replay_dlq_entry)
    }

    /// Replay every entry in the DLQ.
    pub async fn replay_all(&self) -> Result<ReplayResult, HivehookError> {
        let data: ReplayAllData = self
            .transport
            .execute("mutation { replayAllDLQ { deliveries } }", None)
            .await?;
        Ok(data.replay_all_dlq)
    }

    /// Purge entries older than the supplied Go duration string. Pass
    /// `None` to purge everything.
    pub async fn purge(&self, older_than: Option<&str>) -> Result<PurgeResult, HivehookError> {
        let mut v = vars();
        if let Some(o) = older_than {
            v.insert("olderThan".into(), Value::String(o.into()));
        }
        let data: PurgeData = self
            .transport
            .execute(
                "mutation($olderThan: String) { purgeDLQ(olderThan: $olderThan) { purged } }",
                Some(v),
            )
            .await?;
        Ok(data.purge_dlq)
    }
}
