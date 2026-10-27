//! Unit tests for the mealy machine implementation and its command helpers.

use std::collections::HashSet;

use tauri::http::StatusCode;

use crate::core::types::{AutomatonData, AutomatonStore, MealyRunStep, StateData, TransitionData};
use crate::mealy::api::{
    add_mealy_transition, create_new_mealy, data_to_mm, remove_mealy_transition, run_mealy,
    update_mealy_transition,
};
use crate::mealy::types::{MM, MealyTransition};

fn mealy_entry() -> AutomatonData {
    AutomatonData {
        id: 2,
        name: "R".to_string(),
        automatonType: crate::core::types::AutomatonType::MEALY,
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
    }
}

/// Two-state machine: `0 --a/1--> 1 --b/0--> 0`.
fn run_entry() -> AutomatonData {
    let mut entry = mealy_entry();
    add_mealy_transition(&mut entry, 0, 1, 'a', '1').expect("переход a");
    add_mealy_transition(&mut entry, 1, 0, 'b', '0').expect("переход b");
    entry
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
    let automaton = result
        .automaton
        .expect("созданный автомат должен присутствовать");
    assert_eq!(automaton.name, "Мили");
    assert_eq!(automaton.states.len(), 1);
    assert_eq!(automaton.states[0].label, "q0");
    assert!(automaton.states[0].isInitial);
    assert!(!automaton.states[0].isFinal);
    assert!(automaton.transitions.is_empty());
    assert!(automaton.get_alphabet().is_empty());
}

#[test]
fn create_new_uses_given_name() {
    let store = AutomatonStore::new();
    let result = create_new_mealy(&store, Some("Машина 1".to_string()));

    let automaton = result
        .automaton
        .expect("созданный автомат должен присутствовать");
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

    let created =
        add_mealy_transition(&mut entry, 0, 1, 'a', '1').expect("переход должен создаться");

    assert_eq!(created.from, 0);
    assert_eq!(created.to, 1);
    assert_eq!(created.symbol, 'a');
    assert_eq!(created.output, Some('1'));
    assert_eq!(entry.transitions.len(), 1);
    assert_eq!(entry.get_alphabet(), vec!['a']);
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
fn add_transition_fails_on_unknown_states() {
    let mut entry = mealy_entry();

    assert!(add_mealy_transition(&mut entry, 99, 1, 'a', '0').is_err());
    assert!(add_mealy_transition(&mut entry, 0, 99, 'a', '0').is_err());
    assert!(entry.transitions.is_empty());
    assert!(entry.get_alphabet().is_empty());
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
    let mm = builder
        .build()
        .expect("переходы Мили должны собираться в MM");

    assert_eq!(mm.step(0, 'a'), Some((1, '1')));
    assert_eq!(mm.step(1, 'b'), Some((0, '0')));
}

#[test]
fn update_transition_changes_symbol_and_output() {
    let mut entry = mealy_entry();
    let created = add_mealy_transition(&mut entry, 0, 1, 'a', '0').expect("переход");

    let updated =
        update_mealy_transition(&mut entry, created.id, Some('b'), Some('1')).expect("обновление");

    assert_eq!(updated.id, created.id);
    assert_eq!(updated.from, 0);
    assert_eq!(updated.to, 1);
    assert_eq!(updated.symbol, 'b');
    assert_eq!(updated.output, Some('1'));
    assert_eq!(entry.transitions[0].symbol, 'b');
    assert_eq!(entry.transitions[0].output, Some('1'));
}

#[test]
fn update_transition_keeps_fields_when_none() {
    let mut entry = mealy_entry();
    let created = add_mealy_transition(&mut entry, 0, 1, 'a', '0').expect("переход");

    let updated = update_mealy_transition(&mut entry, created.id, None, None).expect("обновление");

    assert_eq!(updated.symbol, 'a');
    assert_eq!(updated.output, Some('0'));
    assert_eq!(entry.get_alphabet(), vec!['a']);
}

#[test]
fn update_transition_reflects_new_symbol_in_derived_alphabet() {
    let mut entry = mealy_entry();
    let created = add_mealy_transition(&mut entry, 0, 1, 'a', '0').expect("переход");

    update_mealy_transition(&mut entry, created.id, Some('b'), None).expect("обновление");

    assert_eq!(entry.get_alphabet(), vec!['b']);
}

#[test]
fn update_transition_fails_on_unknown_transition() {
    let mut entry = mealy_entry();
    add_mealy_transition(&mut entry, 0, 1, 'a', '0').expect("переход");

    let err = update_mealy_transition(&mut entry, 999, Some('b'), None)
        .expect_err("неизвестный переход должен падать");

    assert!(err.contains("999"));
    assert_eq!(entry.transitions.len(), 1);
    assert_eq!(entry.transitions[0].symbol, 'a');
}

#[test]
fn update_transition_fails_on_conflicting_input() {
    let mut entry = mealy_entry();
    add_mealy_transition(&mut entry, 0, 1, 'a', '0').expect("первый переход");
    let second = add_mealy_transition(&mut entry, 0, 0, 'b', '1').expect("второй переход");

    let err = update_mealy_transition(&mut entry, second.id, Some('a'), None)
        .expect_err("конфликт по (from, input) должен падать");

    assert!(err.contains('a'));
    assert_eq!(entry.transitions[1].symbol, 'b');
    assert_eq!(entry.transitions[1].output, Some('1'));
}

#[test]
fn update_transition_allows_same_symbol_from_other_state() {
    let mut entry = mealy_entry();
    add_mealy_transition(&mut entry, 0, 1, 'a', '0').expect("первый переход");
    let second = add_mealy_transition(&mut entry, 1, 0, 'b', '1').expect("второй переход");

    update_mealy_transition(&mut entry, second.id, Some('a'), None)
        .expect("другой источник не конфликтует");

    assert_eq!(entry.transitions[1].symbol, 'a');
}

#[test]
fn update_transition_rejects_epsilon() {
    let mut entry = mealy_entry();
    let created = add_mealy_transition(&mut entry, 0, 1, 'a', '0').expect("переход");

    assert!(update_mealy_transition(&mut entry, created.id, Some('$'), None).is_err());
    assert!(update_mealy_transition(&mut entry, created.id, None, Some('$')).is_err());
    assert_eq!(entry.transitions[0].symbol, 'a');
    assert_eq!(entry.transitions[0].output, Some('0'));
}

#[test]
fn remove_transition_removes_by_id() {
    let mut entry = mealy_entry();
    let first = add_mealy_transition(&mut entry, 0, 1, 'a', '0').expect("первый переход");
    let second = add_mealy_transition(&mut entry, 1, 0, 'b', '1').expect("второй переход");

    remove_mealy_transition(&mut entry, first.id).expect("удаление");

    assert_eq!(entry.transitions.len(), 1);
    assert_eq!(entry.transitions[0].id, second.id);
    assert_eq!(entry.transitions[0].symbol, 'b');
}

#[test]
fn remove_transition_fails_on_unknown_id() {
    let mut entry = mealy_entry();
    let created = add_mealy_transition(&mut entry, 0, 1, 'a', '0').expect("переход");

    let err = remove_mealy_transition(&mut entry, created.id + 1)
        .expect_err("неизвестный переход должен падать");

    assert!(err.contains(&(created.id + 1).to_string()));
    assert_eq!(entry.transitions.len(), 1);
}

#[test]
fn remove_transition_updates_derived_alphabet() {
    let mut entry = mealy_entry();
    let created = add_mealy_transition(&mut entry, 0, 1, 'a', '0').expect("переход");

    remove_mealy_transition(&mut entry, created.id).expect("удаление");

    assert!(entry.transitions.is_empty());
    assert!(entry.get_alphabet().is_empty());
}

#[test]
fn mm_run_produces_output_and_steps() {
    let mm = MM::builder()
        .set_initial(0)
        .transition(0, 'a', 1, '1')
        .transition(1, 'b', 0, '0')
        .build()
        .unwrap();

    let (steps, output) = mm.run(&['a', 'b']);

    assert_eq!(output, "10");
    assert_eq!(
        steps,
        vec![
            MealyRunStep {
                from: 0,
                symbol: 'a',
                to: 1,
                output: '1',
            },
            MealyRunStep {
                from: 1,
                symbol: 'b',
                to: 0,
                output: '0',
            },
        ]
    );
}

#[test]
fn mm_run_stops_when_transition_missing() {
    let mm = MM::builder()
        .set_initial(0)
        .transition(0, 'a', 1, '1')
        .build()
        .unwrap();

    let (steps, output) = mm.run(&['a', 'c']);

    assert_eq!(output, "1");
    assert_eq!(steps.len(), 1);
    assert_eq!(steps[0].symbol, 'a');
}

#[test]
fn mm_run_empty_input_emits_nothing() {
    let mm = MM::builder().set_initial(0).build().unwrap();

    let (steps, output) = mm.run(&[]);

    assert!(steps.is_empty());
    assert!(output.is_empty());
}

#[test]
fn data_to_mm_builds_from_entry() {
    let entry = run_entry();

    let mm = data_to_mm(&entry.states, &entry.transitions).expect("автомат должен собраться");

    assert_eq!(mm.step(0, 'a'), Some((1, '1')));
    assert_eq!(mm.step(1, 'b'), Some((0, '0')));
}

#[test]
fn data_to_mm_requires_initial_state() {
    let mut entry = mealy_entry();
    for state in &mut entry.states {
        state.isInitial = false;
    }

    assert!(data_to_mm(&entry.states, &entry.transitions).is_err());
}

#[test]
fn data_to_mm_ignores_orphan_transitions() {
    let mut entry = mealy_entry();
    entry.transitions.push(TransitionData {
        id: 1,
        from: 0,
        to: 99,
        symbol: 'a',
        output: Some('1'),
    });

    let mm = data_to_mm(&entry.states, &entry.transitions).expect("осиротевший переход не мешает");

    assert!(mm.transitions().is_empty());
}

#[test]
fn data_to_mm_rejects_transition_without_output() {
    let mut entry = mealy_entry();
    entry.transitions.push(TransitionData {
        id: 1,
        from: 0,
        to: 1,
        symbol: 'a',
        output: None,
    });

    assert!(data_to_mm(&entry.states, &entry.transitions).is_err());
}

#[test]
fn run_mealy_full_run_returns_200_and_output() {
    let entry = run_entry();

    let result = run_mealy(&entry, "ab");

    assert_eq!(result.status, StatusCode::OK);
    assert_eq!(result.output, "10");
    assert_eq!(result.steps.len(), 2);
    assert!(result.message.contains("10"));
}

#[test]
fn run_mealy_partial_run_returns_401() {
    let entry = run_entry();

    let result = run_mealy(&entry, "ac");

    assert_eq!(result.status, StatusCode::UNAUTHORIZED);
    assert_eq!(result.output, "1");
    assert_eq!(result.steps.len(), 1);
}

#[test]
fn run_mealy_stuck_on_first_symbol_returns_402() {
    let entry = run_entry();

    let result = run_mealy(&entry, "z");

    assert_eq!(result.status, StatusCode::PAYMENT_REQUIRED);
    assert!(result.output.is_empty());
    assert!(result.steps.is_empty());
}

#[test]
fn run_mealy_empty_input_returns_200() {
    let entry = run_entry();

    let result = run_mealy(&entry, "");

    assert_eq!(result.status, StatusCode::OK);
    assert!(result.output.is_empty());
    assert!(result.steps.is_empty());
}

#[test]
fn run_mealy_invalid_automaton_returns_400() {
    let mut entry = mealy_entry();
    for state in &mut entry.states {
        state.isInitial = false;
    }

    let result = run_mealy(&entry, "a");

    assert_eq!(result.status, StatusCode::BAD_REQUEST);
    assert!(result.output.is_empty());
    assert!(result.steps.is_empty());
}
