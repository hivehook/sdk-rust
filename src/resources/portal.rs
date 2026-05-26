//! Portal token issuance.

use crate::errors::HivehookError;
use crate::resources::_base::vars;
#[cfg(feature = "blocking")]
use crate::transport::BlockingGraphQLTransport;
use crate::types::PortalToken;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
struct GenerateData {
    #[serde(rename = "generatePortalToken")]
    generate_portal_token: PortalToken,
}

#[cfg(feature = "blocking")]
/// Service for issuing portal access tokens.
pub struct PortalService<'a> {
    pub(crate) transport: &'a BlockingGraphQLTransport,
}

#[cfg(feature = "blocking")]
impl<'a> PortalService<'a> {
    /// Generate a short-lived portal token scoped to a single application.
    pub fn generate_token(&self, application_id: &str) -> Result<PortalToken, HivehookError> {
        let mut v = vars();
        v.insert("applicationId".into(), Value::String(application_id.into()));
        let data: GenerateData = self.transport.execute(
            "mutation($applicationId: UUID!) { generatePortalToken(applicationId: $applicationId) { token expiresAt } }",
            Some(v),
        )?;
        Ok(data.generate_portal_token)
    }
}

/// Async variant of the portal service.
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub struct AsyncPortalService<'a> {
    pub(crate) transport: &'a crate::transport::AsyncGraphQLTransport,
}

#[cfg(feature = "async")]
impl<'a> AsyncPortalService<'a> {
    /// Generate a short-lived portal token scoped to a single application.
    pub async fn generate_token(
        &self,
        application_id: &str,
    ) -> Result<PortalToken, HivehookError> {
        let mut v = vars();
        v.insert("applicationId".into(), Value::String(application_id.into()));
        let data: GenerateData = self.transport.execute(
            "mutation($applicationId: UUID!) { generatePortalToken(applicationId: $applicationId) { token expiresAt } }",
            Some(v),
        ).await?;
        Ok(data.generate_portal_token)
    }
}
