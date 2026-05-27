#![allow(missing_docs)]
use crate::errors::HivehookError;
use crate::types::*;
use super::alert_rules::{AlertRuleService, AsyncAlertRuleService, ListAlertRulesOptions};
use super::api_keys::{ApiKeyService, AsyncApiKeyService, ListApiKeysOptions};
use super::applications::{ApplicationService, AsyncApplicationService, ListApplicationsOptions};
use super::audit_logs::{AuditLogService, AsyncAuditLogService, ListAuditLogsOptions};
use super::bookmarks::{BookmarkService, AsyncBookmarkService, ListBookmarksOptions};
use super::deliveries::{DeliveryService, AsyncDeliveryService, ListDeliveriesOptions};
use super::destinations::{DestinationService, AsyncDestinationService, ListDestinationsOptions};
use super::dlq::{DLQService, AsyncDLQService, ListDlqOptions};
use super::endpoints::{EndpointService, AsyncEndpointService, ListEndpointsOptions};
use super::event_type_schemas::{EventTypeSchemaService, AsyncEventTypeSchemaService, ListEventTypeSchemasOptions};
use super::events::{EventService, AsyncEventService, ListEventsOptions};
use super::messages::{MessageService, AsyncMessageService, ListMessagesOptions};
use super::organizations::{OrganizationService, AsyncOrganizationService, ListOrganizationsOptions};
use super::outbound_deliveries::{OutboundDeliveryService, AsyncOutboundDeliveryService, ListOutboundDeliveriesOptions};
use super::outbound_dlq::{OutboundDLQService, AsyncOutboundDLQService, ListOutboundDlqOptions};
use super::sources::{SourceService, AsyncSourceService, ListSourcesOptions};
use super::stream_consumers::{StreamConsumerService, AsyncStreamConsumerService, ListStreamConsumersOptions};
use super::stream_sinks::{StreamSinkService, AsyncStreamSinkService, ListStreamSinksOptions};
use super::streams::{StreamService, AsyncStreamService, ListStreamsOptions};
use super::subscriptions::{SubscriptionService, AsyncSubscriptionService, ListSubscriptionsOptions};
use super::transformations::{TransformationService, AsyncTransformationService, ListTransformationsOptions};
use super::users::{UserService, AsyncUserService, ListUsersOptions};

impl<'a> AlertRuleService<'a> {
    pub fn list_all(&self, options: ListAlertRulesOptions) -> Result<Vec<AlertRule>, HivehookError> {
        let mut opts = options;
        let mut out = Vec::new();
        loop {
            let page = self.list(opts.clone())?;
            out.extend(page.nodes);
            match (page.page_info.has_next_page, page.page_info.end_cursor) {
                (true, Some(c)) => opts.after = Some(c),
                _ => break,
            }
        }
        Ok(out)
    }
}
impl<'a> AsyncAlertRuleService<'a> {
    pub async fn list_all(&self, options: ListAlertRulesOptions) -> Result<Vec<AlertRule>, HivehookError> {
        let mut opts = options;
        let mut out = Vec::new();
        loop {
            let page = self.list(opts.clone()).await?;
            out.extend(page.nodes);
            match (page.page_info.has_next_page, page.page_info.end_cursor) {
                (true, Some(c)) => opts.after = Some(c),
                _ => break,
            }
        }
        Ok(out)
    }
}

impl<'a> ApiKeyService<'a> {
    pub fn list_all(&self, options: ListApiKeysOptions) -> Result<Vec<ApiKey>, HivehookError> {
        let mut opts = options;
        let mut out = Vec::new();
        let mut offset: i32 = opts.offset.unwrap_or(0);
        loop {
            opts.offset = Some(offset);
            let page = self.list(opts.clone())?;
            let n = page.nodes.len() as i32;
            out.extend(page.nodes);
            if !page.page_info.has_next_page || n == 0 {
                break;
            }
            offset += n;
        }
        Ok(out)
    }
}
impl<'a> AsyncApiKeyService<'a> {
    pub async fn list_all(&self, options: ListApiKeysOptions) -> Result<Vec<ApiKey>, HivehookError> {
        let mut opts = options;
        let mut out = Vec::new();
        let mut offset: i32 = opts.offset.unwrap_or(0);
        loop {
            opts.offset = Some(offset);
            let page = self.list(opts.clone()).await?;
            let n = page.nodes.len() as i32;
            out.extend(page.nodes);
            if !page.page_info.has_next_page || n == 0 {
                break;
            }
            offset += n;
        }
        Ok(out)
    }
}

impl<'a> ApplicationService<'a> {
    pub fn list_all(&self, options: ListApplicationsOptions) -> Result<Vec<Application>, HivehookError> {
        let mut opts = options;
        let mut out = Vec::new();
        loop {
            let page = self.list(opts.clone())?;
            out.extend(page.nodes);
            match (page.page_info.has_next_page, page.page_info.end_cursor) {
                (true, Some(c)) => opts.after = Some(c),
                _ => break,
            }
        }
        Ok(out)
    }
}
impl<'a> AsyncApplicationService<'a> {
    pub async fn list_all(&self, options: ListApplicationsOptions) -> Result<Vec<Application>, HivehookError> {
        let mut opts = options;
        let mut out = Vec::new();
        loop {
            let page = self.list(opts.clone()).await?;
            out.extend(page.nodes);
            match (page.page_info.has_next_page, page.page_info.end_cursor) {
                (true, Some(c)) => opts.after = Some(c),
                _ => break,
            }
        }
        Ok(out)
    }
}

impl<'a> AuditLogService<'a> {
    pub fn list_all(&self, options: ListAuditLogsOptions) -> Result<Vec<AuditLog>, HivehookError> {
        let mut opts = options;
        let mut out = Vec::new();
        loop {
            let page = self.list(opts.clone())?;
            out.extend(page.nodes);
            match (page.page_info.has_next_page, page.page_info.end_cursor) {
                (true, Some(c)) => opts.after = Some(c),
                _ => break,
            }
        }
        Ok(out)
    }
}
impl<'a> AsyncAuditLogService<'a> {
    pub async fn list_all(&self, options: ListAuditLogsOptions) -> Result<Vec<AuditLog>, HivehookError> {
        let mut opts = options;
        let mut out = Vec::new();
        loop {
            let page = self.list(opts.clone()).await?;
            out.extend(page.nodes);
            match (page.page_info.has_next_page, page.page_info.end_cursor) {
                (true, Some(c)) => opts.after = Some(c),
                _ => break,
            }
        }
        Ok(out)
    }
}

impl<'a> BookmarkService<'a> {
    pub fn list_all(&self, options: ListBookmarksOptions) -> Result<Vec<Bookmark>, HivehookError> {
        let mut opts = options;
        let mut out = Vec::new();
        loop {
            let page = self.list(opts.clone())?;
            out.extend(page.nodes);
            match (page.page_info.has_next_page, page.page_info.end_cursor) {
                (true, Some(c)) => opts.after = Some(c),
                _ => break,
            }
        }
        Ok(out)
    }
}
impl<'a> AsyncBookmarkService<'a> {
    pub async fn list_all(&self, options: ListBookmarksOptions) -> Result<Vec<Bookmark>, HivehookError> {
        let mut opts = options;
        let mut out = Vec::new();
        loop {
            let page = self.list(opts.clone()).await?;
            out.extend(page.nodes);
            match (page.page_info.has_next_page, page.page_info.end_cursor) {
                (true, Some(c)) => opts.after = Some(c),
                _ => break,
            }
        }
        Ok(out)
    }
}

impl<'a> DeliveryService<'a> {
    pub fn list_all(&self, options: ListDeliveriesOptions) -> Result<Vec<Delivery>, HivehookError> {
        let mut opts = options;
        let mut out = Vec::new();
        loop {
            let page = self.list(opts.clone())?;
            out.extend(page.nodes);
            match (page.page_info.has_next_page, page.page_info.end_cursor) {
                (true, Some(c)) => opts.after = Some(c),
                _ => break,
            }
        }
        Ok(out)
    }
}
impl<'a> AsyncDeliveryService<'a> {
    pub async fn list_all(&self, options: ListDeliveriesOptions) -> Result<Vec<Delivery>, HivehookError> {
        let mut opts = options;
        let mut out = Vec::new();
        loop {
            let page = self.list(opts.clone()).await?;
            out.extend(page.nodes);
            match (page.page_info.has_next_page, page.page_info.end_cursor) {
                (true, Some(c)) => opts.after = Some(c),
                _ => break,
            }
        }
        Ok(out)
    }
}

impl<'a> DestinationService<'a> {
    pub fn list_all(&self, options: ListDestinationsOptions) -> Result<Vec<Destination>, HivehookError> {
        let mut opts = options;
        let mut out = Vec::new();
        loop {
            let page = self.list(opts.clone())?;
            out.extend(page.nodes);
            match (page.page_info.has_next_page, page.page_info.end_cursor) {
                (true, Some(c)) => opts.after = Some(c),
                _ => break,
            }
        }
        Ok(out)
    }
}
impl<'a> AsyncDestinationService<'a> {
    pub async fn list_all(&self, options: ListDestinationsOptions) -> Result<Vec<Destination>, HivehookError> {
        let mut opts = options;
        let mut out = Vec::new();
        loop {
            let page = self.list(opts.clone()).await?;
            out.extend(page.nodes);
            match (page.page_info.has_next_page, page.page_info.end_cursor) {
                (true, Some(c)) => opts.after = Some(c),
                _ => break,
            }
        }
        Ok(out)
    }
}

impl<'a> DLQService<'a> {
    pub fn list_all(&self, options: ListDlqOptions) -> Result<Vec<DlqEntry>, HivehookError> {
        let mut opts = options;
        let mut out = Vec::new();
        loop {
            let page = self.list(opts.clone())?;
            out.extend(page.nodes);
            match (page.page_info.has_next_page, page.page_info.end_cursor) {
                (true, Some(c)) => opts.after = Some(c),
                _ => break,
            }
        }
        Ok(out)
    }
}
impl<'a> AsyncDLQService<'a> {
    pub async fn list_all(&self, options: ListDlqOptions) -> Result<Vec<DlqEntry>, HivehookError> {
        let mut opts = options;
        let mut out = Vec::new();
        loop {
            let page = self.list(opts.clone()).await?;
            out.extend(page.nodes);
            match (page.page_info.has_next_page, page.page_info.end_cursor) {
                (true, Some(c)) => opts.after = Some(c),
                _ => break,
            }
        }
        Ok(out)
    }
}

impl<'a> EndpointService<'a> {
    pub fn list_all(&self, options: ListEndpointsOptions) -> Result<Vec<Endpoint>, HivehookError> {
        let mut opts = options;
        let mut out = Vec::new();
        loop {
            let page = self.list(opts.clone())?;
            out.extend(page.nodes);
            match (page.page_info.has_next_page, page.page_info.end_cursor) {
                (true, Some(c)) => opts.after = Some(c),
                _ => break,
            }
        }
        Ok(out)
    }
}
impl<'a> AsyncEndpointService<'a> {
    pub async fn list_all(&self, options: ListEndpointsOptions) -> Result<Vec<Endpoint>, HivehookError> {
        let mut opts = options;
        let mut out = Vec::new();
        loop {
            let page = self.list(opts.clone()).await?;
            out.extend(page.nodes);
            match (page.page_info.has_next_page, page.page_info.end_cursor) {
                (true, Some(c)) => opts.after = Some(c),
                _ => break,
            }
        }
        Ok(out)
    }
}

impl<'a> EventTypeSchemaService<'a> {
    pub fn list_all(&self, options: ListEventTypeSchemasOptions) -> Result<Vec<EventTypeSchema>, HivehookError> {
        let mut opts = options;
        let mut out = Vec::new();
        loop {
            let page = self.list(opts.clone())?;
            out.extend(page.nodes);
            match (page.page_info.has_next_page, page.page_info.end_cursor) {
                (true, Some(c)) => opts.after = Some(c),
                _ => break,
            }
        }
        Ok(out)
    }
}
impl<'a> AsyncEventTypeSchemaService<'a> {
    pub async fn list_all(&self, options: ListEventTypeSchemasOptions) -> Result<Vec<EventTypeSchema>, HivehookError> {
        let mut opts = options;
        let mut out = Vec::new();
        loop {
            let page = self.list(opts.clone()).await?;
            out.extend(page.nodes);
            match (page.page_info.has_next_page, page.page_info.end_cursor) {
                (true, Some(c)) => opts.after = Some(c),
                _ => break,
            }
        }
        Ok(out)
    }
}

impl<'a> EventService<'a> {
    pub fn list_all(&self, options: ListEventsOptions) -> Result<Vec<Event>, HivehookError> {
        let mut opts = options;
        let mut out = Vec::new();
        loop {
            let page = self.list(opts.clone())?;
            out.extend(page.nodes);
            match (page.page_info.has_next_page, page.page_info.end_cursor) {
                (true, Some(c)) => opts.after = Some(c),
                _ => break,
            }
        }
        Ok(out)
    }
}
impl<'a> AsyncEventService<'a> {
    pub async fn list_all(&self, options: ListEventsOptions) -> Result<Vec<Event>, HivehookError> {
        let mut opts = options;
        let mut out = Vec::new();
        loop {
            let page = self.list(opts.clone()).await?;
            out.extend(page.nodes);
            match (page.page_info.has_next_page, page.page_info.end_cursor) {
                (true, Some(c)) => opts.after = Some(c),
                _ => break,
            }
        }
        Ok(out)
    }
}

impl<'a> MessageService<'a> {
    pub fn list_all(&self, options: ListMessagesOptions) -> Result<Vec<Message>, HivehookError> {
        let mut opts = options;
        let mut out = Vec::new();
        loop {
            let page = self.list(opts.clone())?;
            out.extend(page.nodes);
            match (page.page_info.has_next_page, page.page_info.end_cursor) {
                (true, Some(c)) => opts.after = Some(c),
                _ => break,
            }
        }
        Ok(out)
    }
}
impl<'a> AsyncMessageService<'a> {
    pub async fn list_all(&self, options: ListMessagesOptions) -> Result<Vec<Message>, HivehookError> {
        let mut opts = options;
        let mut out = Vec::new();
        loop {
            let page = self.list(opts.clone()).await?;
            out.extend(page.nodes);
            match (page.page_info.has_next_page, page.page_info.end_cursor) {
                (true, Some(c)) => opts.after = Some(c),
                _ => break,
            }
        }
        Ok(out)
    }
}

impl<'a> OrganizationService<'a> {
    pub fn list_all(&self, options: ListOrganizationsOptions) -> Result<Vec<Organization>, HivehookError> {
        let mut opts = options;
        let mut out = Vec::new();
        let mut offset: i32 = opts.offset.unwrap_or(0);
        loop {
            opts.offset = Some(offset);
            let page = self.list(opts.clone())?;
            let n = page.nodes.len() as i32;
            out.extend(page.nodes);
            if !page.page_info.has_next_page || n == 0 {
                break;
            }
            offset += n;
        }
        Ok(out)
    }
}
impl<'a> AsyncOrganizationService<'a> {
    pub async fn list_all(&self, options: ListOrganizationsOptions) -> Result<Vec<Organization>, HivehookError> {
        let mut opts = options;
        let mut out = Vec::new();
        let mut offset: i32 = opts.offset.unwrap_or(0);
        loop {
            opts.offset = Some(offset);
            let page = self.list(opts.clone()).await?;
            let n = page.nodes.len() as i32;
            out.extend(page.nodes);
            if !page.page_info.has_next_page || n == 0 {
                break;
            }
            offset += n;
        }
        Ok(out)
    }
}

impl<'a> OutboundDeliveryService<'a> {
    pub fn list_all(&self, options: ListOutboundDeliveriesOptions) -> Result<Vec<OutboundDelivery>, HivehookError> {
        let mut opts = options;
        let mut out = Vec::new();
        loop {
            let page = self.list(opts.clone())?;
            out.extend(page.nodes);
            match (page.page_info.has_next_page, page.page_info.end_cursor) {
                (true, Some(c)) => opts.after = Some(c),
                _ => break,
            }
        }
        Ok(out)
    }
}
impl<'a> AsyncOutboundDeliveryService<'a> {
    pub async fn list_all(&self, options: ListOutboundDeliveriesOptions) -> Result<Vec<OutboundDelivery>, HivehookError> {
        let mut opts = options;
        let mut out = Vec::new();
        loop {
            let page = self.list(opts.clone()).await?;
            out.extend(page.nodes);
            match (page.page_info.has_next_page, page.page_info.end_cursor) {
                (true, Some(c)) => opts.after = Some(c),
                _ => break,
            }
        }
        Ok(out)
    }
}

impl<'a> OutboundDLQService<'a> {
    pub fn list_all(&self, options: ListOutboundDlqOptions) -> Result<Vec<OutboundDlqEntry>, HivehookError> {
        let mut opts = options;
        let mut out = Vec::new();
        loop {
            let page = self.list(opts.clone())?;
            out.extend(page.nodes);
            match (page.page_info.has_next_page, page.page_info.end_cursor) {
                (true, Some(c)) => opts.after = Some(c),
                _ => break,
            }
        }
        Ok(out)
    }
}
impl<'a> AsyncOutboundDLQService<'a> {
    pub async fn list_all(&self, options: ListOutboundDlqOptions) -> Result<Vec<OutboundDlqEntry>, HivehookError> {
        let mut opts = options;
        let mut out = Vec::new();
        loop {
            let page = self.list(opts.clone()).await?;
            out.extend(page.nodes);
            match (page.page_info.has_next_page, page.page_info.end_cursor) {
                (true, Some(c)) => opts.after = Some(c),
                _ => break,
            }
        }
        Ok(out)
    }
}

impl<'a> SourceService<'a> {
    pub fn list_all(&self, options: ListSourcesOptions) -> Result<Vec<Source>, HivehookError> {
        let mut opts = options;
        let mut out = Vec::new();
        loop {
            let page = self.list(opts.clone())?;
            out.extend(page.nodes);
            match (page.page_info.has_next_page, page.page_info.end_cursor) {
                (true, Some(c)) => opts.after = Some(c),
                _ => break,
            }
        }
        Ok(out)
    }
}
impl<'a> AsyncSourceService<'a> {
    pub async fn list_all(&self, options: ListSourcesOptions) -> Result<Vec<Source>, HivehookError> {
        let mut opts = options;
        let mut out = Vec::new();
        loop {
            let page = self.list(opts.clone()).await?;
            out.extend(page.nodes);
            match (page.page_info.has_next_page, page.page_info.end_cursor) {
                (true, Some(c)) => opts.after = Some(c),
                _ => break,
            }
        }
        Ok(out)
    }
}

impl<'a> StreamConsumerService<'a> {
    pub fn list_all(&self, stream_id: &str, options: ListStreamConsumersOptions) -> Result<Vec<StreamConsumer>, HivehookError> {
        let mut opts = options;
        let mut out = Vec::new();
        loop {
            let page = self.list(stream_id, opts.clone())?;
            out.extend(page.nodes);
            match (page.page_info.has_next_page, page.page_info.end_cursor) {
                (true, Some(c)) => opts.after = Some(c),
                _ => break,
            }
        }
        Ok(out)
    }
}
impl<'a> AsyncStreamConsumerService<'a> {
    pub async fn list_all(&self, stream_id: &str, options: ListStreamConsumersOptions) -> Result<Vec<StreamConsumer>, HivehookError> {
        let mut opts = options;
        let mut out = Vec::new();
        loop {
            let page = self.list(stream_id, opts.clone()).await?;
            out.extend(page.nodes);
            match (page.page_info.has_next_page, page.page_info.end_cursor) {
                (true, Some(c)) => opts.after = Some(c),
                _ => break,
            }
        }
        Ok(out)
    }
}

impl<'a> StreamSinkService<'a> {
    pub fn list_all(&self, stream_id: &str, options: ListStreamSinksOptions) -> Result<Vec<StreamSink>, HivehookError> {
        let mut opts = options;
        let mut out = Vec::new();
        loop {
            let page = self.list(stream_id, opts.clone())?;
            out.extend(page.nodes);
            match (page.page_info.has_next_page, page.page_info.end_cursor) {
                (true, Some(c)) => opts.after = Some(c),
                _ => break,
            }
        }
        Ok(out)
    }
}
impl<'a> AsyncStreamSinkService<'a> {
    pub async fn list_all(&self, stream_id: &str, options: ListStreamSinksOptions) -> Result<Vec<StreamSink>, HivehookError> {
        let mut opts = options;
        let mut out = Vec::new();
        loop {
            let page = self.list(stream_id, opts.clone()).await?;
            out.extend(page.nodes);
            match (page.page_info.has_next_page, page.page_info.end_cursor) {
                (true, Some(c)) => opts.after = Some(c),
                _ => break,
            }
        }
        Ok(out)
    }
}

impl<'a> StreamService<'a> {
    pub fn list_all(&self, options: ListStreamsOptions) -> Result<Vec<Stream>, HivehookError> {
        let mut opts = options;
        let mut out = Vec::new();
        loop {
            let page = self.list(opts.clone())?;
            out.extend(page.nodes);
            match (page.page_info.has_next_page, page.page_info.end_cursor) {
                (true, Some(c)) => opts.after = Some(c),
                _ => break,
            }
        }
        Ok(out)
    }
}
impl<'a> AsyncStreamService<'a> {
    pub async fn list_all(&self, options: ListStreamsOptions) -> Result<Vec<Stream>, HivehookError> {
        let mut opts = options;
        let mut out = Vec::new();
        loop {
            let page = self.list(opts.clone()).await?;
            out.extend(page.nodes);
            match (page.page_info.has_next_page, page.page_info.end_cursor) {
                (true, Some(c)) => opts.after = Some(c),
                _ => break,
            }
        }
        Ok(out)
    }
}

impl<'a> SubscriptionService<'a> {
    pub fn list_all(&self, options: ListSubscriptionsOptions) -> Result<Vec<Subscription>, HivehookError> {
        let mut opts = options;
        let mut out = Vec::new();
        loop {
            let page = self.list(opts.clone())?;
            out.extend(page.nodes);
            match (page.page_info.has_next_page, page.page_info.end_cursor) {
                (true, Some(c)) => opts.after = Some(c),
                _ => break,
            }
        }
        Ok(out)
    }
}
impl<'a> AsyncSubscriptionService<'a> {
    pub async fn list_all(&self, options: ListSubscriptionsOptions) -> Result<Vec<Subscription>, HivehookError> {
        let mut opts = options;
        let mut out = Vec::new();
        loop {
            let page = self.list(opts.clone()).await?;
            out.extend(page.nodes);
            match (page.page_info.has_next_page, page.page_info.end_cursor) {
                (true, Some(c)) => opts.after = Some(c),
                _ => break,
            }
        }
        Ok(out)
    }
}

impl<'a> TransformationService<'a> {
    pub fn list_all(&self, options: ListTransformationsOptions) -> Result<Vec<Transformation>, HivehookError> {
        let mut opts = options;
        let mut out = Vec::new();
        loop {
            let page = self.list(opts.clone())?;
            out.extend(page.nodes);
            match (page.page_info.has_next_page, page.page_info.end_cursor) {
                (true, Some(c)) => opts.after = Some(c),
                _ => break,
            }
        }
        Ok(out)
    }
}
impl<'a> AsyncTransformationService<'a> {
    pub async fn list_all(&self, options: ListTransformationsOptions) -> Result<Vec<Transformation>, HivehookError> {
        let mut opts = options;
        let mut out = Vec::new();
        loop {
            let page = self.list(opts.clone()).await?;
            out.extend(page.nodes);
            match (page.page_info.has_next_page, page.page_info.end_cursor) {
                (true, Some(c)) => opts.after = Some(c),
                _ => break,
            }
        }
        Ok(out)
    }
}

impl<'a> UserService<'a> {
    pub fn list_all(&self, options: ListUsersOptions) -> Result<Vec<User>, HivehookError> {
        let mut opts = options;
        let mut out = Vec::new();
        let mut offset: i32 = opts.offset.unwrap_or(0);
        loop {
            opts.offset = Some(offset);
            let page = self.list(opts.clone())?;
            let n = page.nodes.len() as i32;
            out.extend(page.nodes);
            if !page.page_info.has_next_page || n == 0 {
                break;
            }
            offset += n;
        }
        Ok(out)
    }
}
impl<'a> AsyncUserService<'a> {
    pub async fn list_all(&self, options: ListUsersOptions) -> Result<Vec<User>, HivehookError> {
        let mut opts = options;
        let mut out = Vec::new();
        let mut offset: i32 = opts.offset.unwrap_or(0);
        loop {
            opts.offset = Some(offset);
            let page = self.list(opts.clone()).await?;
            let n = page.nodes.len() as i32;
            out.extend(page.nodes);
            if !page.page_info.has_next_page || n == 0 {
                break;
            }
            offset += n;
        }
        Ok(out)
    }
}

