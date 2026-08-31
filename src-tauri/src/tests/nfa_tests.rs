use std::collections::{HashMap, HashSet};

use crate::structs::automata::{Automaton, NondeterministicAutomaton};
use crate::commands::nfa_cmd::data_to_nfa;
use crate::structs::data_models::{RunStep, StateData, Trace, TransitionData};
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

    let (traces, accepted) = nfa.run_partial(&['a', 'b']);
    assert!(accepted);
    assert_eq!(traces.len(), 1);
    assert!(traces[0].isFinal);
    assert_eq!(
        traces[0],
        Trace {
            steps: vec![
                RunStep { from: 0, symbol: "a".to_string(), to: 1 },
                RunStep { from: 1, symbol: "b".to_string(), to: 2 },
            ],
            isFinal: true,
        }
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

    // 'b' is not in the alphabet: reading stops before it (0 symbols consumed),
    // the single (empty so far) thread remains, and the string is rejected.
    let (traces, accepted) = nfa.run_partial(&['b']);
    assert!(!accepted);
    assert_eq!(traces, vec![        Trace { steps: vec![], isFinal: false }]);
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

    let (traces, accepted) = nfa.run_partial(&[]);
    assert!(!accepted);
    assert_eq!(traces, vec![        Trace { steps: vec![], isFinal: false }]);
}

#[test]
fn run_partial_full_accepts_empty_when_initial_final() {
    let nfa = NFA::builder()
        .state(0)
        .set_initial(0)
        .set_final(0)
        .build()
        .unwrap();

    let (traces, accepted) = nfa.run_partial(&[]);
    assert!(accepted);
    assert_eq!(traces, vec![        Trace { steps: vec![], isFinal: true }]);
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

    let (traces, accepted) = nfa.run_partial(&['a', 'b']);
    assert!(!accepted);
    assert_eq!(traces.len(), 1);
    // The branch consumed 'a' but could not read 'b': it is interrupted, so even
    // though state q1 is final, the branch is NOT accepting (isFinal = false).
    assert_eq!(
        traces[0],
        Trace {
            steps: vec![RunStep { from: 0, symbol: "a".to_string(), to: 1 }],
            isFinal: false,
        }
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
    let (traces, accepted) = nfa.run_partial(&['a', 'a']);
    assert!(!accepted);
    assert_eq!(traces.len(), 1);
    assert_eq!(traces[0].steps.len(), 1);
}

#[test]
fn run_partial_with_epsilon() {
    // q0 --eps--> q1 --a--> q2 (final)
    // JFLAP Step with Closure: the ε-transition before the symbol is recorded
    // as a '$' step in the trace.
    let nfa = NFA::builder()
        .state(0).state(1).state(2)
        .set_initial(0)
        .set_final(2)
        .symbol('a')
        .epsilon(0, 1)
        .transition(1, 'a', 2)
        .build()
        .unwrap();

    let (traces, accepted) = nfa.run_partial(&['a']);
    assert!(accepted);
    assert_eq!(traces.len(), 1);
    assert_eq!(
        traces[0],
        Trace {
            steps: vec![
                RunStep { from: 0, symbol: "$".to_string(), to: 1 },
                RunStep { from: 1, symbol: "a".to_string(), to: 2 },
            ],
            isFinal: true,
        }
    );
}

#[test]
fn run_partial_closes_after_last_symbol() {
    // q0 --a--> q1 --eps--> q2 (final): the ε-transition AFTER the last symbol
    // must be executed (and recorded) before checking finality.
    let nfa = NFA::builder()
        .state(0).state(1).state(2)
        .set_initial(0)
        .set_final(2)
        .symbol('a')
        .transition(0, 'a', 1)
        .epsilon(1, 2)
        .build()
        .unwrap();

    let (traces, accepted) = nfa.run_partial(&['a']);
    assert!(accepted);
    assert_eq!(traces.len(), 1);
    assert_eq!(
        traces[0],
        Trace {
            steps: vec![
                RunStep { from: 0, symbol: "a".to_string(), to: 1 },
                RunStep { from: 1, symbol: "$".to_string(), to: 2 },
            ],
            isFinal: true,
        }
    );
}

#[test]
fn run_partial_transitive_epsilon_closure_recorded() {
    // q0 --eps--> q1 --eps--> q2 (final). Closure of {q0} = {q0,q1,q2} with both
    // epsilon steps recorded in one trace. No symbols consumed -> accepted.
    let nfa = NFA::builder()
        .state(0).state(1).state(2)
        .set_initial(0)
        .set_final(2)
        .epsilon(0, 1)
        .epsilon(1, 2)
        .build()
        .unwrap();

    let (traces, accepted) = nfa.run_partial(&[]);
    assert!(accepted);
    assert_eq!(traces.len(), 1);
    assert_eq!(
        traces[0],
        Trace {
            steps: vec![
                RunStep { from: 0, symbol: "$".to_string(), to: 1 },
                RunStep { from: 1, symbol: "$".to_string(), to: 2 },
            ],
            isFinal: true,
        }
    );
}

#[test]
fn run_partial_nondeterministic() {
    // q0 --a--> {q0, q1}, q1 --b--> q2 (final)
    // input "aab": every branch is reported:
    //   - q0-a->q1 (dies on second 'a')         1 step
    //   - q0-a->q0-a->q0 (dies on 'b')          2 steps
    //   - q0-a->q0-a->q1-b->q2 (final)          3 steps
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

    let (traces, accepted) = nfa.run_partial(&['a', 'a', 'b']);
    assert!(accepted);
    assert_eq!(traces.len(), 3);
    assert!(traces.iter().any(|t| t.isFinal && t.steps.len() == 3));
    let final_trace = traces.iter().find(|t| t.isFinal).unwrap();
    assert_eq!(
        final_trace.steps,
        vec![
            RunStep { from: 0, symbol: "a".to_string(), to: 0 },
            RunStep { from: 0, symbol: "a".to_string(), to: 1 },
            RunStep { from: 1, symbol: "b".to_string(), to: 2 },
        ]
    );
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

    let (traces, accepted) = nfa.run_partial(&['a']);
    assert!(accepted);
    assert_eq!(traces.len(), 2);
    assert!(traces.contains(
        &Trace {
            steps: vec![RunStep { from: 0, symbol: "a".to_string(), to: 0 }],
            isFinal: false,
        }
    ));
    assert!(traces.contains(
        &Trace {
            steps: vec![RunStep { from: 0, symbol: "a".to_string(), to: 1 }],
            isFinal: true,
        }
    ));
}

#[test]
fn run_partial_converging_branches_keep_separate_histories() {
    // Two nearly-identical paths that converge to the same final state:
    //   q0 -a-> q1 -b-> q3 (final)
    //   q0 -a-> q2 -b-> q3 (final)
    // Both reading streams must be reported separately (2 histories), not merged.
    let nfa = NFA::builder()
        .state(0).state(1).state(2).state(3)
        .set_initial(0)
        .set_final(3)
        .symbol('a').symbol('b')
        .transition(0, 'a', 1)
        .transition(0, 'a', 2)
        .transition(1, 'b', 3)
        .transition(2, 'b', 3)
        .build()
        .unwrap();

    let (traces, accepted) = nfa.run_partial(&['a', 'b']);
    assert!(accepted);
    assert_eq!(traces.len(), 2);
    let t1 = Trace {
        steps: vec![
            RunStep { from: 0, symbol: "a".to_string(), to: 1 },
            RunStep { from: 1, symbol: "b".to_string(), to: 3 },
        ],
        isFinal: true,
    };
    let t2 = Trace {
        steps: vec![
            RunStep { from: 0, symbol: "a".to_string(), to: 2 },
            RunStep { from: 2, symbol: "b".to_string(), to: 3 },
        ],
        isFinal: true,
    };
    assert!(traces.contains(&t1));
    assert!(traces.contains(&t2));
}

#[test]
fn run_partial_reports_every_branch_including_dead() {
    // Reproduces the reported issue: splitting on identical symbols must yield a
    // trace per branch, including ones that terminate early.
    //   q0 -1-> {q1, q2, q3}
    //   q2 -2-> {q4, q5}
    //   q3 -2-> {q6}
    //   q5 -3-> {q7(final)}
    // Running "123" gives 4 branches:
    //   [q0-1->q1]                    (dies on 2)
    //   [q0-1->q3, q3-2->q6]          (dies on 3)
    //   [q0-1->q2, q2-2->q4]          (dies on 3)
    //   [q0-1->q2, q2-2->q5, q5-3->q7](final)
    let nfa = NFA::builder()
        .state(0).state(1).state(2).state(3).state(4).state(5).state(6).state(7)
        .set_initial(0)
        .set_final(7)
        .symbol('1').symbol('2').symbol('3')
        .transition(0, '1', 1)
        .transition(0, '1', 2)
        .transition(0, '1', 3)
        .transition(2, '2', 4)
        .transition(2, '2', 5)
        .transition(3, '2', 6)
        .transition(5, '3', 7)
        .build()
        .unwrap();

    let (traces, accepted) = nfa.run_partial(&['1', '2', '3']);
    assert!(accepted);
    assert_eq!(traces.len(), 4);

    let lens = |t: &Trace| t.steps.len();
    assert!(traces.iter().any(|t| lens(t) == 1 && !t.isFinal));
    assert!(traces.iter().any(|t| lens(t) == 2 && t.steps[1].to == 6 && !t.isFinal));
    assert!(traces.iter().any(|t| lens(t) == 2 && t.steps[1].to == 4 && !t.isFinal));
    let final_trace = traces.iter().find(|t| t.isFinal).unwrap();
    assert_eq!(lens(final_trace), 3);
    assert_eq!(
        final_trace.steps,
        vec![
            RunStep { from: 0, symbol: "1".to_string(), to: 2 },
            RunStep { from: 2, symbol: "2".to_string(), to: 5 },
            RunStep { from: 5, symbol: "3".to_string(), to: 7 },
        ]
    );
}

#[test]
fn data_to_nfa_ignores_orphan_transitions() {
    // Transitions referencing states absent from the states list are dropped at
    // build time, so they do not spawn extra branches.
    let states = vec![
        StateData { id: 0, label: "q0".into(), x: 0.0, y: 0.0, isInitial: true, isFinal: false },
        StateData { id: 1, label: "q1".into(), x: 0.0, y: 0.0, isInitial: false, isFinal: true },
    ];
    let transitions = vec![
        // valid: 0 -1-> 1
        TransitionData { id: 1, from: 0, to: 1, symbol: "1".into() },
        // orphan: 'to' 99 does not exist
        TransitionData { id: 2, from: 0, to: 99, symbol: "1".into() },
        // orphan: 'from' 99 does not exist
        TransitionData { id: 3, from: 99, to: 1, symbol: "1".into() },
    ];

    let nfa = data_to_nfa(&states, &transitions, &[]).unwrap();
    // Only the valid transition should reach final on "1".
    let (traces, accepted) = nfa.run_partial(&['1']);
    assert!(accepted);
    assert_eq!(traces.len(), 1);
    assert_eq!(traces[0].steps, vec![RunStep { from: 0, symbol: "1".to_string(), to: 1 }]);
}

#[test]
fn run_partial_explores_all_branches() {
    // Three independent branches, each ending in the same final state:
    //   q0 -a-> q1 -b-> q4 (final)
    //   q0 -a-> q2 -b-> q4 (final)
    //   q0 -a-> q3 -b-> q4 (final)
    // All three branches must be reported as separate histories.
    let nfa = NFA::builder()
        .state(0).state(1).state(2).state(3).state(4)
        .set_initial(0)
        .set_final(4)
        .symbol('a').symbol('b')
        .transition(0, 'a', 1)
        .transition(0, 'a', 2)
        .transition(0, 'a', 3)
        .transition(1, 'b', 4)
        .transition(2, 'b', 4)
        .transition(3, 'b', 4)
        .build()
        .unwrap();

    let (traces, accepted) = nfa.run_partial(&['a', 'b']);
    assert!(accepted);
    assert_eq!(traces.len(), 3);
    assert!(traces.contains(
        &Trace {
            steps: vec![
                RunStep { from: 0, symbol: "a".to_string(), to: 1 },
                RunStep { from: 1, symbol: "b".to_string(), to: 4 },
            ],
            isFinal: true,
        }
    ));
    assert!(traces.contains(
        &Trace {
            steps: vec![
                RunStep { from: 0, symbol: "a".to_string(), to: 2 },
                RunStep { from: 2, symbol: "b".to_string(), to: 4 },
            ],
            isFinal: true,
        }
    ));
    assert!(traces.contains(
        &Trace {
            steps: vec![
                RunStep { from: 0, symbol: "a".to_string(), to: 3 },
                RunStep { from: 3, symbol: "b".to_string(), to: 4 },
            ],
            isFinal: true,
        }
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

    // 'b' not in alphabet: reading stops before it, thread has 0 steps, rejected.
    let (traces, accepted) = nfa.run_partial(&['b']);
    assert!(!accepted);
    assert_eq!(traces, vec![Trace { steps: vec![], isFinal: false }]);
}

#[test]
fn run_partial_stops_before_symbol_not_in_alphabet() {
    // q0 --a--> q1 --b--> q2 (final); 'x' not in alphabet.
    let nfa = NFA::builder()
        .state(0).state(1).state(2)
        .set_initial(0)
        .set_final(2)
        .symbol('a').symbol('b')
        .transition(0, 'a', 1)
        .transition(1, 'b', 2)
        .build()
        .unwrap();

    // Input "abx": should consume 'a' and 'b' (2 steps), then stop before 'x'.
    let (traces, accepted) = nfa.run_partial(&['a', 'b', 'x']);
    assert!(!accepted);
    assert_eq!(traces.len(), 1);
    assert_eq!(traces[0].steps.len(), 2);
    assert_eq!(
        traces[0],
        Trace {
            steps: vec![
                RunStep { from: 0, symbol: "a".to_string(), to: 1 },
                RunStep { from: 1, symbol: "b".to_string(), to: 2 },
            ],
            isFinal: true,
        }
    );
}

#[test]
fn run_partial_rejects_all_when_no_final_state() {
    // No final states at all -> every string rejected, no histories.
    let nfa = NFA::builder()
        .state(0).state(1).state(2)
        .set_initial(0)
        .symbol('a').symbol('b')
        .transition(0, 'a', 1)
        .transition(0, 'b', 2)
        .transition(0, 'a', 0)
        .build()
        .unwrap();

    let (traces, accepted) = nfa.run_partial(&['a', 'b']);
    assert!(!accepted);
    assert!(traces.is_empty());

    let (traces, accepted) = nfa.run_partial(&[]);
    assert!(!accepted);
    assert!(traces.is_empty());
}
