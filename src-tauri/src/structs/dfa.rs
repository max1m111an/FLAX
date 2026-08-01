use std::collections::{HashMap, HashSet};

use crate::structs::automata::{Automaton, DeterministicAutomaton};
use crate::structs::data_models::RunStep;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DFA {
    states: HashSet<i32>,
    alphabet: HashSet<char>,
    transitions: HashMap<(i32, char), i32>,
    initial_state: i32,
    final_states: HashSet<i32>,
}

#[allow(dead_code)]
impl DFA {
    pub fn new(
        states: HashSet<i32>,
        alphabet: HashSet<char>,
        transitions: HashMap<(i32, char), i32>,
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
            final_states,
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
        if self.final_states.is_empty() {
            return true;
        }
        let reachable = self.reachable_states();
        !reachable.iter().any(|s| self.final_states.contains(s))
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

        if self.final_states.contains(&current) {
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
        self.final_states.contains(&current)
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
    final_states: HashSet<i32>,
}

#[allow(dead_code)]
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

    pub fn build(self) -> Result<DFA, String> {
        let initial_state = self
            .initial_state
            .ok_or("Не указано начальное состояние")?;
        DFA::new(
            self.states,
            self.alphabet,
            self.transitions,
            initial_state,
            self.final_states,
        )
    }
}

#[cfg(test)]
#[path = "dfa_tests.rs"]
mod tests;
