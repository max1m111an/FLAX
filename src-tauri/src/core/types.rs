//! Shared domain types: serde DTOs (the Rust <-> frontend contract), automaton
//! traits and the in-memory automaton registry.

use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::Mutex;

use tauri::http::StatusCode;
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Serde DTOs (serialization contract with the frontend)
// ---------------------------------------------------------------------------

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
    pub symbol: char,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomatonData {
    pub id: i32,
    pub name: String,
    pub states: Vec<StateData>,
    pub transitions: Vec<TransitionData>,
    pub alphabet: Vec<char>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationResult {
    #[serde(with = "http_serde::status_code")]
    pub status: StatusCode,
    pub message: String,
    pub automaton: Option<AutomatonData>,
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
    #[serde(with = "http_serde::status_code")]
    pub status: StatusCode,
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

// ---------------------------------------------------------------------------
// Automaton traits
// ---------------------------------------------------------------------------

#[allow(dead_code)]
pub trait Automaton {
    type State: Clone + Eq + Hash + Debug;
    type Symbol: Clone + Eq + Hash + Debug;

    fn accepts(&self, input: &[Self::Symbol]) -> bool;
    fn states(&self) -> &HashSet<Self::State>;
    fn initial_state(&self) -> &Self::State;
    fn final_states(&self) -> &HashSet<Self::State>;
    fn alphabet(&self) -> &HashSet<Self::Symbol>;

    fn is_accepting(&self, state: &Self::State) -> bool {
        self.final_states().contains(state)
    }
}

#[allow(dead_code)]
pub trait NondeterministicAutomaton: Automaton {
    fn next_states(&self, state: &Self::State, symbol: &Self::Symbol) -> HashSet<&Self::State>;
    fn epsilon_closure(&self, state: &Self::State) -> HashSet<&Self::State>;
}

// ---------------------------------------------------------------------------
// In-memory automaton registry
// ---------------------------------------------------------------------------

pub struct AutomatonStore {
    automata: Mutex<HashMap<i32, AutomatonData>>,
    next_id: Mutex<i32>,
}

impl AutomatonStore {
    pub fn new() -> Self {
        AutomatonStore {
            automata: Mutex::new(HashMap::new()),
            next_id: Mutex::new(1),
        }
    }

    pub fn create(&self, name: String, initial_label: &str) -> AutomatonData {
        let entry = AutomatonData {
            id: 0,
            name,
            states: vec![StateData {
                id: 0,
                label: initial_label.to_string(),
                x: 100.0,
                y: 200.0,
                isInitial: true,
                isFinal: false,
            }],
            transitions: Vec::new(),
            alphabet: Vec::new(),
        };

        self.insert(entry)
    }

    pub fn insert(&self, mut data: AutomatonData) -> AutomatonData {
        let id = {
            let mut next = self.next_id.lock().unwrap();
            let id = *next;
            *next += 1;
            id
        };

        data.id = id;
        self.automata.lock().unwrap().insert(id, data.clone());
        data
    }

    pub fn get(&self, id: i32) -> Option<AutomatonData> {
        self.automata.lock().unwrap().get(&id).cloned()
    }

    pub fn update(&self, data: AutomatonData) {
        self.automata.lock().unwrap().insert(data.id, data);
    }

    pub fn remove(&self, id: i32) -> Option<AutomatonData> {
        self.automata.lock().unwrap().remove(&id)
    }

    #[allow(dead_code)]
    pub fn list_ids(&self) -> Vec<i32> {
        self.automata.lock().unwrap().keys().copied().collect()
    }
}