//! DataXLR8 MCP Core — shared library for all DataXLR8 Rust MCP servers.
//!
//! Provides:
//! - Database connection pool management (PostgreSQL via sqlx)
//! - Standard error types with MCP error codes
//! - Configuration loading from environment variables
//! - Tracing/logging initialization
//! - Common response builders

pub mod config;
pub mod db;
pub mod error;
pub mod logging;

pub use config::Config;
pub use db::Database;
pub use error::{ErrorCode, McpError, McpResult};
