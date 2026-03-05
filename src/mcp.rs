//! Shared MCP tool helpers used by all DataXLR8 MCP servers.
//!
//! Provides schema builders, result constructors, and argument extractors
//! so downstream MCPs don't duplicate this boilerplate.

use rmcp::model::{CallToolResult, Content};
use serde::Serialize;
use std::sync::Arc;

// ============================================================================
// Schema builders
// ============================================================================

/// Build a JSON Schema object for a tool's input_schema.
///
/// # Example
/// ```ignore
/// make_schema(
///     serde_json::json!({
///         "name": { "type": "string", "description": "The name" }
///     }),
///     vec!["name"],
/// )
/// ```
pub fn make_schema(
    properties: serde_json::Value,
    required: Vec<&str>,
) -> Arc<serde_json::Map<String, serde_json::Value>> {
    let mut m = serde_json::Map::new();
    m.insert(
        "type".to_string(),
        serde_json::Value::String("object".to_string()),
    );
    m.insert("properties".to_string(), properties);
    if !required.is_empty() {
        m.insert(
            "required".to_string(),
            serde_json::Value::Array(
                required
                    .into_iter()
                    .map(|s| serde_json::Value::String(s.to_string()))
                    .collect(),
            ),
        );
    }
    Arc::new(m)
}

/// Build an empty JSON Schema object (no properties, no required fields).
/// Used for tools that take no arguments.
pub fn empty_schema() -> Arc<serde_json::Map<String, serde_json::Value>> {
    let mut m = serde_json::Map::new();
    m.insert(
        "type".to_string(),
        serde_json::Value::String("object".to_string()),
    );
    Arc::new(m)
}

// ============================================================================
// Result constructors
// ============================================================================

/// Serialize data to pretty JSON and wrap in a successful CallToolResult.
pub fn json_result<T: Serialize>(data: &T) -> CallToolResult {
    match serde_json::to_string_pretty(data) {
        Ok(json) => CallToolResult::success(vec![Content::text(json)]),
        Err(e) => CallToolResult::error(vec![Content::text(format!(
            "Serialization error: {e}"
        ))]),
    }
}

/// Wrap an error message in a CallToolResult.
pub fn error_result(msg: &str) -> CallToolResult {
    CallToolResult::error(vec![Content::text(msg.to_string())])
}

// ============================================================================
// Argument extractors
// ============================================================================

/// Extract a string value from JSON arguments by key.
pub fn get_str(args: &serde_json::Value, key: &str) -> Option<String> {
    args.get(key).and_then(|v| v.as_str()).map(String::from)
}

/// Extract an i64 value from JSON arguments by key.
pub fn get_i64(args: &serde_json::Value, key: &str) -> Option<i64> {
    args.get(key).and_then(|v| v.as_i64())
}

/// Extract an f64 value from JSON arguments by key.
pub fn get_f64(args: &serde_json::Value, key: &str) -> Option<f64> {
    args.get(key).and_then(|v| v.as_f64())
}

/// Extract a bool value from JSON arguments by key.
pub fn get_bool(args: &serde_json::Value, key: &str) -> Option<bool> {
    args.get(key).and_then(|v| v.as_bool())
}

/// Extract an array of strings from JSON arguments by key.
/// Returns an empty Vec if the key is missing or not an array.
pub fn get_str_array(args: &serde_json::Value, key: &str) -> Vec<String> {
    args.get(key)
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default()
}
