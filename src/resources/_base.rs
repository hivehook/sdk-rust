//! Shared helpers used by every resource service.

use serde::Serialize;
use serde_json::{Map, Value};

/// Insert `value` into `vars` under `key` if it is `Some(_)`.
///
/// Mirrors `build_list_vars(...)` in the Python SDK: optional arguments that
/// are not provided are simply omitted from the variables object.
pub(crate) fn put_opt<T: Serialize>(vars: &mut Map<String, Value>, key: &str, value: Option<T>) {
    if let Some(v) = value {
        if let Ok(serialized) = serde_json::to_value(v) {
            vars.insert(key.to_string(), serialized);
        }
    }
}

/// Convenience constructor for an empty variables map.
pub(crate) fn vars() -> Map<String, Value> {
    Map::new()
}
