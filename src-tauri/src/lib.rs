mod api;
mod core;
mod fa;
mod mealy;

use crate::core::automaton::AutomatonStore;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .manage(AutomatonStore::new())
        .invoke_handler(crate::api::handler)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
