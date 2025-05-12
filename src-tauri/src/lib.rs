// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use dbsql::{get_todos, main, Todo};
use tokio::task::futures;


pub mod dbsql;

#[tauri::command]
async fn fetch_todos() -> Result<Vec<Todo>, String> {
    dbsql::get_todos()
        .await
        .map_err(|e| e.to_string())  // 转换 sqlx::Error 为 String
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    main();
    let rt = tokio::runtime::Runtime::new().unwrap();
    println!("Todos result: {:?}", rt.block_on(get_todos()));

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![fetch_todos, greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
