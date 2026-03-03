use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tracing::info;

/// Shared PostgreSQL connection pool for all DataXLR8 MCP servers.
///
/// All MCPs connect to the same PostgreSQL database but operate on
/// their own schema namespace (e.g., `crm.*`, `sales.*`, `projects.*`).
pub struct Database {
    pool: PgPool,
}

impl Database {
    /// Create a new database connection pool.
    ///
    /// Connects to PostgreSQL using the provided URL.
    /// Default pool size: 5 connections (suitable for an MCP server).
    pub async fn connect(database_url: &str) -> Result<Self, crate::McpError> {
        info!("Connecting to database...");

        let pool = PgPoolOptions::new()
            .max_connections(5)
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
    pub async fn execute_raw(&self, sql: &str) -> Result<(), crate::McpError> {
        info!("Executing schema setup...");
        sqlx::raw_sql(sql)
            .execute(&self.pool)
            .await
            .map_err(|e| crate::McpError::database(format!("Schema setup failed: {e}")))?;
        info!("Schema setup complete");
        Ok(())
    }

    /// Close the connection pool gracefully.
    pub async fn close(&self) {
        self.pool.close().await;
        info!("Database connection closed");
    }
}

impl Clone for Database {
    fn clone(&self) -> Self {
        Self {
            pool: self.pool.clone(),
        }
    }
}
