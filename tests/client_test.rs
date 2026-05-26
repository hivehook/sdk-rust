#![cfg(feature = "blocking")]

use hivehook::HivehookClient;

#[test]
fn client_has_all_services() {
    let client = HivehookClient::new("http://localhost:8080", None)
        .expect("client construction should succeed");
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
