# hivehook (Rust)

Official Rust client for [Hivehook](https://hivehook.com), webhook infrastructure for modern teams (inbound and outbound).

## Install

```toml
[dependencies]
hivehook = "0.1"
```

Or:

```bash
cargo add hivehook
```

## Quick start

```rust
use hivehook::HivehookClient;
use hivehook::resources::sources::CreateSourceInput;
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = HivehookClient::new(
        "http://localhost:8080",
        Some(std::env::var("HIVEHOOK_API_KEY")?),
    )?;

    let source = client.sources().create(CreateSourceInput {
        name: "Stripe production".into(),
        slug: "stripe-prod".into(),
        provider_type: "stripe".into(),
        verify_config: Some(json!({ "secret": "whsec_..." })),
        ..Default::default()
    })?;

    println!(
        "created source {}. POST webhooks to /ingest/{}",
        source.id, source.slug
    );
    Ok(())
}
```

## Webhook signature verification

```rust
use hivehook::Webhook;

let signature = headers.get("x-hivehook-signature").unwrap();
let timestamp: i64 = headers.get("x-hivehook-timestamp").unwrap().parse().unwrap();
let ok = Webhook::verify(body, "your-signing-secret", signature, timestamp, Some(300));
```

## Documentation

See the full reference at [hivehook.com/docs](https://hivehook.com/docs).

## License

MIT. See [LICENSE](LICENSE).
