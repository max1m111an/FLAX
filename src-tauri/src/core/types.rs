//! Общие доменные типы: serde DTO (контракт Rust <-> фронтенд) и
//! вспомогательные функции над ними.

use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use tauri::http::StatusCode;

/// Символ эпсилон-перехода. Никогда не входит во входной алфавит.
pub const EPSILON: &str = "$";

/// Проверяет, что метка перехода непустая, и возвращает её строкой.
///
/// Метка может содержать любое число символов: переход читает её целиком как
/// один фрагмент входной цепочки (как в JFLAP). `what` описывает поле в
/// сообщении об ошибке.
pub fn check_label(text: &str, what: &str) -> Result<String, String> {
    if text.is_empty() {
        return Err(format!("{} '{}' не может быть пустым", what, text));
    }
    Ok(text.to_string())
}

/// Проверяет, что метка `label` целиком совпадает с фрагментом входа `input`,
/// начинающимся с позиции `pos`, и возвращает позицию сразу после прочитанной
/// метки. Пустая метка не совпадает никогда; eps-метка `$` не сдвигает позицию.
pub fn match_label(input: &[String], pos: usize, label: &str) -> Option<usize> {
    if label.is_empty() {
        return None;
    }
    if label == EPSILON {
        return Some(pos);
    }
    let mut next = pos;
    for ch in label.chars() {
        if input.get(next) != Some(&ch.to_string()) {
            return None;
        }
        next += 1;
    }
    Some(next)
}

// ---------------------------------------------------------------------------
// Serde DTO (контракт сериализации с фронтендом)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct StateData {
    pub id: i32,
    pub label: String,
    pub x: f32,
    pub y: f32,
    pub isInitial: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub isFinal: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionData {
    pub id: i32,
    pub from: i32,
    pub to: i32,
    pub symbol: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub enum AutomatonType {
    FA,
    MEALY,
    //MOORE,
    //PDA,
    //TURING,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct AutomatonData {
    pub id: i32,
    pub name: String,
    #[serde(default)]
    pub automatonType: AutomatonType,
    pub states: Vec<StateData>,
    pub transitions: Vec<TransitionData>,
}

impl Default for AutomatonType {
    fn default() -> Self {
        AutomatonType::FA
    }
}

/// Собирает входной алфавит из переходов: уникальные не-eps метки `symbol` в
/// порядке первого появления. Метка — это целиком читаемая переходом строка,
/// а не один символ.
pub fn alphabet_from_transitions(transitions: &[TransitionData]) -> Vec<String> {
    let mut seen: HashSet<String> = HashSet::new();
    let mut alphabet: Vec<String> = Vec::new();
    for t in transitions {
        if t.symbol != EPSILON && seen.insert(t.symbol.clone()) {
            alphabet.push(t.symbol.clone());
        }
    }
    alphabet
}

impl AutomatonData {
    /// Входной алфавит, выведенный из переходов (единый источник правды —
    /// отдельное поле не хранится; алфавит всегда отражает текущие переходы).
    #[allow(dead_code)]
    pub fn get_alphabet(&self) -> Vec<String> {
        alphabet_from_transitions(&self.transitions)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct OperationResult {
    #[serde(with = "http_serde::status_code")]
    pub status: StatusCode,
    pub message: String,
    pub automaton: Option<AutomatonData>,
}

/// Результат разбора файла: признак автомата и общие данные.
#[derive(Debug, Clone)]
#[allow(non_snake_case)]
pub struct LoadedFile {
    pub isAutomaton: bool,
    pub data: AutomatonData,
}

/// Ответ команды загрузки файла. Содержит и признак типа, и данные объекта.
#[derive(Debug, Clone, Serialize)]
#[allow(non_snake_case)]
pub struct LoadResult {
    #[serde(with = "http_serde::status_code")]
    pub status: StatusCode,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isAutomaton: Option<bool>,
    pub data: Option<AutomatonData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusResult {
    #[serde(with = "http_serde::status_code")]
    pub status: StatusCode,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateResult {
    #[serde(with = "http_serde::status_code")]
    pub status: StatusCode,
    pub message: String,
    pub state: Option<StateData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionResult {
    #[serde(with = "http_serde::status_code")]
    pub status: StatusCode,
    pub message: String,
    pub transition: Vec<TransitionData>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunStep {
    pub from: i32,
    pub symbol: String,
    pub to: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Trace {
    pub steps: Vec<RunStep>,
    pub isFinal: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunResult {
    #[serde(with = "http_serde::status_code")]
    pub status: StatusCode,
    pub message: String,
    pub traces: Vec<Trace>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MealyRunStep {
    pub from: i32,
    pub symbol: String,
    pub to: i32,
    pub output: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MealyRunResult {
    #[serde(with = "http_serde::status_code")]
    pub status: StatusCode,
    pub message: String,
    pub output: String,
    pub steps: Vec<MealyRunStep>,
}

/// Результат одной строки пакетного прогона машины Мили (без шагов).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct MealyLineTest {
    pub line: String,
    pub output: String,
    pub isComplete: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MealyMultiRunResult {
    #[serde(with = "http_serde::status_code")]
    pub status: StatusCode,
    pub message: String,
    pub results: Vec<MealyLineTest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct LineTest {
    pub line: String,
    pub isFinal: bool,
    pub correctSymbols: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiRunResult {
    #[serde(with = "http_serde::status_code")]
    pub status: StatusCode,
    pub message: String,
    pub traces: Vec<LineTest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateInputsResult {
    #[serde(with = "http_serde::status_code")]
    pub status: StatusCode,
    pub message: String,
    pub inputs: Vec<String>,
}
