use std::collections::{HashMap, HashSet};

use crate::structs::automata::{Automaton, DeterministicAutomaton};
use crate::structs::data_models::RunStep;
use crate::structs::dfa::{DFA, DFABuilder};

fn make_even_a_dfa() -> DFA {
    // DFA: принимает строки с чётным количеством 'a'
    // q0 (initial, final) --a--> q1, q0 --b--> q0
    // q1 --a--> q0, q1 --b--> q1
    DFABuilder::new()
        .state(0).state(1)
        .set_initial(0)
        .set_final(0)
        .symbols(&['a', 'b'])
        .transition(0, 'a', 1)
        .transition(0, 'b', 0)
        .transition(1, 'a', 0)
        .transition(1, 'b', 1)
        .build()
        .unwrap()
}

#[test]
fn builder_creates_valid_dfa() {
    let dfa = make_even_a_dfa();
    assert_eq!(dfa.states().len(), 2);
    assert_eq!(dfa.initial_state(), &0);
    assert!(dfa.final_states().contains(&0));
}

#[test]
fn builder_fails_without_initial() {
    let result = DFABuilder::new()
        .state(0).state(1)
        .set_final(1)
        .symbol('a')
        .transition(0, 'a', 1)
        .build();
    assert!(result.is_err());
}

#[test]
fn new_fails_with_invalid_accepting() {
    let states = HashSet::from([0]);
    let accepting = HashSet::from([99]);
    let result = DFA::new(states, HashSet::new(), HashMap::new(), 0, accepting);
    assert!(result.is_err());
}

#[test]
fn new_fails_with_invalid_transition_target() {
    let states = HashSet::from([0]);
    let mut transitions: HashMap<(i32, char), i32> = HashMap::new();
    transitions.insert((0, 'a'), 99);
    let result = DFA::new(states, HashSet::from(['a']), transitions, 0, HashSet::new());
    assert!(result.is_err());
}

#[test]
fn accepts_even_a() {
    let dfa = make_even_a_dfa();

    assert!(dfa.accepts(&[]));               // 0 a's - even
    assert!(dfa.accepts(&['b']));            // 0 a's - even
    assert!(dfa.accepts(&['b', 'b']));       // 0 a's - even
    assert!(!dfa.accepts(&['a']));           // 1 a - odd
    assert!(dfa.accepts(&['a', 'a']));       // 2 a's - even
    assert!(!dfa.accepts(&['a', 'a', 'a'])); // 3 a's - odd
    assert!(dfa.accepts(&['a', 'b', 'a']));  // 2 a's - even
    assert!(!dfa.accepts(&['a', 'b', 'b'])); // 1 a - odd
}

#[test]
fn rejects_symbol_not_in_alphabet() {
    let dfa = make_even_a_dfa();
    assert!(!dfa.accepts(&['c']));
    assert!(!dfa.accepts(&['a', 'c']));
}

#[test]
fn next_state_deterministic() {
    let dfa = make_even_a_dfa();

    assert_eq!(dfa.next_state(&0, &'a'), Some(&1));
    assert_eq!(dfa.next_state(&0, &'b'), Some(&0));
    assert_eq!(dfa.next_state(&1, &'a'), Some(&0));
    assert_eq!(dfa.next_state(&1, &'b'), Some(&1));
}

#[test]
fn is_valid_full_dfa() {
    let dfa = make_even_a_dfa();
    assert!(dfa.is_valid().is_ok());
}

#[test]
fn is_valid_incomplete_dfa() {
    // q0 has transition on 'a' but not 'b'
    let dfa = DFABuilder::new()
        .state(0).state(1)
        .set_initial(0)
        .set_final(1)
        .symbols(&['a', 'b'])
        .transition(0, 'a', 1)
        .build()
        .unwrap();

    assert!(dfa.is_valid().is_err());
}

#[test]
fn is_empty_when_no_accepting() {
    let dfa = DFABuilder::new()
        .state(0).state(1)
        .set_initial(0)
        .symbols(&['a'])
        .transition(0, 'a', 1)
        .transition(1, 'a', 0)
        .build()
        .unwrap();

    assert!(dfa.is_empty());
}

#[test]
fn is_not_empty_normal() {
    let dfa = make_even_a_dfa();
    assert!(!dfa.is_empty());
}

#[test]
fn empty_string_accepted_when_initial_final() {
    let dfa = DFABuilder::new()
        .state(0)
        .set_initial(0)
        .set_final(0)
        .symbol('a')
        .transition(0, 'a', 0)
        .build()
        .unwrap();

    assert!(dfa.accepts(&[]));
}

#[test]
fn empty_string_rejected_when_initial_not_final() {
    let dfa = DFABuilder::new()
        .state(0).state(1)
        .set_initial(0)
        .set_final(1)
        .symbol('a')
        .transition(0, 'a', 1)
        .transition(1, 'a', 0)
        .build()
        .unwrap();

    assert!(!dfa.accepts(&[]));
}

#[test]
fn single_state_loop() {
    // q0 --a--> q0, accepting: accepts a*
    let dfa = DFABuilder::new()
        .state(0)
        .set_initial(0)
        .set_final(0)
        .symbol('a')
        .transition(0, 'a', 0)
        .build()
        .unwrap();

    assert!(dfa.accepts(&[]));
    assert!(dfa.accepts(&['a']));
    assert!(dfa.accepts(&['a', 'a']));
    assert!(dfa.accepts(&['a', 'a', 'a']));
    assert!(!dfa.accepts(&['b']));
}

#[test]
fn accepts_only_specific_string() {
    // DFA that accepts only "ab"
    // q0 --a--> q1 --b--> q2 (final)
    // q0 --b--> dead, q1 --a--> dead
    let dfa = DFABuilder::new()
        .state(0).state(1).state(2).state(3)
        .set_initial(0)
        .set_final(2)
        .symbols(&['a', 'b'])
        .transition(0, 'a', 1)
        .transition(0, 'b', 3)
        .transition(1, 'a', 3)
        .transition(1, 'b', 2)
        .transition(2, 'a', 3)
        .transition(2, 'b', 3)
        .transition(3, 'a', 3)
        .transition(3, 'b', 3)
        .build()
        .unwrap();

    assert!(dfa.accepts(&['a', 'b']));
    assert!(!dfa.accepts(&[]));
    assert!(!dfa.accepts(&['a']));
    assert!(!dfa.accepts(&['a', 'b', 'a']));
    assert!(!dfa.accepts(&['b']));
    assert!(!dfa.accepts(&['b', 'a']));
}

#[test]
fn reachable_states_all_connected() {
    let dfa = make_even_a_dfa();
    let reachable = dfa.reachable_states();
    assert_eq!(reachable.len(), 2);
    assert!(reachable.contains(&0));
    assert!(reachable.contains(&1));
}

#[test]
fn reachable_states_partial() {
    // q0 --a--> q1 (final), q2 unreachable
    let dfa = DFABuilder::new()
        .state(0).state(1).state(2)
        .set_initial(0)
        .set_final(1)
        .symbol('a')
        .transition(0, 'a', 1)
        .transition(1, 'a', 1)
        .build()
        .unwrap();

    let reachable = dfa.reachable_states();
    assert_eq!(reachable.len(), 2);
    assert!(reachable.contains(&0));
    assert!(reachable.contains(&1));
    assert!(!reachable.contains(&2));
}

#[test]
fn get_transitions_returns_correct_data() {
    let dfa = make_even_a_dfa();
    let t = dfa.get_transitions();
    assert_eq!(t.len(), 4);
    assert_eq!(t[&(0, 'a')], 1);
    assert_eq!(t[&(0, 'b')], 0);
    assert_eq!(t[&(1, 'a')], 0);
    assert_eq!(t[&(1, 'b')], 1);
}

#[test]
fn run_returns_trace_for_accepted_string() {
    let dfa = make_even_a_dfa();

    let trace = dfa.run(&['a', 'a']).unwrap();
    assert_eq!(trace.len(), 2);
    assert_eq!(
        trace[0],
        RunStep { from: 0, symbol: 'a', to: 1 }
    );
    assert_eq!(
        trace[1],
        RunStep { from: 1, symbol: 'a', to: 0 }
    );
}

#[test]
fn run_rejects_string_without_path() {
    let dfa = make_even_a_dfa();

    assert!(dfa.run(&['a']).is_none());
    assert!(dfa.run(&['a', 'a', 'a']).is_none());
    assert!(dfa.run(&['c']).is_none());
}

#[test]
fn run_accepts_empty_when_initial_final() {
    let dfa = make_even_a_dfa();

    let trace = dfa.run(&[]).unwrap();
    assert!(trace.is_empty());
}
