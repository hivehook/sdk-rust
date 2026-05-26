//! System status service.

use crate::types::SystemStatus;
use serde::Deserialize;

const STATUS_QUERY: &str = r#"query {
    status {
        status dlqSize outboundDlqSize queueDepth activeWorkers totalWorkers uptime version
        sourcesTotal destinationsTotal subscriptionsTotal eventsTotal eventsFailed
        deliveriesTotal deliveriesPending deliveriesDelivered
        messagesTotal outboundDeliveriesTotal outboundDeliveriesPending outboundDeliveriesFailed
    }
}"#;

#[derive(Deserialize)]
pub(crate) struct StatusData {
    pub(crate) status: SystemStatus,
}
#[cfg(feature = "blocking")]
/// Blocking variant of the status service.
pub struct StatusService<'a> {
    pub(crate) transport: &'a crate::transport::BlockingGraphQLTransport,
}

#[cfg(feature = "blocking")]
impl<'a> StatusService<'a> {
    /// Fetch the current system status.
    pub fn get(&self) -> Result<SystemStatus, crate::HivehookError> {
        let data: StatusData = self.transport.execute(STATUS_QUERY, None)?;
        Ok(data.status)
    }
}

/// Async variant of the status service.
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub struct AsyncStatusService<'a> {
    pub(crate) transport: &'a crate::transport::AsyncGraphQLTransport,
}

#[cfg(feature = "async")]
impl<'a> AsyncStatusService<'a> {
    /// Fetch the current system status.
    pub async fn get(&self) -> Result<SystemStatus, crate::HivehookError> {
        let data: StatusData = self.transport.execute(STATUS_QUERY, None).await?;
        Ok(data.status)
    }
}
