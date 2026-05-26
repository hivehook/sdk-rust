//! Outbound message management.

use crate::errors::HivehookError;
use crate::resources::_base::{put_opt, vars};
#[cfg(feature = "blocking")]
use crate::transport::BlockingGraphQLTransport;
use crate::types::{ListResult, Message, OutboundDelivery};
use base64::{engine::general_purpose::STANDARD, Engine};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

const FRAGMENT: &str = "id applicationId eventType payload idempotencyKey status createdAt";
const DELIVERY_FRAGMENT: &str = "id messageId endpointId status attempts maxAttempts nextAttemptAt createdAt";

/// Options for [`MessageService::list`].
#[non_exhaustive]
#[derive(Debug, Default, Clone)]
pub struct ListMessagesOptions {
    /// Filter by application ID.
    pub application_id: Option<String>,
    /// Filter by event type.
    pub event_type: Option<String>,
    /// Filter by status.
    pub status: Option<String>,
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
    messages: ListResult<Message>,
}

#[derive(Deserialize)]
struct GetData {
    message: Option<Message>,
}

#[derive(Deserialize)]
struct SendData {
    #[serde(rename = "sendMessage")]
    send_message: Message,
}

#[derive(Deserialize)]
struct BroadcastData {
    #[serde(rename = "broadcastMessage")]
    broadcast_message: Message,
}

#[derive(Deserialize)]
struct SendDynamicData {
    #[serde(rename = "sendDynamicMessage")]
    send_dynamic_message: OutboundDelivery,
}

/// Builder for the `SendMessage` GraphQL input.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct SendMessageInput {
    /// Owning application identifier.
    pub application_id: String,
    /// Event type identifier.
    pub event_type: String,
    /// Raw payload bytes; the SDK base64-encodes this before sending.
    pub payload: Vec<u8>,
    /// Optional idempotency key.
    pub idempotency_key: Option<String>,
}

#[cfg(feature = "blocking")]
/// Service for sending outbound [`Message`] payloads.
pub struct MessageService<'a> {
    pub(crate) transport: &'a BlockingGraphQLTransport,
}

#[cfg(feature = "blocking")]
impl<'a> MessageService<'a> {
    /// List messages.
    pub fn list(&self, options: ListMessagesOptions) -> Result<ListResult<Message>, HivehookError> {
        let query = format!(
            r#"query($applicationId: UUID, $eventType: String, $status: MessageStatus, $search: String, $limit: Int, $offset: Int, $after: String, $first: Int) {{
                messages(applicationId: $applicationId, eventType: $eventType, status: $status, search: $search, limit: $limit, offset: $offset, after: $after, first: $first) {{
                    nodes {{ {FRAGMENT} }}
                    pageInfo {{ total limit offset endCursor hasNextPage }}
                }}
            }}"#
        );
        let mut v = vars();
        put_opt(&mut v, "applicationId", options.application_id);
        put_opt(&mut v, "eventType", options.event_type);
        put_opt(&mut v, "status", options.status);
        put_opt(&mut v, "search", options.search);
        put_opt(&mut v, "limit", options.limit);
        put_opt(&mut v, "offset", options.offset);
        put_opt(&mut v, "after", options.after);
        put_opt(&mut v, "first", options.first);
        let data: ListData = self.transport.execute(&query, Some(v))?;
        Ok(data.messages)
    }

    /// Get a message by ID.
    pub fn get(&self, id: &str) -> Result<Option<Message>, HivehookError> {
        let query = format!("query($id: UUID!) {{ message(id: $id) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: GetData = self.transport.execute(&query, Some(v))?;
        Ok(data.message)
    }

    /// Send a single message to all endpoints subscribed to the event type.
    pub fn send(&self, input: SendMessageInput) -> Result<Message, HivehookError> {
        let query = format!("mutation($input: SendMessageInput!) {{ sendMessage(input: $input) {{ {FRAGMENT} }} }}");
        let mut payload = serde_json::Map::new();
        payload.insert(
            "applicationId".into(),
            Value::String(input.application_id),
        );
        payload.insert("eventType".into(), Value::String(input.event_type));
        payload.insert(
            "payload".into(),
            Value::String(STANDARD.encode(&input.payload)),
        );
        if let Some(key) = input.idempotency_key {
            payload.insert("idempotencyKey".into(), Value::String(key));
        }
        let mut v = vars();
        v.insert("input".into(), Value::Object(payload));
        let data: SendData = self.transport.execute(&query, Some(v))?;
        Ok(data.send_message)
    }

    /// Broadcast a message to every endpoint of an application.
    pub fn broadcast(&self, input: SendMessageInput) -> Result<Message, HivehookError> {
        let query = format!("mutation($input: BroadcastMessageInput!) {{ broadcastMessage(input: $input) {{ {FRAGMENT} }} }}");
        let mut payload = serde_json::Map::new();
        payload.insert(
            "applicationId".into(),
            Value::String(input.application_id),
        );
        payload.insert("eventType".into(), Value::String(input.event_type));
        payload.insert(
            "payload".into(),
            Value::String(STANDARD.encode(&input.payload)),
        );
        if let Some(key) = input.idempotency_key {
            payload.insert("idempotencyKey".into(), Value::String(key));
        }
        let mut v = vars();
        v.insert("input".into(), Value::Object(payload));
        let data: BroadcastData = self.transport.execute(&query, Some(v))?;
        Ok(data.broadcast_message)
    }

    /// Send a one-off dynamic message to a single ad-hoc URL.
    pub fn send_dynamic(
        &self,
        url: &str,
        event_type: &str,
        payload: &[u8],
        signing_secret: Option<&str>,
        headers: Option<HashMap<String, Value>>,
    ) -> Result<OutboundDelivery, HivehookError> {
        let query = format!("mutation($input: SendDynamicMessageInput!) {{ sendDynamicMessage(input: $input) {{ {DELIVERY_FRAGMENT} }} }}");
        let mut payload_obj = serde_json::Map::new();
        payload_obj.insert("url".into(), Value::String(url.into()));
        payload_obj.insert("eventType".into(), Value::String(event_type.into()));
        payload_obj.insert(
            "payload".into(),
            Value::String(STANDARD.encode(payload)),
        );
        if let Some(s) = signing_secret {
            payload_obj.insert("signingSecret".into(), Value::String(s.into()));
        }
        if let Some(h) = headers {
            payload_obj.insert("headers".into(), serde_json::to_value(h)?);
        }
        let mut v = vars();
        v.insert("input".into(), Value::Object(payload_obj));
        let data: SendDynamicData = self.transport.execute(&query, Some(v))?;
        Ok(data.send_dynamic_message)
    }
}

/// Async variant of the message service.
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub struct AsyncMessageService<'a> {
    pub(crate) transport: &'a crate::transport::AsyncGraphQLTransport,
}

#[cfg(feature = "async")]
impl<'a> AsyncMessageService<'a> {
    /// List messages.
    pub async fn list(
        &self,
        options: ListMessagesOptions,
    ) -> Result<ListResult<Message>, HivehookError> {
        let query = format!(
            r#"query($applicationId: UUID, $eventType: String, $status: MessageStatus, $search: String, $limit: Int, $offset: Int, $after: String, $first: Int) {{
                messages(applicationId: $applicationId, eventType: $eventType, status: $status, search: $search, limit: $limit, offset: $offset, after: $after, first: $first) {{
                    nodes {{ {FRAGMENT} }}
                    pageInfo {{ total limit offset endCursor hasNextPage }}
                }}
            }}"#
        );
        let mut v = vars();
        put_opt(&mut v, "applicationId", options.application_id);
        put_opt(&mut v, "eventType", options.event_type);
        put_opt(&mut v, "status", options.status);
        put_opt(&mut v, "search", options.search);
        put_opt(&mut v, "limit", options.limit);
        put_opt(&mut v, "offset", options.offset);
        put_opt(&mut v, "after", options.after);
        put_opt(&mut v, "first", options.first);
        let data: ListData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.messages)
    }

    /// Get a message by ID.
    pub async fn get(&self, id: &str) -> Result<Option<Message>, HivehookError> {
        let query = format!("query($id: UUID!) {{ message(id: $id) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: GetData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.message)
    }

    /// Send a single message to all endpoints subscribed to the event type.
    pub async fn send(&self, input: SendMessageInput) -> Result<Message, HivehookError> {
        let query = format!("mutation($input: SendMessageInput!) {{ sendMessage(input: $input) {{ {FRAGMENT} }} }}");
        let mut payload = serde_json::Map::new();
        payload.insert("applicationId".into(), Value::String(input.application_id));
        payload.insert("eventType".into(), Value::String(input.event_type));
        payload.insert(
            "payload".into(),
            Value::String(STANDARD.encode(&input.payload)),
        );
        if let Some(key) = input.idempotency_key {
            payload.insert("idempotencyKey".into(), Value::String(key));
        }
        let mut v = vars();
        v.insert("input".into(), Value::Object(payload));
        let data: SendData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.send_message)
    }

    /// Broadcast a message to every endpoint of an application.
    pub async fn broadcast(&self, input: SendMessageInput) -> Result<Message, HivehookError> {
        let query = format!("mutation($input: BroadcastMessageInput!) {{ broadcastMessage(input: $input) {{ {FRAGMENT} }} }}");
        let mut payload = serde_json::Map::new();
        payload.insert("applicationId".into(), Value::String(input.application_id));
        payload.insert("eventType".into(), Value::String(input.event_type));
        payload.insert(
            "payload".into(),
            Value::String(STANDARD.encode(&input.payload)),
        );
        if let Some(key) = input.idempotency_key {
            payload.insert("idempotencyKey".into(), Value::String(key));
        }
        let mut v = vars();
        v.insert("input".into(), Value::Object(payload));
        let data: BroadcastData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.broadcast_message)
    }

    /// Send a one-off dynamic message to a single ad-hoc URL.
    pub async fn send_dynamic(
        &self,
        url: &str,
        event_type: &str,
        payload: &[u8],
        signing_secret: Option<&str>,
        headers: Option<HashMap<String, Value>>,
    ) -> Result<OutboundDelivery, HivehookError> {
        let query = format!("mutation($input: SendDynamicMessageInput!) {{ sendDynamicMessage(input: $input) {{ {DELIVERY_FRAGMENT} }} }}");
        let mut payload_obj = serde_json::Map::new();
        payload_obj.insert("url".into(), Value::String(url.into()));
        payload_obj.insert("eventType".into(), Value::String(event_type.into()));
        payload_obj.insert("payload".into(), Value::String(STANDARD.encode(payload)));
        if let Some(s) = signing_secret {
            payload_obj.insert("signingSecret".into(), Value::String(s.into()));
        }
        if let Some(h) = headers {
            payload_obj.insert("headers".into(), serde_json::to_value(h)?);
        }
        let mut v = vars();
        v.insert("input".into(), Value::Object(payload_obj));
        let data: SendDynamicData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.send_dynamic_message)
    }
}
