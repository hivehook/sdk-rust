//! Organization management.

use crate::errors::HivehookError;
use crate::resources::_base::{put_opt, vars};
#[cfg(feature = "blocking")]
use crate::transport::BlockingGraphQLTransport;
use crate::types::{ListResult, Organization};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

const FRAGMENT: &str = "id name slug ssoEnabled ssoProvider retentionEvents retentionMessages otlpConfig { endpoint headers insecure sampleRate } createdAt updatedAt";

/// Options for [`OrganizationService::list`].
#[non_exhaustive]
#[derive(Debug, Default, Clone)]
pub struct ListOrganizationsOptions {
    /// Free-text search.
    pub search: Option<String>,
    /// Offset-based page size.
    pub limit: Option<i32>,
    /// Offset-based page offset.
    pub offset: Option<i32>,
}

/// Input shape for `OrganizationService::create`.
#[non_exhaustive]
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrganizationInput {
    /// Display name.
    pub name: String,
    /// URL-safe slug.
    pub slug: String,
}

/// Input shape for `OrganizationService::update`.
#[non_exhaustive]
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateOrganizationInput {
    /// New display name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// New slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
}

/// Input shape for `OrganizationService::configure_sso`.
#[non_exhaustive]
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SsoConfigInput {
    /// SSO provider name.
    pub provider: String,
    /// SAML IdP metadata URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idp_metadata_url: Option<String>,
    /// SAML entity ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    /// SAML ACS base URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acs_base_url: Option<String>,
    /// OIDC issuer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    /// OIDC client ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// OIDC client secret.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    /// OIDC redirect URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
}

/// Input shape for `OrganizationService::update_retention`.
#[non_exhaustive]
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RetentionInput {
    /// Event retention in days.
    pub retention_events: i32,
    /// Message retention in days.
    pub retention_messages: i32,
}

/// Input shape for `OrganizationService::configure_otlp`.
#[non_exhaustive]
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OtlpConfigInput {
    /// Collector endpoint.
    pub endpoint: String,
    /// Extra headers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
    /// Whether to allow insecure connections.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insecure: Option<bool>,
    /// Sampling rate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<f64>,
}

#[derive(Deserialize)]
struct ListData {
    organizations: ListResult<Organization>,
}

#[derive(Deserialize)]
struct GetData {
    organization: Option<Organization>,
}

#[derive(Deserialize)]
struct CreateData {
    #[serde(rename = "createOrganization")]
    create_organization: Organization,
}

#[derive(Deserialize)]
struct UpdateData {
    #[serde(rename = "updateOrganization")]
    update_organization: Organization,
}

#[derive(Deserialize)]
struct DeleteData {
    #[serde(rename = "deleteOrganization")]
    delete_organization: bool,
}

#[derive(Deserialize)]
struct ConfigureSsoData {
    #[serde(rename = "configureSSO")]
    configure_sso: Organization,
}

#[derive(Deserialize)]
struct DisableSsoData {
    #[serde(rename = "disableSSO")]
    disable_sso: Organization,
}

#[derive(Deserialize)]
struct UpdateRetentionData {
    #[serde(rename = "updateOrganizationRetention")]
    update_organization_retention: Organization,
}

#[derive(Deserialize)]
struct DeleteDataData {
    #[serde(rename = "deleteOrganizationData")]
    delete_organization_data: bool,
}

#[derive(Deserialize)]
struct ExportDataData {
    #[serde(rename = "exportOrganizationData")]
    export_organization_data: Option<Value>,
}

#[derive(Deserialize)]
struct ConfigureOtlpData {
    #[serde(rename = "configureOTLP")]
    configure_otlp: Organization,
}

#[derive(Deserialize)]
struct DisableOtlpData {
    #[serde(rename = "disableOTLP")]
    disable_otlp: Organization,
}

#[cfg(feature = "blocking")]
/// Service for managing [`Organization`] resources.
pub struct OrganizationService<'a> {
    pub(crate) transport: &'a BlockingGraphQLTransport,
}

#[cfg(feature = "blocking")]
impl<'a> OrganizationService<'a> {
    /// List organizations.
    pub fn list(
        &self,
        options: ListOrganizationsOptions,
    ) -> Result<ListResult<Organization>, HivehookError> {
        let query = format!(
            r#"query($search: String, $limit: Int, $offset: Int) {{
                organizations(search: $search, limit: $limit, offset: $offset) {{
                    nodes {{ {FRAGMENT} }}
                    pageInfo {{ total limit offset endCursor hasNextPage }}
                }}
            }}"#
        );
        let mut v = vars();
        put_opt(&mut v, "search", options.search);
        put_opt(&mut v, "limit", options.limit);
        put_opt(&mut v, "offset", options.offset);
        let data: ListData = self.transport.execute(&query, Some(v))?;
        Ok(data.organizations)
    }

    /// Get an organization by ID.
    pub fn get(&self, id: &str) -> Result<Option<Organization>, HivehookError> {
        let query = format!("query($id: UUID!) {{ organization(id: $id) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: GetData = self.transport.execute(&query, Some(v))?;
        Ok(data.organization)
    }

    /// Create a new organization.
    pub fn create(
        &self,
        input: CreateOrganizationInput,
    ) -> Result<Organization, HivehookError> {
        let query = format!("mutation($input: CreateOrganizationInput!) {{ createOrganization(input: $input) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: CreateData = self.transport.execute(&query, Some(v))?;
        Ok(data.create_organization)
    }

    /// Update an organization.
    pub fn update(
        &self,
        id: &str,
        input: UpdateOrganizationInput,
    ) -> Result<Organization, HivehookError> {
        let query = format!("mutation($id: UUID!, $input: UpdateOrganizationInput!) {{ updateOrganization(id: $id, input: $input) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: UpdateData = self.transport.execute(&query, Some(v))?;
        Ok(data.update_organization)
    }

    /// Delete an organization.
    pub fn delete(&self, id: &str) -> Result<bool, HivehookError> {
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: DeleteData = self.transport.execute(
            "mutation($id: UUID!) { deleteOrganization(id: $id) }",
            Some(v),
        )?;
        Ok(data.delete_organization)
    }

    /// Configure SSO for an organization.
    pub fn configure_sso(
        &self,
        organization_id: &str,
        input: SsoConfigInput,
    ) -> Result<Organization, HivehookError> {
        let query = format!("mutation($organizationId: UUID!, $input: SSOConfigInput!) {{ configureSSO(organizationId: $organizationId, input: $input) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("organizationId".into(), Value::String(organization_id.into()));
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: ConfigureSsoData = self.transport.execute(&query, Some(v))?;
        Ok(data.configure_sso)
    }

    /// Disable SSO for an organization.
    pub fn disable_sso(&self, organization_id: &str) -> Result<Organization, HivehookError> {
        let query = format!("mutation($organizationId: UUID!) {{ disableSSO(organizationId: $organizationId) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("organizationId".into(), Value::String(organization_id.into()));
        let data: DisableSsoData = self.transport.execute(&query, Some(v))?;
        Ok(data.disable_sso)
    }

    /// Update retention settings.
    pub fn update_retention(
        &self,
        organization_id: &str,
        input: RetentionInput,
    ) -> Result<Organization, HivehookError> {
        let query = format!("mutation($organizationId: UUID!, $input: RetentionInput!) {{ updateOrganizationRetention(organizationId: $organizationId, input: $input) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("organizationId".into(), Value::String(organization_id.into()));
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: UpdateRetentionData = self.transport.execute(&query, Some(v))?;
        Ok(data.update_organization_retention)
    }

    /// Delete an organization's data.
    pub fn delete_data(&self, organization_id: &str) -> Result<bool, HivehookError> {
        let mut v = vars();
        v.insert("organizationId".into(), Value::String(organization_id.into()));
        let data: DeleteDataData = self.transport.execute(
            "mutation($organizationId: UUID!) { deleteOrganizationData(organizationId: $organizationId) }",
            Some(v),
        )?;
        Ok(data.delete_organization_data)
    }

    /// Export an organization's data. Returns the raw JSON value returned by
    /// the server (shape is product-defined).
    pub fn export_data(&self, organization_id: &str) -> Result<Option<Value>, HivehookError> {
        let mut v = vars();
        v.insert("organizationId".into(), Value::String(organization_id.into()));
        let data: ExportDataData = self.transport.execute(
            "mutation($organizationId: UUID!) { exportOrganizationData(organizationId: $organizationId) }",
            Some(v),
        )?;
        Ok(data.export_organization_data)
    }

    /// Configure OpenTelemetry export.
    pub fn configure_otlp(
        &self,
        organization_id: &str,
        input: OtlpConfigInput,
    ) -> Result<Organization, HivehookError> {
        let query = format!("mutation($organizationId: UUID!, $input: OTLPConfigInput!) {{ configureOTLP(organizationId: $organizationId, input: $input) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("organizationId".into(), Value::String(organization_id.into()));
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: ConfigureOtlpData = self.transport.execute(&query, Some(v))?;
        Ok(data.configure_otlp)
    }

    /// Disable OpenTelemetry export.
    pub fn disable_otlp(&self, organization_id: &str) -> Result<Organization, HivehookError> {
        let query = format!("mutation($organizationId: UUID!) {{ disableOTLP(organizationId: $organizationId) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("organizationId".into(), Value::String(organization_id.into()));
        let data: DisableOtlpData = self.transport.execute(&query, Some(v))?;
        Ok(data.disable_otlp)
    }
}

/// Async variant of the organization service.
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub struct AsyncOrganizationService<'a> {
    pub(crate) transport: &'a crate::transport::AsyncGraphQLTransport,
}

#[cfg(feature = "async")]
impl<'a> AsyncOrganizationService<'a> {
    /// List organizations.
    pub async fn list(
        &self,
        options: ListOrganizationsOptions,
    ) -> Result<ListResult<Organization>, HivehookError> {
        let query = format!(
            r#"query($search: String, $limit: Int, $offset: Int) {{
                organizations(search: $search, limit: $limit, offset: $offset) {{
                    nodes {{ {FRAGMENT} }}
                    pageInfo {{ total limit offset endCursor hasNextPage }}
                }}
            }}"#
        );
        let mut v = vars();
        put_opt(&mut v, "search", options.search);
        put_opt(&mut v, "limit", options.limit);
        put_opt(&mut v, "offset", options.offset);
        let data: ListData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.organizations)
    }

    /// Get an organization by ID.
    pub async fn get(&self, id: &str) -> Result<Option<Organization>, HivehookError> {
        let query = format!("query($id: UUID!) {{ organization(id: $id) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: GetData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.organization)
    }

    /// Create a new organization.
    pub async fn create(
        &self,
        input: CreateOrganizationInput,
    ) -> Result<Organization, HivehookError> {
        let query = format!("mutation($input: CreateOrganizationInput!) {{ createOrganization(input: $input) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: CreateData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.create_organization)
    }

    /// Update an organization.
    pub async fn update(
        &self,
        id: &str,
        input: UpdateOrganizationInput,
    ) -> Result<Organization, HivehookError> {
        let query = format!("mutation($id: UUID!, $input: UpdateOrganizationInput!) {{ updateOrganization(id: $id, input: $input) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: UpdateData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.update_organization)
    }

    /// Delete an organization.
    pub async fn delete(&self, id: &str) -> Result<bool, HivehookError> {
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: DeleteData = self
            .transport
            .execute(
                "mutation($id: UUID!) { deleteOrganization(id: $id) }",
                Some(v),
            )
            .await?;
        Ok(data.delete_organization)
    }

    /// Configure SSO for an organization.
    pub async fn configure_sso(
        &self,
        organization_id: &str,
        input: SsoConfigInput,
    ) -> Result<Organization, HivehookError> {
        let query = format!("mutation($organizationId: UUID!, $input: SSOConfigInput!) {{ configureSSO(organizationId: $organizationId, input: $input) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("organizationId".into(), Value::String(organization_id.into()));
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: ConfigureSsoData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.configure_sso)
    }

    /// Disable SSO for an organization.
    pub async fn disable_sso(&self, organization_id: &str) -> Result<Organization, HivehookError> {
        let query = format!("mutation($organizationId: UUID!) {{ disableSSO(organizationId: $organizationId) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("organizationId".into(), Value::String(organization_id.into()));
        let data: DisableSsoData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.disable_sso)
    }

    /// Update retention settings.
    pub async fn update_retention(
        &self,
        organization_id: &str,
        input: RetentionInput,
    ) -> Result<Organization, HivehookError> {
        let query = format!("mutation($organizationId: UUID!, $input: RetentionInput!) {{ updateOrganizationRetention(organizationId: $organizationId, input: $input) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("organizationId".into(), Value::String(organization_id.into()));
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: UpdateRetentionData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.update_organization_retention)
    }

    /// Delete an organization's data.
    pub async fn delete_data(&self, organization_id: &str) -> Result<bool, HivehookError> {
        let mut v = vars();
        v.insert("organizationId".into(), Value::String(organization_id.into()));
        let data: DeleteDataData = self.transport.execute(
            "mutation($organizationId: UUID!) { deleteOrganizationData(organizationId: $organizationId) }",
            Some(v),
        ).await?;
        Ok(data.delete_organization_data)
    }

    /// Export an organization's data. Returns the raw JSON value returned by
    /// the server (shape is product-defined).
    pub async fn export_data(
        &self,
        organization_id: &str,
    ) -> Result<Option<Value>, HivehookError> {
        let mut v = vars();
        v.insert("organizationId".into(), Value::String(organization_id.into()));
        let data: ExportDataData = self.transport.execute(
            "mutation($organizationId: UUID!) { exportOrganizationData(organizationId: $organizationId) }",
            Some(v),
        ).await?;
        Ok(data.export_organization_data)
    }

    /// Configure OpenTelemetry export.
    pub async fn configure_otlp(
        &self,
        organization_id: &str,
        input: OtlpConfigInput,
    ) -> Result<Organization, HivehookError> {
        let query = format!("mutation($organizationId: UUID!, $input: OTLPConfigInput!) {{ configureOTLP(organizationId: $organizationId, input: $input) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("organizationId".into(), Value::String(organization_id.into()));
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: ConfigureOtlpData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.configure_otlp)
    }

    /// Disable OpenTelemetry export.
    pub async fn disable_otlp(
        &self,
        organization_id: &str,
    ) -> Result<Organization, HivehookError> {
        let query = format!("mutation($organizationId: UUID!) {{ disableOTLP(organizationId: $organizationId) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("organizationId".into(), Value::String(organization_id.into()));
        let data: DisableOtlpData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.disable_otlp)
    }
}
