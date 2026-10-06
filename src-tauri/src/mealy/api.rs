use tauri::{
    http::StatusCode,
    State,
};

use crate::core::types::{AutomatonStore, OperationResult};

/// Creates a new mealy machine in the store with a single initial state `q0`.
///
/// Kept separate from the Tauri command so it can be unit-tested without a
/// `tauri::State` (the same convention as `data_to_fa` / `test_line` in `fa`).
pub(crate) fn create_new_mealy(state: &AutomatonStore, name: Option<String>) -> OperationResult {
    let entry = state.create(name.unwrap_or_else(|| "Мили".to_string()), "q0");
    OperationResult {
        status: StatusCode::OK,
        message: "Автомат Мили создан".to_string(),
        automaton: Some(entry),
    }
}

/// Creates a new mealy machine with a single initial state `q0`.
#[tauri::command]
pub fn mealy_create_new(state: State<'_, AutomatonStore>, name: Option<String>) -> OperationResult {
    create_new_mealy(&state, name)
}
