use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateData {
    pub id: String,
    pub label: String,
    pub is_initial: bool,
    pub is_final: bool,
    pub position: Option<(f32, f32)>, // координаты для визуализации
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionData {
    pub from: String,
    pub to: String,
    pub symbol: Option<char>, // None для eps-перехода
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NFAData {
    pub states: Vec<StateData>,
    pub transitions: Vec<TransitionData>,
    pub alphabet: Vec<char>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationResult {
    pub success: bool,
    pub message: String,
    pub automaton: Option<NFAData>,
}