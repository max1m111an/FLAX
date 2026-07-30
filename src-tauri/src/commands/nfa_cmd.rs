use tauri::State;

use crate::structs::{
    data_models::{AutomatonKind, OperationResult, StateData, StateResult, StatusResult, TransitionData, TransitionResult},
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
) -> StateResult {
    let mut entry = match state.get(automaton_id) {
        Some(e) => e,
        None => {
            return StateResult {
                status: 400,
                message: format!("Автомат с id {} не найден", automaton_id),
                state: None,
            };
        }
    };

    if entry.states.iter().any(|s| s.id == state_id) {
        return StateResult {
            status: 400,
            message: format!("Состояние {} уже существует", state_id),
            state: None,
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

    let created = entry.states.last().unwrap().clone();
    state.update(entry.clone());
    StateResult {
        status: 200,
        message: format!("Состояние {} добавлено", state_id),
        state: Some(created),
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
) -> StateResult {
    let mut entry = match state.get(automaton_id) {
        Some(e) => e,
        None => {
            return StateResult {
                status: 400,
                message: format!("Автомат с id {} не найден", automaton_id),
                state: None,
            };
        }
    };

    let idx = match entry.states.iter().position(|s| s.id == state_id) {
        Some(i) => i,
        None => {
            return StateResult {
                status: 400,
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
                s.is_initial = false;
            }
        }
        entry.states[idx].is_initial = init;
    }
    if let Some(fin) = is_final {
        entry.states[idx].is_final = fin;
    }

    state.update(entry.clone());
    StateResult {
        status: 200,
        message: format!("Состояние {} обновлено", state_id),
        state: Some(entry.states[idx].clone()),
    }
}

#[tauri::command]
pub fn nfa_remove_state(
    state: State<'_, AutomatonStore>,
    automaton_id: i32,
    state_id: i32,
) -> StatusResult {
    let mut entry = match state.get(automaton_id) {
        Some(e) => e,
        None => {
            return StatusResult {
                status: 400,
                message: format!("Автомат с id {} не найден", automaton_id),
            };
        }
    };

    if !entry.states.iter().any(|s| s.id == state_id) {
        return StatusResult {
            status: 400,
            message: format!("Состояние {} не существует", state_id),
        };
    }

    entry.states.retain(|s| s.id != state_id);
    entry.transitions.retain(|t| t.from != state_id && t.to != state_id);

    state.update(entry.clone());
    StatusResult {
        status: 200,
        message: format!("Состояние {} удалено", state_id),
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
) -> TransitionResult {
    let mut entry = match state.get(automaton_id) {
        Some(e) => e,
        None => {
            return TransitionResult {
                status: 400,
                message: format!("Автомат с id {} не найден", automaton_id),
                transition: None,
            };
        }
    };

    if !entry.states.iter().any(|s| s.id == from) {
        return TransitionResult {
            status: 400,
            message: format!("Состояние {} не существует", from),
            transition: None,
        };
    }
    if !entry.states.iter().any(|s| s.id == to) {
        return TransitionResult {
            status: 400,
            message: format!("Состояние {} не существует", to),
            transition: None,
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
            return TransitionResult {
                status: 400,
                message: format!("Переход {} -> {} по '{}' уже существует", from, to, symbol),
                transition: None,
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

    let created = entry.transitions.last().unwrap().clone();
    state.update(entry.clone());
    TransitionResult {
        status: 200,
        message: format!("{} переход(ов) {} -> {} добавлено", count, from, to),
        transition: Some(created),
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
) -> TransitionResult {
    let mut entry = match state.get(automaton_id) {
        Some(e) => e,
        None => {
            return TransitionResult {
                status: 400,
                message: format!("Автомат с id {} не найден", automaton_id),
                transition: None,
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
            return TransitionResult {
                status: 400,
                message: format!("Переход {} -> {} по '{}' не найден", old_from, old_to, old_symbol),
                transition: None,
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
    TransitionResult {
        status: 200,
        message: "Переход обновлён".to_string(),
        transition: Some(entry.transitions[idx].clone()),
    }
}

#[tauri::command]
pub fn nfa_remove_transition(
    state: State<'_, AutomatonStore>,
    automaton_id: i32,
    from: i32,
    to: i32,
    symbol: char,
) -> StatusResult {
    let mut entry = match state.get(automaton_id) {
        Some(e) => e,
        None => {
            return StatusResult {
                status: 400,
                message: format!("Автомат с id {} не найден", automaton_id),
            };
        }
    };

    let original_count = entry.transitions.len();
    entry
        .transitions
        .retain(|t| !(t.from == from && t.to == to && t.symbol == symbol.to_string()));
    let removed = original_count - entry.transitions.len();

    let (code, msg) = if removed > 0 {
        (200, format!("Удалено {} переход(ов)", removed))
    } else {
        (400, "Переход не найден".to_string())
    };

    state.update(entry.clone());
    StatusResult {
        status: code,
        message: msg,
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
