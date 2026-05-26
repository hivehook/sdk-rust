//! Resource services for the Hivehook API.
//!
//! Each submodule exposes a `*Service<'a>` borrowing the shared transport
//! held by [`crate::HivehookClient`]. Methods return typed structs from
//! [`crate::types`].

pub(crate) mod _base;

pub mod alert_rules;
pub mod api_keys;
pub mod applications;
pub mod audit_logs;
pub mod bookmarks;
pub mod deliveries;
pub mod destinations;
pub mod dlq;
pub mod endpoints;
pub mod event_type_schemas;
pub mod events;
pub mod messages;
pub mod organizations;
pub mod outbound_deliveries;
pub mod outbound_dlq;
pub mod portal;
pub mod sources;
pub mod status;
pub mod stream_consumers;
pub mod stream_sinks;
pub mod streams;
pub mod subscriptions;
pub mod transformations;
pub mod users;
