use std::collections::HashMap;
use std::sync::Mutex;

use crate::structs::data_models::{AutomatonData, AutomatonKind, StateData};

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

    pub fn create(
        &self,
        name: String,
        kind: AutomatonKind,
        initial_label: &str,
    ) -> AutomatonData {
        let id = {
            let mut next = self.next_id.lock().unwrap();
            let id = *next;
            *next += 1;
            id
        };

        let entry = AutomatonData {
            id,
            name,
            kind,
            states: vec![StateData {
                id: 0,
                label: initial_label.to_string(),
                x: 100.0,
                y: 200.0,
                is_initial: true,
                is_final: false,
            }],
            transitions: Vec::new(),
            alphabet: Vec::new(),
            next_transition_id: 1,
        };

        self.automata.lock().unwrap().insert(id, entry.clone());
        entry
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

    pub fn list_ids(&self) -> Vec<i32> {
        self.automata.lock().unwrap().keys().copied().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_store() -> AutomatonStore {
        AutomatonStore::new()
    }

    #[test]
    fn create_returns_automaton_with_id_1() {
        let store = make_store();
        let entry = store.create("Test NFA".to_string(), AutomatonKind::NFA, "q0");
        assert_eq!(entry.id, 1);
        assert_eq!(entry.name, "Test NFA");
        assert_eq!(entry.kind, AutomatonKind::NFA);
    }

    #[test]
    fn create_auto_increments_id() {
        let store = make_store();
        let e1 = store.create("NFA 1".to_string(), AutomatonKind::NFA, "q0");
        let e2 = store.create("DFA 1".to_string(), AutomatonKind::DFA, "q0");
        let e3 = store.create("NFA 2".to_string(), AutomatonKind::NFA, "q0");
        assert_eq!(e1.id, 1);
        assert_eq!(e2.id, 2);
        assert_eq!(e3.id, 3);
    }

    #[test]
    fn create_initial_state_is_correct() {
        let store = make_store();
        let entry = store.create("Test".to_string(), AutomatonKind::NFA, "q0");
        assert_eq!(entry.states.len(), 1);
        assert_eq!(entry.states[0].id, 0);
        assert_eq!(entry.states[0].label, "q0");
        assert!(entry.states[0].is_initial);
        assert!(!entry.states[0].is_final);
        assert!(entry.transitions.is_empty());
        assert!(entry.alphabet.is_empty());
    }

    #[test]
    fn get_existing_automaton() {
        let store = make_store();
        let created = store.create("Test".to_string(), AutomatonKind::NFA, "q0");
        let fetched = store.get(created.id);
        assert!(fetched.is_some());
        let fetched = fetched.unwrap();
        assert_eq!(fetched.id, created.id);
        assert_eq!(fetched.name, "Test");
    }

    #[test]
    fn get_nonexistent_returns_none() {
        let store = make_store();
        assert!(store.get(999).is_none());
    }

    #[test]
    fn update_modifies_automaton() {
        let store = make_store();
        let mut entry = store.create("Test".to_string(), AutomatonKind::NFA, "q0");
        entry.name = "Updated".to_string();
        entry.states.push(StateData {
            id: 1,
            label: "q1".to_string(),
            x: 200.0,
            y: 300.0,
            is_initial: false,
            is_final: true,
        });
        store.update(entry);

        let fetched = store.get(1).unwrap();
        assert_eq!(fetched.name, "Updated");
        assert_eq!(fetched.states.len(), 2);
        assert_eq!(fetched.states[1].id, 1);
        assert!(fetched.states[1].is_final);
    }

    #[test]
    fn remove_returns_automaton() {
        let store = make_store();
        let entry = store.create("Test".to_string(), AutomatonKind::NFA, "q0");
        let removed = store.remove(entry.id);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().id, 1);
        assert!(store.get(1).is_none());
    }

    #[test]
    fn remove_nonexistent_returns_none() {
        let store = make_store();
        assert!(store.remove(999).is_none());
    }

    #[test]
    fn list_ids_empty() {
        let store = make_store();
        assert!(store.list_ids().is_empty());
    }

    #[test]
    fn list_ids_multiple() {
        let store = make_store();
        store.create("NFA 1".to_string(), AutomatonKind::NFA, "q0");
        store.create("DFA 1".to_string(), AutomatonKind::DFA, "q0");
        store.create("NFA 2".to_string(), AutomatonKind::NFA, "q0");

        let mut ids = store.list_ids();
        ids.sort();
        assert_eq!(ids, vec![1, 2, 3]);
    }

    #[test]
    fn store_is_thread_safe() {
        use std::sync::Arc;
        use std::thread;

        let store = Arc::new(make_store());
        let mut handles = vec![];

        for i in 0..10 {
            let store = Arc::clone(&store);
            handles.push(thread::spawn(move || {
                store.create(format!("Auto {}", i), AutomatonKind::NFA, "q0");
            }));
        }

        for h in handles {
            h.join().unwrap();
        }

        let ids = store.list_ids();
        assert_eq!(ids.len(), 10);
    }

    #[test]
    fn create_and_modify_full_workflow() {
        let store = make_store();

        // Create
        let mut entry = store.create("Workflow Test".to_string(), AutomatonKind::NFA, "q0");
        assert_eq!(entry.states.len(), 1);

        // Add state
        entry.states.push(StateData {
            id: 1,
            label: "q1".to_string(),
            x: 300.0,
            y: 200.0,
            is_initial: false,
            is_final: true,
        });
        store.update(entry.clone());
        let fetched = store.get(1).unwrap();
        assert_eq!(fetched.states.len(), 2);

        // Add transition
        store.update(entry);

        // Remove
        let removed = store.remove(1).unwrap();
        assert_eq!(removed.name, "Workflow Test");
        assert!(store.get(1).is_none());
    }
}
