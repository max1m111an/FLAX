use tauri::{
    http::StatusCode,
    State,
};

use crate::core::api::generate_id;
use crate::core::types::{
    AutomatonStore, GenerateInputsResult, LineTest, MultiRunResult, OperationResult, RunResult,
    StateData, StateResult, StatusResult, TransitionData, TransitionResult,
};
use crate::fa::types::{EPSILON, FA, FABuilder};

/// Creates a new finite automaton with a single initial state `q0`.
#[tauri::command]
pub fn fa_create_new(state: State<'_, AutomatonStore>, name: Option<String>) -> OperationResult {
    let entry = state.create(name.unwrap_or_else(|| "Автомат".to_string()), "q0");
    OperationResult {
        status: StatusCode::OK,
        message: "Конечный автомат создан".to_string(),
        automaton: Some(entry),
    }
}

#[tauri::command]
pub fn fa_get(state: State<'_, AutomatonStore>, automaton_id: i32) -> OperationResult {
    match state.get(automaton_id) {
        Some(entry) => OperationResult {
            status: StatusCode::OK,
            message: "Автомат получен".to_string(),
            automaton: Some(entry),
        },
        None => OperationResult {
            status: StatusCode::BAD_REQUEST,
            message: format!("Автомат с id {} не найден", automaton_id),
            automaton: None,
        },
    }
}

#[tauri::command]
pub fn fa_add_state(
    state: State<'_, AutomatonStore>,
    automaton_id: i32,
    label: Option<String>,
    x: Option<f32>,
    y: Option<f32>,
    is_initial: Option<bool>,
    is_final: Option<bool>,
) -> StateResult {
    let mut entry = match state.get(automaton_id) {
        Some(e) => e,
        None => {
            return StateResult {
                status: StatusCode::BAD_REQUEST,
                message: format!("Автомат с id {} не найден", automaton_id),
                state: None,
            };
        }
    };

    let used: std::collections::HashSet<i32> = entry.states.iter().map(|s| s.id).collect();
    let new_id = generate_id(&used);

    entry.states.push(StateData {
        id: new_id,
        label: label.unwrap_or_else(|| format!("q{}", new_id)),
        x: x.unwrap_or(100.0),
        y: y.unwrap_or(200.0),
        isInitial: is_initial.unwrap_or(false),
        isFinal: is_final.unwrap_or(false),
    });

    let created = entry.states.last().unwrap().clone();
    state.update(entry.clone());
    StateResult {
        status: StatusCode::OK,
        message: format!("Состояние {} добавлено", new_id),
        state: Some(created),
    }
}

#[tauri::command]
pub fn fa_update_state(
    state: State<'_, AutomatonStore>,
    automaton_id: i32,
    state_id: i32,
    label: Option<String>,
    x: Option<f32>,
    y: Option<f32>,
    is_initial: Option<bool>,
    is_final: Option<bool>,
) -> StateResult {
    let mut entry = match state.get(automaton_id) {
        Some(e) => e,
        None => {
            return StateResult {
                status: StatusCode::BAD_REQUEST,
                message: format!("Автомат с id {} не найден", automaton_id),
                state: None,
            };
        }
    };

    let idx = match entry.states.iter().position(|s| s.id == state_id) {
        Some(i) => i,
        None => {
            return StateResult {
                status: StatusCode::BAD_REQUEST,
                message: format!("Состояние {} не существует", state_id),
                state: None,
            };
        }
    };

    if let Some(v) = label {
        entry.states[idx].label = v;
    }
    if let Some(v) = x {
        entry.states[idx].x = v;
    }
    if let Some(v) = y {
        entry.states[idx].y = v;
    }
    if let Some(init) = is_initial {
        if init {
            for s in &mut entry.states {
                s.isInitial = false;
            }
        }
        entry.states[idx].isInitial = init;
    }
    if let Some(fin) = is_final {
        entry.states[idx].isFinal = fin;
    }

    state.update(entry.clone());
    StateResult {
        status: StatusCode::OK,
        message: format!("Состояние {} обновлено", state_id),
        state: Some(entry.states[idx].clone()),
    }
}

/// Removes a state together with all transitions touching it (incoming, outgoing
/// and self-loops).
#[tauri::command]
pub fn fa_remove_state(
    state: State<'_, AutomatonStore>,
    automaton_id: i32,
    state_id: i32,
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

    if !entry.states.iter().any(|s| s.id == state_id) {
        return StatusResult {
            status: StatusCode::BAD_REQUEST,
            message: format!("Состояние {} не существует", state_id),
        };
    }

    entry.states.retain(|s| s.id != state_id);
    entry
        .transitions
        .retain(|t| t.from != state_id && t.to != state_id);

    state.update(entry.clone());
    StatusResult {
        status: StatusCode::OK,
        message: format!("Состояние {} удалено", state_id),
    }
}

/// Adds transitions. For each symbol a separate transition with the same
/// `from`/`to` is created. IDs are generated by the server.
///
/// Only exact duplicates `(from, to, symbol)` are rejected; `$` epsilon
/// transitions are allowed.
#[tauri::command]
pub fn fa_add_transition(
    state: State<'_, AutomatonStore>,
    automaton_id: i32,
    from: i32,
    to: i32,
    symbols: Vec<char>,
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

    if !entry.states.iter().any(|s| s.id == from) {
        return TransitionResult {
            status: StatusCode::BAD_REQUEST,
            message: format!("Состояние {} не существует", from),
            transition: vec![],
        };
    }
    if !entry.states.iter().any(|s| s.id == to) {
        return TransitionResult {
            status: StatusCode::BAD_REQUEST,
            message: format!("Состояние {} не существует", to),
            transition: vec![],
        };
    }

    let mut count = 0u32;
    let mut added: Vec<TransitionData> = Vec::new();
    for &symbol in &symbols {
        let conflict_exists = entry
            .transitions
            .iter()
            .any(|t| t.from == from && t.to == to && t.symbol == symbol);

        if conflict_exists {
            return TransitionResult {
                status: StatusCode::BAD_REQUEST,
                message: format!("Переход {} -> {} по '{}' уже существует", from, to, symbol),
                transition: vec![],
            };
        }

        if symbol != EPSILON && !entry.alphabet.contains(&symbol) {
            entry.alphabet.push(symbol);
        }

        let used: std::collections::HashSet<i32> = entry.transitions.iter().map(|t| t.id).collect();
        let tid = generate_id(&used);
        let created = TransitionData {
            id: tid,
            from,
            to,
            symbol,
        };
        entry.transitions.push(created.clone());
        added.push(created);
        count += 1;
    }

    state.update(entry.clone());
    TransitionResult {
        status: StatusCode::OK,
        message: format!("{} переход(ов) {} -> {} добавлено", count, from, to),
        transition: added,
    }
}

/// Partially updates a transition found by `transition_id`; only the passed
/// fields are changed.
#[tauri::command]
pub fn fa_update_transition(
    state: State<'_, AutomatonStore>,
    automaton_id: i32,
    transition_id: i32,
    new_from: Option<i32>,
    new_to: Option<i32>,
    new_symbol: Option<char>,
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

    let idx = match entry.transitions.iter().position(|t| t.id == transition_id) {
        Some(i) => i,
        None => {
            return TransitionResult {
                status: StatusCode::BAD_REQUEST,
                message: format!("Переход {} не найден", transition_id),
                transition: vec![],
            };
        }
    };

    let mut updated = entry.transitions[idx].clone();
    if let Some(f) = new_from {
        updated.from = f;
    }
    if let Some(t) = new_to {
        updated.to = t;
    }
    if let Some(s) = new_symbol {
        updated.symbol = s;
    }

    if updated.symbol != EPSILON && !entry.alphabet.contains(&updated.symbol) {
        entry.alphabet.push(updated.symbol);
    }

    entry.transitions[idx] = updated.clone();
    state.update(entry.clone());
    TransitionResult {
        status: StatusCode::OK,
        message: "Переход обновлён".to_string(),
        transition: vec![updated],
    }
}

#[tauri::command]
pub fn fa_remove_transition(
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

    let original_count = entry.transitions.len();
    entry.transitions.retain(|t| t.id != transition_id);
    let removed = original_count - entry.transitions.len();

    let (status, msg) = if removed > 0 {
        (StatusCode::OK, format!("Удалено {} переход(ов)", removed))
    } else {
        (StatusCode::BAD_REQUEST, "Переход не найден".to_string())
    };

    state.update(entry.clone());
    StatusResult {
        status: status,
        message: msg,
    }
}

/// Runs a string on the automaton (JFLAP-style parallel check via
/// `FA::run_partial`, statuses `200`/`401`/`402`). Takes the input as a whole
/// string (not an array).
#[tauri::command]
pub fn fa_run_str(state: State<'_, AutomatonStore>, automaton_id: i32, input: String) -> RunResult {
    let entry = match state.get(automaton_id) {
        Some(e) => e,
        None => {
            return RunResult {
                status: StatusCode::NOT_FOUND,
                message: format!("Автомат с id {} не найден", automaton_id),
                traces: Vec::new(),
            };
        }
    };

    let chars: Vec<char> = input.chars().collect();
    match data_to_fa(&entry.states, &entry.transitions, &entry.alphabet) {
        Ok(fa) => {
            let (mut traces, accepted) = fa.run_partial(&chars);
            // Order the history so that accepted reading streams (isFinal) are
            // listed before rejected ones.
            traces.sort_by_key(|t| !t.isFinal);
            // `$` (eps-closure) steps are part of the history; count only the
            // symbol transitions actually consumed to derive the processed length.
            let processed_len = traces
                .iter()
                .map(|t| t.steps.iter().filter(|s| s.symbol != EPSILON).count())
                .max()
                .unwrap_or(0);
            let (status, message) = if accepted {
                (StatusCode::OK, format!("Цепочка '{}' принята", input))
            } else if processed_len > 0 {
                (
                    StatusCode::UNAUTHORIZED,
                    format!(
                        "Цепочка '{}' принята частично (обработано {} из {} символов)",
                        input,
                        processed_len,
                        chars.len()
                    ),
                )
            } else {
                (StatusCode::PAYMENT_REQUIRED, format!("Цепочка '{}' отклонена", input))
            };
            RunResult {
                status,
                message,
                traces,
            }
        }
        Err(err) => RunResult {
            status: StatusCode::BAD_REQUEST,
            message: format!("Некорректный автомат: {}", err),
            traces: Vec::new(),
        },
    }
}

/// Runs several strings at once and returns only the pass/fail fact per line
/// (without step traces). Uses the same algorithm as `fa_run_str`.
#[tauri::command]
pub fn fa_multi_run_str(
    state: State<'_, AutomatonStore>,
    automaton_id: i32,
    inputs: Vec<String>,
) -> MultiRunResult {
    let entry = match state.get(automaton_id) {
        Some(e) => e,
        None => {
            return MultiRunResult {
                status: StatusCode::NOT_FOUND,
                message: format!("Автомат с id {} не найден", automaton_id),
                traces: Vec::new(),
            };
        }
    };

    let traces: Vec<LineTest> =
        match data_to_fa(&entry.states, &entry.transitions, &entry.alphabet) {
            Ok(fa) => inputs
                .into_iter()
                .map(|line| {
                    let (is_final, correct_symbols) = test_line(&fa, &line);
                    LineTest {
                        line,
                        isFinal: is_final,
                        correctSymbols: correct_symbols,
                    }
                })
                .collect(),
            Err(_) => {
                return MultiRunResult {
                    status: StatusCode::BAD_REQUEST,
                    message: "Некорректный автомат".to_string(),
                    traces: Vec::new(),
                };
            }
        };

    MultiRunResult {
        status: StatusCode::OK,
        message: String::new(),
        traces,
    }
}

/// Generates an ordered set of test strings covering a wide range of automaton
/// scenarios (empty, singles, paths to every state, transition activation,
/// negative/dead-end cases, cyclic and long strings). Ready to paste into the
/// Multiple Run fields.
#[tauri::command]
pub fn fa_generate_inputs(
    state: State<'_, AutomatonStore>,
    automaton_id: i32,
) -> GenerateInputsResult {
    let entry = match state.get(automaton_id) {
        Some(e) => e,
        None => {
            return GenerateInputsResult {
                status: StatusCode::NOT_FOUND,
                message: format!("Автомат с id {} не найден", automaton_id),
                inputs: Vec::new(),
            };
        }
    };

    let fa = match data_to_fa(&entry.states, &entry.transitions, &entry.alphabet) {
        Ok(n) => n,
        Err(_) => {
            return GenerateInputsResult {
                status: StatusCode::BAD_REQUEST,
                message: "Некорректный автомат".to_string(),
                inputs: Vec::new(),
            };
        }
    };

    let resp_vec: Vec<String> = fa.generate_test_inputs(50, 15);

    GenerateInputsResult {
        status: StatusCode::OK,
        message: format!("Сгенерировано {} тестовых входов", resp_vec.len()),
        inputs: resp_vec,
    }
}

#[tauri::command]
pub fn fa_remove_automaton(state: State<'_, AutomatonStore>, automaton_id: i32) -> StatusResult {
    match state.remove(automaton_id) {
        Some(_) => StatusResult {
            status: StatusCode::OK,
            message: format!("Автомат с id {} удалён", automaton_id),
        },
        None => StatusResult {
            status: StatusCode::BAD_REQUEST,
            message: format!("Автомат с id {} не найден", automaton_id),
        },
    }
}

/// Runs a single line on the automaton. Returns whether the whole line is
/// accepted (`true` if at least one thread reaches a final state after
/// consuming all of it) and how many symbols were consumed correctly (only
/// non-`$` steps).
pub(crate) fn test_line(fa: &FA, input: &str) -> (bool, usize) {
    let chars: Vec<char> = input.chars().collect();
    let (traces, accepted) = fa.run_partial(&chars);
    let correct_symbols = traces
        .iter()
        .map(|t| t.steps.iter().filter(|s| s.symbol != EPSILON).count())
        .max()
        .unwrap_or(0);
    (accepted, correct_symbols)
}

pub(crate) fn data_to_fa(
    states: &[StateData],
    transitions: &[TransitionData],
    alphabet: &[char],
) -> Result<FA, String> {
    let mut builder = FABuilder::new();

    for state in states {
        builder = builder.state(state.id);
        if state.isInitial {
            builder = builder.set_initial(state.id);
        }
        if state.isFinal {
            builder = builder.set_final(state.id);
        }
    }

    for &symbol in alphabet {
        builder = builder.symbol(symbol);
    }

    // Ignore transitions that reference states not present in the automaton
    // (orphan IDs) — they are stale and should not create extra branches.
    let state_ids: std::collections::HashSet<i32> = states.iter().map(|s| s.id).collect();
    for trans in transitions {
        if !state_ids.contains(&trans.from) || !state_ids.contains(&trans.to) {
            continue;
        }
        if trans.symbol == EPSILON {
            builder = builder.epsilon(trans.from, trans.to);
        } else {
            builder = builder.transition(trans.from, trans.symbol, trans.to);
        }
    }

    builder.build()
}