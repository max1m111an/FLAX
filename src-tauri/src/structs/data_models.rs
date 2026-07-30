use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AutomatonKind {
    NFA,
    DFA,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateData {
    pub id: i32,
    pub label: String,
    pub x: f32,
    pub y: f32,
    pub is_initial: bool,
    pub is_final: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionData {
    pub from: i32,
    pub to: i32,
    pub symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
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
