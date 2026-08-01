use std::collections::{HashMap, HashSet};

use crate::structs::automata::{Automaton, NondeterministicAutomaton};
use crate::structs::data_models::RunStep;

pub const EPSILON: char = '$';

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct NFA {
    states: HashSet<i32>,
    alphabet: HashSet<char>,
    transitions: HashMap<(i32, char), HashSet<i32>>,
    initial_state: i32,
    final_states: HashSet<i32>,
}

#[allow(dead_code)]
impl NFA {
    pub fn new(
        states: HashSet<i32>,
        alphabet: HashSet<char>,
        transitions: HashMap<(i32, char), HashSet<i32>>,
        initial_state: i32,
        final_states: HashSet<i32>,
    ) -> Result<Self, String> {
        if !states.contains(&initial_state) {
            return Err(format!(
                "Начальное состояние '{}' не найдено в множестве состояний",
                initial_state
            ));
        }
        for state in &final_states {
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
            final_states,
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
        if self.final_states.is_empty() {
            return true;
        }
        let reachable = self.reachable_states();
        !reachable.iter().any(|s| self.final_states.contains(s))
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
                if nfa.final_states.contains(&state) {
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
        current_states.iter().any(|s| self.final_states.contains(s))
    }

    fn states(&self) -> &HashSet<i32> {
        &self.states
    }

    fn initial_state(&self) -> &i32 {
        &self.initial_state
    }

    fn final_states(&self) -> &HashSet<i32> {
        &self.final_states
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
    final_states: HashSet<i32>,
}

#[allow(dead_code)]
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

    pub fn set_initial(mut self, state: i32) -> Self {
        self.states.insert(state);
        self.initial_state = Some(state);
        self
    }

    pub fn set_final(mut self, state: i32) -> Self {
        self.states.insert(state);
        self.final_states.insert(state);
        self
    }

    pub fn final_states(mut self, states: &[i32]) -> Self {
        for &state in states {
            self.states.insert(state);
            self.final_states.insert(state);
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
            self.final_states,
        )
    }
}
