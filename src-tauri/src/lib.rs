pub mod structs;
mod commands;

#[path = "utils/id_gen.rs"]
mod id_gen;

#[path = "utils/jff.rs"]
mod jff;

#[cfg(test)]
mod tests;

use crate::commands::nfa_cmd::*;
use crate::commands::file_cmd::*;
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
            nfa_run_str,
            nfa_multiple_run_str,
            nfa_generate_inputs,
            nfa_remove_automaton,
            save_jff,
            load_jff,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
