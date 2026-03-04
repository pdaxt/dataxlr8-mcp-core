use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::time::Duration;
use tracing::info;

/// Shared PostgreSQL connection pool for all DataXLR8 MCP servers.
///
/// All MCPs connect to the same PostgreSQL database but operate on
/// their own schema namespace (e.g., `crm.*`, `sales.*`, `projects.*`).
#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    /// Create a new database connection pool.
    ///
    /// Connects to PostgreSQL using the provided URL.
    /// Default pool size: 5 connections (suitable for an MCP server).
    /// Connection timeout: 10 seconds.
    pub async fn connect(database_url: &str) -> Result<Self, crate::McpError> {
        info!("Connecting to database...");

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(10))
            .connect(database_url)
            .await
            .map_err(|e| crate::McpError::database(format!("Failed to connect: {e}")))?;

        info!("Database connected");
        Ok(Self { pool })
    }

    /// Get a reference to the connection pool.
    /// Use this in tool handlers to execute queries.
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Run raw SQL for schema setup.
    /// Each MCP calls this with its own CREATE TABLE statements.
    ///
    /// Uses a transaction to ensure all statements execute atomically.
    /// Unlike `sqlx::raw_sql().execute()` which only runs the first statement,
    /// this splits on semicolons and runs each statement in a transaction.
    pub async fn execute_raw(&self, sql: &str) -> Result<(), crate::McpError> {
        info!("Executing schema setup...");

        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| crate::McpError::database(format!("Failed to begin transaction: {e}")))?;

        for statement in sql.split(';') {
            let trimmed = statement.trim();
            if trimmed.is_empty() {
                continue;
            }
            sqlx::query(trimmed)
                .execute(&mut *tx)
                .await
                .map_err(|e| {
                    crate::McpError::database(format!("Schema setup failed on statement: {e}"))
                })?;
        }

        tx.commit()
            .await
            .map_err(|e| crate::McpError::database(format!("Failed to commit transaction: {e}")))?;

        info!("Schema setup complete");
        Ok(())
    }

    /// Quick health check — verifies the database is reachable.
    /// Returns Ok(()) if a simple query succeeds, Err otherwise.
    pub async fn health_check(&self) -> Result<(), crate::McpError> {
        sqlx::query("SELECT 1")
            .execute(&self.pool)
            .await
            .map_err(|e| crate::McpError::database(format!("Health check failed: {e}")))?;
        Ok(())
    }

    /// Close the connection pool gracefully.
    pub async fn close(&self) {
        self.pool.close().await;
        info!("Database connection closed");
    }
}
