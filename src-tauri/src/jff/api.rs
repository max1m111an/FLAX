use std::fs;
use std::path::Path;

use tauri::{
    http::StatusCode,
    State,
};

use crate::core::types::{AutomatonData, AutomatonStore, OperationResult, StatusResult};
use crate::jff;

#[tauri::command]
pub fn save_jff(state: State<'_, AutomatonStore>, automaton_id: i32, path: String) -> StatusResult {
    let entry = match state.get(automaton_id) {
        Some(e) => e,
        None => {
            return StatusResult {
                status: StatusCode::BAD_REQUEST,
                message: format!("Автомат с id {} не найден", automaton_id),
            };
        }
    };

    let content = jff::to_jff(&entry);

    if let Err(err) = fs::write(&path, content) {
        return StatusResult {
            status: StatusCode::BAD_REQUEST,
            message: format!("Не удалось сохранить файл: {}", err),
        };
    }

    StatusResult {
        status: StatusCode::OK,
        message: format!("Автомат сохранён в {}", path),
    }
}

#[tauri::command]
pub fn load_jff(state: State<'_, AutomatonStore>, path: String) -> OperationResult {
    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(err) => {
            return OperationResult {
                status: StatusCode::BAD_REQUEST,
                message: format!("Не удалось прочитать файл: {}", err),
                automaton: None,
            };
        }
    };

    let parsed = match jff::parse_jff(&content) {
        Ok(p) => p,
        Err(err) => {
            return OperationResult {
                status: StatusCode::BAD_REQUEST,
                message: err,
                automaton: None,
            };
        }
    };

    let name = Path::new(&path)
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "Автомат".to_string());

    let entry = state.insert(AutomatonData {
        id: 0,
        name,
        states: parsed.states,
        transitions: parsed.transitions,
        alphabet: parsed.alphabet,
    });

    OperationResult {
        status: StatusCode::OK,
        message: "Автомат загружен из файла".to_string(),
        automaton: Some(entry),
    }
}
