//! Smoke tests for the async client surface.
//!
//! These do not hit any network. They only verify that an
//! [`AsyncHivehookClient`] can be constructed and that every per-resource
//! service accessor is reachable.

#![cfg(feature = "async")]

use hivehook::AsyncHivehookClient;

#[tokio::test]
async fn async_client_exposes_all_services() {
    let client = AsyncHivehookClient::new("https://api.example.com", Some("hh_test".into()))
        .expect("client construction");

    // Touch every accessor so a removed service triggers a compile error.
    let _ = client.sources();
    let _ = client.destinations();
    let _ = client.subscriptions();
    let _ = client.events();
    let _ = client.deliveries();
    let _ = client.dlq();
    let _ = client.api_keys();
    let _ = client.alert_rules();
    let _ = client.bookmarks();
    let _ = client.event_type_schemas();
    let _ = client.applications();
    let _ = client.endpoints();
    let _ = client.messages();
    let _ = client.outbound_deliveries();
    let _ = client.outbound_dlq();
    let _ = client.status();
    let _ = client.transformations();
    let _ = client.portal();
    let _ = client.streams();
    let _ = client.stream_consumers();
    let _ = client.stream_sinks();
    let _ = client.organizations();
    let _ = client.users();
    let _ = client.audit_logs();
}

#[tokio::test]
async fn async_client_accepts_no_api_key() {
    let _ = AsyncHivehookClient::new("https://api.example.com", None)
        .expect("client construction without api key");
}
