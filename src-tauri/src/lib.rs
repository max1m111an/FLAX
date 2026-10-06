mod core;
mod fa;
mod jff;
mod mealy;

use crate::core::types::AutomatonStore;
use crate::fa::api::*;
use crate::jff::api::*;
use crate::mealy::api::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .manage(AutomatonStore::new())
        .invoke_handler(tauri::generate_handler![
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
            mealy_create_new,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}