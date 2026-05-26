//! Transport-layer integration tests using `wiremock`.
//!
//! Covers HTTP error classification (429 → `RateLimit`, 5xx → `ServerError`,
//! GraphQL `NOT_FOUND` extension propagation) and the bounded retry policy.

#![cfg(feature = "async")]

use hivehook::errors::HivehookError;
use hivehook::AsyncHivehookClient;
use std::time::Duration;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

async fn build_client(server: &MockServer, max_retries: u32) -> AsyncHivehookClient {
    AsyncHivehookClient::builder(server.uri(), Some("hh_test".into()))
        .max_retries(max_retries)
        .timeout(Duration::from_secs(5))
        .build()
        .expect("client builds")
}

/// Drive an HTTP request through the SDK via the status service.
async fn execute(client: &AsyncHivehookClient) -> Result<(), HivehookError> {
    client.status().get().await.map(|_| ())
}

#[tokio::test]
async fn rate_limit_with_retry_after_then_success() {
    let server = MockServer::start().await;

    // First response: 429 with Retry-After: 1
    Mock::given(method("POST"))
        .and(path("/graphql"))
        .respond_with(
            ResponseTemplate::new(429)
                .insert_header("Retry-After", "1")
                .set_body_string(r#"{"errors":[{"message":"slow down"}]}"#),
        )
        .up_to_n_times(1)
        .mount(&server)
        .await;

    // Second response: success with a minimal GraphQL data envelope.
    Mock::given(method("POST"))
        .and(path("/graphql"))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            r#"{"data":{"status":{"status":"healthy","version":"test"}}}"#,
        ))
        .mount(&server)
        .await;

    let client = build_client(&server, 2).await;
    let started = std::time::Instant::now();
    let res = execute(&client).await;
    let elapsed = started.elapsed();

    assert!(res.is_ok(), "expected success after retry, got {:?}", res);
    // The Retry-After header was 1s, so the elapsed time should reflect a wait.
    assert!(
        elapsed >= Duration::from_millis(900),
        "expected retry-after honored (~1s), got {:?}",
        elapsed
    );
}

#[tokio::test]
async fn rate_limit_terminal_when_retries_exhausted() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/graphql"))
        .respond_with(
            ResponseTemplate::new(429)
                .insert_header("Retry-After", "0")
                .set_body_string(r#"{"errors":[{"message":"slow down"}]}"#),
        )
        .mount(&server)
        .await;

    let client = build_client(&server, 1).await;
    let res = execute(&client).await;
    match res {
        Err(HivehookError::RateLimit { retry_after, message }) => {
            assert_eq!(retry_after, Some(Duration::from_secs(0)));
            assert!(message.contains("slow down"), "message was {message:?}");
        }
        other => panic!("expected RateLimit, got {:?}", other),
    }
}

#[tokio::test]
async fn server_error_retries_then_succeeds() {
    let server = MockServer::start().await;

    // First two responses: 503
    Mock::given(method("POST"))
        .and(path("/graphql"))
        .respond_with(
            ResponseTemplate::new(503).set_body_string(r#"{"errors":[{"message":"down"}]}"#),
        )
        .up_to_n_times(2)
        .mount(&server)
        .await;

    // Then success.
    Mock::given(method("POST"))
        .and(path("/graphql"))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            r#"{"data":{"status":{"status":"healthy","version":"test"}}}"#,
        ))
        .mount(&server)
        .await;

    let client = build_client(&server, 3).await;
    let res = execute(&client).await;
    assert!(res.is_ok(), "expected success after 2 retries: {:?}", res);
}

#[tokio::test]
async fn server_error_terminal_returns_server_error_variant() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/graphql"))
        .respond_with(
            ResponseTemplate::new(502).set_body_string(r#"{"errors":[{"message":"bad gateway"}]}"#),
        )
        .mount(&server)
        .await;

    // No retries. Fail fast.
    let client = build_client(&server, 0).await;
    let res = execute(&client).await;
    match res {
        Err(HivehookError::ServerError { status, message }) => {
            assert_eq!(status, 502);
            assert!(message.contains("bad gateway"));
        }
        other => panic!("expected ServerError, got {:?}", other),
    }
}

#[tokio::test]
async fn not_found_preserves_graphql_extensions() {
    let server = MockServer::start().await;

    let body = r#"{
        "errors": [{
            "message": "source not found",
            "extensions": {
                "code": "NOT_FOUND",
                "resource": "source",
                "id": "abc-123"
            }
        }]
    }"#;

    Mock::given(method("POST"))
        .and(path("/graphql"))
        .respond_with(ResponseTemplate::new(200).set_body_string(body))
        .mount(&server)
        .await;

    let client = build_client(&server, 0).await;
    let res = execute(&client).await;
    match res {
        Err(HivehookError::NotFound {
            message,
            extensions,
        }) => {
            assert_eq!(message, "source not found");
            let ext = extensions.expect("extensions populated");
            assert_eq!(ext.get("code").and_then(|v| v.as_str()), Some("NOT_FOUND"));
            assert_eq!(
                ext.get("resource").and_then(|v| v.as_str()),
                Some("source")
            );
            assert_eq!(ext.get("id").and_then(|v| v.as_str()), Some("abc-123"));
        }
        other => panic!("expected NotFound, got {:?}", other),
    }
}
