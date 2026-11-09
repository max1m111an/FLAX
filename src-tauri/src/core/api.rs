use std::collections::HashSet;
use std::fs;
use std::path::Path;

use rand::Rng;
use tauri::{State, http::StatusCode};

use crate::core::types::{
    AutomatonData, AutomatonStore, AutomatonType, LoadResult, LoadedFile, StatusResult,
};
use crate::fa::jff::parse_jff;

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
        let parsed = parse_jff(content)?;
        Ok((
            parsed.isAutomaton,
            AutomatonData {
                id: 0,
                name: String::new(),
                automatonType: crate::core::types::AutomatonType::FA,
                states: parsed.states,
                transitions: parsed.transitions,
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
    let entry = match state.get(automaton_id) {
        Some(e) => e,
        None => {
            return StatusResult {
                status: StatusCode::BAD_REQUEST,
                message: format!("Автомат с id {} не найден", automaton_id),
            };
        }
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
