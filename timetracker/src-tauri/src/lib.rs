mod config;

use tauri::{Manager, State};

use crate::config::{AppConfig, ConfigState};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_config(config: State<Option<ConfigState>>) -> Option<AppConfig> {
    config.as_ref().map(|config| config.get())
}

#[tauri::command]
fn get_error(error: State<String>) -> String {
    error.to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_vnidrop_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_config, get_error])
        .setup(|app| {
            let mut err_state = String::new();
            let mut config_state = None;
            let dir = app.path().app_config_dir()?;
            match ConfigState::new(dir) {
                Ok(c) => config_state = Some(c),
                Err(e) => err_state = e.to_string(),
            };
            app.manage(err_state);
            app.manage(config_state);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
