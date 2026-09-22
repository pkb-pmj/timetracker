mod db;

use specta_typescript::semantic;
use specta_typescript::Typescript;
use tauri::{Manager, State, Wry};
use tauri_plugin_store::StoreExt;
use tauri_specta::{collect_commands, collect_events, Builder};
use uuid::Uuid;

use crate::db::{activities, Database, UuidGenerator};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
#[specta::specta]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
#[specta::specta]
async fn db_test(db: State<'_, Database>) -> Result<i64, String> {
    db.test().await.map_err(|e| e.to_string())
}

#[tauri::command]
#[specta::specta]
async fn create_event(db: State<'_, Database>, name: String) -> Result<Uuid, String> {
    db.create_event(&name).await.map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let specta_builder = Builder::<Wry>::new()
        .semantic_types(semantic::Configuration::default())
        .commands(collect_commands![
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
        .events(collect_events![]);

    #[cfg(debug_assertions)]
    specta_builder
        .export(Typescript::default(), "../src/lib/bindings.ts")
        .unwrap();

    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_vnidrop_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(specta_builder.invoke_handler())
        .setup(move |app| {
            specta_builder.mount_events(app);

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
