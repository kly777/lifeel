// src/models/todo.rs
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use crate::db::connection::get_pool;
use sqlx::Row;

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}

impl Todo {
  /// 获取所有 Todo
  pub async fn fetch_all() -> Result<Vec<Self>, sqlx::Error> {
      let pool =get_pool()?;
      sqlx::query_as::<_, Self>("SELECT * FROM todos")
          .fetch_all(pool)
          .await
  }

  /// 根据 ID 获取单个 Todo
  pub async fn get_by_id(id: i32) -> Result<Self, sqlx::Error> {
      let pool = get_pool()?;
      sqlx::query_as::<_, Self>("SELECT * FROM todos WHERE id = ?")
          .bind(id)
          .fetch_one(pool)
          .await
  }

  /// 创建新 Todo（返回插入后的完整记录）
  pub async fn create(title: &str, completed: bool) -> Result<Self, sqlx::Error> {
      let pool = get_pool()?;
      let result = sqlx::query(
          "INSERT INTO todos (title, completed) VALUES (?, ?) RETURNING id, title, completed",
      )
      .bind(title)
      .bind(completed)
      .fetch_one(pool)
      .await?;

      Ok(Self {
          id: result.get("id"),
          title: result.get("title"),
          completed: result.get("completed"),
      })
  }

  /// 更新 Todo 状态
  pub async fn update(&self) -> Result<(), sqlx::Error> {
      let pool = get_pool()?;
      sqlx::query("UPDATE todos SET title = ?, completed = ? WHERE id = ?")
          .bind(&self.title)
          .bind(self.completed)
          .bind(self.id)
          .execute(pool)
          .await?;
      Ok(())
  }

  /// 删除当前 Todo
  pub async fn delete(&self) -> Result<(), sqlx::Error> {
      let pool = get_pool()?;
      sqlx::query("DELETE FROM todos WHERE id = ?")
          .bind(self.id)
          .execute(pool)
          .await?;
      Ok(())
  }

  /// 批量删除已完成 Todo
  pub async fn delete_completed() -> Result<u64, sqlx::Error> {
      let pool = get_pool()?;
      let result = sqlx::query("DELETE FROM todos WHERE completed = 1")
          .execute(pool)
          .await?;
      Ok(result.rows_affected())
  }
}