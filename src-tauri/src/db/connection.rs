// src/db/connection.rs
use sqlx::SqlitePool;
use tokio::sync::OnceCell;

static POOL: OnceCell<SqlitePool> = OnceCell::const_new();

pub async fn init_pool(db_url: &str) -> Result<(), sqlx::Error> {
    let pool = SqlitePool::connect(db_url).await?;
    POOL.set(pool)
        .map_err(|_| sqlx::Error::PoolClosed)
}

pub fn get_pool() -> Result<&'static SqlitePool, sqlx::Error> {
    POOL.get().ok_or(sqlx::Error::PoolClosed)
}