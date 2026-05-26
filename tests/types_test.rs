//! Smoke tests for serde parsing of API-shape JSON into the typed structs.

use hivehook::types::{
    Destination, FilterConfig, FilterRule, ListResult, Source, Subscription, SystemStatus,
};

#[test]
fn parse_source_from_camel_case() {
    let json = serde_json::json!({
        "id": "s_1",
        "name": "Stripe webhooks",
        "slug": "stripe-prod",
        "providerType": "stripe",
        "verifyConfig": {"secret": "whsec_xxx"},
        "status": "active",
        "rateLimitRps": 100,
        "spikeProtection": true,
        "maxIngestRps": 200,
        "createdAt": "2026-01-01T00:00:00Z",
    });
    let s: Source = serde_json::from_value(json).expect("source parses");
    assert_eq!(s.id, "s_1");
    assert_eq!(s.provider_type, "stripe");
    assert!(s.spike_protection);
    assert_eq!(s.max_ingest_rps, 200);
    assert!(s.verify_config.is_some());
}

#[test]
fn parse_destination_with_renamed_type_field() {
    let json = serde_json::json!({
        "id": "d_1",
        "name": "Customer URL",
        "url": "https://example.com/hook",
        "signingSecret": "whsec",
        "status": "active",
        "type": "HTTP",
        "timeoutMs": 5000,
        "ordered": false,
        "createdAt": "2026-01-01T00:00:00Z",
    });
    let d: Destination = serde_json::from_value(json).expect("destination parses");
    assert_eq!(d.type_, "HTTP");
    assert_eq!(d.timeout_ms, 5000);
    assert_eq!(d.health_score, 1.0); // default
    assert_eq!(d.output_format, "default"); // default
}

#[test]
fn parse_list_result() {
    let json = serde_json::json!({
        "nodes": [
            {"id": "s_1", "name": "a", "slug": "a", "providerType": "generic", "status": "active", "createdAt": ""},
            {"id": "s_2", "name": "b", "slug": "b", "providerType": "generic", "status": "active", "createdAt": ""},
        ],
        "pageInfo": {"total": 2, "limit": 50, "offset": 0, "endCursor": null, "hasNextPage": false}
    });
    let lr: ListResult<Source> = serde_json::from_value(json).expect("list parses");
    assert_eq!(lr.nodes.len(), 2);
    assert_eq!(lr.page_info.total, 2);
    assert!(!lr.page_info.has_next_page);
}

#[test]
fn parse_subscription_with_nested_filter_rules() {
    let json = serde_json::json!({
        "id": "sub_1",
        "name": "prod-only",
        "sourceId": "s_1",
        "destinationId": "d_1",
        "filterConfig": {
            "eventTypes": ["charge.succeeded"],
            "rules": [
                {"operator": "and", "rules": [
                    {"operator": "eq", "path": "$.env", "value": "prod"}
                ]}
            ]
        },
        "transformConfig": null,
        "enabled": true,
        "createdAt": ""
    });
    let sub: Subscription = serde_json::from_value(json).expect("subscription parses");
    let fc: &FilterConfig = sub.filter_config.as_ref().expect("filter config");
    let rules: &Vec<FilterRule> = fc.rules.as_ref().expect("rules");
    assert_eq!(rules[0].operator, "and");
    let nested = rules[0].rules.as_ref().expect("nested rules");
    assert_eq!(nested[0].path.as_deref(), Some("$.env"));
}

#[test]
fn parse_system_status() {
    let json = serde_json::json!({
        "status": "healthy",
        "dlqSize": 0,
        "outboundDlqSize": 2,
        "queueDepth": 10,
        "activeWorkers": 4,
        "totalWorkers": 8,
        "uptime": 3600,
        "version": "1.2.3",
        "sourcesTotal": 5,
        "destinationsTotal": 10,
        "subscriptionsTotal": 15,
        "eventsTotal": 1000,
        "eventsFailed": 1,
        "deliveriesTotal": 2000,
        "deliveriesPending": 5,
        "deliveriesDelivered": 1995,
        "messagesTotal": 0,
        "outboundDeliveriesTotal": 0,
        "outboundDeliveriesPending": 0,
        "outboundDeliveriesFailed": 0
    });
    let s: SystemStatus = serde_json::from_value(json).expect("status parses");
    assert_eq!(s.status, "healthy");
    assert_eq!(s.version, "1.2.3");
    assert_eq!(s.deliveries_delivered, 1995);
}
