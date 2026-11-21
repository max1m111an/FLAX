use std::collections::HashSet;

use tauri::State;

use crate::core::api::{check_automaton_mealy_multi_run, check_automaton_mealy_run, generate_id};
use crate::core::automaton::AutomatonStore;
use crate::core::types::{
    AutomatonData, AutomatonType, GenerateInputsResult, MealyLineTest, MealyMultiRunResult,
    MealyRunResult, OperationResult, Response, StateData, StateResult, TransitionData,
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
    let created = state.create(name.unwrap_or_else(|| "Мили".to_string()), "q0");
    let entry = state
        .modify(created.id, |entry| {
            entry.automatonType = AutomatonType::MEALY;
            entry.states[0].isFinal = None;
            entry.clone()
        })
        .unwrap_or(created);
    OperationResult {
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
/// она либо обрабатывает весь вход, либо останавливается на первой паре
/// `(state, input)` без перехода. Факт полной обработки отражается в длине
/// `steps` и выходной строке.
pub(crate) fn run_mealy(entry: &AutomatonData, input: &str) -> Result<MealyRunResult, Response> {
    let mm = match data_to_mm(&entry.states, &entry.transitions) {
        Ok(mm) => mm,
        Err(err) => {
            return Err(Response {
                status: 400,
                message: format!("Некорректный автомат: {}", err),
            });
        }
    };

    let chars: Vec<char> = input.chars().collect();
    let (steps, output) = mm.run(&chars);

    Ok(MealyRunResult { output, steps })
}

/// Прогоняет несколько цепочек на машине Мили и возвращает по одной записи на
/// строку без трасс шагов. Частичный прогон отражается через `isComplete`.
pub(crate) fn multi_run_mealy(
    entry: &AutomatonData,
    inputs: &[String],
) -> Result<MealyMultiRunResult, Response> {
    match crate::core::api::multi_run_automaton(entry, inputs, data_to_mm, |mm, line| {
        let chars: Vec<char> = line.chars().collect();
        let (steps, output) = mm.run(&chars);
        MealyLineTest {
            line: line.to_string(),
            output,
            isComplete: steps.len() == chars.len(),
        }
    }) {
        Ok(results) => Ok(MealyMultiRunResult { results }),
        Err(err) => Err(Response {
            status: 400,
            message: format!("Некорректный автомат: {}", err),
        }),
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
) -> Result<StateResult, Response> {
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
) -> Result<StateResult, Response> {
    crate::fa::api::fa_update_state(state, automaton_id, state_id, label, x, y, is_initial, None)
}

/// Удаляет состояние и все связанные с ним переходы. Делегирует в
/// `fa_remove_state`.
#[tauri::command]
pub fn mealy_remove_state(
    state: State<'_, AutomatonStore>,
    automaton_id: i32,
    state_id: i32,
) -> Response {
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
) -> Result<TransitionResult, Response> {
    match state.modify(automaton_id, |entry| {
        add_mealy_transition(entry, from, to, input, output)
    }) {
        Some(Ok(created)) => Ok(TransitionResult {
            transition: vec![created],
        }),
        Some(Err(message)) => Err(Response {
            status: 400,
            message,
        }),
        None => Err(Response {
            status: 404,
            message: format!("Автомат с id {} не найден", automaton_id),
        }),
    }
}

#[tauri::command]
pub fn mealy_update_transition(
    state: State<'_, AutomatonStore>,
    automaton_id: i32,
    transition_id: i32,
    input: Option<char>,
    output: Option<char>,
) -> Result<TransitionResult, Response> {
    match state.modify(automaton_id, |entry| {
        update_mealy_transition(entry, transition_id, input, output)
    }) {
        Some(Ok(updated)) => Ok(TransitionResult {
            transition: vec![updated],
        }),
        Some(Err(message)) => Err(Response {
            status: 400,
            message,
        }),
        None => Err(Response {
            status: 404,
            message: format!("Автомат с id {} не найден", automaton_id),
        }),
    }
}

#[tauri::command]
pub fn mealy_remove_transition(
    state: State<'_, AutomatonStore>,
    automaton_id: i32,
    transition_id: i32,
) -> Response {
    match state.modify(automaton_id, |entry| {
        remove_mealy_transition(entry, transition_id)
    }) {
        Some(Ok(())) => Response {
            status: 200,
            message: format!("Удалён переход {}", transition_id),
        },
        Some(Err(message)) => Response {
            status: 400,
            message,
        },
        None => Response {
            status: 404,
            message: format!("Автомат с id {} не найден", automaton_id),
        },
    }
}

#[tauri::command]
pub fn mealy_run_str(
    state: State<'_, AutomatonStore>,
    automaton_id: i32,
    input: String,
) -> Result<MealyRunResult, Response> {
    let entry = check_automaton_mealy_run(state.inner(), automaton_id)?;

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
) -> Result<MealyMultiRunResult, Response> {
    let entry = check_automaton_mealy_multi_run(state.inner(), automaton_id)?;

    multi_run_mealy(&entry, &inputs)
}

/// Генерирует упорядоченный набор входов для Multiple Run машины Мили.
/// Делегирует общему генератору ядра, который добавляет `$` и символы вне
/// входного алфавита как негативные сценарии.
#[tauri::command]
pub fn mealy_generate_inputs(
    state: State<'_, AutomatonStore>,
    automaton_id: i32,
) -> Result<GenerateInputsResult, Response> {
    crate::core::api::gen_multi_run_for_output(state.inner(), automaton_id)
}

/// Удаляет машину Мили из хранилища. Делегирует в `core::api::remove_automaton`:
/// хранилище общее для FA и MM.
#[tauri::command]
pub fn mealy_remove_automaton(state: State<'_, AutomatonStore>, automaton_id: i32) -> Response {
    crate::core::api::remove_automaton(state, automaton_id)
}
