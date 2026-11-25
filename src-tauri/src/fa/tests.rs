//! Модульные тесты реализации конечного автомата и вспомогательных функций его команд.

use std::collections::{HashMap, HashSet};

use crate::core::api::gen_multi_run_for_output;
use crate::core::automaton::{Automaton, AutomatonStore, NondeterministicAutomaton};
use crate::core::types::{AutomatonData, AutomatonType, RunStep, StateData, Trace, TransitionData};
use crate::fa::api::{data_to_fa, multi_run_fa, run_fa_line, test_line};
use crate::fa::types::{EPSILON, FA};

#[test]
fn builder_creates_valid_fa() {
    let fa = FA::builder()
        .state(0)
        .state(1)
        .state(2)
        .set_initial(0)
        .set_final(2)
        .symbol('a')
        .symbol('b')
        .transition(0, 'a', 1)
        .transition(1, 'b', 2)
        .build();
    assert!(fa.is_ok());
    let fa = fa.unwrap();
    assert_eq!(fa.states().len(), 3);
    assert_eq!(fa.initial_state(), &0);
    assert!(fa.final_states().contains(&2));
}

#[test]
fn builder_fails_without_initial() {
    let result = FA::builder()
        .state(0)
        .state(1)
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
    let result = FA::new(states, HashSet::new(), HashMap::new(), 0, final_states);
    assert!(result.is_err());
}

#[test]
fn new_fails_with_invalid_transition_state() {
    let states = HashSet::from([0]);
    let mut transitions: HashMap<(i32, char), HashSet<i32>> = HashMap::new();
    transitions.insert((0, 'a'), HashSet::from([99]));
    let result = FA::new(states, HashSet::from(['a']), transitions, 0, HashSet::new());
    assert!(result.is_err());
}

#[test]
fn accepts_simple_string() {
    let fa = FA::builder()
        .state(0)
        .state(1)
        .state(2)
        .set_initial(0)
        .set_final(2)
        .symbol('a')
        .symbol('b')
        .transition(0, 'a', 1)
        .transition(1, 'b', 2)
        .build()
        .unwrap();

    assert!(fa.accepts(&['a', 'b']));
    assert!(!fa.accepts(&['a']));
    assert!(!fa.accepts(&['b']));
    assert!(!fa.accepts(&['a', 'b', 'a']));
    assert!(!fa.accepts(&[]));
}

#[test]
fn accepts_with_epsilon_transition() {
    // q0 --eps--> q1 --a--> q2 (допускающее)
    let fa = FA::builder()
        .state(0)
        .state(1)
        .state(2)
        .set_initial(0)
        .set_final(2)
        .symbol('a')
        .epsilon(0, 1)
        .transition(1, 'a', 2)
        .build()
        .unwrap();

    assert!(fa.accepts(&['a']));
    assert!(!fa.accepts(&['b']));
    assert!(!fa.accepts(&[]));
}

#[test]
fn accepts_with_nondeterminism() {
    // q0 --a--> {q0, q1}, q1 --b--> q2 (допускающее)
    // принимает a*b
    let fa = FA::builder()
        .state(0)
        .state(1)
        .state(2)
        .set_initial(0)
        .set_final(2)
        .symbol('a')
        .symbol('b')
        .transition(0, 'a', 0)
        .transition(0, 'a', 1)
        .transition(1, 'b', 2)
        .build()
        .unwrap();

    assert!(fa.accepts(&['a', 'b']));
    assert!(fa.accepts(&['a', 'a', 'b']));
    assert!(fa.accepts(&['a', 'a', 'a', 'b']));
    assert!(!fa.accepts(&['b']));
    assert!(!fa.accepts(&['a']));
    assert!(!fa.accepts(&['a', 'b', 'a']));
    assert!(!fa.accepts(&['b', 'a']));
}

#[test]
fn accepts_with_multiple_epsilon_closures() {
    // q0 --eps--> q1 --eps--> q2 --a--> q3 (допускающее)
    let fa = FA::builder()
        .state(0)
        .state(1)
        .state(2)
        .state(3)
        .set_initial(0)
        .set_final(3)
        .symbol('a')
        .epsilon(0, 1)
        .epsilon(1, 2)
        .transition(2, 'a', 3)
        .build()
        .unwrap();

    assert!(fa.accepts(&['a']));
    assert!(!fa.accepts(&[]));
}

#[test]
fn rejects_symbol_not_in_alphabet() {
    let fa = FA::builder()
        .state(0)
        .state(1)
        .set_initial(0)
        .set_final(1)
        .symbol('a')
        .transition(0, 'a', 1)
        .build()
        .unwrap();

    assert!(!fa.accepts(&['b']));
}

#[test]
fn empty_string_accepted_when_initial_is_final() {
    let fa = FA::builder()
        .state(0)
        .set_initial(0)
        .set_final(0)
        .build()
        .unwrap();

    assert!(fa.accepts(&[]));
    assert!(!fa.accepts(&['a']));
}

#[test]
fn epsilon_closure_includes_all_reachable() {
    let fa = FA::builder()
        .state(0)
        .state(1)
        .state(2)
        .state(3)
        .set_initial(0)
        .set_final(3)
        .symbol('a')
        .epsilon(0, 1)
        .epsilon(1, 2)
        .transition(2, 'a', 3)
        .build()
        .unwrap();

    let closure = fa.epsilon_closure(&0);
    assert!(closure.contains(&0));
    assert!(closure.contains(&1));
    assert!(closure.contains(&2));
    assert!(!closure.contains(&3));
}

#[test]
fn next_states_returns_correct_set() {
    let fa = FA::builder()
        .state(0)
        .state(1)
        .state(2)
        .set_initial(0)
        .set_final(2)
        .symbol('a')
        .transition(0, 'a', 1)
        .transition(0, 'a', 2)
        .build()
        .unwrap();

    let next = fa.next_states(&0, &'a');
    assert!(next.contains(&1));
    assert!(next.contains(&2));
    assert_eq!(next.len(), 2);
}

#[test]
fn is_empty_when_no_accepting() {
    let fa = FA::builder()
        .state(0)
        .state(1)
        .set_initial(0)
        .symbol('a')
        .transition(0, 'a', 1)
        .build()
        .unwrap();

    assert!(fa.is_empty());
}

#[test]
fn is_empty_when_accepting_unreachable() {
    let fa = FA::builder()
        .state(0)
        .state(1)
        .set_initial(0)
        .set_final(1)
        .symbol('a')
        .build()
        .unwrap();

    assert!(fa.is_empty());
}

#[test]
fn is_not_empty_when_accepting_reachable() {
    let fa = FA::builder()
        .state(0)
        .state(1)
        .set_initial(0)
        .set_final(1)
        .symbol('a')
        .transition(0, 'a', 1)
        .build()
        .unwrap();

    assert!(!fa.is_empty());
}

#[test]
fn reachable_states_follows_epsilon() {
    let fa = FA::builder()
        .state(0)
        .state(1)
        .state(2)
        .set_initial(0)
        .set_final(2)
        .symbol('a')
        .epsilon(0, 1)
        .transition(1, 'a', 2)
        .build()
        .unwrap();

    let reachable = fa.reachable_states();
    assert!(reachable.contains(&0));
    assert!(reachable.contains(&1));
    assert!(reachable.contains(&2));
}

#[test]
fn fa_to_data_roundtrip_via_builder() {
    let fa = FA::builder()
        .state(0)
        .state(1)
        .state(2)
        .set_initial(0)
        .set_final(2)
        .symbol('a')
        .symbol('b')
        .transition(0, 'a', 0)
        .transition(0, 'a', 1)
        .transition(0, 'b', 0)
        .transition(1, 'b', 2)
        .epsilon(0, 1)
        .build()
        .unwrap();

    // Проверяем, что переходы сохраняются корректно
    let t = fa.get_transitions();
    assert!(t.contains_key(&(0, 'a')));
    assert!(t.contains_key(&(0, EPSILON))); // eps
    assert!(t[&(0, 'a')].contains(&0));
    assert!(t[&(0, 'a')].contains(&1));
}

#[test]
fn many_states_complex_fa() {
    // КА для (a|b)*abb
    let fa = FA::builder()
        .state(0)
        .state(1)
        .state(2)
        .state(3)
        .set_initial(0)
        .set_final(3)
        .symbol('a')
        .symbol('b')
        .transition(0, 'a', 0)
        .transition(0, 'b', 0)
        .transition(0, 'a', 1)
        .transition(1, 'b', 2)
        .transition(2, 'b', 3)
        .build()
        .unwrap();

    assert!(fa.accepts(&['a', 'b', 'b']));
    assert!(fa.accepts(&['a', 'a', 'b', 'b']));
    assert!(fa.accepts(&['b', 'a', 'b', 'b']));
    assert!(fa.accepts(&['a', 'b', 'a', 'b', 'b']));
    assert!(!fa.accepts(&['a', 'b']));
    assert!(!fa.accepts(&['a', 'b', 'b', 'a']));
}

#[test]
fn run_returns_trace_for_simple_fa() {
    let fa = FA::builder()
        .state(0)
        .state(1)
        .state(2)
        .set_initial(0)
        .set_final(2)
        .symbol('a')
        .symbol('b')
        .transition(0, 'a', 1)
        .transition(1, 'b', 2)
        .build()
        .unwrap();

    let trace = fa.run(&['a', 'b']).unwrap();
    assert_eq!(trace.len(), 2);
    assert_eq!(
        trace[0],
        RunStep {
            from: 0,
            symbol: 'a',
            to: 1
        }
    );
    assert_eq!(
        trace[1],
        RunStep {
            from: 1,
            symbol: 'b',
            to: 2
        }
    );
}

#[test]
fn run_tracks_epsilon_steps() {
    let fa = FA::builder()
        .state(0)
        .state(1)
        .state(2)
        .set_initial(0)
        .set_final(2)
        .symbol('a')
        .epsilon(0, 1)
        .transition(1, 'a', 2)
        .build()
        .unwrap();

    let trace = fa.run(&['a']).unwrap();
    assert_eq!(trace.len(), 2);
    assert_eq!(
        trace[0],
        RunStep {
            from: 0,
            symbol: EPSILON,
            to: 1
        }
    );
    assert_eq!(
        trace[1],
        RunStep {
            from: 1,
            symbol: 'a',
            to: 2
        }
    );
}

#[test]
fn run_finds_accepting_branch() {
    // q0 --a--> {q0, q1}, q1 --b--> q2 (допускающее); q0 --a--> q0
    let fa = FA::builder()
        .state(0)
        .state(1)
        .state(2)
        .set_initial(0)
        .set_final(2)
        .symbol('a')
        .symbol('b')
        .transition(0, 'a', 0)
        .transition(0, 'a', 1)
        .transition(1, 'b', 2)
        .build()
        .unwrap();

    let trace = fa.run(&['a', 'a', 'b']).unwrap();
    assert_eq!(trace.len(), 3);
    assert_eq!(
        trace[0],
        RunStep {
            from: 0,
            symbol: 'a',
            to: 0
        }
    );
    assert_eq!(
        trace[1],
        RunStep {
            from: 0,
            symbol: 'a',
            to: 1
        }
    );
    assert_eq!(
        trace[2],
        RunStep {
            from: 1,
            symbol: 'b',
            to: 2
        }
    );
}

#[test]
fn run_rejects_when_no_path() {
    let fa = FA::builder()
        .state(0)
        .state(1)
        .set_initial(0)
        .set_final(1)
        .symbol('a')
        .transition(0, 'a', 1)
        .build()
        .unwrap();

    assert!(fa.run(&['a', 'a']).is_none());
    assert!(fa.run(&['b']).is_none());
    assert!(fa.run(&[]).is_none());
}

#[test]
fn run_accepts_empty_when_initial_is_final() {
    let fa = FA::builder()
        .state(0)
        .set_initial(0)
        .set_final(0)
        .build()
        .unwrap();

    let trace = fa.run(&[]).unwrap();
    assert!(trace.is_empty());
}

#[test]
fn run_partial_full_acceptance() {
    let fa = FA::builder()
        .state(0)
        .state(1)
        .state(2)
        .set_initial(0)
        .set_final(2)
        .symbol('a')
        .symbol('b')
        .transition(0, 'a', 1)
        .transition(1, 'b', 2)
        .build()
        .unwrap();

    let (traces, accepted) = fa.run_partial(&['a', 'b']);
    assert!(accepted);
    assert_eq!(traces.len(), 1);
    assert!(traces[0].isFinal);
    assert_eq!(
        traces[0],
        Trace {
            steps: vec![
                RunStep {
                    from: 0,
                    symbol: 'a',
                    to: 1
                },
                RunStep {
                    from: 1,
                    symbol: 'b',
                    to: 2
                },
            ],
            isFinal: true,
        }
    );
}

#[test]
fn run_partial_rejected_no_path() {
    let fa = FA::builder()
        .state(0)
        .state(1)
        .set_initial(0)
        .set_final(1)
        .symbol('a')
        .transition(0, 'a', 1)
        .build()
        .unwrap();

    // 'b' нет в алфавите: чтение останавливается перед ним (прочитано 0 символов),
    // остаётся единственный (пока пустой) поток, цепочка отклоняется.
    let (traces, accepted) = fa.run_partial(&['b']);
    assert!(!accepted);
    assert_eq!(
        traces,
        vec![Trace {
            steps: vec![],
            isFinal: false
        }]
    );
}

#[test]
fn run_partial_rejected_empty_on_non_final() {
    let fa = FA::builder()
        .state(0)
        .state(1)
        .set_initial(0)
        .set_final(1)
        .symbol('a')
        .transition(0, 'a', 1)
        .build()
        .unwrap();

    let (traces, accepted) = fa.run_partial(&[]);
    assert!(!accepted);
    assert_eq!(
        traces,
        vec![Trace {
            steps: vec![],
            isFinal: false
        }]
    );
}

#[test]
fn run_partial_full_accepts_empty_when_initial_final() {
    let fa = FA::builder()
        .state(0)
        .set_initial(0)
        .set_final(0)
        .build()
        .unwrap();

    let (traces, accepted) = fa.run_partial(&[]);
    assert!(accepted);
    assert_eq!(
        traces,
        vec![Trace {
            steps: vec![],
            isFinal: true
        }]
    );
}

#[test]
fn run_partial_stuck_mid_string() {
    // q0 --a--> q1, из q1 нет перехода по 'b'
    let fa = FA::builder()
        .state(0)
        .state(1)
        .set_initial(0)
        .set_final(1)
        .symbol('a')
        .symbol('b')
        .transition(0, 'a', 1)
        .build()
        .unwrap();

    let (traces, accepted) = fa.run_partial(&['a', 'b']);
    assert!(!accepted);
    assert_eq!(traces.len(), 1);
    // Ветвь прочитала 'a', но не смогла прочитать 'b': она прервана, поэтому, хотя
    // состояние q1 финальное, ветвь НЕ допускающая (isFinal = false).
    assert_eq!(
        traces[0],
        Trace {
            steps: vec![RunStep {
                from: 0,
                symbol: 'a',
                to: 1
            }],
            isFinal: false,
        }
    );
}

#[test]
fn run_partial_consumed_all_not_final() {
    // q0 --a--> q1 (не финальное)
    let fa = FA::builder()
        .state(0)
        .state(1)
        .set_initial(0)
        .set_final(1)
        .symbol('a')
        .transition(0, 'a', 1)
        .build()
        .unwrap();

    // Вход "aa": первая 'a' ведёт в q1, для второй 'a' нет перехода из q1
    let (traces, accepted) = fa.run_partial(&['a', 'a']);
    assert!(!accepted);
    assert_eq!(traces.len(), 1);
    assert_eq!(traces[0].steps.len(), 1);
}

#[test]
fn run_partial_with_epsilon() {
    // q0 --eps--> q1 --a--> q2 (финальное). eps ветвится: можно остаться в q0
    // (тогда 'a' не прочитать — тупик) или пройти по eps в q1 и прочитать 'a'.
    let fa = FA::builder()
        .state(0)
        .state(1)
        .state(2)
        .set_initial(0)
        .set_final(2)
        .symbol('a')
        .epsilon(0, 1)
        .transition(1, 'a', 2)
        .build()
        .unwrap();

    let (traces, accepted) = fa.run_partial(&['a']);
    assert!(accepted);
    assert_eq!(traces.len(), 2);
    assert_eq!(
        traces[0],
        Trace {
            steps: vec![],
            isFinal: false,
        }
    );
    assert_eq!(
        traces[1],
        Trace {
            steps: vec![
                RunStep {
                    from: 0,
                    symbol: '$',
                    to: 1
                },
                RunStep {
                    from: 1,
                    symbol: 'a',
                    to: 2
                },
            ],
            isFinal: true,
        }
    );
}

#[test]
fn run_partial_closes_after_last_symbol() {
    // q0 --a--> q1 --eps--> q2 (финальное): eps ПОСЛЕ последнего символа тоже
    // ветвится — остаться в q1 (тупик) или пройти в q2 (принятие).
    let fa = FA::builder()
        .state(0)
        .state(1)
        .state(2)
        .set_initial(0)
        .set_final(2)
        .symbol('a')
        .transition(0, 'a', 1)
        .epsilon(1, 2)
        .build()
        .unwrap();

    let (traces, accepted) = fa.run_partial(&['a']);
    assert!(accepted);
    assert_eq!(traces.len(), 2);
    assert_eq!(
        traces[0],
        Trace {
            steps: vec![RunStep {
                from: 0,
                symbol: 'a',
                to: 1
            }],
            isFinal: false,
        }
    );
    assert_eq!(
        traces[1],
        Trace {
            steps: vec![
                RunStep {
                    from: 0,
                    symbol: 'a',
                    to: 1
                },
                RunStep {
                    from: 1,
                    symbol: '$',
                    to: 2
                },
            ],
            isFinal: true,
        }
    );
}

#[test]
fn run_partial_transitive_epsilon_closure_recorded() {
    // q0 --eps--> q1 --eps--> q2 (финальное). Каждый eps-переход ветвится, поэтому
    // пустой вход даёт три трассы: остаться в q0, дойти до q1 и дойти до q2.
    let fa = FA::builder()
        .state(0)
        .state(1)
        .state(2)
        .set_initial(0)
        .set_final(2)
        .epsilon(0, 1)
        .epsilon(1, 2)
        .build()
        .unwrap();

    let (traces, accepted) = fa.run_partial(&[]);
    assert!(accepted);
    assert_eq!(traces.len(), 3);
    assert_eq!(
        traces[0],
        Trace {
            steps: vec![],
            isFinal: false
        }
    );
    assert_eq!(
        traces[1],
        Trace {
            steps: vec![RunStep {
                from: 0,
                symbol: '$',
                to: 1
            }],
            isFinal: false,
        }
    );
    assert_eq!(
        traces[2],
        Trace {
            steps: vec![
                RunStep {
                    from: 0,
                    symbol: '$',
                    to: 1
                },
                RunStep {
                    from: 1,
                    symbol: '$',
                    to: 2
                },
            ],
            isFinal: true,
        }
    );
}

#[test]
fn run_partial_epsilon_branches_keep_dead_traces() {
    // q0 -1-> q1, q1 -eps-> q2, q2 -2-> q3 (финальное), вход "12".
    // Ветвь, оставшаяся в q1, не может прочитать '2' и остаётся тупиковой;
    // ветвь, прошедшая по eps, принимается. Тупик не отменяет принятие.
    let fa = FA::builder()
        .state(0)
        .state(1)
        .state(2)
        .state(3)
        .set_initial(0)
        .set_final(3)
        .symbol('1')
        .symbol('2')
        .transition(0, '1', 1)
        .epsilon(1, 2)
        .transition(2, '2', 3)
        .build()
        .unwrap();

    let (traces, accepted) = fa.run_partial(&['1', '2']);
    assert!(accepted);
    assert_eq!(traces.len(), 2);
    assert_eq!(
        traces[0],
        Trace {
            steps: vec![RunStep {
                from: 0,
                symbol: '1',
                to: 1
            }],
            isFinal: false,
        }
    );
    assert_eq!(
        traces[1],
        Trace {
            steps: vec![
                RunStep {
                    from: 0,
                    symbol: '1',
                    to: 1
                },
                RunStep {
                    from: 1,
                    symbol: '$',
                    to: 2
                },
                RunStep {
                    from: 2,
                    symbol: '2',
                    to: 3
                },
            ],
            isFinal: true,
        }
    );
}

#[test]
fn run_partial_multiple_epsilon_branches() {
    // q0 -eps-> {q1, q2}; q1 -a-> q3 (финальное), q2 -a-> q4 (не финальное).
    // Три ветви: тупик в q0, принятие через q1, отказ через q2.
    let fa = FA::builder()
        .state(0)
        .state(1)
        .state(2)
        .state(3)
        .state(4)
        .set_initial(0)
        .set_final(3)
        .symbol('a')
        .epsilon(0, 1)
        .epsilon(0, 2)
        .transition(1, 'a', 3)
        .transition(2, 'a', 4)
        .build()
        .unwrap();

    let (traces, accepted) = fa.run_partial(&['a']);
    assert!(accepted);
    assert_eq!(traces.len(), 3);
    let accepted_step = RunStep {
        from: 1,
        symbol: 'a',
        to: 3,
    };
    let rejected_step = RunStep {
        from: 2,
        symbol: 'a',
        to: 4,
    };
    assert!(traces.iter().any(|t| t.steps.is_empty() && !t.isFinal));
    assert!(
        traces
            .iter()
            .any(|t| t.isFinal && t.steps.last() == Some(&accepted_step))
    );
    assert!(
        traces
            .iter()
            .any(|t| !t.isFinal && t.steps.last() == Some(&rejected_step))
    );
}

#[test]
fn run_partial_epsilon_cycle_terminates() {
    // q0 -eps-> q1 -eps-> q0 (цикл), q1 -a-> q2 (финальное). Разворачивание eps
    // останавливается по visited, бесконечных трасс не возникает.
    let fa = FA::builder()
        .state(0)
        .state(1)
        .state(2)
        .set_initial(0)
        .set_final(2)
        .symbol('a')
        .epsilon(0, 1)
        .epsilon(1, 0)
        .transition(1, 'a', 2)
        .build()
        .unwrap();

    let (traces, accepted) = fa.run_partial(&['a']);
    assert!(accepted);
    assert_eq!(traces.len(), 2);
    assert!(traces.iter().all(|t| t.steps.len() <= 2));
    let expected = vec![
        RunStep {
            from: 0,
            symbol: '$',
            to: 1,
        },
        RunStep {
            from: 1,
            symbol: 'a',
            to: 2,
        },
    ];
    assert!(traces.iter().any(|t| t.isFinal && t.steps == expected));
}

#[test]
fn run_fa_line_keeps_dead_traces_on_acceptance() {
    // Проверка ответа fa_run_str через тестируемый хелпер: статус 200, final-ветвь
    // первая, тупиковая ветвь сохранена, processed_len считает только символы.
    let entry = multi_run_entry(
        vec![
            multi_run_state(0, true, false),
            multi_run_state(1, false, false),
            multi_run_state(2, false, false),
            multi_run_state(3, false, true),
        ],
        vec![
            multi_run_transition(1, 0, 1, '1'),
            multi_run_transition(2, 1, 2, EPSILON),
            multi_run_transition(3, 2, 3, '2'),
        ],
    );

    let result = run_fa_line(&entry, "12");
    assert_eq!(result.status, tauri::http::StatusCode::OK);
    assert_eq!(result.traces.len(), 2);
    assert!(result.traces[0].isFinal);
    assert!(!result.traces[1].isFinal);
    assert_eq!(
        result.traces[0].steps,
        vec![
            RunStep {
                from: 0,
                symbol: '1',
                to: 1
            },
            RunStep {
                from: 1,
                symbol: '$',
                to: 2
            },
            RunStep {
                from: 2,
                symbol: '2',
                to: 3
            },
        ]
    );
}

#[test]
fn run_fa_line_rejects_unreadable_trailing_symbol() {
    // Тот же автомат: q0 -1-> q1, q1 -eps-> q2, q2 -2-> q3 (финальное).
    // "12" проходит, а "123" нет: после '2' поток дошёл до финального q3,
    // но перехода по '3' нет, поэтому accepting-ветвь становится тупиковой.
    let entry = multi_run_entry(
        vec![
            multi_run_state(0, true, false),
            multi_run_state(1, false, false),
            multi_run_state(2, false, false),
            multi_run_state(3, false, true),
        ],
        vec![
            multi_run_transition(1, 0, 1, '1'),
            multi_run_transition(2, 1, 2, EPSILON),
            multi_run_transition(3, 2, 3, '2'),
        ],
    );

    let accepted = run_fa_line(&entry, "12");
    assert_eq!(accepted.status, tauri::http::StatusCode::OK);
    assert!(accepted.traces.iter().any(|t| t.isFinal));

    let rejected = run_fa_line(&entry, "123");
    assert_eq!(rejected.status, tauri::http::StatusCode::UNAUTHORIZED);
    assert_eq!(rejected.traces.len(), 2);
    assert!(rejected.traces.iter().all(|t| !t.isFinal));
    assert!(rejected.message.contains("обработано 2 из 3"));
    assert_eq!(
        rejected.traces[1].steps,
        vec![
            RunStep {
                from: 0,
                symbol: '1',
                to: 1
            },
            RunStep {
                from: 1,
                symbol: '$',
                to: 2
            },
            RunStep {
                from: 2,
                symbol: '2',
                to: 3
            },
        ]
    );
}

#[test]
fn run_partial_nondeterministic() {
    // q0 --a--> {q0, q1}, q1 --b--> q2 (финальное)
    // вход "aab": сообщается о каждой ветви:
    //   - q0-a->q1 (обрывается на второй 'a')    1 шаг
    //   - q0-a->q0-a->q0 (обрывается на 'b')     2 шага
    //   - q0-a->q0-a->q1-b->q2 (финальное)       3 шага
    let fa = FA::builder()
        .state(0)
        .state(1)
        .state(2)
        .set_initial(0)
        .set_final(2)
        .symbol('a')
        .symbol('b')
        .transition(0, 'a', 0)
        .transition(0, 'a', 1)
        .transition(1, 'b', 2)
        .build()
        .unwrap();

    let (traces, accepted) = fa.run_partial(&['a', 'a', 'b']);
    assert!(accepted);
    assert_eq!(traces.len(), 3);
    assert!(traces.iter().any(|t| t.isFinal && t.steps.len() == 3));
    let final_trace = traces.iter().find(|t| t.isFinal).unwrap();
    assert_eq!(
        final_trace.steps,
        vec![
            RunStep {
                from: 0,
                symbol: 'a',
                to: 0
            },
            RunStep {
                from: 0,
                symbol: 'a',
                to: 1
            },
            RunStep {
                from: 1,
                symbol: 'b',
                to: 2
            },
        ]
    );
}

#[test]
fn run_partial_nondeterministic_parallel() {
    // q0 --a--> {q0, q1}, отдельные истории чтения для каждой ветви
    let fa = FA::builder()
        .state(0)
        .state(1)
        .set_initial(0)
        .set_final(1)
        .symbol('a')
        .transition(0, 'a', 0)
        .transition(0, 'a', 1)
        .build()
        .unwrap();

    let (traces, accepted) = fa.run_partial(&['a']);
    assert!(accepted);
    assert_eq!(traces.len(), 2);
    assert!(traces.contains(&Trace {
        steps: vec![RunStep {
            from: 0,
            symbol: 'a',
            to: 0
        }],
        isFinal: false,
    }));
    assert!(traces.contains(&Trace {
        steps: vec![RunStep {
            from: 0,
            symbol: 'a',
            to: 1
        }],
        isFinal: true,
    }));
}

#[test]
fn run_partial_converging_branches_keep_separate_histories() {
    // Два почти одинаковых пути, сходящихся в одно финальное состояние:
    //   q0 -a-> q1 -b-> q3 (финальное)
    //   q0 -a-> q2 -b-> q3 (финальное)
    // Оба потока чтения должны сообщаться отдельно (2 истории), а не сливаться.
    let fa = FA::builder()
        .state(0)
        .state(1)
        .state(2)
        .state(3)
        .set_initial(0)
        .set_final(3)
        .symbol('a')
        .symbol('b')
        .transition(0, 'a', 1)
        .transition(0, 'a', 2)
        .transition(1, 'b', 3)
        .transition(2, 'b', 3)
        .build()
        .unwrap();

    let (traces, accepted) = fa.run_partial(&['a', 'b']);
    assert!(accepted);
    assert_eq!(traces.len(), 2);
    let t1 = Trace {
        steps: vec![
            RunStep {
                from: 0,
                symbol: 'a',
                to: 1,
            },
            RunStep {
                from: 1,
                symbol: 'b',
                to: 3,
            },
        ],
        isFinal: true,
    };
    let t2 = Trace {
        steps: vec![
            RunStep {
                from: 0,
                symbol: 'a',
                to: 2,
            },
            RunStep {
                from: 2,
                symbol: 'b',
                to: 3,
            },
        ],
        isFinal: true,
    };
    assert!(traces.contains(&t1));
    assert!(traces.contains(&t2));
}

#[test]
fn run_partial_reports_every_branch_including_dead() {
    // Воспроизводит найденную ошибку: ветвление по одинаковым символам должно давать
    // трассу на каждую ветвь, включая досрочно завершившиеся.
    //   q0 -1-> {q1, q2, q3}
    //   q2 -2-> {q4, q5}
    //   q3 -2-> {q6}
    //   q5 -3-> {q7(финальное)}
    // Прогон "123" даёт 4 ветви:
    //   [q0-1->q1]                    (обрывается на 2)
    //   [q0-1->q3, q3-2->q6]          (обрывается на 3)
    //   [q0-1->q2, q2-2->q4]          (обрывается на 3)
    //   [q0-1->q2, q2-2->q5, q5-3->q7](финальное)
    let fa = FA::builder()
        .state(0)
        .state(1)
        .state(2)
        .state(3)
        .state(4)
        .state(5)
        .state(6)
        .state(7)
        .set_initial(0)
        .set_final(7)
        .symbol('1')
        .symbol('2')
        .symbol('3')
        .transition(0, '1', 1)
        .transition(0, '1', 2)
        .transition(0, '1', 3)
        .transition(2, '2', 4)
        .transition(2, '2', 5)
        .transition(3, '2', 6)
        .transition(5, '3', 7)
        .build()
        .unwrap();

    let (traces, accepted) = fa.run_partial(&['1', '2', '3']);
    assert!(accepted);
    assert_eq!(traces.len(), 4);

    let lens = |t: &Trace| t.steps.len();
    assert!(traces.iter().any(|t| lens(t) == 1 && !t.isFinal));
    assert!(
        traces
            .iter()
            .any(|t| lens(t) == 2 && t.steps[1].to == 6 && !t.isFinal)
    );
    assert!(
        traces
            .iter()
            .any(|t| lens(t) == 2 && t.steps[1].to == 4 && !t.isFinal)
    );
    let final_trace = traces.iter().find(|t| t.isFinal).unwrap();
    assert_eq!(lens(final_trace), 3);
    assert_eq!(
        final_trace.steps,
        vec![
            RunStep {
                from: 0,
                symbol: '1',
                to: 2
            },
            RunStep {
                from: 2,
                symbol: '2',
                to: 5
            },
            RunStep {
                from: 5,
                symbol: '3',
                to: 7
            },
        ]
    );
}

#[test]
fn data_to_fa_ignores_orphan_transitions() {
    // Переходы, ссылающиеся на состояния, отсутствующие в списке состояний,
    // отбрасываются при построении и не порождают лишних ветвей.
    let states = vec![
        StateData {
            id: 0,
            label: "q0".into(),
            x: 0.0,
            y: 0.0,
            isInitial: true,
            isFinal: Some(false),
        },
        StateData {
            id: 1,
            label: "q1".into(),
            x: 0.0,
            y: 0.0,
            isInitial: false,
            isFinal: Some(true),
        },
    ];
    let transitions = vec![
        // корректный: 0 -1-> 1
        TransitionData {
            id: 1,
            from: 0,
            to: 1,
            symbol: '1',
            output: None,
        },
        // осиротевший: 'to' 99 не существует
        TransitionData {
            id: 2,
            from: 0,
            to: 99,
            symbol: '1',
            output: None,
        },
        // осиротевший: 'from' 99 не существует
        TransitionData {
            id: 3,
            from: 99,
            to: 1,
            symbol: '1',
            output: None,
        },
    ];

    let fa = data_to_fa(&states, &transitions).unwrap();
    // Только корректный переход должен приводить в финальное состояние на "1".
    let (traces, accepted) = fa.run_partial(&['1']);
    assert!(accepted);
    assert_eq!(traces.len(), 1);
    assert_eq!(
        traces[0].steps,
        vec![RunStep {
            from: 0,
            symbol: '1',
            to: 1
        }]
    );
}

#[test]
fn run_partial_explores_all_branches() {
    // Три независимые ветви, каждая заканчивается в одном и том же финальном состоянии:
    //   q0 -a-> q1 -b-> q4 (финальное)
    //   q0 -a-> q2 -b-> q4 (финальное)
    //   q0 -a-> q3 -b-> q4 (финальное)
    // Все три ветви должны сообщаться как отдельные истории.
    let fa = FA::builder()
        .state(0)
        .state(1)
        .state(2)
        .state(3)
        .state(4)
        .set_initial(0)
        .set_final(4)
        .symbol('a')
        .symbol('b')
        .transition(0, 'a', 1)
        .transition(0, 'a', 2)
        .transition(0, 'a', 3)
        .transition(1, 'b', 4)
        .transition(2, 'b', 4)
        .transition(3, 'b', 4)
        .build()
        .unwrap();

    let (traces, accepted) = fa.run_partial(&['a', 'b']);
    assert!(accepted);
    assert_eq!(traces.len(), 3);
    assert!(traces.contains(&Trace {
        steps: vec![
            RunStep {
                from: 0,
                symbol: 'a',
                to: 1
            },
            RunStep {
                from: 1,
                symbol: 'b',
                to: 4
            },
        ],
        isFinal: true,
    }));
    assert!(traces.contains(&Trace {
        steps: vec![
            RunStep {
                from: 0,
                symbol: 'a',
                to: 2
            },
            RunStep {
                from: 2,
                symbol: 'b',
                to: 4
            },
        ],
        isFinal: true,
    }));
    assert!(traces.contains(&Trace {
        steps: vec![
            RunStep {
                from: 0,
                symbol: 'a',
                to: 3
            },
            RunStep {
                from: 3,
                symbol: 'b',
                to: 4
            },
        ],
        isFinal: true,
    }));
}

#[test]
fn run_partial_symbol_not_in_alphabet() {
    let fa = FA::builder()
        .state(0)
        .state(1)
        .set_initial(0)
        .set_final(1)
        .symbol('a')
        .transition(0, 'a', 1)
        .build()
        .unwrap();

    // 'b' нет в алфавите: чтение останавливается перед ним, у потока 0 шагов, отклонено.
    let (traces, accepted) = fa.run_partial(&['b']);
    assert!(!accepted);
    assert_eq!(
        traces,
        vec![Trace {
            steps: vec![],
            isFinal: false
        }]
    );
}

#[test]
fn run_partial_stops_before_symbol_not_in_alphabet() {
    // q0 --a--> q1 --b--> q2 (финальное); 'x' нет в алфавите.
    let fa = FA::builder()
        .state(0)
        .state(1)
        .state(2)
        .set_initial(0)
        .set_final(2)
        .symbol('a')
        .symbol('b')
        .transition(0, 'a', 1)
        .transition(1, 'b', 2)
        .build()
        .unwrap();

    // Вход "abx": должны быть прочитаны 'a' и 'b' (2 шага), затем остановка перед 'x'.
    // Поток дошёл до финального q2, но не дочитал вход, поэтому isFinal = false.
    let (traces, accepted) = fa.run_partial(&['a', 'b', 'x']);
    assert!(!accepted);
    assert_eq!(traces.len(), 1);
    assert_eq!(traces[0].steps.len(), 2);
    assert_eq!(
        traces[0],
        Trace {
            steps: vec![
                RunStep {
                    from: 0,
                    symbol: 'a',
                    to: 1
                },
                RunStep {
                    from: 1,
                    symbol: 'b',
                    to: 2
                },
            ],
            isFinal: false,
        }
    );
}

#[test]
fn run_partial_rejects_all_when_no_final_state() {
    // Финальных состояний нет вовсе -> любая строка отклоняется, историй нет.
    let fa = FA::builder()
        .state(0)
        .state(1)
        .state(2)
        .set_initial(0)
        .symbol('a')
        .symbol('b')
        .transition(0, 'a', 1)
        .transition(0, 'b', 2)
        .transition(0, 'a', 0)
        .build()
        .unwrap();

    let (traces, accepted) = fa.run_partial(&['a', 'b']);
    assert!(!accepted);
    assert!(traces.is_empty());

    let (traces, accepted) = fa.run_partial(&[]);
    assert!(!accepted);
    assert!(traces.is_empty());
}

#[test]
fn test_line_reports_is_final_and_correct_symbols() {
    // q0 --a--> q1 --a--> q2 (финальное). Состояние q1 не финальное, поэтому "a" и
    // "aa" читаются полностью, но только "aa" заканчивается в финальном состоянии.
    let fa = FA::builder()
        .state(0)
        .state(1)
        .state(2)
        .set_initial(0)
        .set_final(2)
        .symbol('a')
        .transition(0, 'a', 1)
        .transition(1, 'a', 2)
        .build()
        .unwrap();

    // Пустая строка: 0 корректных символов, не принята.
    assert_eq!(test_line(&fa, ""), (false, 0));
    // "a": остановка в q1, которое не финальное; прочитан 1 символ.
    assert_eq!(test_line(&fa, "a"), (false, 1));
    // "aa": полное принятие, 2 символа.
    assert_eq!(test_line(&fa, "aa"), (true, 2));
    // "aaa": все потоки обрываются; прочитать можно только 2 символа.
    assert_eq!(test_line(&fa, "aaa"), (false, 2));
}

#[test]
fn test_line_counts_only_non_epsilon_steps() {
    // q0 --eps--> q1 --a--> q2 (финальное). Шаг `$` не считается прочитанным
    // символом, поэтому "a" даёт correctSymbols = 1 (а не 2).
    let fa = FA::builder()
        .state(0)
        .state(1)
        .state(2)
        .set_initial(0)
        .set_final(2)
        .symbol('a')
        .epsilon(0, 1)
        .transition(1, 'a', 2)
        .build()
        .unwrap();

    assert_eq!(test_line(&fa, "a"), (true, 1));
}

fn multi_run_state(id: i32, is_initial: bool, is_final: bool) -> StateData {
    StateData {
        id,
        label: format!("q{}", id),
        x: 0.0,
        y: 0.0,
        isInitial: is_initial,
        isFinal: Some(is_final),
    }
}

fn multi_run_transition(id: i32, from: i32, to: i32, symbol: char) -> TransitionData {
    TransitionData {
        id,
        from,
        to,
        symbol,
        output: None,
    }
}

fn multi_run_entry(states: Vec<StateData>, transitions: Vec<TransitionData>) -> AutomatonData {
    AutomatonData {
        id: 1,
        name: "multi-run".to_string(),
        automatonType: crate::core::types::AutomatonType::FA,
        states,
        transitions,
    }
}

#[test]
fn multi_run_fa_reports_results_in_input_order() {
    let entry = multi_run_entry(
        vec![
            multi_run_state(0, true, false),
            multi_run_state(1, false, false),
            multi_run_state(2, false, true),
        ],
        vec![
            multi_run_transition(1, 0, 1, 'a'),
            multi_run_transition(2, 1, 2, 'a'),
        ],
    );
    let inputs = ["", "a", "aa", "aaa"].map(String::from);

    let result = multi_run_fa(&entry, &inputs);

    assert_eq!(result.status, tauri::http::StatusCode::OK);
    assert!(result.message.is_empty());
    assert_eq!(result.traces.len(), 4);

    let lines: Vec<&str> = result.traces.iter().map(|t| t.line.as_str()).collect();
    assert_eq!(lines, ["", "a", "aa", "aaa"]);

    assert_eq!(
        (result.traces[0].isFinal, result.traces[0].correctSymbols),
        (false, 0)
    );
    assert_eq!(
        (result.traces[1].isFinal, result.traces[1].correctSymbols),
        (false, 1)
    );
    assert_eq!(
        (result.traces[2].isFinal, result.traces[2].correctSymbols),
        (true, 2)
    );
    assert_eq!(
        (result.traces[3].isFinal, result.traces[3].correctSymbols),
        (false, 2)
    );
}

#[test]
fn multi_run_fa_invalid_automaton_returns_400() {
    let entry = multi_run_entry(vec![multi_run_state(0, false, true)], Vec::new());

    let result = multi_run_fa(&entry, &["a".to_string()]);

    assert_eq!(result.status, tauri::http::StatusCode::BAD_REQUEST);
    assert_eq!(result.message, "Некорректный автомат");
    assert!(result.traces.is_empty());
}

#[test]
fn multi_run_fa_empty_inputs_returns_200_and_no_traces() {
    let entry = multi_run_entry(vec![multi_run_state(0, true, true)], Vec::new());

    let result = multi_run_fa(&entry, &[]);

    assert_eq!(result.status, tauri::http::StatusCode::OK);
    assert!(result.message.is_empty());
    assert!(result.traces.is_empty());
}

#[test]
fn generate_test_inputs_empty_singles_and_sorted() {
    let fa = FA::builder()
        .state(0)
        .state(1)
        .state(2)
        .set_initial(0)
        .set_final(2)
        .symbol('a')
        .symbol('b')
        .transition(0, 'a', 1)
        .transition(1, 'b', 2)
        .build()
        .unwrap();

    let inputs = fa.generate_test_inputs(50, 15);
    assert!(inputs.contains(&"".to_string()));
    assert!(inputs.contains(&"a".to_string()));
    assert!(inputs.contains(&"b".to_string()));
    // минимальный путь до финального состояния (через промежуточное состояние)
    assert!(inputs.contains(&"ab".to_string()));
    assert!(inputs.len() <= 50);
    // отсортировано по возрастанию длины
    assert!(inputs.windows(2).all(|w| w[0].len() <= w[1].len()));
    // без дубликатов
    let mut uniq = inputs.clone();
    uniq.dedup();
    assert_eq!(uniq, inputs);
    // хотя бы один негативный случай с символом вне алфавита
    assert!(
        inputs
            .iter()
            .any(|s| s.chars().any(|c| !fa.alphabet().contains(&c)))
    );
}

#[test]
fn generate_test_inputs_repeats_loop_two_or_three_times_before_final() {
    // q0 --b--> q0 (петля), q0 --a--> q1 (финальное)
    let fa = FA::builder()
        .state(0)
        .state(1)
        .set_initial(0)
        .set_final(1)
        .symbol('a')
        .symbol('b')
        .transition(0, 'a', 1)
        .transition(0, 'b', 0)
        .build()
        .unwrap();

    let inputs = fa.generate_test_inputs(50, 15);
    // 2 и 3 прохода по петле с продолжением до финального состояния
    assert!(inputs.contains(&"bba".to_string()));
    assert!(inputs.contains(&"bbba".to_string()));
    // без длинных цепочек из десятков повторов
    assert!(inputs.iter().all(|s| s.chars().count() <= 4));
    // символов вне алфавита — ровно 3 строки
    let outside: Vec<&String> = inputs
        .iter()
        .filter(|s| s.chars().any(|c| !['a', 'b'].contains(&c)))
        .collect();
    assert_eq!(outside.len(), 3);
    assert!(outside.iter().any(|s| s.as_str() == "$"));
}

#[test]
fn generate_test_inputs_handles_epsilon_automaton() {
    // q0 --eps--> q1 --a--> q2 (финальное): кратчайший путь до q2 — "a", а не "$a"
    let fa = FA::builder()
        .state(0)
        .state(1)
        .state(2)
        .set_initial(0)
        .set_final(2)
        .symbol('a')
        .epsilon(0, 1)
        .transition(1, 'a', 2)
        .build()
        .unwrap();

    let inputs = fa.generate_test_inputs(50, 15);
    assert!(inputs.contains(&"a".to_string()));
    assert!(inputs.contains(&"".to_string()));
    // `$` присутствует только как отдельный негативный вход, а не внутри путей
    assert!(inputs.iter().filter(|s| s.contains('$')).all(|s| s == "$"));
}

fn make_even_a_fa() -> FA {
    // Детерминированный автомат: принимает строки с чётным количеством 'a'
    // q0 (начальное, финальное) --a--> q1, q0 --b--> q0
    // q1 --a--> q0, q1 --b--> q1
    FA::builder()
        .state(0)
        .state(1)
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
fn builder_creates_valid_deterministic_fa() {
    let fa = make_even_a_fa();
    assert_eq!(fa.states().len(), 2);
    assert_eq!(fa.initial_state(), &0);
    assert!(fa.final_states().contains(&0));
    assert!(fa.is_deterministic());
}

#[test]
fn is_deterministic_detects_nondeterministic_cases() {
    let with_epsilon = FA::builder()
        .state(0)
        .set_initial(0)
        .symbol('a')
        .epsilon(0, 1)
        .build()
        .unwrap();
    assert!(!with_epsilon.is_deterministic());

    let branching = FA::builder()
        .state(0)
        .state(1)
        .state(2)
        .set_initial(0)
        .symbol('a')
        .transition(0, 'a', 1)
        .transition(0, 'a', 2)
        .build()
        .unwrap();
    assert!(!branching.is_deterministic());
}

#[test]
fn accepts_even_a() {
    let fa = make_even_a_fa();

    assert!(fa.accepts(&[])); // 0 'a' — чётно
    assert!(fa.accepts(&['b'])); // 0 'a' — чётно
    assert!(fa.accepts(&['b', 'b'])); // 0 'a' — чётно
    assert!(!fa.accepts(&['a'])); // 1 'a' — нечётно
    assert!(fa.accepts(&['a', 'a'])); // 2 'a' — чётно
    assert!(!fa.accepts(&['a', 'a', 'a'])); // 3 'a' — нечётно
    assert!(fa.accepts(&['a', 'b', 'a'])); // 2 'a' — чётно
    assert!(!fa.accepts(&['a', 'b', 'b'])); // 1 'a' — нечётно
    assert!(!fa.accepts(&['c']));
    assert!(!fa.accepts(&['a', 'c']));
}

#[test]
fn next_states_single_target_for_deterministic_fa() {
    let fa = make_even_a_fa();

    assert_eq!(fa.next_states(&0, &'a'), HashSet::from([&1]));
    assert_eq!(fa.next_states(&0, &'b'), HashSet::from([&0]));
    assert_eq!(fa.next_states(&1, &'a'), HashSet::from([&0]));
    assert_eq!(fa.next_states(&1, &'b'), HashSet::from([&1]));
}

#[test]
fn empty_string_rejected_when_initial_not_final() {
    let fa = FA::builder()
        .state(0)
        .state(1)
        .set_initial(0)
        .set_final(1)
        .symbol('a')
        .transition(0, 'a', 1)
        .transition(1, 'a', 0)
        .build()
        .unwrap();

    assert!(!fa.accepts(&[]));
}

#[test]
fn single_state_loop() {
    // q0 --a--> q0, допускающее: принимает a*
    let fa = FA::builder()
        .state(0)
        .set_initial(0)
        .set_final(0)
        .symbol('a')
        .transition(0, 'a', 0)
        .build()
        .unwrap();

    assert!(fa.accepts(&[]));
    assert!(fa.accepts(&['a']));
    assert!(fa.accepts(&['a', 'a']));
    assert!(fa.accepts(&['a', 'a', 'a']));
    assert!(!fa.accepts(&['b']));
}

#[test]
fn accepts_only_specific_string() {
    // Автомат, принимающий только "ab"
    // q0 --a--> q1 --b--> q2 (финальное)
    // q0 --b--> тупик, q1 --a--> тупик
    let fa = FA::builder()
        .state(0)
        .state(1)
        .state(2)
        .state(3)
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

    assert!(fa.accepts(&['a', 'b']));
    assert!(!fa.accepts(&[]));
    assert!(!fa.accepts(&['a']));
    assert!(!fa.accepts(&['a', 'b', 'a']));
    assert!(!fa.accepts(&['b']));
    assert!(!fa.accepts(&['b', 'a']));
}

#[test]
fn reachable_states_all_connected() {
    let fa = make_even_a_fa();
    let reachable = fa.reachable_states();
    assert_eq!(reachable.len(), 2);
    assert!(reachable.contains(&0));
    assert!(reachable.contains(&1));
}

#[test]
fn reachable_states_partial() {
    // q0 --a--> q1 (финальное), q2 недостижимо
    let fa = FA::builder()
        .state(0)
        .state(1)
        .state(2)
        .set_initial(0)
        .set_final(1)
        .symbol('a')
        .transition(0, 'a', 1)
        .transition(1, 'a', 1)
        .build()
        .unwrap();

    let reachable = fa.reachable_states();
    assert_eq!(reachable.len(), 2);
    assert!(reachable.contains(&0));
    assert!(reachable.contains(&1));
    assert!(!reachable.contains(&2));
}

#[test]
fn get_transitions_returns_correct_data() {
    let fa = make_even_a_fa();
    let t = fa.get_transitions();
    assert_eq!(t.len(), 4);
    assert_eq!(t[&(0, 'a')], HashSet::from([1]));
    assert_eq!(t[&(0, 'b')], HashSet::from([0]));
    assert_eq!(t[&(1, 'a')], HashSet::from([0]));
    assert_eq!(t[&(1, 'b')], HashSet::from([1]));
}

#[test]
fn run_returns_trace_for_accepted_string() {
    let fa = make_even_a_fa();

    let trace = fa.run(&['a', 'a']).unwrap();
    assert_eq!(trace.len(), 2);
    assert_eq!(
        trace[0],
        RunStep {
            from: 0,
            symbol: 'a',
            to: 1
        }
    );
    assert_eq!(
        trace[1],
        RunStep {
            from: 1,
            symbol: 'a',
            to: 0
        }
    );
}

#[test]
fn run_rejects_string_without_path() {
    let fa = make_even_a_fa();

    assert!(fa.run(&['a']).is_none());
    assert!(fa.run(&['a', 'a', 'a']).is_none());
    assert!(fa.run(&['c']).is_none());
}

#[test]
fn generate_test_inputs_handles_unicode_loop_symbols() {
    // Многобайтовые символы не должны ломать разрезание строки и повторы петли.
    let fa = FA::builder()
        .set_initial(0)
        .set_final(0)
        .symbol('я')
        .transition(0, 'я', 0)
        .build()
        .unwrap();

    let inputs = fa.generate_test_inputs(50, 15);

    assert!(inputs.contains(&"я".to_string()));
    assert!(inputs.contains(&"яя".to_string()));
    assert!(inputs.contains(&"яяя".to_string()));
}

#[test]
fn fa_generate_inputs_command_path_keeps_loop_strings() {
    // Путь, который использует frontend: fa_generate_inputs -> core helper.
    let store = AutomatonStore::new();
    let entry = store.insert(AutomatonData {
        id: 0,
        name: "Loop".to_string(),
        automatonType: AutomatonType::FA,
        states: vec![StateData {
            id: 0,
            label: "q0".to_string(),
            x: 0.0,
            y: 0.0,
            isInitial: true,
            isFinal: Some(true),
        }],
        transitions: vec![TransitionData {
            id: 1,
            from: 0,
            to: 0,
            symbol: 'я',
            output: None,
        }],
    });

    let result = gen_multi_run_for_output(&store, entry.id);

    assert_eq!(result.status, tauri::http::StatusCode::OK);
    assert!(result.inputs.contains(&"я".to_string()));
    assert!(result.inputs.contains(&"яя".to_string()));
    assert!(result.inputs.contains(&"яяя".to_string()));
}

#[test]
fn generate_test_inputs_keeps_cycles_when_cap_is_exceeded() {
    let mut builder = FA::builder()
        .set_initial(0)
        .set_final(0)
        .symbol('a')
        .transition(0, 'a', 0);
    for i in 1..12 {
        let ch = char::from(b'a' + i as u8);
        builder = builder.symbol(ch).transition(0, ch, i).set_final(i);
    }
    let fa = builder.build().unwrap();

    let inputs = fa.generate_test_inputs(50, 15);

    assert_eq!(inputs.len(), 50);
    assert!(inputs.contains(&"aa".to_string()));
    assert!(inputs.contains(&"aaa".to_string()));
    assert!(
        inputs
            .windows(2)
            .all(|w| w[0].chars().count() <= w[1].chars().count())
    );
}
