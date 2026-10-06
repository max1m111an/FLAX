use std::collections::HashSet;

use tauri::{http::StatusCode, State};

use crate::core::api::generate_id;
use crate::core::types::{
    AutomatonData, AutomatonStore, OperationResult, TransitionData, TransitionResult,
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
/// untouched. The input symbol is appended to the alphabet (there is no
/// separate output alphabet in the store: it is derived from transitions).
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
        return Err("Машина Мили не поддерживает ε-переходы".to_string());
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

    if !entry.alphabet.contains(&input) {
        entry.alphabet.push(input);
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
