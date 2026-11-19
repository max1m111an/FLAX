use std::collections::HashSet;
use std::fs;
use std::path::Path;

use rand::Rng;
use tauri::{State, http::StatusCode};

use crate::core::automaton::AutomatonStore;
use crate::core::jff::{JffParsed, parse_jff};
use crate::core::types::{
    AutomatonData,
    AutomatonType,
    GenerateInputsResult,
    LoadResult,
    LoadedFile,
    MealyMultiRunResult,
    MealyRunResult,
    MultiRunResult,
    RunResult,
    StateData,
    StatusResult,
    TransitionData,
    //TransitionResult, OperationResult, StateResult
};

pub const MAX_ID: i32 = 10_000_000;

pub fn generate_id(used: &HashSet<i32>) -> i32 {
    let mut rng = rand::thread_rng();
    loop {
        let id = rng.gen_range(1..=MAX_ID);
        if !used.contains(&id) {
            return id;
        }
    }
}

/// Загружает JFLAP-файл и регистрирует объект в хранилище.
#[tauri::command]
pub fn load_file(state: State<'_, AutomatonStore>, path: String) -> LoadResult {
    match load_via_file_with(&state, &path, "Автомат", |content| {
        let (is_automaton, automaton_type, states, transitions) = match parse_jff(content)? {
            JffParsed::Automaton(automaton_type, automaton) => (
                true,
                automaton_type,
                automaton.states,
                automaton.transitions,
            ),
            JffParsed::Grammar(_) => (false, AutomatonType::FA, Vec::new(), Vec::new()),
        };
        Ok((
            is_automaton,
            AutomatonData {
                id: 0,
                name: String::new(),
                automatonType: automaton_type,
                states,
                transitions,
            },
        ))
    }) {
        Ok(loaded) => LoadResult {
            status: StatusCode::OK,
            message: if loaded.isAutomaton {
                "Автомат загружен из файла".to_string()
            } else {
                "Формальный язык загружен из файла (заглушка)".to_string()
            },
            isAutomaton: Some(loaded.isAutomaton),
            data: Some(loaded.data),
        },
        Err(message) => LoadResult {
            status: StatusCode::BAD_REQUEST,
            message,
            isAutomaton: None,
            data: None,
        },
    }
}

/// Загружает формальный объект из файла и регистрирует его в хранилище.
///
/// Разбор содержимого делегируется `parse`: он должен вернуть признак
/// автомата, прочитанный из JFLAP-элемента `<type>`, и общие данные. Так
/// `core` не зависит от конкретного формата (FA, Mealy, грамматика, ...), а
/// новые формальные языки подключаются через собственный парсер.
pub(crate) fn load_via_file_with<F>(
    store: &AutomatonStore,
    path: &str,
    default_name: &str,
    parse: F,
) -> Result<LoadedFile, String>
where
    F: FnOnce(&str) -> Result<(bool, AutomatonData), String>,
{
    let content =
        fs::read_to_string(path).map_err(|err| format!("Не удалось прочитать файл: {}", err))?;

    let (is_automaton, mut data) = parse(&content)?;

    data.id = 0;
    data.name = Path::new(path)
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| default_name.to_string());

    let data = store.insert(data);

    Ok(LoadedFile {
        isAutomaton: is_automaton,
        data,
    })
}

/// Универсальный эндпоинт сохранения: берёт объект из хранилища и пишет его
/// в файл в формате, соответствующем типу автомата.
#[tauri::command]
pub fn save_file(
    state: State<'_, AutomatonStore>,
    automaton_id: i32,
    path: String,
) -> StatusResult {
    let entry = match check_automaton_status(state.inner(), automaton_id) {
        Ok(e) => e,
        Err(e) => return e,
    };

    save_automaton(&entry, &path)
}

/// Сериализует автомат в формате его типа и записывает результат в `path`.
/// Новый тип автомата подключается добавлением ветки в `match`.
pub(crate) fn save_automaton(entry: &AutomatonData, path: &str) -> StatusResult {
    let content = match entry.automatonType {
        AutomatonType::FA => crate::fa::jff::to_jff(entry),
        AutomatonType::MEALY => crate::mealy::jff::to_jff(entry),
    };

    save_file_with(path, &content)
}

/// Записывает готовое содержимое в файл, инкапсулируя работу с файловой
/// системой и формирование ответа об успехе/ошибке.
pub(crate) fn save_file_with(path: &str, content: &str) -> StatusResult {
    match fs::write(path, content) {
        Ok(()) => StatusResult {
            status: StatusCode::OK,
            message: format!("Автомат сохранён в {}", path),
        },
        Err(err) => StatusResult {
            status: StatusCode::BAD_REQUEST,
            message: format!("Не удалось сохранить файл: {}", err),
        },
    }
}

/// Общий драйвер пакетного прогона: строит автомат один раз (`build`) и
/// прогоняет по нему все строки (`run_line`), сохраняя порядок `inputs`.
///
/// `core` ничего не знает о конкретном типе автомата и формате ответа:
/// построение (`data_to_fa` / `data_to_mm`) и обработка одной строки
/// передаются замыканиями, а ошибка построения пробрасывается как `Err`.
pub(crate) fn multi_run_automaton<A, T, B, R>(
    data: &AutomatonData,
    inputs: &[String],
    build: B,
    run_line: R,
) -> Result<Vec<T>, String>
where
    B: FnOnce(&[StateData], &[TransitionData]) -> Result<A, String>,
    R: Fn(&A, &str) -> T,
{
    let automaton = build(&data.states, &data.transitions)?;
    Ok(inputs
        .iter()
        .map(|line| run_line(&automaton, line))
        .collect())
}

/// Инкапсулированная проверка существования автомата.
/// При отсутствии возвращает переданный `err_resp`, что позволяет
/// использовать функцию с разными типами ответов команд.
///
/// usage: `let entry = check_automaton(store, automaton_id, err_resp)?;`
#[warn(dead_code)]
pub(crate) fn check_automaton<T>(
    store: &AutomatonStore,
    automaton_id: i32,
    err_resp: T,
) -> Result<AutomatonData, T> {
    store.get(automaton_id).ok_or(err_resp)
}

/// Проверка существования автомата с шаблоном ошибки [`StatusResult`].
#[warn(dead_code)]
pub(crate) fn check_automaton_status(
    store: &AutomatonStore,
    automaton_id: i32,
) -> Result<AutomatonData, StatusResult> {
    check_automaton(
        store,
        automaton_id,
        StatusResult {
            status: StatusCode::NOT_FOUND,
            message: format!("Автомат с id {} не найден", automaton_id),
        },
    )
}

/// Проверка существования автомата с шаблоном ошибки [`StateResult`].
//pub(crate) fn check_automaton_state(
//    store: &AutomatonStore,
//    automaton_id: i32,
//) -> Result<AutomatonData, StateResult> {
//    check_automaton(
//        store,
//        automaton_id,
//        StateResult {
//            status: StatusCode::NOT_FOUND,
//            message: format!("Автомат с id {} не найден", automaton_id),
//            state: None,
//        },
//    )
//}

/// Проверка существования автомата с шаблоном ошибки [`TransitionResult`].
//#[warn(dead_code)]
//pub(crate) fn check_automaton_transition(
//    store: &AutomatonStore,
//    automaton_id: i32,
//) -> Result<AutomatonData, TransitionResult> {
//    check_automaton(
//        store,
//        automaton_id,
//        TransitionResult {
//            status: StatusCode::NOT_FOUND,
//            message: format!("Автомат с id {} не найден", automaton_id),
//            transition: Vec::new(),
//        },
//    )
//}

/// Проверка существования автомата с шаблоном ошибки [`RunResult`].
#[warn(dead_code)]
pub(crate) fn check_automaton_run(
    store: &AutomatonStore,
    automaton_id: i32,
) -> Result<AutomatonData, RunResult> {
    check_automaton(
        store,
        automaton_id,
        RunResult {
            status: StatusCode::NOT_FOUND,
            message: format!("Автомат с id {} не найден", automaton_id),
            traces: Vec::new(),
        },
    )
}

/// Проверка существования автомата с шаблоном ошибки [`MultiRunResult`].
#[warn(dead_code)]
pub(crate) fn check_automaton_multi_run(
    store: &AutomatonStore,
    automaton_id: i32,
) -> Result<AutomatonData, MultiRunResult> {
    check_automaton(
        store,
        automaton_id,
        MultiRunResult {
            status: StatusCode::NOT_FOUND,
            message: format!("Автомат с id {} не найден", automaton_id),
            traces: Vec::new(),
        },
    )
}

/// Проверка существования автомата с шаблоном ошибки [`MealyRunResult`].
#[warn(dead_code)]
pub(crate) fn check_automaton_mealy_run(
    store: &AutomatonStore,
    automaton_id: i32,
) -> Result<AutomatonData, MealyRunResult> {
    check_automaton(
        store,
        automaton_id,
        MealyRunResult {
            status: StatusCode::NOT_FOUND,
            message: format!("Автомат с id {} не найден", automaton_id),
            output: String::new(),
            steps: Vec::new(),
        },
    )
}

/// Проверка существования автомата с шаблоном ошибки [`MealyMultiRunResult`].
#[warn(dead_code)]
pub(crate) fn check_automaton_mealy_multi_run(
    store: &AutomatonStore,
    automaton_id: i32,
) -> Result<AutomatonData, MealyMultiRunResult> {
    check_automaton(
        store,
        automaton_id,
        MealyMultiRunResult {
            status: StatusCode::NOT_FOUND,
            message: format!("Автомат с id {} не найден", automaton_id),
            results: Vec::new(),
        },
    )
}

/// Проверка существования автомата с шаблоном ошибки [`OperationResult`].
//#[warn(dead_code)]
//pub(crate) fn check_automaton_operation(
//    store: &AutomatonStore,
//    automaton_id: i32,
//) -> Result<AutomatonData, OperationResult> {
//    check_automaton(
//        store,
//        automaton_id,
//        OperationResult {
//            status: StatusCode::NOT_FOUND,
//            message: format!("Автомат с id {} не найден", automaton_id),
//            automaton: None,
//        },
//    )
//}

/// Проверка существования автомата с шаблоном ошибки [`GenerateInputsResult`].
#[warn(dead_code)]
pub(crate) fn check_automaton_generate_inputs(
    store: &AutomatonStore,
    automaton_id: i32,
) -> Result<AutomatonData, GenerateInputsResult> {
    check_automaton(
        store,
        automaton_id,
        GenerateInputsResult {
            status: StatusCode::NOT_FOUND,
            message: format!("Автомат с id {} не найден", automaton_id),
            inputs: Vec::new(),
        },
    )
}
