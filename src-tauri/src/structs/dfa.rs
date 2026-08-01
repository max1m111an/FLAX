use std::collections::{HashMap, HashSet};

use crate::structs::automata::{Automaton, DeterministicAutomaton};
use crate::structs::data_models::RunStep;

#[derive(Debug, Clone)]
pub struct DFA {
    states: HashSet<i32>,
    alphabet: HashSet<char>,
    transitions: HashMap<(i32, char), i32>,
    initial_state: i32,
    accepting_states: HashSet<i32>,
}

impl DFA {
    pub fn new(
        states: HashSet<i32>,
        alphabet: HashSet<char>,
        transitions: HashMap<(i32, char), i32>,
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
        for ((from, symbol), to) in &transitions {
            if !states.contains(from) {
                return Err(format!(
                    "Исходное состояние '{}' перехода не найдено в множестве состояний",
                    from
                ));
            }
            if !alphabet.contains(symbol) {
                return Err(format!("Символ '{}' не принадлежит алфавиту", symbol));
            }
            if !states.contains(to) {
                return Err(format!(
                    "Целевое состояние '{}' перехода не найдено в множестве состояний",
                    to
                ));
            }
        }
        Ok(DFA {
            states,
            alphabet,
            transitions,
            initial_state,
            accepting_states,
        })
    }

    pub fn builder() -> DFABuilder {
        DFABuilder::new()
    }

    pub fn get_transitions(&self) -> &HashMap<(i32, char), i32> {
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
            for symbol in &self.alphabet {
                if let Some(&next) = self.transitions.get(&(current, *symbol)) {
                    if !reachable.contains(&next) {
                        stack.push(next);
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

    pub fn is_valid(&self) -> Result<(), String> {
        for state in &self.states {
            for symbol in &self.alphabet {
                if !self.transitions.contains_key(&(*state, *symbol)) {
                    return Err(format!(
                        "Отсутствует переход из состояния '{}' по символу '{}'",
                        state, symbol
                    ));
                }
            }
        }
        Ok(())
    }

    pub fn run(&self, input: &[char]) -> Option<Vec<RunStep>> {
        if input.iter().any(|s| !self.alphabet.contains(s)) {
            return None;
        }

        let mut current = self.initial_state;
        let mut steps: Vec<RunStep> = Vec::new();
        for &symbol in input {
            match self.transitions.get(&(current, symbol)) {
                Some(&next) => {
                    steps.push(RunStep {
                        from: current,
                        symbol: symbol.to_string(),
                        to: next,
                    });
                    current = next;
                }
                None => return None,
            }
        }

        if self.accepting_states.contains(&current) {
            Some(steps)
        } else {
            None
        }
    }
}

impl Automaton for DFA {
    type State = i32;
    type Symbol = char;

    fn accepts(&self, input: &[char]) -> bool {
        let mut current = self.initial_state;
        for &symbol in input {
            if !self.alphabet.contains(&symbol) {
                return false;
            }
            match self.transitions.get(&(current, symbol)) {
                Some(&next) => current = next,
                None => return false,
            }
        }
        self.accepting_states.contains(&current)
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

impl DeterministicAutomaton for DFA {
    fn next_state(&self, state: &i32, symbol: &char) -> Option<&i32> {
        self.transitions.get(&(*state, *symbol))
    }
}

#[derive(Debug, Clone, Default)]
pub struct DFABuilder {
    states: HashSet<i32>,
    alphabet: HashSet<char>,
    transitions: HashMap<(i32, char), i32>,
    initial_state: Option<i32>,
    accepting_states: HashSet<i32>,
}

impl DFABuilder {
    pub fn new() -> Self {
        DFABuilder::default()
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
        self.transitions.insert((from, symbol), to);
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

    pub fn build(self) -> Result<DFA, String> {
        let initial_state = self
            .initial_state
            .ok_or("Не указано начальное состояние")?;
        DFA::new(
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
    use crate::structs::automata::{Automaton, DeterministicAutomaton};

    fn make_even_a_dfa() -> DFA {
        // DFA: принимает строки с чётным количеством 'a'
        // q0 (initial, final) --a--> q1, q0 --b--> q0
        // q1 --a--> q0, q1 --b--> q1
        DFABuilder::new()
            .state(0).state(1)
            .initial(0)
            .accepting(0)
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
        assert!(dfa.accepting_states().contains(&0));
    }

    #[test]
    fn builder_fails_without_initial() {
        let result = DFABuilder::new()
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
            .initial(0)
            .accepting(1)
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
            .initial(0)
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
            .initial(0)
            .accepting(0)
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
            .initial(0)
            .accepting(1)
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
            .initial(0)
            .accepting(0)
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
            .initial(0)
            .accepting(2)
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
            .initial(0)
            .accepting(1)
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
            RunStep { from: 0, symbol: "a".to_string(), to: 1 }
        );
        assert_eq!(
            trace[1],
            RunStep { from: 1, symbol: "a".to_string(), to: 0 }
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
}
