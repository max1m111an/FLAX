use tauri::command;

use crate::structs::{
    automata::Automaton,
    data_models::{NFAData, OperationResult, StateData, TransitionData},
    nfa::{NFA, NFABuilder},
};

/// Создает новый NFA (начало работы)
#[command]
pub fn create_new_nfa() -> OperationResult {
    // Создаем пустой NFA с начальным состоянием q0
    let nfa = NFABuilder::new()
        .state("q0")
        .initial("q0")
        .build();
    
    match nfa {
        Ok(nfa) => {
            OperationResult {
                success: true,
                message: format!("NFA успешно создан"),
                automaton: Some(nfa_to_data(&nfa)),
            }
        }
        Err(e) => {
            OperationResult {
                success: false,
                message: format!("Ошибка создания NFA: {}", e),
                automaton: None,
            }
        }
    }
}

/// Добавляет новое состояние в NFA
#[command]
pub fn add_state(current_nfa: NFAData, state_id: String, label: String) -> OperationResult {
    // Преобразуем NFAData в NFA
    let nfa_result: Result<NFA, String> = data_to_nfa(&current_nfa);
    
    let nfa: NFA = match nfa_result {
        Ok(nfa) => nfa,
        Err(e) => {
            return OperationResult {
                success: false,
                message: format!("Ошибка преобразования данных: {}", e),
                automaton: Some(current_nfa),
            };
        }
    };
    
    // Проверяем, существует ли уже такое состояние
    if nfa.states().contains(&state_id) {
        return OperationResult {
            success: false,
            message: format!("Состояние '{}' уже существует", state_id),
            automaton: Some(current_nfa),
        };
    }
    
    let new_state: StateData = StateData {
        id: state_id.clone(),
        label,
        is_initial: false,
        is_final: false,
        position: None,
    };

    let mut updated_states: Vec<StateData> = current_nfa.states.clone();
    updated_states.push(new_state);

    let updated_nfa: NFAData = NFAData {
        states: updated_states,
        transitions: current_nfa.transitions.clone(),
        alphabet: current_nfa.alphabet.clone(),
    };
    
    // Для простоты примера вернем ошибку
    OperationResult {
        success: true,
        message: format!("Состояние '{}' добавлено", state_id),
        automaton: Some(updated_nfa), // Здесь должен быть обновленный автомат
    }
}

/// Добавляет переход между состояниями
#[command]
pub fn add_transition(
    current_nfa: NFAData, 
    from: String, 
    to: String, 
    symbol: Option<char>
) -> OperationResult {
    let from_exists: bool = current_nfa.states.iter().any(|s| s.id == from);
    let to_exists: bool = current_nfa.states.iter().any(|s| s.id == to);

    if !from_exists {
        return OperationResult {
            success: false,
            message: format!("Исходное состояние '{}' не существует", from),
            automaton: Some(current_nfa),
        };
    }
    if !to_exists {
        return OperationResult {
            success: false,
            message: format!("Целевое состояние '{}' не существует", to),
            automaton: Some(current_nfa),
        };
    }

    let transition_exists: bool = current_nfa.transitions.iter().any(|t| 
        t.from == from && t.to == to && t.symbol == symbol
    );
    if transition_exists {
        let symbol_str: String = match symbol {
            Some(c) => format!("'{}'", c),
            None => "ε".to_string(),
        };
        return OperationResult {
            success: false,
            message: format!("Переход из '{}' в '{}' по {} уже существует", from, to, symbol_str),
            automaton: Some(current_nfa),
        };
    }

    let mut updated_alphabet: Vec<char> = current_nfa.alphabet.clone();
    if let Some(s) = symbol {
        if !updated_alphabet.contains(&s) {
            updated_alphabet.push(s);
        }
    }

    let new_transition: TransitionData = TransitionData {
        from: from.clone(),
        to: to.clone(),
        symbol,
    };

    let mut updated_transitions: Vec<TransitionData> = current_nfa.transitions.clone();
    updated_transitions.push(new_transition);

    let updated_nfa: NFAData = NFAData {
        states: current_nfa.states.clone(),
        transitions: updated_transitions,
        alphabet: updated_alphabet,
    };

    let symbol_str: String = match symbol {
        Some(c) => format!("'{}'", c),
        None => "ε".to_string(),
    };

    OperationResult {
        success: true,
        message: format!("Переход из '{}' в '{}' по {} добавлен", from, to, symbol_str),
        automaton: Some(updated_nfa),
    }
}

/// Удаляет состояние
#[command]
pub fn remove_state(current_nfa: NFAData, state_id: String) -> OperationResult {
    // Проверяем, существует ли состояние
    if !current_nfa.states.iter().any(|s| s.id == state_id) {
        return OperationResult {
            success: false,
            message: format!("Состояние '{}' не существует", state_id),
            automaton: Some(current_nfa),
        };
    }
    
    // Удаляем состояние
    let updated_states: Vec<StateData> = current_nfa.states
        .into_iter()
        .filter(|s| s.id != state_id)
        .collect();
    
    // Удаляем все переходы, связанные с этим состоянием
    let updated_transitions: Vec<TransitionData> = current_nfa.transitions
        .into_iter()
        .filter(|t| t.from != state_id && t.to != state_id)
        .collect();
    
    OperationResult {
        success: true,
        message: format!("Состояние '{}' удалено", state_id),
        automaton: Some(NFAData {
            states: updated_states,
            transitions: updated_transitions,
            alphabet: current_nfa.alphabet,
        }),
    }
}

/// Удаляет переход
#[command]
pub fn remove_transition(current_nfa: NFAData, from: String, to: String, symbol: Option<char>) -> OperationResult {
    let updated_transitions: Vec<TransitionData> = current_nfa.transitions
        .into_iter()
        .filter(|t| !(t.from == from && t.to == to && t.symbol == symbol))
        .collect();
    
    let symbol_str = match symbol {
        Some(c) => format!("'{}'", c),
        None => "ε".to_string(),
    };
    
    OperationResult {
        success: true,
        message: format!("Переход из '{}' в '{}' по {} удален", from, to, symbol_str),
        automaton: Some(NFAData {
            states: current_nfa.states,
            transitions: updated_transitions,
            alphabet: current_nfa.alphabet,
        }),
    }
}

/// Проверяет строку на принятие автоматом
#[command]
pub fn check_string(current_nfa: NFAData, input: String) -> OperationResult {
    match data_to_nfa(&current_nfa) {
        Ok(nfa) => {
            let chars: Vec<char> = input.chars().collect();
            let accepts: bool = nfa.accepts(&chars);
            
            OperationResult {
                success: true,
                message: format!(
                    "Строка '{}' {} принята",
                    input,
                    if accepts { "" } else { "не" }
                ),
                automaton: Some(current_nfa),
            }
        }
        Err(e) => {
            OperationResult {
                success: false,
                message: format!("Ошибка проверки строки: {}", e),
                automaton: Some(current_nfa),
            }
        }
    }
}

/// Вспомогательная функция: преобразует NFA в NFAData для отправки на фронтенд
fn nfa_to_data(nfa: &NFA) -> NFAData {
    let states: Vec<StateData> = nfa.states()
        .iter()
        .map(|id| StateData {
            id: id.clone(),
            label: id.clone(),
            is_initial: id == nfa.initial_state(),
            is_final: nfa.accepting_states().contains(id),
            position: None, // Позиция будет установлена на фронтенде
        })
        .collect();
    
    let mut transitions: Vec<TransitionData> = Vec::new();
    for ((from, symbol_opt), to_set) in nfa.get_transitions() {
        for to in to_set {
            transitions.push(TransitionData {
                from: from.clone(),
                to: to.clone(),
                symbol: *symbol_opt,
            });
        }
    }
    
    NFAData {
        states,
        transitions,
        alphabet: nfa.alphabet().iter().copied().collect(),
    }
}

/// Вспомогательная функция: преобразует NFAData в NFA
fn data_to_nfa(data: &NFAData) -> Result<NFA, String> {
    let mut builder: NFABuilder = NFABuilder::new();
    
    // Добавляем состояния
    for state in &data.states {
        builder = builder.state(state.id.clone());
        
        if state.is_initial {
            builder = builder.initial(state.id.clone());
        }
        
        if state.is_final {
            builder = builder.accepting(state.id.clone());
        }
    }
    
    // Добавляем алфавит
    for &symbol in &data.alphabet {
        builder = builder.symbol(symbol);
    }
    
    // Добавляем переходы
    for trans in &data.transitions {
        match trans.symbol {
            Some(symbol) => {
                builder = builder.transition(
                    trans.from.clone(),
                    symbol,
                    trans.to.clone()
                );
            }
            None => {
                builder = builder.epsilon(
                    trans.from.clone(),
                    trans.to.clone()
                );
            }
        }
    }
    
    builder.build()
}