use serde::Serialize;
use thiserror::Error;

/// Standard error codes for MCP tool failures.
/// Maps directly from the TypeScript `ErrorCode` enum in mcp-base.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorCode {
    InvalidInput,
    NotFound,
    Unauthorized,
    RateLimited,
    Internal,
    ExternalService,
    DatabaseError,
    ConfigError,
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidInput => write!(f, "INVALID_INPUT"),
            Self::NotFound => write!(f, "NOT_FOUND"),
            Self::Unauthorized => write!(f, "UNAUTHORIZED"),
            Self::RateLimited => write!(f, "RATE_LIMITED"),
            Self::Internal => write!(f, "INTERNAL"),
            Self::ExternalService => write!(f, "EXTERNAL_SERVICE"),
            Self::DatabaseError => write!(f, "DATABASE_ERROR"),
            Self::ConfigError => write!(f, "CONFIG_ERROR"),
        }
    }
}

/// Structured error for MCP tool handlers.
/// Carries a machine-readable code and optional details.
#[derive(Debug, Error)]
#[error("[{code}] {message}")]
pub struct McpError {
    pub code: ErrorCode,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

impl McpError {
    pub fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            details: None,
        }
    }

    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = Some(details);
        self
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::NotFound, message)
    }

    pub fn invalid_input(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::InvalidInput, message)
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::Internal, message)
    }

    pub fn database(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::DatabaseError, message)
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::Unauthorized, message)
    }

    pub fn external_service(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::ExternalService, message)
    }
}

impl From<sqlx::Error> for McpError {
    fn from(err: sqlx::Error) -> Self {
        Self::database(err.to_string())
    }
}

impl From<serde_json::Error> for McpError {
    fn from(err: serde_json::Error) -> Self {
        Self::invalid_input(format!("JSON error: {err}"))
    }
}

/// Convenience Result type for MCP operations.
pub type McpResult<T> = Result<T, McpError>;
