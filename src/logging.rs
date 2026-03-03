use tracing_subscriber::{fmt, EnvFilter};

/// Initialize tracing/logging for an MCP server.
///
/// Logs are written to stderr (stdout is reserved for MCP protocol).
/// Format: structured text with timestamps.
/// Level controlled by RUST_LOG env var (default: "info").
pub fn init(log_level: &str) {
    let filter = EnvFilter::try_new(log_level).unwrap_or_else(|_| EnvFilter::new("info"));

    fmt()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)
        .with_target(true)
        .with_thread_ids(false)
        .with_file(false)
        .with_line_number(false)
        .init();
}
