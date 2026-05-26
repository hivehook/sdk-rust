//! Inbound webhook signature verification.
//!
//! Hivehook signs every outbound webhook with an HMAC-SHA256 of
//! `{timestamp}.{payload}`. Use [`Webhook::verify`] from your receiver to
//! validate the signature.

use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};

type HmacSha256 = Hmac<Sha256>;

/// Static helper for signing and verifying Hivehook webhook payloads.
pub struct Webhook;

impl Webhook {
    /// Header carrying the `v1=...` HMAC signature.
    pub const HEADER_SIGNATURE: &'static str = "X-Hivehook-Signature";
    /// Header carrying the Unix timestamp used when signing.
    pub const HEADER_TIMESTAMP: &'static str = "X-Hivehook-Timestamp";
    /// Header carrying the unique message ID.
    pub const HEADER_MESSAGE_ID: &'static str = "X-Hivehook-Message-ID";

    /// Sign `payload` with `secret` and `timestamp`, producing a `v1=hex…` value
    /// suitable for the [`HEADER_SIGNATURE`](Self::HEADER_SIGNATURE) header.
    pub fn sign(payload: &str, secret: &str, timestamp: i64) -> String {
        let message = format!("{}.{}", timestamp, payload);
        let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
            .expect("HMAC accepts a key of any size");
        mac.update(message.as_bytes());
        let result = mac.finalize();
        format!("v1={}", hex::encode(result.into_bytes()))
    }

    /// Verify a signature against the supplied payload, secret, and timestamp.
    ///
    /// `signature` may be a bare `v1=hex…` value or a comma-separated list of
    /// `scheme=value` pairs as used by other webhook providers. Only the
    /// `v1` element is consulted.
    ///
    /// Tolerance semantics:
    /// - `None`: skip the freshness check entirely.
    /// - `Some(0)`: strict: any non-zero clock drift between signer and
    ///   verifier rejects the signature.
    /// - `Some(n)` with `n > 0`: signatures whose timestamp differs from the
    ///   verifier's clock by more than `n` seconds are rejected.
    ///
    /// If the system clock cannot be read, the function returns `false`
    /// rather than silently bypassing the freshness check.
    pub fn verify(
        payload: &str,
        secret: &str,
        signature: &str,
        timestamp: i64,
        tolerance_seconds: Option<i64>,
    ) -> bool {
        if let Some(tol) = tolerance_seconds {
            let now = match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(d) => d.as_secs() as i64,
                Err(_) => return false,
            };
            let age = (now - timestamp).unsigned_abs();
            let limit = tol.max(0) as u64;
            if age > limit {
                return false;
            }
        }
        let provided = match extract_v1(signature) {
            Some(v) => v,
            None => return false,
        };
        let expected = Self::sign(payload, secret, timestamp);
        constant_time_eq(expected.as_bytes(), provided.as_bytes())
    }

    /// Verify a signature against a primary secret and (optionally) a
    /// secondary rotation secret. Returns `true` if either accepts the
    /// signature.
    pub fn verify_with_rotation(
        payload: &str,
        primary: &str,
        secondary: Option<&str>,
        signature: &str,
        timestamp: i64,
        tolerance_seconds: Option<i64>,
    ) -> bool {
        if Self::verify(payload, primary, signature, timestamp, tolerance_seconds) {
            return true;
        }
        if let Some(sec) = secondary {
            return Self::verify(payload, sec, signature, timestamp, tolerance_seconds);
        }
        false
    }
}

/// Extract the canonical `v1=hex…` element from a (possibly multi-scheme)
/// signature header. Returns the full `v1=…` value, including the prefix, so
/// it can be compared directly against [`Webhook::sign`] output.
fn extract_v1(signature: &str) -> Option<String> {
    for part in signature.split(',') {
        let trimmed = part.trim();
        if let Some(rest) = trimmed.strip_prefix("v1=") {
            return Some(format!("v1={}", rest));
        }
    }
    None
}

fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut result: u8 = 0;
    for (x, y) in a.iter().zip(b.iter()) {
        result |= x ^ y;
    }
    result == 0
}
