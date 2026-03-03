use std::env;

/// Base configuration shared by all DataXLR8 MCP servers.
///
/// Each MCP can extend this with its own fields.
/// Loads from environment variables (with .env file support via dotenvy).
#[derive(Debug, Clone)]
pub struct Config {
    /// PostgreSQL connection URL.
    /// Format: postgres://user:pass@host:port/dbname
    pub database_url: String,

    /// Log level filter (e.g. "info", "debug", "dataxlr8=trace").
    pub log_level: String,

    /// Server name (used in logging and MCP server info).
    pub server_name: String,
}

impl Config {
    /// Load configuration from environment variables.
    /// Call `dotenvy::dotenv().ok()` before this to load .env files.
    pub fn from_env(server_name: impl Into<String>) -> Result<Self, crate::McpError> {
        let database_url = env::var("DATABASE_URL").map_err(|_| {
            crate::McpError::new(
                crate::ErrorCode::ConfigError,
                "DATABASE_URL environment variable is required",
            )
        })?;

        let log_level = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());

        Ok(Self {
            database_url,
            log_level,
            server_name: server_name.into(),
        })
    }
}
