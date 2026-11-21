//! Общие доменные типы: serde DTO (контракт Rust <-> фронтенд) и
//! вспомогательные функции над ними.

use std::collections::HashSet;

use serde::{Deserialize, Serialize};

/// Символ эпсилон-перехода. Никогда не входит во входной алфавит.
pub const EPSILON: char = '$';

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
    pub symbol: char,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<char>,
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

/// Собирает входной алфавит из переходов: уникальные не-eps `symbol` в порядке
/// первого появления.
pub fn alphabet_from_transitions(transitions: &[TransitionData]) -> Vec<char> {
    let mut seen: HashSet<char> = HashSet::new();
    let mut alphabet: Vec<char> = Vec::new();
    for t in transitions {
        if t.symbol != EPSILON && seen.insert(t.symbol) {
            alphabet.push(t.symbol);
        }
    }
    alphabet
}

impl AutomatonData {
    /// Входной алфавит, выведенный из переходов (единый источник правды —
    /// отдельное поле не хранится; алфавит всегда отражает текущие переходы).
    #[allow(dead_code)]
    pub fn get_alphabet(&self) -> Vec<char> {
        alphabet_from_transitions(&self.transitions)
    }
}

/// Ответ команды: числовой статус и сообщение. Используется для команд без
/// полезной нагрузки и как `Err` команд, возвращающих данные.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub status: u32,
    pub message: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct OperationResult {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isAutomaton: Option<bool>,
    pub data: Option<AutomatonData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateResult {
    pub state: Option<StateData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionResult {
    pub transition: Vec<TransitionData>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunStep {
    pub from: i32,
    pub symbol: char,
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
    pub traces: Vec<Trace>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MealyRunStep {
    pub from: i32,
    pub symbol: char,
    pub to: i32,
    pub output: char,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MealyRunResult {
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
    pub traces: Vec<LineTest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateInputsResult {
    pub inputs: Vec<String>,
}
