// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod structs;
mod commands;

use crate::{
    commands::nfa_cmd::{
        create_new_nfa,
        add_state,
        add_transition,
        remove_state,
        remove_transition,
    },
};


#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            create_new_nfa,
            add_state,
            add_transition,
            remove_state,
            remove_transition,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
