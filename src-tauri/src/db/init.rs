// src/main.rs
use tauri::async_runtime::block_on;
use crate::db::{connection, migration};

#[tokio::main]
pub async fn init() -> Result<(), Box<dyn std::error::Error>> {
    connection::init_pool("sqlite:data.db?mode=rwc").await?;
    migration::run_migrations(connection::get_pool()?).await?;
    Ok(())
}