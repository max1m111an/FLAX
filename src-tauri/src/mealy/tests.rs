//! Unit tests for the mealy machine implementation and its command helpers.

use std::collections::HashSet;

use tauri::http::StatusCode;

use crate::core::types::AutomatonStore;
use crate::mealy::api::create_new_mealy;
use crate::mealy::types::{MM, MealyTransition};

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
