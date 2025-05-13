// src/db/migration.rs
use sqlx::{SqlitePool, Executor};

pub async fn run_migrations(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    pool.execute(include_str!("./schema.sql")).await?;
    Ok(())
}