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

async fn init_pool() -> SqlitePool {
    SqlitePool::connect("sqlite:data.db?mode=rwc")  // 持久化到文件
        .await
        .expect("Failed to create pool")
}
pub async fn get_todos() -> Result<Vec<Todo>, sqlx::Error> {
    let pool = POOL.get().expect("Database pool not initialized");
    sqlx::query_as::<_, Todo>("SELECT id, title, completed FROM todos")
        .fetch_all(pool)
        .await
}

pub fn main() {
    // 创建Tokio运行时
    let rt = Runtime::new().expect("无法创建Tokio运行时");

    // 在运行时中执行异步逻辑
    rt.block_on(async {
        let pool = POOL.get_or_init(init_pool).await;
        // 后续原有数据库操作代码...
        // 创建表
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

        // 插入todo
        sqlx::query("INSERT INTO todos (title, completed) VALUES (?, ?)")
            .bind("Buy milk")
            .bind(false)
            .execute(pool)
            .await
            .expect("插入todo失败");

        // 查询todos
        let todos = sqlx::query_as::<_, Todo>("SELECT id, title, completed FROM todos")
            .fetch_all(pool)
            .await
            .expect("查询todos失败");
        println!("Todos: {:?}", todos);

        // 更新todo
        sqlx::query("UPDATE todos SET completed = ? WHERE id = ?")
            .bind(true)
            .bind(1)
            .execute(pool)
            .await
            .expect("更新todo失败");



        // 插入用户数据
        sqlx::query("INSERT INTO users (name, age) VALUES (?, ?)")
            .bind("Alice")
            .bind(30)
            .execute(pool)
            .await
            .expect("插入用户数据失败");
    });
}
