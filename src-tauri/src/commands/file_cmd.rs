use std::fs;

use tauri::State;

use crate::{
    jff,
    structs::{data_models::StatusResult, store::AutomatonStore},
};

#[tauri::command]
pub fn save_jff(
    state: State<'_, AutomatonStore>,
    automaton_id: i32,
    path: String,
) -> StatusResult {
    let entry = match state.get(automaton_id) {
        Some(e) => e,
        None => {
            return StatusResult {
                status: 400,
                message: format!("Автомат с id {} не найден", automaton_id),
            };
        }
    };

    let content = jff::to_jff(&entry);

    if let Err(err) = fs::write(&path, content) {
        return StatusResult {
            status: 400,
            message: format!("Не удалось сохранить файл: {}", err),
        };
    }

    StatusResult {
        status: 200,
        message: format!("Автомат сохранён в {}", path),
    }
}
