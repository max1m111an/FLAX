use tauri::State;

use crate::structs::{
    automata::Automaton,
    data_models::{AutomatonKind, CheckResult, OperationResult, StateData, TransitionData},
    nfa::{NFA, EPSILON, NFABuilder},
    store::AutomatonStore,
};

#[tauri::command]
pub fn create_new_nfa(
    state: State<'_, AutomatonStore>,
    name: Option<String>,
) -> OperationResult {
    let entry = state.create(
        name.unwrap_or_else(|| "NFA".to_string()),
        AutomatonKind::NFA,
        "q0",
    );
    OperationResult {
        status: 200,
        message: "NFA успешно создан".to_string(),
        automaton: Some(entry),
    }
}

#[tauri::command]
pub fn nfa_get(state: State<'_, AutomatonStore>, automaton_id: i32) -> OperationResult {
    match state.get(automaton_id) {
        Some(entry) => OperationResult {
            status: 200,
            message: "NFA получен".to_string(),
            automaton: Some(entry),
        },
        None => OperationResult {
            status: 400,
            message: format!("Автомат с id {} не найден", automaton_id),
            automaton: None,
        },
    }
}

#[tauri::command]
pub fn nfa_add_state(
    state: State<'_, AutomatonStore>,
    automaton_id: i32,
    state_id: i32,
    label: Option<String>,
    x: Option<f32>,
    y: Option<f32>,
    is_initial: Option<bool>,
    is_final: Option<bool>,
) -> OperationResult {
    let mut entry = match state.get(automaton_id) {
        Some(e) => e,
        None => {
            return OperationResult {
                status: 400,
                message: format!("Автомат с id {} не найден", automaton_id),
                automaton: None,
            };
        }
    };

    if entry.states.iter().any(|s| s.id == state_id) {
        return OperationResult {
            status: 400,
            message: format!("Состояние {} уже существует", state_id),
            automaton: Some(entry),
        };
    }

    entry.states.push(StateData {
        id: state_id,
        label: label.unwrap_or_else(|| format!("q{}", state_id)),
        x: x.unwrap_or(100.0),
        y: y.unwrap_or(200.0),
        is_initial: is_initial.unwrap_or(false),
        is_final: is_final.unwrap_or(false),
    });

    state.update(entry.clone());
    OperationResult {
        status: 200,
        message: format!("Состояние {} добавлено", state_id),
        automaton: Some(entry),
    }
}

#[tauri::command]
pub fn nfa_update_state(
    state: State<'_, AutomatonStore>,
    automaton_id: i32,
    state_id: i32,
    label: Option<String>,
    x: Option<f32>,
    y: Option<f32>,
    is_initial: Option<bool>,
    is_final: Option<bool>,
) -> OperationResult {
    let mut entry = match state.get(automaton_id) {
        Some(e) => e,
        None => {
            return OperationResult {
                status: 400,
                message: format!("Автомат с id {} не найден", automaton_id),
                automaton: None,
            };
        }
    };

    let idx = match entry.states.iter().position(|s| s.id == state_id) {
        Some(i) => i,
        None => {
            return OperationResult {
                status: 400,
                message: format!("Состояние {} не существует", state_id),
                automaton: Some(entry),
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
                s.is_initial = false;
            }
        }
        entry.states[idx].is_initial = init;
    }
    if let Some(fin) = is_final {
        entry.states[idx].is_final = fin;
    }

    state.update(entry.clone());
    OperationResult {
        status: 200,
        message: format!("Состояние {} обновлено", state_id),
        automaton: Some(entry),
    }
}

#[tauri::command]
pub fn nfa_remove_state(
    state: State<'_, AutomatonStore>,
    automaton_id: i32,
    state_id: i32,
) -> OperationResult {
    let mut entry = match state.get(automaton_id) {
        Some(e) => e,
        None => {
            return OperationResult {
                status: 400,
                message: format!("Автомат с id {} не найден", automaton_id),
                automaton: None,
            };
        }
    };

    if !entry.states.iter().any(|s| s.id == state_id) {
        return OperationResult {
            status: 400,
            message: format!("Состояние {} не существует", state_id),
            automaton: Some(entry),
        };
    }

    entry.states.retain(|s| s.id != state_id);
    entry.transitions.retain(|t| t.from != state_id && t.to != state_id);

    state.update(entry.clone());
    OperationResult {
        status: 200,
        message: format!("Состояние {} удалено", state_id),
        automaton: Some(entry),
    }
}

#[tauri::command]
pub fn nfa_add_transition(
    state: State<'_, AutomatonStore>,
    automaton_id: i32,
    from: i32,
    to: i32,
    symbols: Vec<char>,
    label: Option<String>,
) -> OperationResult {
    let mut entry = match state.get(automaton_id) {
        Some(e) => e,
        None => {
            return OperationResult {
                status: 400,
                message: format!("Автомат с id {} не найден", automaton_id),
                automaton: None,
            };
        }
    };

    if !entry.states.iter().any(|s| s.id == from) {
        return OperationResult {
            status: 400,
            message: format!("Состояние {} не существует", from),
            automaton: Some(entry),
        };
    }
    if !entry.states.iter().any(|s| s.id == to) {
        return OperationResult {
            status: 400,
            message: format!("Состояние {} не существует", to),
            automaton: Some(entry),
        };
    }

    let mut count = 0u32;
    for &symbol in &symbols {
        let sym_str = symbol.to_string();
        let already_exists = entry
            .transitions
            .iter()
            .any(|t| t.from == from && t.to == to && t.symbol == sym_str);

        if already_exists {
            return OperationResult {
                status: 400,
                message: format!("Переход {} -> {} по '{}' уже существует", from, to, symbol),
                automaton: Some(entry),
            };
        }

        if symbol != EPSILON && !entry.alphabet.contains(&symbol) {
            entry.alphabet.push(symbol);
        }

        entry.transitions.push(TransitionData {
            from,
            to,
            symbol: sym_str,
            label: label.clone(),
        });
        count += 1;
    }

    state.update(entry.clone());
    OperationResult {
        status: 200,
        message: format!("{} переход(ов) {} -> {} добавлено", count, from, to),
        automaton: Some(entry),
    }
}

#[tauri::command]
pub fn nfa_update_transition(
    state: State<'_, AutomatonStore>,
    automaton_id: i32,
    old_from: i32,
    old_to: i32,
    old_symbol: char,
    new_from: Option<i32>,
    new_to: Option<i32>,
    new_symbol: Option<char>,
    new_label: Option<Option<String>>,
) -> OperationResult {
    let mut entry = match state.get(automaton_id) {
        Some(e) => e,
        None => {
            return OperationResult {
                status: 400,
                message: format!("Автомат с id {} не найден", automaton_id),
                automaton: None,
            };
        }
    };

    let idx = match entry
        .transitions
        .iter()
        .position(|t| t.from == old_from && t.to == old_to && t.symbol == old_symbol.to_string())
    {
        Some(i) => i,
        None => {
            return OperationResult {
                status: 400,
                message: format!("Переход {} -> {} по '{}' не найден", old_from, old_to, old_symbol),
                automaton: Some(entry),
            };
        }
    };

    if let Some(f) = new_from {
        entry.transitions[idx].from = f;
    }
    if let Some(t) = new_to {
        entry.transitions[idx].to = t;
    }
    if let Some(s) = new_symbol {
        entry.transitions[idx].symbol = s.to_string();
    }
    if let Some(l) = new_label {
        entry.transitions[idx].label = l;
    }

    state.update(entry.clone());
    OperationResult {
        status: 200,
        message: "Переход обновлён".to_string(),
        automaton: Some(entry),
    }
}

#[tauri::command]
pub fn nfa_remove_transition(
    state: State<'_, AutomatonStore>,
    automaton_id: i32,
    from: i32,
    to: i32,
    symbol: char,
) -> OperationResult {
    let mut entry = match state.get(automaton_id) {
        Some(e) => e,
        None => {
            return OperationResult {
                status: 400,
                message: format!("Автомат с id {} не найден", automaton_id),
                automaton: None,
            };
        }
    };

    let original_count = entry.transitions.len();
    entry
        .transitions
        .retain(|t| !(t.from == from && t.to == to && t.symbol == symbol.to_string()));
    let removed = original_count - entry.transitions.len();

    let (_status, _message) = if removed > 0 {
        (200, format!("Удалено {} переход(ов)", removed))
    } else {
        (400, "Переход не найден".to_string())
    };

    state.update(entry.clone());
    OperationResult {
        status: _status,
        message: _message,
        automaton: Some(entry),
    }
}

#[tauri::command]
pub fn nfa_check_string(
    state: State<'_, AutomatonStore>,
    automaton_id: i32,
    input: String,
) -> CheckResult {
    let entry = match state.get(automaton_id) {
        Some(e) => e,
        None => {
            return CheckResult {
                status: 400,
                message: format!("Автомат с id {} не найден", automaton_id),
                accepted: false,
            };
        }
    };

    match data_to_nfa(&entry.states, &entry.transitions, &entry.alphabet) {
        Ok(nfa) => {
            let chars: Vec<char> = input.chars().collect();
            let accepted = nfa.accepts(&chars);
            CheckResult {
                status: 200,
                message: format!(
                    "Строка '{}' {} принята NFA",
                    input,
                    if accepted { "" } else { "не " }
                ),
                accepted,
            }
        }
        Err(e) => CheckResult {
            status: 400,
            message: format!("Ошибка проверки строки: {}", e),
            accepted: false,
        },
    }
}

fn data_to_nfa(
    states: &[StateData],
    transitions: &[TransitionData],
    alphabet: &[char],
) -> Result<NFA, String> {
    let mut builder = NFABuilder::new();

    for state in states {
        builder = builder.state(state.id);
        if state.is_initial {
            builder = builder.initial(state.id);
        }
        if state.is_final {
            builder = builder.accepting(state.id);
        }
    }

    for &symbol in alphabet {
        builder = builder.symbol(symbol);
    }

    for trans in transitions {
        if trans.symbol == EPSILON.to_string() {
            builder = builder.epsilon(trans.from, trans.to);
        } else if let Some(symbol) = trans.symbol.chars().next() {
            builder = builder.transition(trans.from, symbol, trans.to);
        }
    }

    builder.build()
}
