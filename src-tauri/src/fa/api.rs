use tauri::{State, http::StatusCode};

use crate::core::api::{check_automaton_multi_run, check_automaton_run, generate_id};
use crate::core::automaton::AutomatonStore;
use crate::core::types::{
    AutomatonData, GenerateInputsResult, LineTest, MultiRunResult, OperationResult, RunResult,
    StateData, StateResult, StatusResult, TransitionData, TransitionResult,
    alphabet_from_transitions,
};
use crate::fa::types::{EPSILON, FA, FABuilder};

/// Создаёт новый конечный автомат с единственным начальным состоянием `q0`.
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
pub fn fa_add_state(
    state: State<'_, AutomatonStore>,
    automaton_id: i32,
    label: Option<String>,
    x: Option<f32>,
    y: Option<f32>,
    is_initial: Option<bool>,
    is_final: Option<bool>,
) -> StateResult {
    let (new_id, created) = match state.modify(automaton_id, |entry| {
        let used: std::collections::HashSet<i32> = entry.states.iter().map(|s| s.id).collect();
        let new_id = generate_id(&used);

        entry.states.push(StateData {
            id: new_id,
            label: label.unwrap_or_else(|| format!("q{}", new_id)),
            x: x.unwrap_or(100.0),
            y: y.unwrap_or(200.0),
            isInitial: is_initial.unwrap_or(false),
            isFinal: Some(is_final.unwrap_or(false)),
        });

        let created = entry.states.last().unwrap().clone();
        (new_id, created)
    }) {
        Some(v) => v,
        None => {
            return StateResult {
                status: StatusCode::NOT_FOUND,
                message: format!("Автомат с id {} не найден", automaton_id),
                state: None,
            };
        }
    };

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
    let updated = match state.modify(automaton_id, |entry| {
        let idx = entry
            .states
            .iter()
            .position(|s| s.id == state_id)
            .ok_or_else(|| format!("Состояние {} не существует", state_id))?;

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
            entry.states[idx].isFinal = Some(fin);
        }

        Ok(entry.states[idx].clone())
    }) {
        Some(Ok(updated)) => updated,
        Some(Err(message)) => {
            return StateResult {
                status: StatusCode::BAD_REQUEST,
                message,
                state: None,
            };
        }
        None => {
            return StateResult {
                status: StatusCode::NOT_FOUND,
                message: format!("Автомат с id {} не найден", automaton_id),
                state: None,
            };
        }
    };

    StateResult {
        status: StatusCode::OK,
        message: format!("Состояние {} обновлено", state_id),
        state: Some(updated),
    }
}

/// Удаляет состояние вместе со всеми связанными с ним переходами (входящими,
/// исходящими и петлями).
#[tauri::command]
pub fn fa_remove_state(
    state: State<'_, AutomatonStore>,
    automaton_id: i32,
    state_id: i32,
) -> StatusResult {
    match state.modify(automaton_id, |entry| {
        if !entry.states.iter().any(|s| s.id == state_id) {
            return Err(format!("Состояние {} не существует", state_id));
        }

        entry.states.retain(|s| s.id != state_id);
        entry
            .transitions
            .retain(|t| t.from != state_id && t.to != state_id);
        Ok(())
    }) {
        Some(Ok(())) => StatusResult {
            status: StatusCode::OK,
            message: format!("Состояние {} удалено", state_id),
        },
        Some(Err(message)) => StatusResult {
            status: StatusCode::BAD_REQUEST,
            message,
        },
        None => StatusResult {
            status: StatusCode::NOT_FOUND,
            message: format!("Автомат с id {} не найден", automaton_id),
        },
    }
}

/// Добавляет переходы. Для каждого символа создаётся отдельный переход с теми же
/// `from`/`to`. Идентификаторы генерирует сервер.
///
/// Отклоняются только точные дубликаты `(from, to, symbol)`; eps-переходы `$`
/// разрешены.
#[tauri::command]
pub fn fa_add_transition(
    state: State<'_, AutomatonStore>,
    automaton_id: i32,
    from: i32,
    to: i32,
    symbols: Vec<char>,
) -> TransitionResult {
    let added = match state.modify(automaton_id, |entry| {
        if !entry.states.iter().any(|s| s.id == from) {
            return Err(format!("Состояние {} не существует", from));
        }
        if !entry.states.iter().any(|s| s.id == to) {
            return Err(format!("Состояние {} не существует", to));
        }

        // Сначала проверяем весь пакет символов, чтобы при конфликте не оставить
        // частично добавленные переходы (семантика «всё или ничего»).
        let mut planned: Vec<char> = Vec::with_capacity(symbols.len());
        for &symbol in &symbols {
            let duplicate = planned.contains(&symbol)
                || entry
                    .transitions
                    .iter()
                    .any(|t| t.from == from && t.to == to && t.symbol == symbol);
            if duplicate {
                return Err(format!(
                    "Переход {} -> {} по '{}' уже существует",
                    from, to, symbol
                ));
            }
            planned.push(symbol);
        }

        let mut added: Vec<TransitionData> = Vec::with_capacity(planned.len());
        for symbol in planned {
            let used: std::collections::HashSet<i32> =
                entry.transitions.iter().map(|t| t.id).collect();
            let created = TransitionData {
                id: generate_id(&used),
                from,
                to,
                symbol,
                output: None,
            };
            entry.transitions.push(created.clone());
            added.push(created);
        }
        Ok(added)
    }) {
        Some(Ok(added)) => added,
        Some(Err(message)) => {
            return TransitionResult {
                status: StatusCode::BAD_REQUEST,
                message,
                transition: vec![],
            };
        }
        None => {
            return TransitionResult {
                status: StatusCode::NOT_FOUND,
                message: format!("Автомат с id {} не найден", automaton_id),
                transition: vec![],
            };
        }
    };

    TransitionResult {
        status: StatusCode::OK,
        message: format!("{} переход(ов) {} -> {} добавлено", added.len(), from, to),
        transition: added,
    }
}

/// Частично обновляет переход, найденный по `transition_id`; меняются только
/// переданные поля.
#[tauri::command]
pub fn fa_update_transition(
    state: State<'_, AutomatonStore>,
    automaton_id: i32,
    transition_id: i32,
    new_from: Option<i32>,
    new_to: Option<i32>,
    new_symbol: Option<char>,
) -> TransitionResult {
    let updated = match state.modify(automaton_id, |entry| {
        let idx = entry
            .transitions
            .iter()
            .position(|t| t.id == transition_id)
            .ok_or_else(|| format!("Переход {} не найден", transition_id))?;

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

        entry.transitions[idx] = updated.clone();
        Ok(updated)
    }) {
        Some(Ok(updated)) => updated,
        Some(Err(message)) => {
            return TransitionResult {
                status: StatusCode::BAD_REQUEST,
                message,
                transition: vec![],
            };
        }
        None => {
            return TransitionResult {
                status: StatusCode::NOT_FOUND,
                message: format!("Автомат с id {} не найден", automaton_id),
                transition: vec![],
            };
        }
    };

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
    let removed = match state.modify(automaton_id, |entry| {
        let original_count = entry.transitions.len();
        entry.transitions.retain(|t| t.id != transition_id);
        original_count - entry.transitions.len()
    }) {
        Some(removed) => removed,
        None => {
            return StatusResult {
                status: StatusCode::NOT_FOUND,
                message: format!("Автомат с id {} не найден", automaton_id),
            };
        }
    };

    let (status, msg) = if removed > 0 {
        (StatusCode::OK, format!("Удалено {} переход(ов)", removed))
    } else {
        (StatusCode::NOT_FOUND, "Переход не найден".to_string())
    };

    StatusResult {
        status: status,
        message: msg,
    }
}

/// Прогоняет цепочку на автомате (параллельная проверка в стиле JFLAP через
/// `FA::run_partial`, статусы `200`/`401`/`402`). Принимает вход целой строкой
/// (не массивом).
#[tauri::command]
pub fn fa_run_str(state: State<'_, AutomatonStore>, automaton_id: i32, input: String) -> RunResult {
    let entry = match check_automaton_run(state.inner(), automaton_id) {
        Ok(e) => e,
        Err(e) => return e,
    };

    run_fa_line(&entry, &input)
}

/// Тестируемый прогон одной строки без `State`: строит КА, запускает
/// `FA::run_partial` (где eps-переходы ветвятся) и формирует `RunResult`.
/// Возвращает все трейсы, включая тупиковые; статус 200 выставляется, если
/// принята хотя бы одна ветвь.
pub(crate) fn run_fa_line(entry: &AutomatonData, input: &str) -> RunResult {
    let chars: Vec<char> = input.chars().collect();
    match data_to_fa(&entry.states, &entry.transitions) {
        Ok(fa) => {
            let (mut traces, accepted) = fa.run_partial(&chars);
            // Упорядочиваем историю так, чтобы допускающие потоки чтения (isFinal) шли
            // раньше отвергнутых.
            traces.sort_by_key(|t| !t.isFinal);
            // Шаги `$` (eps-ветвление) входят в историю; для длины обработанной части
            // считаем только реально прочитанные символьные переходы.
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
                (
                    StatusCode::PAYMENT_REQUIRED,
                    format!("Цепочка '{}' отклонена", input),
                )
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

/// Прогоняет сразу несколько цепочек и возвращает по каждой строке только факт
/// принятия/отклонения (без трасс шагов). Использует тот же алгоритм, что и
/// `fa_run_str`.
#[tauri::command]
pub fn fa_multi_run_str(
    state: State<'_, AutomatonStore>,
    automaton_id: i32,
    inputs: Vec<String>,
) -> MultiRunResult {
    let entry = match check_automaton_multi_run(state.inner(), automaton_id) {
        Ok(e) => e,
        Err(e) => return e,
    };

    multi_run_fa(&entry, &inputs)
}

/// Тестируемый пакетный прогон: строит КА один раз и пропускает каждый вход
/// через `test_line`, сохраняя порядок входов. Некорректный автомат даёт `400`.
pub(crate) fn multi_run_fa(entry: &AutomatonData, inputs: &[String]) -> MultiRunResult {
    match crate::core::api::multi_run_automaton(entry, inputs, data_to_fa, |fa, line| {
        let (is_final, correct_symbols) = test_line(fa, line);
        LineTest {
            line: line.to_string(),
            isFinal: is_final,
            correctSymbols: correct_symbols,
        }
    }) {
        Ok(traces) => MultiRunResult {
            status: StatusCode::OK,
            message: String::new(),
            traces,
        },
        Err(_) => MultiRunResult {
            status: StatusCode::BAD_REQUEST,
            message: "Некорректный автомат".to_string(),
            traces: Vec::new(),
        },
    }
}

/// Генерирует упорядоченный набор тестовых цепочек, покрывающий широкий спектр
/// сценариев (пустая строка, одиночные символы, пути до каждого состояния,
/// срабатывание переходов, негативные/тупиковые случаи и короткие повторы
/// циклов (2–3 прохода). Результат готов для вставки в поля Multiple Run.
#[tauri::command]
pub fn fa_generate_inputs(
    state: State<'_, AutomatonStore>,
    automaton_id: i32,
) -> GenerateInputsResult {
    // Генерация живёт в core: оттуда же берут входы и команда Мили. Так
    // исправления петель/юникода применяются и к этому frontend-методу.
    crate::core::api::gen_multi_run_for_output(state.inner(), automaton_id)
}

/// Удаляет автомат по его id.
#[tauri::command]
pub fn fa_remove_automaton(state: State<'_, AutomatonStore>, automaton_id: i32) -> StatusResult {
    crate::core::api::remove_automaton(state, automaton_id)
}

/// Прогоняет одну строку на автомате. Возвращает, принята ли строка целиком
/// (`true`, если хотя бы один поток после прочтения всей строки оказался в
/// финальном состоянии), и сколько символов прочитано корректно (учитываются
/// только шаги, отличные от `$`).
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
) -> Result<FA, String> {
    let mut builder = FABuilder::new();

    for state in states {
        builder = builder.state(state.id);
        if state.isInitial {
            builder = builder.set_initial(state.id);
        }
        if state.isFinal.unwrap() {
            builder = builder.set_final(state.id);
        }
    }

    for symbol in alphabet_from_transitions(transitions) {
        builder = builder.symbol(symbol);
    }

    // Игнорируем переходы, ссылающиеся на отсутствующие в автомате состояния
    // (осиротевшие ID): они устарели и не должны порождать лишние ветви.
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
