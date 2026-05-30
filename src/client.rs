//! Blocking top-level [`HivehookClient`] entry point.
//!
//! Available when the `blocking` feature is enabled (on by default). For the
//! async variant see [`crate::AsyncHivehookClient`].

use crate::errors::HivehookError;
use crate::resources::*;
use crate::transport::{BlockingGraphQLTransport, DEFAULT_MAX_RETRIES};
use std::time::Duration;

/// The blocking entry point to the Hivehook API.
///
/// A single client holds a connection pool internally and is cheap to clone-by-
/// reference (via `&` borrows) into the per-resource services. All service
/// methods are synchronous `pub fn` and block the current thread.
///
/// Available when the `blocking` feature is enabled (on by default).
#[derive(Clone)]
pub struct HivehookClient {
    transport: BlockingGraphQLTransport,
}

impl std::fmt::Debug for HivehookClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HivehookClient")
            .field("transport", &self.transport)
            .finish()
    }
}

/// Builder for [`HivehookClient`] when non-default options are required.
#[derive(Debug, Clone)]
pub struct HivehookClientBuilder {
    base_url: String,
    api_key: Option<String>,
    timeout: Option<Duration>,
    max_retries: u32,
}

impl HivehookClientBuilder {
    /// Override the per-request timeout.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Override the maximum number of retry attempts (default 2). A value of
    /// `0` disables retries.
    pub fn max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    /// Finish construction.
    pub fn build(self) -> Result<HivehookClient, HivehookError> {
        Ok(HivehookClient {
            transport: BlockingGraphQLTransport::with_options(
                &self.base_url,
                self.api_key,
                self.timeout,
                self.max_retries,
            )?,
        })
    }
}

impl HivehookClient {
    /// Create a new blocking client targeting `base_url` (without the
    /// `/graphql` suffix) using the supplied API key.
    pub fn new(base_url: &str, api_key: Option<String>) -> Result<Self, HivehookError> {
        Ok(Self {
            transport: BlockingGraphQLTransport::new(base_url, api_key)?,
        })
    }

    /// Construct a builder for non-default options (timeout, retry count).
    pub fn builder(base_url: impl Into<String>, api_key: Option<String>) -> HivehookClientBuilder {
        HivehookClientBuilder {
            base_url: base_url.into(),
            api_key,
            timeout: None,
            max_retries: DEFAULT_MAX_RETRIES,
        }
    }

    /// Sources service (inbound webhook endpoints).
    pub fn sources(&self) -> sources::SourceService<'_> {
        sources::SourceService { transport: &self.transport }
    }

    /// Destinations service (outbound delivery targets).
    pub fn destinations(&self) -> destinations::DestinationService<'_> {
        destinations::DestinationService { transport: &self.transport }
    }

    /// Subscriptions service (source-to-destination wiring).
    pub fn subscriptions(&self) -> subscriptions::SubscriptionService<'_> {
        subscriptions::SubscriptionService { transport: &self.transport }
    }

    /// Events service (received inbound events).
    pub fn events(&self) -> events::EventService<'_> {
        events::EventService { transport: &self.transport }
    }

    /// Deliveries service (per-destination delivery attempts).
    pub fn deliveries(&self) -> deliveries::DeliveryService<'_> {
        deliveries::DeliveryService { transport: &self.transport }
    }

    /// Inbound dead-letter queue service.
    pub fn dlq(&self) -> dlq::DLQService<'_> {
        dlq::DLQService { transport: &self.transport }
    }

    /// API key management service.
    pub fn api_keys(&self) -> api_keys::ApiKeyService<'_> {
        api_keys::ApiKeyService { transport: &self.transport }
    }

    /// Alert-rule service.
    pub fn alert_rules(&self) -> alert_rules::AlertRuleService<'_> {
        alert_rules::AlertRuleService { transport: &self.transport }
    }

    /// Bookmark service.
    pub fn bookmarks(&self) -> bookmarks::BookmarkService<'_> {
        bookmarks::BookmarkService { transport: &self.transport }
    }

    /// Event-type schema service.
    pub fn event_type_schemas(&self) -> event_type_schemas::EventTypeSchemaService<'_> {
        event_type_schemas::EventTypeSchemaService { transport: &self.transport }
    }

    /// Outbound application service.
    pub fn applications(&self) -> applications::ApplicationService<'_> {
        applications::ApplicationService { transport: &self.transport }
    }

    /// Outbound endpoint service.
    pub fn endpoints(&self) -> endpoints::EndpointService<'_> {
        endpoints::EndpointService { transport: &self.transport }
    }

    /// Outbound message service.
    pub fn messages(&self) -> messages::MessageService<'_> {
        messages::MessageService { transport: &self.transport }
    }

    /// Outbound delivery service.
    pub fn outbound_deliveries(&self) -> outbound_deliveries::OutboundDeliveryService<'_> {
        outbound_deliveries::OutboundDeliveryService { transport: &self.transport }
    }

    /// Outbound dead-letter queue service.
    pub fn outbound_dlq(&self) -> outbound_dlq::OutboundDLQService<'_> {
        outbound_dlq::OutboundDLQService { transport: &self.transport }
    }

    /// System status service.
    pub fn status(&self) -> status::StatusService<'_> {
        status::StatusService { transport: &self.transport }
    }

    /// Transformation service.
    pub fn transformations(&self) -> transformations::TransformationService<'_> {
        transformations::TransformationService { transport: &self.transport }
    }

    /// Portal-token service.
    pub fn portal(&self) -> portal::PortalService<'_> {
        portal::PortalService { transport: &self.transport }
    }

    /// Stream service.
    pub fn streams(&self) -> streams::StreamService<'_> {
        streams::StreamService { transport: &self.transport }
    }

    /// Stream consumer service.
    pub fn stream_consumers(&self) -> stream_consumers::StreamConsumerService<'_> {
        stream_consumers::StreamConsumerService { transport: &self.transport }
    }

    /// Stream sink service.
    pub fn stream_sinks(&self) -> stream_sinks::StreamSinkService<'_> {
        stream_sinks::StreamSinkService { transport: &self.transport }
    }

    /// Organization service.
    pub fn organizations(&self) -> organizations::OrganizationService<'_> {
        organizations::OrganizationService { transport: &self.transport }
    }

    /// User-management service.
    pub fn users(&self) -> users::UserService<'_> {
        users::UserService { transport: &self.transport }
    }

    /// Audit-log service.
    pub fn audit_logs(&self) -> audit_logs::AuditLogService<'_> {
        audit_logs::AuditLogService { transport: &self.transport }
    }

    /// Meta-event webhook configuration service.
    pub fn meta_event_configs(&self) -> meta_event_configs::MetaEventConfigService<'_> {
        meta_event_configs::MetaEventConfigService { transport: &self.transport }
    }
}
