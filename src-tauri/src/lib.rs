// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use dbsql::{main, Todo};
use sqlx::Error;
pub mod dbsql;

#[tauri::command]
async fn fetch_todos() -> Result<Vec<Todo>, String> {
    Todo::fetch_all().await.map_err(|e| e.to_string()) // 转换 sqlx::Error 为 String
}

#[tauri::command]
async fn create_todo(title: String, completed: bool) -> Result<Todo, String> {
    Todo::create(&title, completed)
        .await
        .map_err(|e| e.to_string()) // 转换 sqlx::Error 为 String
}

#[tauri::command]
async fn delete_todo(id: i32) -> Result<(), String> {
    let todo = Todo::get_by_id(id).await.map_err(|e| e.to_string())?;
    todo.delete().await.map_err(|e| e.to_string()) // 转换 sqlx::Error 为 String
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    main();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            fetch_todos,
            create_todo,
            delete_todo,
            greet
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
