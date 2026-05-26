//! Async top-level [`AsyncHivehookClient`] entry point.
//!
//! Available when the `async` feature is enabled (on by default). Mirrors the
//! blocking [`crate::HivehookClient`] but every per-resource method returns an
//! `Async*Service` whose methods are `async fn`. Uses [`reqwest::Client`]
//! under the hood and requires a running async runtime (typically Tokio).

use crate::errors::HivehookError;
use crate::resources::*;
use crate::transport::{AsyncGraphQLTransport, DEFAULT_MAX_RETRIES};
use std::time::Duration;

/// The async entry point to the Hivehook API.
///
/// A single client holds a connection pool internally and can be cheaply
/// borrowed (via `&` references) into per-resource async services. All
/// service methods are `async fn` and must be awaited.
///
/// Available when the `async` feature is enabled (on by default).
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
#[derive(Clone)]
pub struct AsyncHivehookClient {
    transport: AsyncGraphQLTransport,
}

impl std::fmt::Debug for AsyncHivehookClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AsyncHivehookClient")
            .field("transport", &self.transport)
            .finish()
    }
}

/// Builder for [`AsyncHivehookClient`] when non-default options are required.
#[derive(Debug, Clone)]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub struct AsyncHivehookClientBuilder {
    base_url: String,
    api_key: Option<String>,
    timeout: Option<Duration>,
    max_retries: u32,
}

impl AsyncHivehookClientBuilder {
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
    pub fn build(self) -> Result<AsyncHivehookClient, HivehookError> {
        Ok(AsyncHivehookClient {
            transport: AsyncGraphQLTransport::with_options(
                &self.base_url,
                self.api_key,
                self.timeout,
                self.max_retries,
            )?,
        })
    }
}

impl AsyncHivehookClient {
    /// Create a new async client targeting `base_url` (without the
    /// `/graphql` suffix) using the supplied API key.
    pub fn new(base_url: &str, api_key: Option<String>) -> Result<Self, HivehookError> {
        Ok(Self {
            transport: AsyncGraphQLTransport::new(base_url, api_key)?,
        })
    }

    /// Construct a builder for non-default options (timeout, retry count).
    pub fn builder(
        base_url: impl Into<String>,
        api_key: Option<String>,
    ) -> AsyncHivehookClientBuilder {
        AsyncHivehookClientBuilder {
            base_url: base_url.into(),
            api_key,
            timeout: None,
            max_retries: DEFAULT_MAX_RETRIES,
        }
    }

    /// Async sources service (inbound webhook endpoints).
    pub fn sources(&self) -> sources::AsyncSourceService<'_> {
        sources::AsyncSourceService { transport: &self.transport }
    }

    /// Async destinations service (outbound delivery targets).
    pub fn destinations(&self) -> destinations::AsyncDestinationService<'_> {
        destinations::AsyncDestinationService { transport: &self.transport }
    }

    /// Async subscriptions service (source-to-destination wiring).
    pub fn subscriptions(&self) -> subscriptions::AsyncSubscriptionService<'_> {
        subscriptions::AsyncSubscriptionService { transport: &self.transport }
    }

    /// Async events service (received inbound events).
    pub fn events(&self) -> events::AsyncEventService<'_> {
        events::AsyncEventService { transport: &self.transport }
    }

    /// Async deliveries service (per-destination delivery attempts).
    pub fn deliveries(&self) -> deliveries::AsyncDeliveryService<'_> {
        deliveries::AsyncDeliveryService { transport: &self.transport }
    }

    /// Async inbound dead-letter queue service.
    pub fn dlq(&self) -> dlq::AsyncDLQService<'_> {
        dlq::AsyncDLQService { transport: &self.transport }
    }

    /// Async API key management service.
    pub fn api_keys(&self) -> api_keys::AsyncApiKeyService<'_> {
        api_keys::AsyncApiKeyService { transport: &self.transport }
    }

    /// Async alert-rule service.
    pub fn alert_rules(&self) -> alert_rules::AsyncAlertRuleService<'_> {
        alert_rules::AsyncAlertRuleService { transport: &self.transport }
    }

    /// Async bookmark service.
    pub fn bookmarks(&self) -> bookmarks::AsyncBookmarkService<'_> {
        bookmarks::AsyncBookmarkService { transport: &self.transport }
    }

    /// Async event-type schema service.
    pub fn event_type_schemas(&self) -> event_type_schemas::AsyncEventTypeSchemaService<'_> {
        event_type_schemas::AsyncEventTypeSchemaService { transport: &self.transport }
    }

    /// Async outbound application service.
    pub fn applications(&self) -> applications::AsyncApplicationService<'_> {
        applications::AsyncApplicationService { transport: &self.transport }
    }

    /// Async outbound endpoint service.
    pub fn endpoints(&self) -> endpoints::AsyncEndpointService<'_> {
        endpoints::AsyncEndpointService { transport: &self.transport }
    }

    /// Async outbound message service.
    pub fn messages(&self) -> messages::AsyncMessageService<'_> {
        messages::AsyncMessageService { transport: &self.transport }
    }

    /// Async outbound delivery service.
    pub fn outbound_deliveries(&self) -> outbound_deliveries::AsyncOutboundDeliveryService<'_> {
        outbound_deliveries::AsyncOutboundDeliveryService { transport: &self.transport }
    }

    /// Async outbound dead-letter queue service.
    pub fn outbound_dlq(&self) -> outbound_dlq::AsyncOutboundDLQService<'_> {
        outbound_dlq::AsyncOutboundDLQService { transport: &self.transport }
    }

    /// Async system status service.
    pub fn status(&self) -> status::AsyncStatusService<'_> {
        status::AsyncStatusService { transport: &self.transport }
    }

    /// Async transformation service.
    pub fn transformations(&self) -> transformations::AsyncTransformationService<'_> {
        transformations::AsyncTransformationService { transport: &self.transport }
    }

    /// Async portal-token service.
    pub fn portal(&self) -> portal::AsyncPortalService<'_> {
        portal::AsyncPortalService { transport: &self.transport }
    }

    /// Async stream service.
    pub fn streams(&self) -> streams::AsyncStreamService<'_> {
        streams::AsyncStreamService { transport: &self.transport }
    }

    /// Async stream consumer service.
    pub fn stream_consumers(&self) -> stream_consumers::AsyncStreamConsumerService<'_> {
        stream_consumers::AsyncStreamConsumerService { transport: &self.transport }
    }

    /// Async stream sink service.
    pub fn stream_sinks(&self) -> stream_sinks::AsyncStreamSinkService<'_> {
        stream_sinks::AsyncStreamSinkService { transport: &self.transport }
    }

    /// Async organization service.
    pub fn organizations(&self) -> organizations::AsyncOrganizationService<'_> {
        organizations::AsyncOrganizationService { transport: &self.transport }
    }

    /// Async user-management service.
    pub fn users(&self) -> users::AsyncUserService<'_> {
        users::AsyncUserService { transport: &self.transport }
    }

    /// Async audit-log service.
    pub fn audit_logs(&self) -> audit_logs::AsyncAuditLogService<'_> {
        audit_logs::AsyncAuditLogService { transport: &self.transport }
    }
}
