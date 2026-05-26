//! Bookmarks resource.

use crate::errors::HivehookError;
use crate::resources::_base::{put_opt, vars};
#[cfg(feature = "blocking")]
use crate::transport::BlockingGraphQLTransport;
use crate::types::{Bookmark, ListResult};
use serde::Deserialize;
use serde_json::Value;

const FRAGMENT: &str = "id eventId name notes createdAt";

/// Options for [`BookmarkService::list`].
#[non_exhaustive]
#[derive(Debug, Default, Clone)]
pub struct ListBookmarksOptions {
    /// Filter by event ID.
    pub event_id: Option<String>,
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
    bookmarks: ListResult<Bookmark>,
}

#[derive(Deserialize)]
struct GetData {
    bookmark: Option<Bookmark>,
}

#[derive(Deserialize)]
struct CreateData {
    #[serde(rename = "createBookmark")]
    create_bookmark: Bookmark,
}

#[derive(Deserialize)]
struct DeleteData {
    #[serde(rename = "deleteBookmark")]
    delete_bookmark: bool,
}

#[cfg(feature = "blocking")]
/// Service for managing [`Bookmark`] resources.
pub struct BookmarkService<'a> {
    pub(crate) transport: &'a BlockingGraphQLTransport,
}

#[cfg(feature = "blocking")]
impl<'a> BookmarkService<'a> {
    /// List bookmarks.
    pub fn list(
        &self,
        options: ListBookmarksOptions,
    ) -> Result<ListResult<Bookmark>, HivehookError> {
        let query = format!(
            r#"query($eventId: UUID, $search: String, $limit: Int, $offset: Int, $after: String, $first: Int) {{
                bookmarks(eventId: $eventId, search: $search, limit: $limit, offset: $offset, after: $after, first: $first) {{
                    nodes {{ {FRAGMENT} }}
                    pageInfo {{ total limit offset endCursor hasNextPage }}
                }}
            }}"#
        );
        let mut v = vars();
        put_opt(&mut v, "eventId", options.event_id);
        put_opt(&mut v, "search", options.search);
        put_opt(&mut v, "limit", options.limit);
        put_opt(&mut v, "offset", options.offset);
        put_opt(&mut v, "after", options.after);
        put_opt(&mut v, "first", options.first);
        let data: ListData = self.transport.execute(&query, Some(v))?;
        Ok(data.bookmarks)
    }

    /// Get a bookmark by ID.
    pub fn get(&self, id: &str) -> Result<Option<Bookmark>, HivehookError> {
        let query = format!("query($id: UUID!) {{ bookmark(id: $id) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: GetData = self.transport.execute(&query, Some(v))?;
        Ok(data.bookmark)
    }

    /// Create a bookmark on an event.
    pub fn create(
        &self,
        event_id: &str,
        name: &str,
        notes: &str,
    ) -> Result<Bookmark, HivehookError> {
        let query = format!(
            "mutation($eventId: UUID!, $name: String, $notes: String) {{ createBookmark(eventId: $eventId, name: $name, notes: $notes) {{ {FRAGMENT} }} }}"
        );
        let mut v = vars();
        v.insert("eventId".into(), Value::String(event_id.into()));
        v.insert("name".into(), Value::String(name.into()));
        v.insert("notes".into(), Value::String(notes.into()));
        let data: CreateData = self.transport.execute(&query, Some(v))?;
        Ok(data.create_bookmark)
    }

    /// Delete a bookmark.
    pub fn delete(&self, id: &str) -> Result<bool, HivehookError> {
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: DeleteData = self
            .transport
            .execute("mutation($id: UUID!) { deleteBookmark(id: $id) }", Some(v))?;
        Ok(data.delete_bookmark)
    }
}

/// Async variant of the bookmark service.
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub struct AsyncBookmarkService<'a> {
    pub(crate) transport: &'a crate::transport::AsyncGraphQLTransport,
}

#[cfg(feature = "async")]
impl<'a> AsyncBookmarkService<'a> {
    /// List bookmarks.
    pub async fn list(
        &self,
        options: ListBookmarksOptions,
    ) -> Result<ListResult<Bookmark>, HivehookError> {
        let query = format!(
            r#"query($eventId: UUID, $search: String, $limit: Int, $offset: Int, $after: String, $first: Int) {{
                bookmarks(eventId: $eventId, search: $search, limit: $limit, offset: $offset, after: $after, first: $first) {{
                    nodes {{ {FRAGMENT} }}
                    pageInfo {{ total limit offset endCursor hasNextPage }}
                }}
            }}"#
        );
        let mut v = vars();
        put_opt(&mut v, "eventId", options.event_id);
        put_opt(&mut v, "search", options.search);
        put_opt(&mut v, "limit", options.limit);
        put_opt(&mut v, "offset", options.offset);
        put_opt(&mut v, "after", options.after);
        put_opt(&mut v, "first", options.first);
        let data: ListData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.bookmarks)
    }

    /// Get a bookmark by ID.
    pub async fn get(&self, id: &str) -> Result<Option<Bookmark>, HivehookError> {
        let query = format!("query($id: UUID!) {{ bookmark(id: $id) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: GetData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.bookmark)
    }

    /// Create a bookmark on an event.
    pub async fn create(
        &self,
        event_id: &str,
        name: &str,
        notes: &str,
    ) -> Result<Bookmark, HivehookError> {
        let query = format!(
            "mutation($eventId: UUID!, $name: String, $notes: String) {{ createBookmark(eventId: $eventId, name: $name, notes: $notes) {{ {FRAGMENT} }} }}"
        );
        let mut v = vars();
        v.insert("eventId".into(), Value::String(event_id.into()));
        v.insert("name".into(), Value::String(name.into()));
        v.insert("notes".into(), Value::String(notes.into()));
        let data: CreateData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.create_bookmark)
    }

    /// Delete a bookmark.
    pub async fn delete(&self, id: &str) -> Result<bool, HivehookError> {
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: DeleteData = self
            .transport
            .execute("mutation($id: UUID!) { deleteBookmark(id: $id) }", Some(v))
            .await?;
        Ok(data.delete_bookmark)
    }
}
