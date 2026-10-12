use std::collections::HashSet;

use tauri::{http::StatusCode, State};

use crate::core::api::generate_id;
use crate::core::types::{
    AutomatonData, AutomatonStore, OperationResult, StatusResult, TransitionData, TransitionResult,
};
use crate::fa::types::EPSILON;

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

/// Appends one mealy transition `from --input/output--> to` to the entry.
///
/// Validation is performed before any mutation, so an `Err` leaves the entry
/// untouched. The alphabet is derived from the transitions
/// (`AutomatonData::get_alphabet`), so it is not maintained here.
pub(crate) fn add_mealy_transition(
    entry: &mut AutomatonData,
    from: i32,
    to: i32,
    input: char,
    output: char,
) -> Result<TransitionData, String> {
    if !entry.states.iter().any(|s| s.id == from) {
        return Err(format!("Состояние {} не существует", from));
    }
    if !entry.states.iter().any(|s| s.id == to) {
        return Err(format!("Состояние {} не существует", to));
    }
    if input == EPSILON || output == EPSILON {
        return Err("Машина Мили не поддерживает eps-переходы".to_string());
    }
    if entry
        .transitions
        .iter()
        .any(|t| t.from == from && t.symbol == input)
    {
        return Err(format!(
            "Переход из {} по входу '{}' уже существует",
            from, input
        ));
    }

    let used: HashSet<i32> = entry.transitions.iter().map(|t| t.id).collect();
    let id = generate_id(&used);
    let created = TransitionData {
        id,
        from,
        to,
        symbol: input,
        output: Some(output),
    };
    entry.transitions.push(created.clone());
    Ok(created)
}

/// Updates the `input` and/or `output` symbol of an existing transition.
///
/// `None` leaves the corresponding field unchanged; `from`/`to`/`id` are never
/// touched. Validation is performed before any mutation, so an `Err` leaves the
/// entry untouched. A conflicting `(from, input)` pair is rejected to preserve
/// the mealy determinism invariant. The alphabet is derived from the transitions
/// (`AutomatonData::get_alphabet`), so it is not maintained here.
pub(crate) fn update_mealy_transition(
    entry: &mut AutomatonData,
    transition_id: i32,
    input: Option<char>,
    output: Option<char>,
) -> Result<TransitionData, String> {
    let idx = entry
        .transitions
        .iter()
        .position(|t| t.id == transition_id)
        .ok_or_else(|| format!("Переход {} не найден", transition_id))?;

    let mut updated = entry.transitions[idx].clone();

    if let Some(symbol) = input {
        updated.symbol = symbol;
    }
    if let Some(symbol) = output {
        updated.output = Some(symbol);
    }

    if updated.symbol == EPSILON || updated.output == Some(EPSILON) {
        return Err("Машина Мили не поддерживает eps-переходы".to_string());
    }
    if entry
        .transitions
        .iter()
        .any(|t| t.id != transition_id && t.from == updated.from && t.symbol == updated.symbol)
    {
        return Err(format!(
            "Переход из {} по входу '{}' уже существует",
            updated.from, updated.symbol
        ));
    }

    entry.transitions[idx] = updated.clone();
    Ok(updated)
}

/// Removes the transition with the given `transition_id` from the entry.
///
/// Returns `Err` when no transition matches the id, so the caller can tell
/// "not found" apart from success without counting entries; in that case the
/// entry is left unchanged. The alphabet is derived from the transitions, so it
/// updates automatically.
pub(crate) fn remove_mealy_transition(
    entry: &mut AutomatonData,
    transition_id: i32,
) -> Result<(), String> {
    let len_before = entry.transitions.len();
    entry.transitions.retain(|t| t.id != transition_id);
    if entry.transitions.len() == len_before {
        return Err(format!("Переход {} не найден", transition_id));
    }
    Ok(())
}

/// Creates a new mealy machine with a single initial state `q0`.
#[tauri::command]
pub fn mealy_create_new(state: State<'_, AutomatonStore>, name: Option<String>) -> OperationResult {
    create_new_mealy(&state, name)
}

/// Adds a single mealy transition `from --input/output--> to`.
#[tauri::command]
pub fn mealy_add_transition(
    state: State<'_, AutomatonStore>,
    automaton_id: i32,
    from: i32,
    to: i32,
    input: char,
    output: char,
) -> TransitionResult {
    let mut entry = match state.get(automaton_id) {
        Some(e) => e,
        None => {
            return TransitionResult {
                status: StatusCode::BAD_REQUEST,
                message: format!("Автомат с id {} не найден", automaton_id),
                transition: vec![],
            };
        }
    };

    match add_mealy_transition(&mut entry, from, to, input, output) {
        Ok(created) => {
            state.update(entry);
            TransitionResult {
                status: StatusCode::OK,
                message: format!(
                    "Переход {} -{}-> {} (выход '{}') добавлен",
                    from, input, to, output
                ),
                transition: vec![created],
            }
        }
        Err(message) => TransitionResult {
            status: StatusCode::BAD_REQUEST,
            message,
            transition: vec![],
        },
    }
}

#[tauri::command]
pub fn mealy_update_transition(
    state: State<'_, AutomatonStore>,
    automaton_id: i32,
    transition_id: i32,
    input: Option<char>,
    output: Option<char>,
) -> TransitionResult {
    let mut entry = match state.get(automaton_id) {
        Some(e) => e,
        None => {
            return TransitionResult {
                status: StatusCode::BAD_REQUEST,
                message: format!("Автомат с id {} не найден", automaton_id),
                transition: vec![],
            };
        }
    };

    match update_mealy_transition(&mut entry, transition_id, input, output) {
        Ok(updated) => {
            state.update(entry);
            TransitionResult {
                status: StatusCode::OK,
                message: format!("Переход {} обновлён", transition_id),
                transition: vec![updated],
            }
        }
        Err(message) => TransitionResult {
            status: StatusCode::BAD_REQUEST,
            message,
            transition: vec![],
        },
    }
}

#[tauri::command]
pub fn mealy_remove_transition(
    state: State<'_, AutomatonStore>,
    automaton_id: i32,
    transition_id: i32,
) -> StatusResult {
    let mut entry = match state.get(automaton_id) {
        Some(e) => e,
        None => {
            return StatusResult {
                status: StatusCode::BAD_REQUEST,
                message: format!("Автомат с id {} не найден", automaton_id),
            };
        }
    };

    match remove_mealy_transition(&mut entry, transition_id) {
        Ok(()) => {
            state.update(entry);
            StatusResult {
                status: StatusCode::OK,
                message: format!("Удалён переход {}", transition_id),
            }
        }
        Err(message) => StatusResult {
            status: StatusCode::BAD_REQUEST,
            message,
        },
    }
}
