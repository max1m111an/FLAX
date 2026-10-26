//! Unit tests for the shared in-memory automaton registry.

use crate::core::api::load_via_file_with;
use crate::core::types::{
    AutomatonData, AutomatonStore, StateData, TransitionData, alphabet_from_transitions,
};

fn make_store() -> AutomatonStore {
    AutomatonStore::new()
}

#[test]
fn create_returns_automaton_with_id_1() {
    let store = make_store();
    let entry = store.create("Test FA".to_string(), "q0");
    assert_eq!(entry.id, 1);
    assert_eq!(entry.name, "Test FA");
}

#[test]
fn create_auto_increments_id() {
    let store = make_store();
    let e1 = store.create("FA 1".to_string(), "q0");
    let e2 = store.create("DFA 1".to_string(), "q0");
    let e3 = store.create("FA 2".to_string(), "q0");
    assert_eq!(e1.id, 1);
    assert_eq!(e2.id, 2);
    assert_eq!(e3.id, 3);
}

#[test]
fn create_initial_state_is_correct() {
    let store = make_store();
    let entry = store.create("Test".to_string(), "q0");
    assert_eq!(entry.states.len(), 1);
    assert_eq!(entry.states[0].id, 0);
    assert_eq!(entry.states[0].label, "q0");
    assert!(entry.states[0].isInitial);
    assert!(!entry.states[0].isFinal);
    assert!(entry.transitions.is_empty());
    assert!(entry.get_alphabet().is_empty());
}

#[test]
fn get_existing_automaton() {
    let store = make_store();
    let created = store.create("Test".to_string(), "q0");
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
    let mut entry = store.create("Test".to_string(), "q0");
    entry.name = "Updated".to_string();
    entry.states.push(StateData {
        id: 1,
        label: "q1".to_string(),
        x: 200.0,
        y: 300.0,
        isInitial: false,
        isFinal: true,
    });
    store.update(entry);

    let fetched = store.get(1).unwrap();
    assert_eq!(fetched.name, "Updated");
    assert_eq!(fetched.states.len(), 2);
    assert_eq!(fetched.states[1].id, 1);
    assert!(fetched.states[1].isFinal);
}

#[test]
fn remove_returns_automaton() {
    let store = make_store();
    let entry = store.create("Test".to_string(), "q0");
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
    store.create("FA 1".to_string(), "q0");
    store.create("DFA 1".to_string(), "q0");
    store.create("FA 2".to_string(), "q0");

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
            store.create(format!("Auto {}", i), "q0");
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    let ids = store.list_ids();
    assert_eq!(ids.len(), 10);
}

#[test]
fn insert_assigns_fresh_id_and_stores() {
    let store = make_store();
    let data = AutomatonData {
        id: 99,
        name: "Loaded".to_string(),
        states: Vec::new(),
        transitions: Vec::new(),
    };

    let stored = store.insert(data);
    assert_eq!(stored.id, 1);
    assert_eq!(stored.name, "Loaded");
    assert!(stored.states.is_empty());

    let fetched = store.get(1).unwrap();
    assert_eq!(fetched.name, "Loaded");

    let again = store.insert(AutomatonData {
        id: 100,
        name: "Second".to_string(),
        states: Vec::new(),
        transitions: Vec::new(),
    });
    assert_eq!(again.id, 2);
}

#[test]
fn create_and_modify_full_workflow() {
    let store = make_store();

    // Create
    let mut entry = store.create("Workflow Test".to_string(), "q0");
    assert_eq!(entry.states.len(), 1);

    // Add state
    entry.states.push(StateData {
        id: 1,
        label: "q1".to_string(),
        x: 300.0,
        y: 200.0,
        isInitial: false,
        isFinal: true,
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

#[test]
fn transition_output_is_optional_in_json() {
    let fa = TransitionData {
        id: 1,
        from: 0,
        to: 1,
        symbol: 'a',
        output: None,
    };
    let json = serde_json::to_string(&fa).unwrap();
    assert!(!json.contains("output"));
    let back: TransitionData = serde_json::from_str(&json).unwrap();
    assert_eq!(back.output, None);

    let mm = TransitionData {
        id: 2,
        from: 0,
        to: 1,
        symbol: 'a',
        output: Some('1'),
    };
    let json = serde_json::to_string(&mm).unwrap();
    assert!(json.contains("\"output\":\"1\""));
    let back: TransitionData = serde_json::from_str(&json).unwrap();
    assert_eq!(back.output, Some('1'));
}

fn trans(id: i32, from: i32, to: i32, symbol: char) -> TransitionData {
    TransitionData {
        id,
        from,
        to,
        symbol,
        output: None,
    }
}

#[test]
fn get_alphabet_is_derived_from_transitions() {
    let mut entry = make_store().create("Test".to_string(), "q0");
    entry.transitions = vec![
        trans(1, 0, 1, 'a'),
        trans(2, 1, 0, 'b'),
        trans(3, 1, 1, 'a'),
        trans(4, 0, 0, '$'),
    ];

    assert_eq!(entry.get_alphabet(), vec!['a', 'b']);
}

#[test]
fn alphabet_from_transitions_skips_epsilon_and_duplicates() {
    let transitions = vec![
        trans(1, 0, 1, '$'),
        trans(2, 0, 1, 'x'),
        trans(3, 1, 0, '$'),
        trans(4, 1, 0, 'x'),
    ];

    assert_eq!(alphabet_from_transitions(&transitions), vec!['x']);
    assert!(alphabet_from_transitions(&[]).is_empty());
}

#[test]
fn serializes_derived_alphabet() {
    let mut entry = make_store().create("Test".to_string(), "q0");
    entry.transitions = vec![trans(1, 0, 1, 'a')];

    let value = serde_json::to_value(&entry).unwrap();
    assert_eq!(value["alphabet"], serde_json::json!(["a"]));
}

fn temp_file(name: &str, contents: &str) -> std::path::PathBuf {
    let path = std::env::temp_dir().join(format!("flax_{}_{}", std::process::id(), name));
    std::fs::write(&path, contents).unwrap();
    path
}

#[test]
fn load_via_file_reads_parses_names_and_inserts() {
    let path = temp_file("load_ok.jff", "payload");
    let expected_name = path.file_stem().unwrap().to_string_lossy().to_string();
    let store = make_store();

    let loaded = load_via_file_with(&store, path.to_str().unwrap(), "Fallback", |content| {
        assert_eq!(content, "payload");
        Ok((
            true,
            AutomatonData {
                id: 999,
                name: "ignored".to_string(),
                states: Vec::new(),
                transitions: Vec::new(),
            },
        ))
    })
    .unwrap();

    assert!(loaded.isAutomaton);
    assert_eq!(loaded.data.id, 1);
    assert_eq!(loaded.data.name, expected_name);
    assert!(store.get(1).is_some());

    std::fs::remove_file(&path).unwrap();
}

#[test]
fn load_via_file_with_preserves_formal_language_flag() {
    let path = temp_file("load_formal.jff", "payload");
    let store = make_store();

    let loaded = load_via_file_with(&store, path.to_str().unwrap(), "Fallback", |_| {
        Ok((
            false,
            AutomatonData {
                id: 0,
                name: String::new(),
                states: Vec::new(),
                transitions: Vec::new(),
            },
        ))
    })
    .unwrap();

    assert!(!loaded.isAutomaton);
    assert!(loaded.data.states.is_empty());
    assert!(loaded.data.transitions.is_empty());
    assert!(store.get(loaded.data.id).is_some());

    std::fs::remove_file(&path).unwrap();
}

#[test]
fn load_via_file_reports_read_error() {
    let store = make_store();
    let path = std::env::temp_dir().join(format!("flax_{}_missing_file.jff", std::process::id()));

    let err = load_via_file_with(&store, path.to_str().unwrap(), "Fallback", |_| {
        panic!("parser must not run for an unreadable file")
    })
    .unwrap_err();

    assert!(err.contains("Не удалось прочитать файл"));
    assert!(store.list_ids().is_empty());
}

#[test]
fn load_via_file_propagates_parser_error() {
    let path = temp_file("load_bad.jff", "bad");
    let store = make_store();

    let err = load_via_file_with(&store, path.to_str().unwrap(), "Fallback", |_| {
        Err("Некорректный формат".to_string())
    })
    .unwrap_err();

    assert_eq!(err, "Некорректный формат");
    assert!(store.list_ids().is_empty());

    std::fs::remove_file(&path).unwrap();
}

#[test]
fn load_result_serializes_is_automaton_and_data() {
    let result = crate::core::types::LoadResult {
        status: tauri::http::StatusCode::OK,
        message: "loaded".to_string(),
        isAutomaton: Some(true),
        data: Some(AutomatonData {
            id: 1,
            name: "Test".to_string(),
            states: Vec::new(),
            transitions: Vec::new(),
        }),
    };

    let value = serde_json::to_value(&result).unwrap();
    assert_eq!(value["isAutomaton"], true);
    assert_eq!(value["data"]["id"], 1);
}
