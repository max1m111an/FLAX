use std::fs;
use std::path::Path;

use tauri::State;

use crate::{
    jff,
    structs::{
        data_models::{AutomatonData, OperationResult, StatusResult},
        store::AutomatonStore,
    },
};

#[tauri::command]
pub fn save_jff(state: State<'_, AutomatonStore>, automaton_id: i32, path: String) -> StatusResult {
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

#[tauri::command]
pub fn load_jff(state: State<'_, AutomatonStore>, path: String) -> OperationResult {
    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(err) => {
            return OperationResult {
                status: 400,
                message: format!("Не удалось прочитать файл: {}", err),
                automaton: None,
            };
        }
    };

    let parsed = match jff::parse_jff(&content) {
        Ok(p) => p,
        Err(err) => {
            return OperationResult {
                status: 400,
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
        kind: jff::infer_kind(&parsed.states, &parsed.transitions),
        states: parsed.states,
        transitions: parsed.transitions,
        alphabet: parsed.alphabet,
    });

    OperationResult {
        status: 200,
        message: "Автомат загружен из файла".to_string(),
        automaton: Some(entry),
    }
}
