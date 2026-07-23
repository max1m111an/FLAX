mod structs;
mod commands;

use crate::commands::nfa_cmd::*;
use crate::commands::dfa_cmd::*;
use crate::structs::store::AutomatonStore;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AutomatonStore::new())
        .invoke_handler(tauri::generate_handler![
            greet,
            create_new_nfa,
            nfa_get,
            nfa_add_state,
            nfa_update_state,
            nfa_remove_state,
            nfa_add_transition,
            nfa_update_transition,
            nfa_remove_transition,
            nfa_check_string,
            create_new_dfa,
            dfa_get,
            dfa_add_state,
            dfa_update_state,
            dfa_remove_state,
            dfa_add_transition,
            dfa_update_transition,
            dfa_remove_transition,
            dfa_check_string,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
