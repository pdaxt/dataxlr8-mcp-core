//! DataXLR8 MCP Core — shared library for all DataXLR8 Rust MCP servers.
//!
//! Provides:
//! - Database connection pool management (PostgreSQL via sqlx)
//! - Standard error types with MCP error codes
//! - Configuration loading from environment variables
//! - Tracing/logging initialization
//! - Shared MCP tool helpers (schema builders, result constructors, arg extractors)
//! - Common data types (PersonData, CompanyData, EmailVerification)

pub mod config;
pub mod db;
pub mod error;
pub mod logging;
pub mod mcp;
pub mod types;

pub use config::Config;
pub use db::Database;
pub use error::{ErrorCode, McpError, McpResult};

// Re-export commonly used types so downstream crates don't need
// to add sqlx, chrono, uuid as direct dependencies for basic usage.
pub use sqlx::PgPool;
