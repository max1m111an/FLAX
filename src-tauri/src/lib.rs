mod core;
mod fa;
mod jff;

use crate::core::types::AutomatonStore;
use crate::fa::api::*;
use crate::jff::api::*;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .manage(AutomatonStore::new())
        .invoke_handler(tauri::generate_handler![
            greet,
            fa_create_new,
            fa_get,
            fa_add_state,
            fa_update_state,
            fa_remove_state,
            fa_add_transition,
            fa_update_transition,
            fa_remove_transition,
            fa_run_str,
            fa_multi_run_str,
            fa_generate_inputs,
            fa_remove_automaton,
            save_jff,
            load_jff,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}