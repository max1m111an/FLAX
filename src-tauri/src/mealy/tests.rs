//! Unit tests for the mealy machine implementation and its command helpers.

use std::collections::HashSet;

use tauri::http::StatusCode;

use crate::core::types::{AutomatonData, AutomatonStore, StateData};
use crate::mealy::api::{add_mealy_transition, create_new_mealy};
use crate::mealy::types::{MM, MealyTransition};

fn mealy_entry() -> AutomatonData {
    AutomatonData {
        id: 1,
        name: "Мили".to_string(),
        states: vec![
            StateData {
                id: 0,
                label: "q0".to_string(),
                x: 0.0,
                y: 0.0,
                isInitial: true,
                isFinal: false,
            },
            StateData {
                id: 1,
                label: "q1".to_string(),
                x: 200.0,
                y: 0.0,
                isInitial: false,
                isFinal: false,
            },
        ],
        transitions: vec![],
        alphabet: vec![],
    }
}

#[test]
fn builder_creates_valid_mm() {
    let mm = MM::builder()
        .state(0)
        .state(1)
        .set_initial(0)
        .transition(0, 'a', 1, '0')
        .build();
    assert!(mm.is_ok());
    let mm = mm.unwrap();
    assert_eq!(mm.states().len(), 2);
    assert_eq!(mm.initial_state(), 0);
}

#[test]
fn builder_fails_without_initial() {
    let result = MM::builder().state(0).transition(0, 'a', 1, '0').build();
    assert!(result.is_err());
}

#[test]
fn builder_registers_input_output_and_states() {
    let mm = MM::builder()
        .set_initial(0)
        .transition(0, 'a', 1, '1')
        .build()
        .unwrap();

    assert!(mm.input_alphabet().contains(&'a'));
    assert!(mm.output_alphabet().contains(&'1'));
    assert!(mm.states().contains(&0));
    assert!(mm.states().contains(&1));
}

#[test]
fn builder_accepts_explicit_alphabets() {
    let mm = MM::builder()
        .inputs(&['a', 'b'])
        .outputs(&['0', '1'])
        .set_initial(0)
        .build()
        .unwrap();

    assert_eq!(mm.input_alphabet().len(), 2);
    assert_eq!(mm.output_alphabet().len(), 2);
    assert_eq!(mm.transitions().len(), 0);
}

#[test]
fn new_fails_when_initial_missing() {
    let result = MM::new(
        HashSet::from([0]),
        HashSet::from(['a']),
        HashSet::from(['0']),
        Vec::new(),
        99,
    );
    assert!(result.is_err());
}

#[test]
fn new_fails_on_unknown_transition_state() {
    let states = HashSet::from([0]);
    let inputs = HashSet::from(['a']);
    let outputs = HashSet::from(['0']);

    let bad_from = MM::new(
        states.clone(),
        inputs.clone(),
        outputs.clone(),
        vec![MealyTransition::new(99, 'a', 0, '0')],
        0,
    );
    assert!(bad_from.is_err());

    let bad_to = MM::new(
        states,
        inputs,
        outputs,
        vec![MealyTransition::new(0, 'a', 99, '0')],
        0,
    );
    assert!(bad_to.is_err());
}

#[test]
fn new_fails_on_input_outside_alphabet() {
    let result = MM::new(
        HashSet::from([0, 1]),
        HashSet::from(['a']),
        HashSet::from(['0']),
        vec![MealyTransition::new(0, 'b', 1, '0')],
        0,
    );
    assert!(result.is_err());
}

#[test]
fn new_fails_on_output_outside_alphabet() {
    let result = MM::new(
        HashSet::from([0, 1]),
        HashSet::from(['a']),
        HashSet::from(['0']),
        vec![MealyTransition::new(0, 'a', 1, '9')],
        0,
    );
    assert!(result.is_err());
}

#[test]
fn new_fails_on_conflicting_state_input_pair() {
    let result = MM::new(
        HashSet::from([0, 1]),
        HashSet::from(['a']),
        HashSet::from(['0', '1']),
        vec![
            MealyTransition::new(0, 'a', 1, '0'),
            MealyTransition::new(0, 'a', 0, '1'),
        ],
        0,
    );
    assert!(result.is_err());
}

#[test]
fn step_returns_target_and_output() {
    let mm = MM::builder()
        .set_initial(0)
        .transition(0, 'a', 1, '1')
        .transition(1, 'b', 0, '0')
        .build()
        .unwrap();

    assert_eq!(mm.step(0, 'a'), Some((1, '1')));
    assert_eq!(mm.step(1, 'b'), Some((0, '0')));
    assert_eq!(mm.step(0, 'b'), None);
    assert_eq!(mm.step(1, 'a'), None);
}

#[test]
fn create_new_returns_ok_with_single_initial_state() {
    let store = AutomatonStore::new();
    let result = create_new_mealy(&store, None);

    assert_eq!(result.status, StatusCode::OK);
    let automaton = result.automaton.expect("созданный автомат должен присутствовать");
    assert_eq!(automaton.name, "Мили");
    assert_eq!(automaton.states.len(), 1);
    assert_eq!(automaton.states[0].label, "q0");
    assert!(automaton.states[0].isInitial);
    assert!(!automaton.states[0].isFinal);
    assert!(automaton.transitions.is_empty());
    assert!(automaton.alphabet.is_empty());
}

#[test]
fn create_new_uses_given_name() {
    let store = AutomatonStore::new();
    let result = create_new_mealy(&store, Some("Машина 1".to_string()));

    let automaton = result.automaton.expect("созданный автомат должен присутствовать");
    assert_eq!(automaton.name, "Машина 1");
}

#[test]
fn create_new_assigns_fresh_ids() {
    let store = AutomatonStore::new();

    let first = create_new_mealy(&store, None)
        .automaton
        .expect("первый автомат должен присутствовать");
    let second = create_new_mealy(&store, None)
        .automaton
        .expect("второй автомат должен присутствовать");

    assert_ne!(first.id, second.id);
}


#[test]
fn add_transition_stores_output_and_extends_alphabet() {
    let mut entry = mealy_entry();

    let created = add_mealy_transition(&mut entry, 0, 1, 'a', '1').expect("переход должен создаться");

    assert_eq!(created.from, 0);
    assert_eq!(created.to, 1);
    assert_eq!(created.symbol, 'a');
    assert_eq!(created.output, Some('1'));
    assert_eq!(entry.transitions.len(), 1);
    assert_eq!(entry.alphabet, vec!['a']);
}

#[test]
fn add_transition_assigns_fresh_ids() {
    let mut entry = mealy_entry();

    let first = add_mealy_transition(&mut entry, 0, 1, 'a', '0').expect("первый переход");
    let second = add_mealy_transition(&mut entry, 1, 0, 'b', '1').expect("второй переход");

    assert_ne!(first.id, second.id);
    assert_eq!(entry.transitions.len(), 2);
}

#[test]
fn add_transition_keeps_existing_alphabet_symbols() {
    let mut entry = mealy_entry();
    entry.alphabet.push('a');

    add_mealy_transition(&mut entry, 0, 1, 'a', '1').expect("переход должен создаться");

    assert_eq!(entry.alphabet, vec!['a']);
}

#[test]
fn add_transition_fails_on_unknown_states() {
    let mut entry = mealy_entry();

    assert!(add_mealy_transition(&mut entry, 99, 1, 'a', '0').is_err());
    assert!(add_mealy_transition(&mut entry, 0, 99, 'a', '0').is_err());
    assert!(entry.transitions.is_empty());
    assert!(entry.alphabet.is_empty());
}

#[test]
fn add_transition_fails_on_conflicting_input() {
    let mut entry = mealy_entry();
    add_mealy_transition(&mut entry, 0, 1, 'a', '0').expect("первый переход");

    let err = add_mealy_transition(&mut entry, 0, 0, 'a', '1').expect_err("конфликт должен падать");

    assert!(err.contains('a'));
    assert_eq!(entry.transitions.len(), 1);
    assert_eq!(entry.transitions[0].output, Some('0'));
}

#[test]
fn add_transition_rejects_epsilon() {
    let mut entry = mealy_entry();

    assert!(add_mealy_transition(&mut entry, 0, 1, '$', '0').is_err());
    assert!(add_mealy_transition(&mut entry, 0, 1, 'a', '$').is_err());
    assert!(entry.transitions.is_empty());
}

#[test]
fn added_transitions_build_mm_with_outputs() {
    let mut entry = mealy_entry();
    add_mealy_transition(&mut entry, 0, 1, 'a', '1').expect("a");
    add_mealy_transition(&mut entry, 1, 0, 'b', '0').expect("b");

    let mut builder = MM::builder().set_initial(0);
    for &s in entry.states.iter().map(|s| &s.id) {
        builder = builder.state(s);
    }
    for t in &entry.transitions {
        builder = builder.transition(t.from, t.symbol, t.to, t.output.expect("выход задан"));
    }
    let mm = builder.build().expect("переходы Мили должны собираться в MM");

    assert_eq!(mm.step(0, 'a'), Some((1, '1')));
    assert_eq!(mm.step(1, 'b'), Some((0, '0')));
}
