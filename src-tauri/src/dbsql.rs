use sqlx::Row;
use sqlx::SqlitePool;
use tokio::runtime::Runtime;
use tokio::sync::OnceCell;

static POOL: OnceCell<SqlitePool> = OnceCell::const_new();

#[derive(sqlx::FromRow, Debug, serde::Serialize, serde::Deserialize)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}
// 在 Todo 结构体下增加 CRUD 实现
impl Todo {
    /// 获取所有 Todo
    pub async fn fetch_all() -> Result<Vec<Self>, sqlx::Error> {
        let pool = POOL.get().expect("Database pool not initialized");
        sqlx::query_as::<_, Self>("SELECT * FROM todos")
            .fetch_all(pool)
            .await
    }

    /// 根据 ID 获取单个 Todo
    pub async fn get_by_id(id: i32) -> Result<Self, sqlx::Error> {
        let pool = POOL.get().expect("Database pool not initialized");
        sqlx::query_as::<_, Self>("SELECT * FROM todos WHERE id = ?")
            .bind(id)
            .fetch_one(pool)
            .await
    }

    /// 创建新 Todo（返回插入后的完整记录）
    pub async fn create(title: &str, completed: bool) -> Result<Self, sqlx::Error> {
        let pool = POOL.get().expect("Database pool not initialized");
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
        let pool = POOL.get().expect("Database pool not initialized");
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
        let pool = POOL.get().expect("Database pool not initialized");
        sqlx::query("DELETE FROM todos WHERE id = ?")
            .bind(self.id)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// 批量删除已完成 Todo
    pub async fn delete_completed() -> Result<u64, sqlx::Error> {
        let pool = POOL.get().expect("Database pool not initialized");
        let result = sqlx::query("DELETE FROM todos WHERE completed = 1")
            .execute(pool)
            .await?;
        Ok(result.rows_affected())
    }
}

// 修改 main 函数测试新方法
pub fn main() {
    let rt = Runtime::new().expect("无法创建Tokio运行时");

    rt.block_on(async {
        let pool = POOL.get_or_init(init_pool).await;
        initialize_database(pool).await;

        // 测试 CRUD 流程
        let new_todo = Todo::create("Learn Rust", false).await.expect("创建失败");
        println!("Created: {:?}", new_todo);

        let mut todo = Todo::get_by_id(new_todo.id).await.expect("查询失败");
        println!("Fetched: {:?}", todo);

        todo.completed = true;
        todo.update().await.expect("更新失败");
        println!("Updated: {:?}", todo);

        todo.delete().await.expect("删除失败");
        println!("Deleted ID: {}", todo.id);
    });
}
pub async fn init_pool() -> SqlitePool {
    SqlitePool::connect("sqlite:data.db?mode=rwc") // 持久化到文件
        .await
        .expect("Failed to create pool")
}
// pub async fn get_todos() -> Result<Vec<Todo>, sqlx::Error> {
//     let pool = POOL.get().expect("Database pool not initialized");
//     sqlx::query_as::<_, Todo>("SELECT id, title, completed FROM todos")
//         .fetch_all(pool)
//         .await
// }
// async fn fetch_todos(pool: &SqlitePool) {
//     let todos = sqlx::query_as::<_, Todo>("SELECT * FROM todos")
//         .fetch_all(pool)
//         .await
//         .expect("查询todos失败");
//     println!("Current todos: {:?}", todos);
// }

// async fn update_todo(pool: &SqlitePool, todo_id: i32, completed: bool) {
//     sqlx::query("UPDATE todos SET completed = ? WHERE id = ?")
//         .bind(completed)
//         .bind(todo_id)
//         .execute(pool)
//         .await
//         .expect("更新todo失败");
// }
async fn initialize_database(pool: &SqlitePool) {
    create_tables(pool).await;
    insert_sample_data(pool).await;
}

async fn create_tables(pool: &SqlitePool) {
    sqlx::query(
        r#"
            CREATE TABLE IF NOT EXISTS todos (
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                completed BOOLEAN NOT NULL DEFAULT 0
            );

            CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                age INTEGER NOT NULL
            )
        "#,
    )
    .execute(pool)
    .await
    .expect("创建表失败");
}

async fn insert_sample_data(pool: &SqlitePool) {
    insert_sample_todos(pool).await;
    insert_sample_users(pool).await;
}
async fn insert_sample_todos(pool: &SqlitePool) {
    sqlx::query("INSERT INTO todos (title, completed) VALUES (?, ?)")
        .bind("Buy milk")
        .bind(false)
        .execute(pool)
        .await
        .expect("插入todo失败");
}

async fn insert_sample_users(pool: &SqlitePool) {
    sqlx::query("INSERT INTO users (name, age) VALUES (?, ?)")
        .bind("Alice")
        .bind(30)
        .execute(pool)
        .await
        .expect("插入用户数据失败");
}
// pub fn main() {
//     // 创建Tokio运行时
//     let rt = Runtime::new().expect("无法创建Tokio运行时");

//     // 在运行时中执行异步逻辑
//     rt.block_on(async {
//         let pool = POOL.get_or_init(init_pool).await;
//         // 初始化流程
//         initialize_database(pool).await;

//         // 查询todos
//         let todos = sqlx::query_as::<_, Todo>("SELECT id, title, completed FROM todos")
//             .fetch_all(pool)
//             .await
//             .expect("查询todos失败");
//         println!("Todos: {:?}", todos);

//         fetch_todos(pool).await;
//         update_todo(pool, 1, true).await;
//         fetch_todos(pool).await;
//     });
// }
