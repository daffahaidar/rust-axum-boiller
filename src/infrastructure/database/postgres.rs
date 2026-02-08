use sqlx::postgres::{PgPoolOptions, PgPool};
use std::time::Duration;
use crate::infrastructure::errors::AppError;

pub struct Database {
    pub pool: PgPool,
}

impl Database {
    pub async fn new(connection_string: &str) -> Result<Self, AppError> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(3))
            .connect(connection_string)
            .await
            .map_err(AppError::DatabaseError)?;

        Ok(Self { pool })
    }
}
