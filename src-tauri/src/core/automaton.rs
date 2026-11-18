//! Трейты автоматов и реестр автоматов в памяти.

use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::Mutex;
use std::sync::atomic::{AtomicI32, Ordering};

use crate::core::types::{AutomatonData, AutomatonType, StateData};

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

pub struct AutomatonStore {
    automata: Mutex<HashMap<i32, AutomatonData>>,
    next_id: AtomicI32,
}

impl AutomatonStore {
    pub fn new() -> Self {
        AutomatonStore {
            automata: Mutex::new(HashMap::new()),
            next_id: AtomicI32::new(1),
        }
    }

    pub fn create(&self, name: String, initial_label: &str) -> AutomatonData {
        let entry = AutomatonData {
            id: 0,
            name,
            automatonType: AutomatonType::FA,
            states: vec![StateData {
                id: 0,
                label: initial_label.to_string(),
                x: 100.0,
                y: 200.0,
                isInitial: true,
                isFinal: Some(false),
            }],
            transitions: Vec::new(),
        };

        self.insert(entry)
    }

    pub fn insert(&self, mut data: AutomatonData) -> AutomatonData {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);

        data.id = id;
        self.automata.lock().unwrap().insert(id, data.clone());
        data
    }

    pub fn get(&self, id: i32) -> Option<AutomatonData> {
        self.automata.lock().unwrap().get(&id).cloned()
    }

    /// Атомарно изменяет автомат под блокировкой карты, не клонируя запись
    /// целиком. `f` получает `&mut AutomatonData` и выполняется при удержанном
    /// замке.
    ///
    /// Возвращает `None`, если автомата с таким `id` нет, иначе — результат `f`.
    /// Изменения фиксируются сразу по возвращении `f`, поэтому для семантики
    /// «всё или ничего» `f` должен проверить все условия до первой мутации.
    pub fn modify<R>(&self, id: i32, f: impl FnOnce(&mut AutomatonData) -> R) -> Option<R> {
        self.automata.lock().unwrap().get_mut(&id).map(f)
    }

    pub fn remove(&self, id: i32) -> Option<AutomatonData> {
        self.automata.lock().unwrap().remove(&id)
    }

    #[allow(dead_code)]
    pub fn list_ids(&self) -> Vec<i32> {
        self.automata.lock().unwrap().keys().copied().collect()
    }
}
