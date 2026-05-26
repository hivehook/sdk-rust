use hivehook::Webhook;
use std::time::{SystemTime, UNIX_EPOCH};

const SECRET: &str = "whsec_test123";
const PAYLOAD: &str = r#"{"event":"test"}"#;

fn now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

#[test]
fn header_constants() {
    assert_eq!("X-Hivehook-Signature", Webhook::HEADER_SIGNATURE);
    assert_eq!("X-Hivehook-Timestamp", Webhook::HEADER_TIMESTAMP);
    assert_eq!("X-Hivehook-Message-ID", Webhook::HEADER_MESSAGE_ID);
}

#[test]
fn sign_produces_v1_prefix() {
    let sig = Webhook::sign(PAYLOAD, SECRET, 1700000000);
    assert!(sig.starts_with("v1="));
    assert_eq!(67, sig.len());
}

#[test]
fn sign_is_deterministic() {
    let sig1 = Webhook::sign(PAYLOAD, SECRET, 1700000000);
    let sig2 = Webhook::sign(PAYLOAD, SECRET, 1700000000);
    assert_eq!(sig1, sig2);
}

#[test]
fn different_secrets_produce_different_signatures() {
    let sig1 = Webhook::sign(PAYLOAD, SECRET, 1700000000);
    let sig2 = Webhook::sign(PAYLOAD, "different", 1700000000);
    assert_ne!(sig1, sig2);
}

#[test]
fn verify_valid() {
    let ts = now();
    let sig = Webhook::sign(PAYLOAD, SECRET, ts);
    assert!(Webhook::verify(PAYLOAD, SECRET, &sig, ts, Some(300)));
}

#[test]
fn reject_wrong_secret() {
    let ts = now();
    let sig = Webhook::sign(PAYLOAD, SECRET, ts);
    assert!(!Webhook::verify(PAYLOAD, "wrong", &sig, ts, Some(300)));
}

#[test]
fn reject_expired() {
    let ts = now() - 600;
    let sig = Webhook::sign(PAYLOAD, SECRET, ts);
    assert!(!Webhook::verify(PAYLOAD, SECRET, &sig, ts, Some(300)));
}

#[test]
fn skip_timestamp_check() {
    let ts = now() - 600;
    let sig = Webhook::sign(PAYLOAD, SECRET, ts);
    assert!(Webhook::verify(PAYLOAD, SECRET, &sig, ts, None));
}

#[test]
fn rotation_primary() {
    let ts = now();
    let sig = Webhook::sign(PAYLOAD, "primary", ts);
    assert!(Webhook::verify_with_rotation(PAYLOAD, "primary", Some("secondary"), &sig, ts, Some(300)));
}

#[test]
fn rotation_secondary() {
    let ts = now();
    let sig = Webhook::sign(PAYLOAD, "secondary", ts);
    assert!(Webhook::verify_with_rotation(PAYLOAD, "primary", Some("secondary"), &sig, ts, Some(300)));
}

#[test]
fn rotation_reject() {
    let ts = now();
    assert!(!Webhook::verify_with_rotation(PAYLOAD, "primary", Some("secondary"), "v1=bad", ts, Some(300)));
}
