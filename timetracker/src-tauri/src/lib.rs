mod db;

use tauri::{Manager, State};
use tauri_plugin_store::StoreExt;
use uuid::Uuid;

use crate::db::{activities, Database, UuidGenerator};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn db_test(db: State<'_, Database>) -> Result<i64, String> {
    db.test().await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn create_event(db: State<'_, Database>, name: String) -> Result<Uuid, String> {
    db.create_event(&name).await.map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_vnidrop_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            db_test,
            create_event,
            activities::start_activity,
            activities::update_activity_start,
            activities::update_activity_end,
            activities::update_activity_name,
            activities::get_activity,
            activities::get_all_activities,
        ])
        .setup(|app| {
            // Store plugin uses .app_data_dir() internally
            let store = app.store("config.json")?;
            store.set("key", "value");
            println!("{:?}", store.get("key"));

            app.manage(UuidGenerator::new());

            let database_path = app
                .path()
                .app_data_dir()
                .expect("app_data_dir returned None")
                .join("db.sqlite3");
            let db = tauri::async_runtime::block_on(Database::new(&database_path))?;
            app.manage(db);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
