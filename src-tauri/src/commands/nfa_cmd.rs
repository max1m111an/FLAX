use tauri::State;

use crate::{
    id_gen,
    structs::{
        data_models::{
            AutomatonKind, GenerateInputsResult, LineTest, MultiRunResult, OperationResult,
            RunResult, StateData, StateResult, StatusResult, TransitionData, TransitionResult,
        },
        nfa::{EPSILON, NFA, NFABuilder},
        store::AutomatonStore,
    },
};

#[tauri::command]
pub fn create_new_nfa(state: State<'_, AutomatonStore>, name: Option<String>) -> OperationResult {
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

    let used: std::collections::HashSet<i32> = entry.states.iter().map(|s| s.id).collect();
    let new_id = id_gen::generate_id(&used);

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
        status: 200,
        message: format!("Состояние {} добавлено", new_id),
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
    entry
        .transitions
        .retain(|t| t.from != state_id && t.to != state_id);

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
) -> TransitionResult {
    let mut entry = match state.get(automaton_id) {
        Some(e) => e,
        None => {
            return TransitionResult {
                status: 400,
                message: format!("Автомат с id {} не найден", automaton_id),
                transition: vec![],
            };
        }
    };

    if !entry.states.iter().any(|s| s.id == from) {
        return TransitionResult {
            status: 400,
            message: format!("Состояние {} не существует", from),
            transition: vec![],
        };
    }
    if !entry.states.iter().any(|s| s.id == to) {
        return TransitionResult {
            status: 400,
            message: format!("Состояние {} не существует", to),
            transition: vec![],
        };
    }

    let mut count = 0u32;
    let mut added: Vec<TransitionData> = Vec::new();
    for &symbol in &symbols {
        let sym_str = symbol;
        let already_exists = entry
            .transitions
            .iter()
            .any(|t| t.from == from && t.to == to && t.symbol == sym_str);

        if already_exists {
            return TransitionResult {
                status: 400,
                message: format!("Переход {} -> {} по '{}' уже существует", from, to, symbol),
                transition: vec![],
            };
        }

        if symbol != EPSILON && !entry.alphabet.contains(&symbol) {
            entry.alphabet.push(symbol);
        }

        let used: std::collections::HashSet<i32> = entry.transitions.iter().map(|t| t.id).collect();
        let tid = id_gen::generate_id(&used);
        let created = TransitionData {
            id: tid,
            from,
            to,
            symbol: sym_str,
        };
        entry.transitions.push(created.clone());
        added.push(created);
        count += 1;
    }

    state.update(entry.clone());
    TransitionResult {
        status: 200,
        message: format!("{} переход(ов) {} -> {} добавлено", count, from, to),
        transition: added,
    }
}

#[tauri::command]
pub fn nfa_update_transition(
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
                status: 400,
                message: format!("Автомат с id {} не найден", automaton_id),
                transition: vec![],
            };
        }
    };

    let idx = match entry.transitions.iter().position(|t| t.id == transition_id) {
        Some(i) => i,
        None => {
            return TransitionResult {
                status: 400,
                message: format!("Переход {} не найден", transition_id),
                transition: vec![],
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
        entry.transitions[idx].symbol = s;
    }

    state.update(entry.clone());
    TransitionResult {
        status: 200,
        message: "Переход обновлён".to_string(),
        transition: vec![entry.transitions[idx].clone()],
    }
}

#[tauri::command]
pub fn nfa_remove_transition(
    state: State<'_, AutomatonStore>,
    automaton_id: i32,
    transition_id: i32,
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
    entry.transitions.retain(|t| t.id != transition_id);
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

#[tauri::command]
pub fn nfa_run_str(
    state: State<'_, AutomatonStore>,
    automaton_id: i32,
    input: String,
) -> RunResult {
    let entry = match state.get(automaton_id) {
        Some(e) => e,
        None => {
            return RunResult {
                status: 404,
                message: format!("Автомат с id {} не найден", automaton_id),
                traces: Vec::new(),
            };
        }
    };

    let chars: Vec<char> = input.chars().collect();
    match data_to_nfa(&entry.states, &entry.transitions, &entry.alphabet) {
        Ok(nfa) => {
            let (mut traces, accepted) = nfa.run_partial(&chars);
            // Order the history so that accepted reading streams (isFinal) are
            // listed before rejected ones.
            traces.sort_by_key(|t| !t.isFinal);
            // `$` (ε-closure) steps are part of the history; count only the
            // symbol transitions actually consumed to derive the processed length.
            let processed_len = traces
                .iter()
                .map(|t| t.steps.iter().filter(|s| s.symbol != EPSILON).count())
                .max()
                .unwrap_or(0);
            let (status, message) = if accepted {
                (200u16, format!("Цепочка '{}' принята", input))
            } else if processed_len > 0 {
                (
                    401,
                    format!(
                        "Цепочка '{}' принята частично (обработано {} из {} символов)",
                        input,
                        processed_len,
                        chars.len()
                    ),
                )
            } else {
                (402, format!("Цепочка '{}' отклонена", input))
            };
            RunResult {
                status,
                message,
                traces,
            }
        }
        Err(err) => RunResult {
            status: 400,
            message: format!("Некорректный автомат: {}", err),
            traces: Vec::new(),
        },
    }
}

#[tauri::command]
pub fn nfa_multiple_run_str(
    state: State<'_, AutomatonStore>,
    automaton_id: i32,
    inputs: Vec<String>,
) -> MultiRunResult {
    let entry = match state.get(automaton_id) {
        Some(e) => e,
        None => {
            return MultiRunResult {
                status: 404,
                message: format!("Автомат с id {} не найден", automaton_id),
                traces: Vec::new(),
            };
        }
    };

    let nfa = match data_to_nfa(&entry.states, &entry.transitions, &entry.alphabet) {
        Ok(n) => n,
        Err(_) => {
            return MultiRunResult {
                status: 400,
                message: format!("Некорректный автомат"),
                traces: Vec::new(),
            };
        }
    };

    let traces: Vec<LineTest> = inputs
        .into_iter()
        .map(|line| {
            let (is_final, correct_symbols) = test_line(&nfa, &line);
            LineTest {
                line,
                isFinal: is_final,
                correctSymbols: correct_symbols,
            }
        })
        .collect();

    MultiRunResult {
        status: 200,
        message: format!(""),
        traces,
    }
}

/// Generates an ordered set of test strings covering a wide range of NFA
/// scenarios (empty, singles, paths to every state, transition activation,
/// negative/dead-end cases, cyclic and long strings). Ready to paste into the
/// Multiple Run fields.
#[tauri::command]
pub fn nfa_generate_inputs(
    state: State<'_, AutomatonStore>,
    automaton_id: i32,
) -> GenerateInputsResult {
    let entry = match state.get(automaton_id) {
        Some(e) => e,
        None => {
            return GenerateInputsResult {
                status: 404,
                message: format!("Автомат с id {} не найден", automaton_id),
                inputs: Vec::new(),
            };
        }
    };

    let nfa = match data_to_nfa(&entry.states, &entry.transitions, &entry.alphabet) {
        Ok(n) => n,
        Err(_) => {
            return GenerateInputsResult {
                status: 400,
                message: format!("Некорректный автомат"),
                inputs: Vec::new(),
            };
        }
    };

    let resp_vec: Vec<String> = nfa.generate_test_inputs(50, 15);

    GenerateInputsResult {
        status: 200,
        message: format!("Сгенерировано {} тестовых входов", resp_vec.len()),
        inputs: resp_vec,
    }
}

/// Runs a single line on the NFA. Returns whether the whole line is accepted
/// (`true` if at least one thread reaches a final state after consuming all of
/// it) and how many symbols were consumed correctly (only non-`$` steps).
pub(crate) fn test_line(nfa: &NFA, input: &str) -> (bool, usize) {
    let chars: Vec<char> = input.chars().collect();
    let (traces, accepted) = nfa.run_partial(&chars);
    let correct_symbols = traces
        .iter()
        .map(|t| t.steps.iter().filter(|s| s.symbol != EPSILON).count())
        .max()
        .unwrap_or(0);
    (accepted, correct_symbols)
}

#[tauri::command]
pub fn nfa_remove_automaton(state: State<'_, AutomatonStore>, automaton_id: i32) -> StatusResult {
    match state.remove(automaton_id) {
        Some(_) => StatusResult {
            status: 200,
            message: format!("Автомат с id {} удалён", automaton_id),
        },
        None => StatusResult {
            status: 400,
            message: format!("Автомат с id {} не найден", automaton_id),
        },
    }
}

pub(crate) fn data_to_nfa(
    states: &[StateData],
    transitions: &[TransitionData],
    alphabet: &[char],
) -> Result<NFA, String> {
    let mut builder = NFABuilder::new();

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
