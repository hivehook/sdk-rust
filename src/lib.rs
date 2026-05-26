//! # Hivehook Rust SDK
//!
//! Typed Rust client for the [Hivehook](https://hivehook.com) GraphQL API.
//!
//! ## Quick start
//!
//! ```rust,no_run
//! # #[cfg(feature = "blocking")]
//! # fn run() -> Result<(), hivehook::HivehookError> {
//! use hivehook::HivehookClient;
//! use hivehook::resources::sources::CreateSourceInput;
//! use serde_json::json;
//!
//! let client = HivehookClient::new(
//!     "http://localhost:8080",
//!     Some("hh_xxx".into()),
//! )?;
//!
//! let mut input = CreateSourceInput::default();
//! input.name = "Stripe production".into();
//! input.slug = "stripe-prod".into();
//! input.provider_type = "stripe".into();
//! input.verify_config = Some(json!({ "secret": "whsec_..." }));
//! let source = client.sources().create(input)?;
//!
//! println!("created source {}. POST webhooks to /ingest/{}", source.id, source.slug);
//! # Ok(()) }
//! ```
//!
//! Resource services live under [`resources`] and return typed structs from
//! [`types`]. Inbound webhook signature verification lives in [`webhook`].

#![warn(missing_docs)]

#[cfg(feature = "async")]
pub mod async_client;
#[cfg(feature = "blocking")]
pub mod client;
pub mod errors;
pub mod resources;
pub mod transport;
pub mod types;
pub mod webhook;

#[cfg(feature = "async")]
pub use async_client::AsyncHivehookClient;
#[cfg(feature = "blocking")]
pub use client::HivehookClient;
pub use errors::HivehookError;
pub use webhook::Webhook;
