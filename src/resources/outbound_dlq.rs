//! Outbound dead-letter queue service.

use crate::errors::HivehookError;
use crate::resources::_base::{put_opt, vars};
#[cfg(feature = "blocking")]
use crate::transport::BlockingGraphQLTransport;
use crate::types::{ListResult, OutboundDlqEntry, PurgeResult, ReplayResult};
use serde::Deserialize;
use serde_json::Value;

const FRAGMENT: &str = "id deliveryId messageId lastError replayedAt createdAt";

/// Options for [`OutboundDLQService::list`].
#[non_exhaustive]
#[derive(Debug, Default, Clone)]
pub struct ListOutboundDlqOptions {
    /// Filter by message ID.
    pub message_id: Option<String>,
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
    #[serde(rename = "outboundDlqEntries")]
    outbound_dlq_entries: ListResult<OutboundDlqEntry>,
}

#[derive(Deserialize)]
struct ReplayData {
    #[serde(rename = "replayOutboundDlqEntry")]
    replay_outbound_dlq_entry: bool,
}

#[derive(Deserialize)]
struct ReplayAllData {
    #[serde(rename = "replayAllOutboundDlq")]
    replay_all_outbound_dlq: ReplayResult,
}

#[derive(Deserialize)]
struct PurgeData {
    #[serde(rename = "purgeOutboundDlq")]
    purge_outbound_dlq: PurgeResult,
}

#[cfg(feature = "blocking")]
/// Service for inspecting and replaying the outbound DLQ.
#[allow(clippy::upper_case_acronyms)]
pub struct OutboundDLQService<'a> {
    pub(crate) transport: &'a BlockingGraphQLTransport,
}

#[cfg(feature = "blocking")]
impl<'a> OutboundDLQService<'a> {
    /// List outbound DLQ entries.
    pub fn list(
        &self,
        options: ListOutboundDlqOptions,
    ) -> Result<ListResult<OutboundDlqEntry>, HivehookError> {
        let query = format!(
            r#"query($messageId: UUID, $replayed: Boolean, $search: String, $limit: Int, $offset: Int, $after: String, $first: Int) {{
                outboundDlqEntries(messageId: $messageId, replayed: $replayed, search: $search, limit: $limit, offset: $offset, after: $after, first: $first) {{
                    nodes {{ {FRAGMENT} }}
                    pageInfo {{ total limit offset endCursor hasNextPage }}
                }}
            }}"#
        );
        let mut v = vars();
        put_opt(&mut v, "messageId", options.message_id);
        put_opt(&mut v, "replayed", options.replayed);
        put_opt(&mut v, "search", options.search);
        put_opt(&mut v, "limit", options.limit);
        put_opt(&mut v, "offset", options.offset);
        put_opt(&mut v, "after", options.after);
        put_opt(&mut v, "first", options.first);
        let data: ListData = self.transport.execute(&query, Some(v))?;
        Ok(data.outbound_dlq_entries)
    }

    /// Replay a single outbound DLQ entry.
    pub fn replay(&self, id: &str) -> Result<bool, HivehookError> {
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: ReplayData = self.transport.execute(
            "mutation($id: UUID!) { replayOutboundDlqEntry(id: $id) }",
            Some(v),
        )?;
        Ok(data.replay_outbound_dlq_entry)
    }

    /// Replay every entry in the outbound DLQ.
    pub fn replay_all(&self) -> Result<ReplayResult, HivehookError> {
        let data: ReplayAllData = self
            .transport
            .execute("mutation { replayAllOutboundDlq { deliveries } }", None)?;
        Ok(data.replay_all_outbound_dlq)
    }

    /// Purge entries older than the supplied Go duration string. Pass `None`
    /// to purge everything.
    pub fn purge(&self, older_than: Option<&str>) -> Result<PurgeResult, HivehookError> {
        let mut v = vars();
        if let Some(o) = older_than {
            v.insert("olderThan".into(), Value::String(o.into()));
        }
        let data: PurgeData = self.transport.execute(
            "mutation($olderThan: String) { purgeOutboundDlq(olderThan: $olderThan) { purged } }",
            Some(v),
        )?;
        Ok(data.purge_outbound_dlq)
    }
}

/// Async variant of the outbound DLQ service.
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
#[allow(clippy::upper_case_acronyms)]
pub struct AsyncOutboundDLQService<'a> {
    pub(crate) transport: &'a crate::transport::AsyncGraphQLTransport,
}

#[cfg(feature = "async")]
impl<'a> AsyncOutboundDLQService<'a> {
    /// List outbound DLQ entries.
    pub async fn list(
        &self,
        options: ListOutboundDlqOptions,
    ) -> Result<ListResult<OutboundDlqEntry>, HivehookError> {
        let query = format!(
            r#"query($messageId: UUID, $replayed: Boolean, $search: String, $limit: Int, $offset: Int, $after: String, $first: Int) {{
                outboundDlqEntries(messageId: $messageId, replayed: $replayed, search: $search, limit: $limit, offset: $offset, after: $after, first: $first) {{
                    nodes {{ {FRAGMENT} }}
                    pageInfo {{ total limit offset endCursor hasNextPage }}
                }}
            }}"#
        );
        let mut v = vars();
        put_opt(&mut v, "messageId", options.message_id);
        put_opt(&mut v, "replayed", options.replayed);
        put_opt(&mut v, "search", options.search);
        put_opt(&mut v, "limit", options.limit);
        put_opt(&mut v, "offset", options.offset);
        put_opt(&mut v, "after", options.after);
        put_opt(&mut v, "first", options.first);
        let data: ListData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.outbound_dlq_entries)
    }

    /// Replay a single outbound DLQ entry.
    pub async fn replay(&self, id: &str) -> Result<bool, HivehookError> {
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: ReplayData = self
            .transport
            .execute(
                "mutation($id: UUID!) { replayOutboundDlqEntry(id: $id) }",
                Some(v),
            )
            .await?;
        Ok(data.replay_outbound_dlq_entry)
    }

    /// Replay every entry in the outbound DLQ.
    pub async fn replay_all(&self) -> Result<ReplayResult, HivehookError> {
        let data: ReplayAllData = self
            .transport
            .execute("mutation { replayAllOutboundDlq { deliveries } }", None)
            .await?;
        Ok(data.replay_all_outbound_dlq)
    }

    /// Purge entries older than the supplied Go duration string. Pass `None`
    /// to purge everything.
    pub async fn purge(&self, older_than: Option<&str>) -> Result<PurgeResult, HivehookError> {
        let mut v = vars();
        if let Some(o) = older_than {
            v.insert("olderThan".into(), Value::String(o.into()));
        }
        let data: PurgeData = self.transport.execute(
            "mutation($olderThan: String) { purgeOutboundDlq(olderThan: $olderThan) { purged } }",
            Some(v),
        ).await?;
        Ok(data.purge_outbound_dlq)
    }
}
