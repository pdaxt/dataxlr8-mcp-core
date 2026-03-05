# dataxlr8-mcp-core

Shared core library for all DataXLR8 Rust MCP servers.

## What It Does

Provides the common foundation that every DataXLR8 MCP server depends on — database connection pooling, configuration, error handling, logging, and MCP tool helpers. Instead of each server reimplementing boilerplate, they import this crate and get a consistent, tested base.

## Modules

| Module | Description |
|--------|-------------|
| `config` | Configuration loading from environment variables |
| `db` | PostgreSQL connection pool management via sqlx |
| `error` | Standard error types with MCP error codes (`McpError`, `McpResult`) |
| `logging` | Tracing/logging initialization |
| `mcp` | Shared MCP tool helpers — schema builders, result constructors, arg extractors |
| `types` | Common data types (`PersonData`, `CompanyData`, `EmailVerification`) |

## Quick Start

```bash
# Add as a dependency in your MCP server's Cargo.toml
[dependencies]
dataxlr8-mcp-core = { path = "../dataxlr8-mcp-core" }
```

```bash
# Build
cargo build

# Required environment variable
export DATABASE_URL=postgres://user:pass@localhost:5432/dataxlr8
```

## Usage

```rust
use dataxlr8_mcp_core::{Config, Database, McpError, McpResult};

let config = Config::from_env()?;
let db = Database::connect(&config.database_url).await?;
```

## Part of the [DataXLR8](https://github.com/pdaxt) Platform
