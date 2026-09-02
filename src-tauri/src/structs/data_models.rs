use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AutomatonKind {
    NFA,
    DFA,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct StateData {
    pub id: i32,
    pub label: String,
    pub x: f32,
    pub y: f32,
    pub isInitial: bool,
    pub isFinal: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionData {
    pub id: i32,
    pub from: i32,
    pub to: i32,
    pub symbol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomatonData {
    pub id: i32,
    pub name: String,
    pub kind: AutomatonKind,
    pub states: Vec<StateData>,
    pub transitions: Vec<TransitionData>,
    pub alphabet: Vec<char>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationResult {
    pub status: u16,
    pub message: String,
    pub automaton: Option<AutomatonData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusResult {
    pub status: u16,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateResult {
    pub status: u16,
    pub message: String,
    pub state: Option<StateData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionResult {
    pub status: u16,
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
    pub status: u16,
    pub message: String,
    pub traces: Vec<Trace>,
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
    pub status: u16,
    pub message: String,
    pub traces: Vec<LineTest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateInputsResult {
    pub status: u16,
    pub message: String,
    pub inputs: Vec<String>,
}