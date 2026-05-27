#![cfg(feature = "async")]

use hivehook::AsyncHivehookClient;
use serde_json::json;
use wiremock::matchers::{body_partial_json, method};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn list_all_walks_every_page() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(body_partial_json(json!({"variables": {"offset": 0}})))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            r#"{"data":{"users":{"nodes":[{"id":"u1"},{"id":"u2"}],"pageInfo":{"hasNextPage":true}}}}"#,
        ))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(body_partial_json(json!({"variables": {"offset": 2}})))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            r#"{"data":{"users":{"nodes":[{"id":"u3"}],"pageInfo":{"hasNextPage":false}}}}"#,
        ))
        .mount(&server)
        .await;

    let client = AsyncHivehookClient::new(&server.uri(), Some("hh_test".into()))
        .expect("client construction");

    let users = client
        .users()
        .list_all(Default::default())
        .await
        .expect("list_all");

    let ids: Vec<String> = users.into_iter().map(|u| u.id).collect();
    assert_eq!(ids, vec!["u1", "u2", "u3"]);
}
