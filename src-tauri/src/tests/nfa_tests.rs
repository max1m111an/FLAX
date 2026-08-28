use std::collections::{HashMap, HashSet};

use crate::structs::automata::{Automaton, NondeterministicAutomaton};
use crate::structs::data_models::RunStep;
use crate::structs::nfa::{EPSILON, NFA};

#[test]
fn builder_creates_valid_nfa() {
    let nfa = NFA::builder()
        .state(0).state(1).state(2)
        .set_initial(0)
        .set_final(2)
        .symbol('a').symbol('b')
        .transition(0, 'a', 1)
        .transition(1, 'b', 2)
        .build();
    assert!(nfa.is_ok());
    let nfa = nfa.unwrap();
    assert_eq!(nfa.states().len(), 3);
    assert_eq!(nfa.initial_state(), &0);
    assert!(nfa.final_states().contains(&2));
}

#[test]
fn builder_fails_without_initial() {
    let result = NFA::builder()
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
    let final_states = HashSet::from([99]);
    let result = NFA::new(states, HashSet::new(), HashMap::new(), 0, final_states);
    assert!(result.is_err());
}

#[test]
fn new_fails_with_invalid_transition_state() {
    let states = HashSet::from([0]);
    let mut transitions: HashMap<(i32, char), HashSet<i32>> = HashMap::new();
    transitions.insert((0, 'a'), HashSet::from([99]));
    let result = NFA::new(states, HashSet::from(['a']), transitions, 0, HashSet::new());
    assert!(result.is_err());
}

#[test]
fn accepts_simple_string() {
    let nfa = NFA::builder()
        .state(0).state(1).state(2)
        .set_initial(0)
        .set_final(2)
        .symbol('a').symbol('b')
        .transition(0, 'a', 1)
        .transition(1, 'b', 2)
        .build()
        .unwrap();

    assert!(nfa.accepts(&['a', 'b']));
    assert!(!nfa.accepts(&['a']));
    assert!(!nfa.accepts(&['b']));
    assert!(!nfa.accepts(&['a', 'b', 'a']));
    assert!(!nfa.accepts(&[]));
}

#[test]
fn accepts_with_epsilon_transition() {
    // q0 --eps--> q1 --a--> q2 (accepting)
    let nfa = NFA::builder()
        .state(0).state(1).state(2)
        .set_initial(0)
        .set_final(2)
        .symbol('a')
        .epsilon(0, 1)
        .transition(1, 'a', 2)
        .build()
        .unwrap();

    assert!(nfa.accepts(&['a']));
    assert!(!nfa.accepts(&['b']));
    assert!(!nfa.accepts(&[]));
}

#[test]
fn accepts_with_nondeterminism() {
    // q0 --a--> {q0, q1}, q1 --b--> q2 (accepting)
    // accepts a*b
    let nfa = NFA::builder()
        .state(0).state(1).state(2)
        .set_initial(0)
        .set_final(2)
        .symbol('a').symbol('b')
        .transition(0, 'a', 0)
        .transition(0, 'a', 1)
        .transition(1, 'b', 2)
        .build()
        .unwrap();

    assert!(nfa.accepts(&['a', 'b']));
    assert!(nfa.accepts(&['a', 'a', 'b']));
    assert!(nfa.accepts(&['a', 'a', 'a', 'b']));
    assert!(!nfa.accepts(&['b']));
    assert!(!nfa.accepts(&['a']));
    assert!(!nfa.accepts(&['a', 'b', 'a']));
    assert!(!nfa.accepts(&['b', 'a']));
}

#[test]
fn accepts_with_multiple_epsilon_closures() {
    // q0 --eps--> q1 --eps--> q2 --a--> q3 (accepting)
    let nfa = NFA::builder()
        .state(0).state(1).state(2).state(3)
        .set_initial(0)
        .set_final(3)
        .symbol('a')
        .epsilon(0, 1)
        .epsilon(1, 2)
        .transition(2, 'a', 3)
        .build()
        .unwrap();

    assert!(nfa.accepts(&['a']));
    assert!(!nfa.accepts(&[]));
}

#[test]
fn rejects_symbol_not_in_alphabet() {
    let nfa = NFA::builder()
        .state(0).state(1)
        .set_initial(0)
        .set_final(1)
        .symbol('a')
        .transition(0, 'a', 1)
        .build()
        .unwrap();

    assert!(!nfa.accepts(&['b']));
}

#[test]
fn empty_string_accepted_when_initial_is_final() {
    let nfa = NFA::builder()
        .state(0)
        .set_initial(0)
        .set_final(0)
        .build()
        .unwrap();

    assert!(nfa.accepts(&[]));
    assert!(!nfa.accepts(&['a']));
}

#[test]
fn epsilon_closure_includes_all_reachable() {
    let nfa = NFA::builder()
        .state(0).state(1).state(2).state(3)
        .set_initial(0)
        .set_final(3)
        .symbol('a')
        .epsilon(0, 1)
        .epsilon(1, 2)
        .transition(2, 'a', 3)
        .build()
        .unwrap();

    let closure = nfa.epsilon_closure(&0);
    assert!(closure.contains(&0));
    assert!(closure.contains(&1));
    assert!(closure.contains(&2));
    assert!(!closure.contains(&3));
}

#[test]
fn next_states_returns_correct_set() {
    let nfa = NFA::builder()
        .state(0).state(1).state(2)
        .set_initial(0)
        .set_final(2)
        .symbol('a')
        .transition(0, 'a', 1)
        .transition(0, 'a', 2)
        .build()
        .unwrap();

    let next = nfa.next_states(&0, &'a');
    assert!(next.contains(&1));
    assert!(next.contains(&2));
    assert_eq!(next.len(), 2);
}

#[test]
fn is_empty_when_no_accepting() {
    let nfa = NFA::builder()
        .state(0).state(1)
        .set_initial(0)
        .symbol('a')
        .transition(0, 'a', 1)
        .build()
        .unwrap();

    assert!(nfa.is_empty());
}

#[test]
fn is_empty_when_accepting_unreachable() {
    let nfa = NFA::builder()
        .state(0).state(1)
        .set_initial(0)
        .set_final(1)
        .symbol('a')
        .build()
        .unwrap();

    assert!(nfa.is_empty());
}

#[test]
fn is_not_empty_when_accepting_reachable() {
    let nfa = NFA::builder()
        .state(0).state(1)
        .set_initial(0)
        .set_final(1)
        .symbol('a')
        .transition(0, 'a', 1)
        .build()
        .unwrap();

    assert!(!nfa.is_empty());
}

#[test]
fn reachable_states_follows_epsilon() {
    let nfa = NFA::builder()
        .state(0).state(1).state(2)
        .set_initial(0)
        .set_final(2)
        .symbol('a')
        .epsilon(0, 1)
        .transition(1, 'a', 2)
        .build()
        .unwrap();

    let reachable = nfa.reachable_states();
    assert!(reachable.contains(&0));
    assert!(reachable.contains(&1));
    assert!(reachable.contains(&2));
}

#[test]
fn nfa_to_data_roundtrip_via_builder() {
    let nfa = NFA::builder()
        .state(0).state(1).state(2)
        .set_initial(0)
        .set_final(2)
        .symbol('a').symbol('b')
        .transition(0, 'a', 0)
        .transition(0, 'a', 1)
        .transition(0, 'b', 0)
        .transition(1, 'b', 2)
        .epsilon(0, 1)
        .build()
        .unwrap();

    // Test that transitions are stored correctly
    let t = nfa.get_transitions();
    assert!(t.contains_key(&(0, 'a')));
    assert!(t.contains_key(&(0, EPSILON))); // epsilon
    assert!(t[&(0, 'a')].contains(&0));
    assert!(t[&(0, 'a')].contains(&1));
}

#[test]
fn many_states_complex_nfa() {
    // NFA for (a|b)*abb
    let nfa = NFA::builder()
        .state(0).state(1).state(2).state(3)
        .set_initial(0)
        .set_final(3)
        .symbol('a').symbol('b')
        .transition(0, 'a', 0)
        .transition(0, 'b', 0)
        .transition(0, 'a', 1)
        .transition(1, 'b', 2)
        .transition(2, 'b', 3)
        .build()
        .unwrap();

    assert!(nfa.accepts(&['a', 'b', 'b']));
    assert!(nfa.accepts(&['a', 'a', 'b', 'b']));
    assert!(nfa.accepts(&['b', 'a', 'b', 'b']));
    assert!(nfa.accepts(&['a', 'b', 'a', 'b', 'b']));
    assert!(!nfa.accepts(&['a', 'b']));
    assert!(!nfa.accepts(&['a', 'b', 'b', 'a']));
}

#[test]
fn run_returns_trace_for_simple_nfa() {
    let nfa = NFA::builder()
        .state(0).state(1).state(2)
        .set_initial(0)
        .set_final(2)
        .symbol('a').symbol('b')
        .transition(0, 'a', 1)
        .transition(1, 'b', 2)
        .build()
        .unwrap();

    let trace = nfa.run(&['a', 'b']).unwrap();
    assert_eq!(trace.len(), 2);
    assert_eq!(
        trace[0],
        RunStep { from: 0, symbol: "a".to_string(), to: 1 }
    );
    assert_eq!(
        trace[1],
        RunStep { from: 1, symbol: "b".to_string(), to: 2 }
    );
}

#[test]
fn run_tracks_epsilon_steps() {
    let nfa = NFA::builder()
        .state(0).state(1).state(2)
        .set_initial(0)
        .set_final(2)
        .symbol('a')
        .epsilon(0, 1)
        .transition(1, 'a', 2)
        .build()
        .unwrap();

    let trace = nfa.run(&['a']).unwrap();
    assert_eq!(trace.len(), 2);
    assert_eq!(
        trace[0],
        RunStep { from: 0, symbol: "$".to_string(), to: 1 }
    );
    assert_eq!(
        trace[1],
        RunStep { from: 1, symbol: "a".to_string(), to: 2 }
    );
}

#[test]
fn run_finds_accepting_branch() {
    // q0 --a--> {q0, q1}, q1 --b--> q2 (accepting); q0 --a--> q0
    let nfa = NFA::builder()
        .state(0).state(1).state(2)
        .set_initial(0)
        .set_final(2)
        .symbol('a').symbol('b')
        .transition(0, 'a', 0)
        .transition(0, 'a', 1)
        .transition(1, 'b', 2)
        .build()
        .unwrap();

    let trace = nfa.run(&['a', 'a', 'b']).unwrap();
    assert_eq!(trace.len(), 3);
    assert_eq!(
        trace[0],
        RunStep { from: 0, symbol: "a".to_string(), to: 0 }
    );
    assert_eq!(
        trace[1],
        RunStep { from: 0, symbol: "a".to_string(), to: 1 }
    );
    assert_eq!(
        trace[2],
        RunStep { from: 1, symbol: "b".to_string(), to: 2 }
    );
}

#[test]
fn run_rejects_when_no_path() {
    let nfa = NFA::builder()
        .state(0).state(1)
        .set_initial(0)
        .set_final(1)
        .symbol('a')
        .transition(0, 'a', 1)
        .build()
        .unwrap();

    assert!(nfa.run(&['a', 'a']).is_none());
    assert!(nfa.run(&['b']).is_none());
    assert!(nfa.run(&[]).is_none());
}

#[test]
fn run_accepts_empty_when_initial_is_final() {
    let nfa = NFA::builder()
        .state(0)
        .set_initial(0)
        .set_final(0)
        .build()
        .unwrap();

    let trace = nfa.run(&[]).unwrap();
    assert!(trace.is_empty());
}

#[test]
fn run_partial_full_acceptance() {
    let nfa = NFA::builder()
        .state(0).state(1).state(2)
        .set_initial(0)
        .set_final(2)
        .symbol('a').symbol('b')
        .transition(0, 'a', 1)
        .transition(1, 'b', 2)
        .build()
        .unwrap();

    let (histories, accepted) = nfa.run_partial(&['a', 'b']);
    assert!(accepted);
    assert_eq!(histories.len(), 1);
    assert_eq!(
        histories[0],
        vec![
            RunStep { from: 0, symbol: "a".to_string(), to: 1 },
            RunStep { from: 1, symbol: "b".to_string(), to: 2 },
        ]
    );
}

#[test]
fn run_partial_rejected_no_path() {
    let nfa = NFA::builder()
        .state(0).state(1)
        .set_initial(0)
        .set_final(1)
        .symbol('a')
        .transition(0, 'a', 1)
        .build()
        .unwrap();

    let (histories, accepted) = nfa.run_partial(&['b']);
    assert!(!accepted);
    assert!(histories.is_empty());
}

#[test]
fn run_partial_rejected_empty_on_non_final() {
    let nfa = NFA::builder()
        .state(0).state(1)
        .set_initial(0)
        .set_final(1)
        .symbol('a')
        .transition(0, 'a', 1)
        .build()
        .unwrap();

    let (histories, accepted) = nfa.run_partial(&[]);
    assert!(!accepted);
    assert_eq!(histories, vec![Vec::<RunStep>::new()]);
}

#[test]
fn run_partial_full_accepts_empty_when_initial_final() {
    let nfa = NFA::builder()
        .state(0)
        .set_initial(0)
        .set_final(0)
        .build()
        .unwrap();

    let (histories, accepted) = nfa.run_partial(&[]);
    assert!(accepted);
    assert_eq!(histories, vec![Vec::<RunStep>::new()]);
}

#[test]
fn run_partial_stuck_mid_string() {
    // q0 --a--> q1, no transition from q1 on 'b'
    let nfa = NFA::builder()
        .state(0).state(1)
        .set_initial(0)
        .set_final(1)
        .symbol('a').symbol('b')
        .transition(0, 'a', 1)
        .build()
        .unwrap();

    let (histories, accepted) = nfa.run_partial(&['a', 'b']);
    assert!(!accepted);
    assert_eq!(histories.len(), 1);
    assert_eq!(
        histories[0],
        vec![RunStep { from: 0, symbol: "a".to_string(), to: 1 }]
    );
}

#[test]
fn run_partial_consumed_all_not_final() {
    // q0 --a--> q1 (not final)
    let nfa = NFA::builder()
        .state(0).state(1)
        .set_initial(0)
        .set_final(1)
        .symbol('a')
        .transition(0, 'a', 1)
        .build()
        .unwrap();

    // Input "aa" — first 'a' goes to q1, second 'a' has no transition from q1
    let (histories, accepted) = nfa.run_partial(&['a', 'a']);
    assert!(!accepted);
    assert_eq!(histories.len(), 1);
    assert_eq!(histories[0].len(), 1);
}

#[test]
fn run_partial_with_epsilon() {
    // q0 --eps--> q1 --a--> q2 (final)
    let nfa = NFA::builder()
        .state(0).state(1).state(2)
        .set_initial(0)
        .set_final(2)
        .symbol('a')
        .epsilon(0, 1)
        .transition(1, 'a', 2)
        .build()
        .unwrap();

    let (histories, accepted) = nfa.run_partial(&['a']);
    assert!(accepted);
    assert_eq!(histories.len(), 1);
    assert_eq!(
        histories[0],
        vec![RunStep { from: 1, symbol: "a".to_string(), to: 2 }]
    );
}

#[test]
fn run_partial_nondeterministic() {
    // q0 --a--> {q0, q1}, q1 --b--> q2 (final)
    // input "aab": branches merge, accepted path q0->q0->q1->q2
    let nfa = NFA::builder()
        .state(0).state(1).state(2)
        .set_initial(0)
        .set_final(2)
        .symbol('a').symbol('b')
        .transition(0, 'a', 0)
        .transition(0, 'a', 1)
        .transition(1, 'b', 2)
        .build()
        .unwrap();

    let (histories, accepted) = nfa.run_partial(&['a', 'a', 'b']);
    assert!(accepted);
    assert_eq!(histories.len(), 1);
    assert_eq!(histories[0].len(), 3);
}

#[test]
fn run_partial_nondeterministic_parallel() {
    // q0 --a--> {q0, q1}, separate reading histories per branch
    let nfa = NFA::builder()
        .state(0).state(1)
        .set_initial(0)
        .set_final(1)
        .symbol('a')
        .transition(0, 'a', 0)
        .transition(0, 'a', 1)
        .build()
        .unwrap();

    let (histories, accepted) = nfa.run_partial(&['a']);
    assert!(accepted);
    assert_eq!(histories.len(), 2);
    assert!(histories.contains(
        &vec![RunStep { from: 0, symbol: "a".to_string(), to: 0 }]
    ));
    assert!(histories.contains(
        &vec![RunStep { from: 0, symbol: "a".to_string(), to: 1 }]
    ));
}

#[test]
fn run_partial_symbol_not_in_alphabet() {
    let nfa = NFA::builder()
        .state(0).state(1)
        .set_initial(0)
        .set_final(1)
        .symbol('a')
        .transition(0, 'a', 1)
        .build()
        .unwrap();

    let (histories, accepted) = nfa.run_partial(&['b']);
    assert!(!accepted);
    assert!(histories.is_empty());
}
