use std::collections::HashSet;
use std::fs;
use std::path::Path;

use rand::Rng;
use tauri::{State, http::StatusCode};

use crate::core::types::{AutomatonData, AutomatonStore, LoadResult, LoadedFile};
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
pub fn load_via_file(state: State<'_, AutomatonStore>, path: String) -> LoadResult {
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
