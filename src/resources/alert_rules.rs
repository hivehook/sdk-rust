//! Alert-rule management.

use crate::errors::HivehookError;
use crate::resources::_base::{put_opt, vars};
#[cfg(feature = "blocking")]
use crate::transport::BlockingGraphQLTransport;
use crate::types::{AlertRule, EmailAlertConfig, ListResult, SlackAlertConfig};
use serde::{Deserialize, Serialize};
use serde_json::Value;

const FRAGMENT: &str = "id name conditionType threshold webhookUrl channel emailConfig { to subjectTemplate } slackConfig { webhookUrl channel } cooldown enabled createdAt";

/// Options for [`AlertRuleService::list`].
#[non_exhaustive]
#[derive(Debug, Default, Clone)]
pub struct ListAlertRulesOptions {
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

/// Input shape for `AlertRuleService::create`.
#[non_exhaustive]
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAlertRuleInput {
    /// Human-readable name.
    pub name: String,
    /// Type of condition that triggers the alert.
    pub condition_type: String,
    /// Numeric threshold.
    pub threshold: i32,
    /// Cooldown between firings (Go duration).
    pub cooldown: String,
    /// Whether the rule starts enabled.
    pub enabled: bool,
    /// Webhook URL for the `WEBHOOK` channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<String>,
    /// Output channel (`WEBHOOK`, `EMAIL`, `SLACK`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    /// Email channel configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_config: Option<EmailAlertConfig>,
    /// Slack channel configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slack_config: Option<SlackAlertConfig>,
}

/// Input shape for `AlertRuleService::update`.
#[non_exhaustive]
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAlertRuleInput {
    /// New name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// New condition type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_type: Option<String>,
    /// New threshold.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<i32>,
    /// New webhook URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<String>,
    /// New channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    /// New email config.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_config: Option<EmailAlertConfig>,
    /// New slack config.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slack_config: Option<SlackAlertConfig>,
    /// New cooldown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cooldown: Option<String>,
    /// New enabled state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Deserialize)]
struct ListData {
    #[serde(rename = "alertRules")]
    alert_rules: ListResult<AlertRule>,
}

#[derive(Deserialize)]
struct GetData {
    #[serde(rename = "alertRule")]
    alert_rule: Option<AlertRule>,
}

#[derive(Deserialize)]
struct CreateData {
    #[serde(rename = "createAlertRule")]
    create_alert_rule: AlertRule,
}

#[derive(Deserialize)]
struct UpdateData {
    #[serde(rename = "updateAlertRule")]
    update_alert_rule: AlertRule,
}

#[derive(Deserialize)]
struct DeleteData {
    #[serde(rename = "deleteAlertRule")]
    delete_alert_rule: bool,
}

#[derive(Deserialize)]
struct TestData {
    #[serde(rename = "testAlertRule")]
    test_alert_rule: bool,
}

#[cfg(feature = "blocking")]
/// Service for managing [`AlertRule`] resources.
pub struct AlertRuleService<'a> {
    pub(crate) transport: &'a BlockingGraphQLTransport,
}

#[cfg(feature = "blocking")]
impl<'a> AlertRuleService<'a> {
    /// List alert rules.
    pub fn list(
        &self,
        options: ListAlertRulesOptions,
    ) -> Result<ListResult<AlertRule>, HivehookError> {
        let query = format!(
            r#"query($enabled: Boolean, $search: String, $limit: Int, $offset: Int, $after: String, $first: Int) {{
                alertRules(enabled: $enabled, search: $search, limit: $limit, offset: $offset, after: $after, first: $first) {{
                    nodes {{ {FRAGMENT} }}
                    pageInfo {{ total limit offset endCursor hasNextPage }}
                }}
            }}"#
        );
        let mut v = vars();
        put_opt(&mut v, "enabled", options.enabled);
        put_opt(&mut v, "search", options.search);
        put_opt(&mut v, "limit", options.limit);
        put_opt(&mut v, "offset", options.offset);
        put_opt(&mut v, "after", options.after);
        put_opt(&mut v, "first", options.first);
        let data: ListData = self.transport.execute(&query, Some(v))?;
        Ok(data.alert_rules)
    }

    /// Get an alert rule by ID.
    pub fn get(&self, id: &str) -> Result<Option<AlertRule>, HivehookError> {
        let query = format!("query($id: UUID!) {{ alertRule(id: $id) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: GetData = self.transport.execute(&query, Some(v))?;
        Ok(data.alert_rule)
    }

    /// Create a new alert rule.
    pub fn create(&self, input: CreateAlertRuleInput) -> Result<AlertRule, HivehookError> {
        let query = format!(
            "mutation($input: CreateAlertRuleInput!) {{ createAlertRule(input: $input) {{ {FRAGMENT} }} }}"
        );
        let mut v = vars();
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: CreateData = self.transport.execute(&query, Some(v))?;
        Ok(data.create_alert_rule)
    }

    /// Update an existing alert rule.
    pub fn update(
        &self,
        id: &str,
        input: UpdateAlertRuleInput,
    ) -> Result<AlertRule, HivehookError> {
        let query = format!(
            "mutation($id: UUID!, $input: UpdateAlertRuleInput!) {{ updateAlertRule(id: $id, input: $input) {{ {FRAGMENT} }} }}"
        );
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: UpdateData = self.transport.execute(&query, Some(v))?;
        Ok(data.update_alert_rule)
    }

    /// Delete an alert rule.
    pub fn delete(&self, id: &str) -> Result<bool, HivehookError> {
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: DeleteData = self
            .transport
            .execute("mutation($id: UUID!) { deleteAlertRule(id: $id) }", Some(v))?;
        Ok(data.delete_alert_rule)
    }

    /// Trigger a test firing of an alert rule.
    pub fn test(&self, id: &str) -> Result<bool, HivehookError> {
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: TestData = self
            .transport
            .execute("mutation($id: UUID!) { testAlertRule(id: $id) }", Some(v))?;
        Ok(data.test_alert_rule)
    }
}

/// Async variant of the alert-rule service.
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub struct AsyncAlertRuleService<'a> {
    pub(crate) transport: &'a crate::transport::AsyncGraphQLTransport,
}

#[cfg(feature = "async")]
impl<'a> AsyncAlertRuleService<'a> {
    /// List alert rules.
    pub async fn list(
        &self,
        options: ListAlertRulesOptions,
    ) -> Result<ListResult<AlertRule>, HivehookError> {
        let query = format!(
            r#"query($enabled: Boolean, $search: String, $limit: Int, $offset: Int, $after: String, $first: Int) {{
                alertRules(enabled: $enabled, search: $search, limit: $limit, offset: $offset, after: $after, first: $first) {{
                    nodes {{ {FRAGMENT} }}
                    pageInfo {{ total limit offset endCursor hasNextPage }}
                }}
            }}"#
        );
        let mut v = vars();
        put_opt(&mut v, "enabled", options.enabled);
        put_opt(&mut v, "search", options.search);
        put_opt(&mut v, "limit", options.limit);
        put_opt(&mut v, "offset", options.offset);
        put_opt(&mut v, "after", options.after);
        put_opt(&mut v, "first", options.first);
        let data: ListData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.alert_rules)
    }

    /// Get an alert rule by ID.
    pub async fn get(&self, id: &str) -> Result<Option<AlertRule>, HivehookError> {
        let query = format!("query($id: UUID!) {{ alertRule(id: $id) {{ {FRAGMENT} }} }}");
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: GetData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.alert_rule)
    }

    /// Create a new alert rule.
    pub async fn create(&self, input: CreateAlertRuleInput) -> Result<AlertRule, HivehookError> {
        let query = format!(
            "mutation($input: CreateAlertRuleInput!) {{ createAlertRule(input: $input) {{ {FRAGMENT} }} }}"
        );
        let mut v = vars();
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: CreateData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.create_alert_rule)
    }

    /// Update an existing alert rule.
    pub async fn update(
        &self,
        id: &str,
        input: UpdateAlertRuleInput,
    ) -> Result<AlertRule, HivehookError> {
        let query = format!(
            "mutation($id: UUID!, $input: UpdateAlertRuleInput!) {{ updateAlertRule(id: $id, input: $input) {{ {FRAGMENT} }} }}"
        );
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        v.insert("input".into(), serde_json::to_value(input)?);
        let data: UpdateData = self.transport.execute(&query, Some(v)).await?;
        Ok(data.update_alert_rule)
    }

    /// Delete an alert rule.
    pub async fn delete(&self, id: &str) -> Result<bool, HivehookError> {
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: DeleteData = self
            .transport
            .execute("mutation($id: UUID!) { deleteAlertRule(id: $id) }", Some(v))
            .await?;
        Ok(data.delete_alert_rule)
    }

    /// Trigger a test firing of an alert rule.
    pub async fn test(&self, id: &str) -> Result<bool, HivehookError> {
        let mut v = vars();
        v.insert("id".into(), Value::String(id.into()));
        let data: TestData = self
            .transport
            .execute("mutation($id: UUID!) { testAlertRule(id: $id) }", Some(v))
            .await?;
        Ok(data.test_alert_rule)
    }
}
