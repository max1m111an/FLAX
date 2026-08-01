use std::collections::{HashMap, HashSet};

use crate::structs::automata::{Automaton, NondeterministicAutomaton};
use crate::structs::data_models::RunStep;

pub const EPSILON: char = '$';

#[derive(Debug, Clone)]
pub struct NFA {
    states: HashSet<i32>,
    alphabet: HashSet<char>,
    transitions: HashMap<(i32, char), HashSet<i32>>,
    initial_state: i32,
    accepting_states: HashSet<i32>,
}

impl NFA {
    pub fn new(
        states: HashSet<i32>,
        alphabet: HashSet<char>,
        transitions: HashMap<(i32, char), HashSet<i32>>,
        initial_state: i32,
        accepting_states: HashSet<i32>,
    ) -> Result<Self, String> {
        if !states.contains(&initial_state) {
            return Err(format!(
                "Начальное состояние '{}' не найдено в множестве состояний",
                initial_state
            ));
        }
        for state in &accepting_states {
            if !states.contains(state) {
                return Err(format!(
                    "Допускающее состояние '{}' не найдено в множестве состояний",
                    state
                ));
            }
        }
        for ((from, symbol), to_states) in &transitions {
            if !states.contains(from) {
                return Err(format!(
                    "Исходное состояние '{}' перехода не найдено в множестве состояний",
                    from
                ));
            }
            if *symbol != EPSILON && !alphabet.contains(symbol) {
                return Err(format!("Символ '{}' не принадлежит алфавиту", symbol));
            }
            for to in to_states {
                if !states.contains(to) {
                    return Err(format!(
                        "Целевое состояние '{}' перехода не найдено в множестве состояний",
                        to
                    ));
                }
            }
        }
        Ok(NFA {
            states,
            alphabet,
            transitions,
            initial_state,
            accepting_states,
        })
    }

    pub fn builder() -> NFABuilder {
        NFABuilder::new()
    }

    pub fn get_transitions(&self) -> &HashMap<(i32, char), HashSet<i32>> {
        &self.transitions
    }

    pub fn reachable_states(&self) -> HashSet<i32> {
        let mut reachable: HashSet<i32> = HashSet::new();
        let mut stack: Vec<i32> = vec![self.initial_state];
        while let Some(current) = stack.pop() {
            if reachable.contains(&current) {
                continue;
            }
            reachable.insert(current);
            for symbol in self.alphabet.iter().copied().chain(std::iter::once(EPSILON))
            {
                if let Some(next_states) = self.transitions.get(&(current, symbol)) {
                    for next in next_states {
                        if !reachable.contains(next) {
                            stack.push(*next);
                        }
                    }
                }
            }
        }
        reachable
    }

    pub fn is_empty(&self) -> bool {
        if self.accepting_states.is_empty() {
            return true;
        }
        let reachable = self.reachable_states();
        !reachable.iter().any(|s| self.accepting_states.contains(s))
    }

    pub fn run(&self, input: &[char]) -> Option<Vec<RunStep>> {
        if input.iter().any(|s| !self.alphabet.contains(s)) {
            return None;
        }

        let mut steps: Vec<RunStep> = Vec::new();
        let mut visited: HashSet<(i32, usize)> = HashSet::new();

        fn explore(
            nfa: &NFA,
            state: i32,
            pos: usize,
            input: &[char],
            steps: &mut Vec<RunStep>,
            visited: &mut HashSet<(i32, usize)>,
        ) -> bool {
            if !visited.insert((state, pos)) {
                return false;
            }

            if pos == input.len() {
                if nfa.accepting_states.contains(&state) {
                    return true;
                }
                if let Some(next_states) = nfa.transitions.get(&(state, EPSILON)) {
                    for &next in next_states {
                        steps.push(RunStep {
                            from: state,
                            symbol: EPSILON.to_string(),
                            to: next,
                        });
                        if explore(nfa, next, pos, input, steps, visited) {
                            return true;
                        }
                        steps.pop();
                    }
                }
                return false;
            }

            if let Some(next_states) = nfa.transitions.get(&(state, EPSILON)) {
                for &next in next_states {
                    steps.push(RunStep {
                        from: state,
                        symbol: EPSILON.to_string(),
                        to: next,
                    });
                    if explore(nfa, next, pos, input, steps, visited) {
                        return true;
                    }
                    steps.pop();
                }
            }

            let symbol = input[pos];
            if let Some(next_states) = nfa.transitions.get(&(state, symbol)) {
                for &next in next_states {
                    steps.push(RunStep {
                        from: state,
                        symbol: symbol.to_string(),
                        to: next,
                    });
                    if explore(nfa, next, pos + 1, input, steps, visited) {
                        return true;
                    }
                    steps.pop();
                }
            }

            false
        }

        if explore(self, self.initial_state, 0, input, &mut steps, &mut visited) {
            Some(steps)
        } else {
            None
        }
    }

    fn epsilon_closure_owned(&self, state: i32) -> HashSet<i32> {
        let mut closure: HashSet<i32> = HashSet::new();
        let mut stack: Vec<i32> = vec![state];
        while let Some(current) = stack.pop() {
            if closure.contains(&current) {
                continue;
            }
            closure.insert(current);
            if let Some(next_states) = self.transitions.get(&(current, EPSILON)) {
                for next in next_states {
                    if !closure.contains(next) {
                        stack.push(*next);
                    }
                }
            }
        }
        closure
    }

    fn next_states_owned(&self, state: i32, symbol: char) -> HashSet<i32> {
        self.transitions
            .get(&(state, symbol))
            .cloned()
            .unwrap_or_default()
    }

    fn next_states_with_epsilon_owned(&self, state: i32, symbol: char) -> HashSet<i32> {
        let direct = self.next_states_owned(state, symbol);
        let mut result: HashSet<i32> = HashSet::new();
        for s in direct {
            result.extend(self.epsilon_closure_owned(s));
        }
        result
    }
}

impl Automaton for NFA {
    type State = i32;
    type Symbol = char;

    fn accepts(&self, input: &[char]) -> bool {
        let mut current_states: HashSet<i32> = self.epsilon_closure_owned(self.initial_state);
        for &symbol in input {
            if !self.alphabet.contains(&symbol) {
                return false;
            }
            let mut next_states: HashSet<i32> = HashSet::new();
            for state in current_states {
                next_states.extend(self.next_states_with_epsilon_owned(state, symbol));
            }
            current_states = next_states;
            if current_states.is_empty() {
                return false;
            }
        }
        current_states.iter().any(|s| self.accepting_states.contains(s))
    }

    fn states(&self) -> &HashSet<i32> {
        &self.states
    }

    fn initial_state(&self) -> &i32 {
        &self.initial_state
    }

    fn accepting_states(&self) -> &HashSet<i32> {
        &self.accepting_states
    }

    fn alphabet(&self) -> &HashSet<char> {
        &self.alphabet
    }
}

impl NondeterministicAutomaton for NFA {
    fn epsilon_closure(&self, state: &i32) -> HashSet<&i32> {
        self.epsilon_closure_owned(*state)
            .into_iter()
            .collect::<HashSet<_>>()
            .into_iter()
            .map(|s| self.states.get(&s).unwrap())
            .collect()
    }

    fn next_states(&self, state: &i32, symbol: &char) -> HashSet<&i32> {
        self.transitions
            .get(&(*state, *symbol))
            .map(|states| states.iter().collect())
            .unwrap_or_default()
    }
}

#[derive(Debug, Clone, Default)]
pub struct NFABuilder {
    states: HashSet<i32>,
    alphabet: HashSet<char>,
    transitions: HashMap<(i32, char), HashSet<i32>>,
    initial_state: Option<i32>,
    accepting_states: HashSet<i32>,
}

impl NFABuilder {
    pub fn new() -> Self {
        NFABuilder::default()
    }

    pub fn state(mut self, state: i32) -> Self {
        self.states.insert(state);
        self
    }

    pub fn states(mut self, states: &[i32]) -> Self {
        for &state in states {
            self.states.insert(state);
        }
        self
    }

    pub fn symbol(mut self, symbol: char) -> Self {
        self.alphabet.insert(symbol);
        self
    }

    pub fn symbols(mut self, symbols: &[char]) -> Self {
        for &symbol in symbols {
            self.alphabet.insert(symbol);
        }
        self
    }

    pub fn transition(mut self, from: i32, symbol: char, to: i32) -> Self {
        self.states.insert(from);
        self.states.insert(to);
        self.alphabet.insert(symbol);
        self.transitions
            .entry((from, symbol))
            .or_default()
            .insert(to);
        self
    }

    pub fn epsilon(mut self, from: i32, to: i32) -> Self {
        self.states.insert(from);
        self.states.insert(to);
        self.transitions
            .entry((from, EPSILON))
            .or_default()
            .insert(to);
        self
    }

    pub fn initial(mut self, state: i32) -> Self {
        self.states.insert(state);
        self.initial_state = Some(state);
        self
    }

    pub fn accepting(mut self, state: i32) -> Self {
        self.states.insert(state);
        self.accepting_states.insert(state);
        self
    }

    pub fn accepting_states(mut self, states: &[i32]) -> Self {
        for &state in states {
            self.states.insert(state);
            self.accepting_states.insert(state);
        }
        self
    }

    pub fn build(self) -> Result<NFA, String> {
        let initial_state = self
            .initial_state
            .ok_or("Не указано начальное состояние")?;
        NFA::new(
            self.states,
            self.alphabet,
            self.transitions,
            initial_state,
            self.accepting_states,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::automata::{Automaton, NondeterministicAutomaton};

    #[test]
    fn builder_creates_valid_nfa() {
        let nfa = NFA::builder()
            .state(0).state(1).state(2)
            .initial(0)
            .accepting(2)
            .symbol('a').symbol('b')
            .transition(0, 'a', 1)
            .transition(1, 'b', 2)
            .build();
        assert!(nfa.is_ok());
        let nfa = nfa.unwrap();
        assert_eq!(nfa.states().len(), 3);
        assert_eq!(nfa.initial_state(), &0);
        assert!(nfa.accepting_states().contains(&2));
    }

    #[test]
    fn builder_fails_without_initial() {
        let result = NFA::builder()
            .state(0).state(1)
            .accepting(1)
            .symbol('a')
            .transition(0, 'a', 1)
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn new_fails_with_invalid_accepting() {
        let states = HashSet::from([0]);
        let accepting = HashSet::from([99]);
        let result = NFA::new(states, HashSet::new(), HashMap::new(), 0, accepting);
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
            .initial(0)
            .accepting(2)
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
            .initial(0)
            .accepting(2)
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
            .initial(0)
            .accepting(2)
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
            .initial(0)
            .accepting(3)
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
            .initial(0)
            .accepting(1)
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
            .initial(0)
            .accepting(0)
            .build()
            .unwrap();

        assert!(nfa.accepts(&[]));
        assert!(!nfa.accepts(&['a']));
    }

    #[test]
    fn epsilon_closure_includes_all_reachable() {
        let nfa = NFA::builder()
            .state(0).state(1).state(2).state(3)
            .initial(0)
            .accepting(3)
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
            .initial(0)
            .accepting(2)
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
            .initial(0)
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
            .initial(0)
            .accepting(1)
            .symbol('a')
            .build()
            .unwrap();

        assert!(nfa.is_empty());
    }

    #[test]
    fn is_not_empty_when_accepting_reachable() {
        let nfa = NFA::builder()
            .state(0).state(1)
            .initial(0)
            .accepting(1)
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
            .initial(0)
            .accepting(2)
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
            .initial(0)
            .accepting(2)
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
            .initial(0)
            .accepting(3)
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
            .initial(0)
            .accepting(2)
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
            .initial(0)
            .accepting(2)
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
            .initial(0)
            .accepting(2)
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
            .initial(0)
            .accepting(1)
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
            .initial(0)
            .accepting(0)
            .build()
            .unwrap();

        let trace = nfa.run(&[]).unwrap();
        assert!(trace.is_empty());
    }
}
