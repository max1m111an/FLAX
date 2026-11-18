use std::collections::HashSet;

use tauri::{State, http::StatusCode};

use crate::core::api::generate_id;
use crate::core::types::{
    AutomatonData, AutomatonStore, AutomatonType, MealyLineTest, MealyMultiRunResult,
    MealyRunResult, OperationResult, StateData, StateResult, StatusResult, TransitionData,
    TransitionResult,
};
use crate::fa::types::EPSILON;
use crate::mealy::types::MealyBuilder;

/// Создаёт в хранилище новую машину Мили с единственным начальным состоянием
/// `q0` и `automatonType = MEALY`.
///
/// Вынесена отдельно от Tauri-команды, чтобы тестироваться без `tauri::State`
/// (то же соглашение, что `data_to_fa` / `test_line` в `fa`).
pub(crate) fn create_new_mealy(state: &AutomatonStore, name: Option<String>) -> OperationResult {
    let mut entry = state.create(name.unwrap_or_else(|| "Мили".to_string()), "q0");
    entry.automatonType = AutomatonType::MEALY;
    entry.states[0].isFinal = None;
    state.update(entry.clone());
    OperationResult {
        status: StatusCode::OK,
        message: "Автомат Мили создан".to_string(),
        automaton: Some(entry),
    }
}

/// Добавляет в запись один переход машины Мили `from --input/output--> to`.
///
/// Валидация выполняется до любых изменений, поэтому при `Err` запись не
/// меняется. Алфавит выводится из переходов (`AutomatonData::get_alphabet`),
/// поэтому здесь он не поддерживается.
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

/// Обновляет символ `input` и/или `output` существующего перехода.
///
/// `None` оставляет соответствующее поле без изменений; `from`/`to`/`id` никогда
/// не меняются. Валидация выполняется до любых изменений, поэтому при `Err` запись
/// не меняется. Конфликтующая пара `(from, input)` отклоняется, чтобы сохранить
/// инвариант детерминизма машины Мили. Алфавит выводится из переходов
/// (`AutomatonData::get_alphabet`), поэтому здесь он не поддерживается.
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

/// Удаляет из записи переход с заданным `transition_id`.
///
/// Возвращает `Err`, если перехода с таким id нет, чтобы вызывающий код мог
/// отличить «не найдено» от успеха без подсчёта элементов; в этом случае запись
/// не меняется. Алфавит выводится из переходов, поэтому обновляется автоматически.
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

/// Строит [`MM`](crate::mealy::types::MM) из сохранённых DTO, аналогично
/// `data_to_fa` в модуле `fa`. Переходы, ссылающиеся на отсутствующие состояния,
/// игнорируются (осиротевшие ID). Переход без выходного символа или с символом
/// eps делает автомат некорректным.
pub(crate) fn data_to_mm(
    states: &[StateData],
    transitions: &[TransitionData],
) -> Result<crate::mealy::types::MM, String> {
    let mut builder = MealyBuilder::new();

    let mut has_initial = false;
    for state in states {
        builder = builder.state(state.id);
        if state.isInitial {
            builder = builder.set_initial(state.id);
            has_initial = true;
        }
    }
    if !has_initial {
        return Err("Начальное состояние не задано".to_string());
    }

    let state_ids: std::collections::HashSet<i32> = states.iter().map(|s| s.id).collect();
    for trans in transitions {
        if !state_ids.contains(&trans.from) || !state_ids.contains(&trans.to) {
            continue;
        }
        if trans.symbol == EPSILON {
            return Err("Машина Мили не поддерживает eps-переходы".to_string());
        }
        let output = trans
            .output
            .ok_or_else(|| format!("Переход {} не имеет выходного символа", trans.id))?;
        builder = builder.transition(trans.from, trans.symbol, trans.to, output);
    }

    builder.build()
}

/// Прогоняет цепочку на машине Мили. У машины Мили нет допускающих состояний:
/// она либо обрабатывает весь вход (статус `200` с полученным выходом), либо
/// останавливается на первой паре `(state, input)` без перехода (`401`, если
/// часть символов обработана, `402`, если ни одного).
pub(crate) fn run_mealy(entry: &AutomatonData, input: &str) -> MealyRunResult {
    let mm = match data_to_mm(&entry.states, &entry.transitions) {
        Ok(mm) => mm,
        Err(err) => {
            return MealyRunResult {
                status: StatusCode::BAD_REQUEST,
                message: format!("Некорректный автомат: {}", err),
                output: String::new(),
                steps: Vec::new(),
            };
        }
    };

    let chars: Vec<char> = input.chars().collect();
    let (steps, output) = mm.run(&chars);
    let processed = steps.len();
    let total = chars.len();

    let (status, message) = if processed == total {
        (
            StatusCode::OK,
            format!("Цепочка '{}' обработана, выход '{}'", input, output),
        )
    } else if processed > 0 {
        (
            StatusCode::UNAUTHORIZED,
            format!(
                "Цепочка '{}' обработана частично (обработано {} из {} символов)",
                input, processed, total
            ),
        )
    } else {
        (
            StatusCode::PAYMENT_REQUIRED,
            format!("Цепочка '{}' не обработана", input),
        )
    };

    MealyRunResult {
        status,
        message,
        output,
        steps,
    }
}

/// Прогоняет несколько цепочек на машине Мили и возвращает по одной записи на
/// строку без трасс шагов. Частичный прогон отражается через `isComplete`, а не
/// статусом строки: статус запроса — `200` для корректной машины и `400`, если
/// сама машина некорректна.
pub(crate) fn multi_run_mealy(entry: &AutomatonData, inputs: &[String]) -> MealyMultiRunResult {
    match crate::core::api::multi_run_automaton(entry, inputs, data_to_mm, |mm, line| {
        let chars: Vec<char> = line.chars().collect();
        let (steps, output) = mm.run(&chars);
        MealyLineTest {
            line: line.to_string(),
            output,
            isComplete: steps.len() == chars.len(),
        }
    }) {
        Ok(results) => MealyMultiRunResult {
            status: StatusCode::OK,
            message: String::new(),
            results,
        },
        Err(err) => MealyMultiRunResult {
            status: StatusCode::BAD_REQUEST,
            message: format!("Некорректный автомат: {}", err),
            results: Vec::new(),
        },
    }
}

/// Создаёт новую машину Мили с единственным начальным состоянием `q0`.
#[tauri::command]
pub fn mealy_create_new(state: State<'_, AutomatonStore>, name: Option<String>) -> OperationResult {
    create_new_mealy(&state, name)
}

/// Добавляет состояние в машину Мили. Делегирует в `fa_add_state`: состояния
/// в хранилище общие для FA и MM.
#[tauri::command]
pub fn mealy_add_state(
    state: State<'_, AutomatonStore>,
    automaton_id: i32,
    label: Option<String>,
    x: Option<f32>,
    y: Option<f32>,
    is_initial: Option<bool>,
) -> StateResult {
    crate::fa::api::fa_add_state(state, automaton_id, label, x, y, is_initial, None)
}

/// Обновляет состояние машины Мили. Делегирует в `fa_update_state`.
#[tauri::command]
#[allow(clippy::too_many_arguments)]
pub fn mealy_update_state(
    state: State<'_, AutomatonStore>,
    automaton_id: i32,
    state_id: i32,
    label: Option<String>,
    x: Option<f32>,
    y: Option<f32>,
    is_initial: Option<bool>,
) -> StateResult {
    crate::fa::api::fa_update_state(
        state,
        automaton_id,
        state_id,
        label,
        x,
        y,
        is_initial,
        None,
    )
}

/// Удаляет состояние и все связанные с ним переходы. Делегирует в
/// `fa_remove_state`.
#[tauri::command]
pub fn mealy_remove_state(
    state: State<'_, AutomatonStore>,
    automaton_id: i32,
    state_id: i32,
) -> StatusResult {
    crate::fa::api::fa_remove_state(state, automaton_id, state_id)
}

/// Добавляет один переход машины Мили `from --input/output--> to`.
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

#[tauri::command]
pub fn mealy_run_str(
    state: State<'_, AutomatonStore>,
    automaton_id: i32,
    input: String,
) -> MealyRunResult {
    let entry = match state.get(automaton_id) {
        Some(e) => e,
        None => {
            return MealyRunResult {
                status: StatusCode::NOT_FOUND,
                message: format!("Автомат с id {} не найден", automaton_id),
                output: String::new(),
                steps: Vec::new(),
            };
        }
    };

    run_mealy(&entry, &input)
}

/// Прогоняет сразу несколько цепочек и возвращает по каждой строке только выход
/// и признак полноты (без трасс шагов). Использует тот же алгоритм, что и
/// `mealy_run_str`.
#[tauri::command]
pub fn mealy_multi_run(
    state: State<'_, AutomatonStore>,
    automaton_id: i32,
    inputs: Vec<String>,
) -> MealyMultiRunResult {
    let entry = match state.get(automaton_id) {
        Some(e) => e,
        None => {
            return MealyMultiRunResult {
                status: StatusCode::NOT_FOUND,
                message: format!("Автомат с id {} не найден", automaton_id),
                results: Vec::new(),
            };
        }
    };

    multi_run_mealy(&entry, &inputs)
}

/// Удаляет машину Мили из хранилища. Делегирует в `fa_remove_automaton`:
/// хранилище общее для FA и MM.
#[tauri::command]
pub fn mealy_remove_automaton(state: State<'_, AutomatonStore>, automaton_id: i32) -> StatusResult {
    crate::fa::api::fa_remove_automaton(state, automaton_id)
}
