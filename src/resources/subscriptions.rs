//! Subscriptions resource: source-to-destination wiring.

use crate::resources::_base::{put_opt, vars};
use crate::types::{FilterConfig, ListResult, Subscription, TransformConfig};
use crate::HivehookError;
use serde::{Deserialize, Serialize};
use serde_json::Value;

const FRAGMENT: &str = "id name sourceId destinationId filterConfig { eventTypes regex bodyMatch { path value operator } rules { path operator value rules { path operator value } } } transformConfig { envelope headers } enabled createdAt";

/// Options for the `list` method on the subscription service.
#[non_exhaustive]
#[derive(Debug, Default, Clone)]
pub struct ListSubscriptionsOptions {
    /// Filter by source ID.
    pub source_id: Option<String>,
    /// Filter by destination ID.
    pub destination_id: Option<String>,
    /// Filter by enabled state.
    pub enabled: Option<bool>,
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

/// Input shape for `create` on the subscription service.
#[non_exhaustive]
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSubscriptionInput {
    /// Human-readable name.
    pub name: String,
    /// Source identifier.
    pub source_id: String,
    /// Destination identifier.
    pub destination_id: String,
    /// Optional filter configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_config: Option<FilterConfig>,
    /// Optional transformation configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_config: Option<TransformConfig>,
    /// Whether the subscription should start enabled.
    pub enabled: bool,
}

/// Input shape for `update` on the subscription service.
#[non_exhaustive]
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSubscriptionInput {
    /// New name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// New enabled state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// New filter configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_config: Option<FilterConfig>,
    /// New transformation configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_config: Option<TransformConfig>,
}

#[derive(Deserialize)]
struct ListData {
    subscriptions: ListResult<Subscription>,
}

#[derive(Deserialize)]
struct GetData {
    subscription: Option<Subscription>,
}

#[derive(Deserialize)]
struct CreateData {
    #[serde(rename = "createSubscription")]
    create_subscription: Subscription,
}

#[derive(Deserialize)]
struct UpdateData {
    #[serde(rename = "updateSubscription")]
    update_subscription: Subscription,
}

#[derive(Deserialize)]
struct DeleteData {
    #[serde(rename = "deleteSubscription")]
    delete_subscription: bool,
}
#[cfg(feature = "blocking")]
/// Blocking variant of the subscription service.
pub struct SubscriptionService<'a> {
    pub(crate) transport: &'a crate::transport::BlockingGraphQLTransport,
}

#[cfg(feature = "blocking")]
impl<'a> SubscriptionService<'a> {
    /// List subscriptions.
    pub fn list(
        &self,
        options: ListSubscriptionsOptions,
    ) -> Result<ListResult<Subscription>, HivehookError> {
        let query = format!(
            r#"query($sourceId: UUID, $destinationId: UUID, $enabled: Boolean, $search: String, $limit: Int, $offset: Int, $after: String, $first: Int) {{
                subscriptions(sourceId: $sourceId, destinationId: $destinationId, enabled: $enabled, search: $search, limit: $limit, offset: $offset, after: $after, first: $first) {{
                    nodes {{ {FRAGMENT} }}
                    pageInfo {{ total limit offset endCursor hasNextPage }}
                }}
            }}"#
        );
        let mut v = vars();
        put_opt(&mut v, "sourceId", options.source_id);
        put_opt(&mut v, "destinationId", options.destination_id);
        put_opt(&mut v, "enabled", options.enabled);
        put_opt(&mut v, "search", options.search);
        put_opt(&mut v, "limit", options.limit);
        put_opt(&mut v, "offset", options.offset);
        put_opt(&mut v, "after", options.after);
        put_opt(&mut v, "first", options.first);
        let data: ListData = self.transport.execute(&query, Some(v))?;
        Ok(data.subscriptions)
    }

    /// Get a subscription by ID.
    pub fn get(&self, id: &str) -> Result<Option<Subscription>, HivehookError> {
        let query = format!("query($id: UUID!) {{ subscription(id: $id) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: GetData = self.transport.execute(&query, Some(v))?;
        Ok(data.subscription)
    }

    /// Create a new subscription.
    pub fn create(
        &self,
        input: CreateSubscriptionInput,
    ) -> Result<Subscription, HivehookError> {
        let query = format!("mutation($input: CreateSubscriptionInput!) {{ createSubscription(input: $input) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: CreateData = self.transport.execute(&query, Some(v))?;
        Ok(data.create_subscription)
    }

    /// Update an existing subscription.
    pub fn update(
        &self,
        id: &str,
        input: UpdateSubscriptionInput,
    ) -> Result<Subscription, HivehookError> {
        let query = format!("mutation($id: UUID!, $input: UpdateSubscriptionInput!) {{ updateSubscription(id: $id, input: $input) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: UpdateData = self.transport.execute(&query, Some(v))?;
        Ok(data.update_subscription)
    }

    /// Delete a subscription.
    pub fn delete(&self, id: &str) -> Result<bool, HivehookError> {
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: DeleteData = self
            .transport
            .execute("mutation($id: UUID!) { deleteSubscription(id: $id) }", Some(v))?;
        Ok(data.delete_subscription)
    }
}

/// Async variant of the subscription service.
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub struct AsyncSubscriptionService<'a> {
    pub(crate) transport: &'a crate::transport::AsyncGraphQLTransport,
}

#[cfg(feature = "async")]
impl<'a> AsyncSubscriptionService<'a> {
    /// List subscriptions.
    pub async fn list(
        &self,
        options: ListSubscriptionsOptions,
    ) -> Result<ListResult<Subscription>, HivehookError> {
        let query = format!(
            r#"query($sourceId: UUID, $destinationId: UUID, $enabled: Boolean, $search: String, $limit: Int, $offset: Int, $after: String, $first: Int) {{
                subscriptions(sourceId: $sourceId, destinationId: $destinationId, enabled: $enabled, search: $search, limit: $limit, offset: $offset, after: $after, first: $first) {{
                    nodes {{ {FRAGMENT} }}
                    pageInfo {{ total limit offset endCursor hasNextPage }}
                }}
            }}"#
        );
        let mut v = vars();
        put_opt(&mut v, "sourceId", options.source_id);
        put_opt(&mut v, "destinationId", options.destination_id);
        put_opt(&mut v, "enabled", options.enabled);
        put_opt(&mut v, "search", options.search);
        put_opt(&mut v, "limit", options.limit);
        put_opt(&mut v, "offset", options.offset);
        put_opt(&mut v, "after", options.after);
        put_opt(&mut v, "first", options.first);
        let data: ListData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.subscriptions)
    }

    /// Get a subscription by ID.
    pub async fn get(&self, id: &str) -> Result<Option<Subscription>, HivehookError> {
        let query = format!("query($id: UUID!) {{ subscription(id: $id) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: GetData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.subscription)
    }

    /// Create a new subscription.
    pub async fn create(
        &self,
        input: CreateSubscriptionInput,
    ) -> Result<Subscription, HivehookError> {
        let query = format!("mutation($input: CreateSubscriptionInput!) {{ createSubscription(input: $input) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: CreateData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.create_subscription)
    }

    /// Update an existing subscription.
    pub async fn update(
        &self,
        id: &str,
        input: UpdateSubscriptionInput,
    ) -> Result<Subscription, HivehookError> {
        let query = format!("mutation($id: UUID!, $input: UpdateSubscriptionInput!) {{ updateSubscription(id: $id, input: $input) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: UpdateData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.update_subscription)
    }

    /// Delete a subscription.
    pub async fn delete(&self, id: &str) -> Result<bool, HivehookError> {
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: DeleteData = self
            .transport
            .execute("mutation($id: UUID!) { deleteSubscription(id: $id) }", Some(v))
            .await?;
        Ok(data.delete_subscription)
    }
}
