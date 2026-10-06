//! Mealy machine: a deterministic transducer that emits an output symbol on
//! every transition.
//!
//! The output belongs to the transition (not to the state), so the machine has
//! neither accepting states nor epsilon transitions: it is defined by
//! `(states, input alphabet, output alphabet, transitions, initial state)` and
//! is deterministic — for every `(state, input)` pair there is at most one
//! transition.

use std::collections::{HashMap, HashSet};

/// One Mealy transition: in state `from`, reading `input`, move to `to` and
/// emit `output`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MealyTransition {
    pub from: i32,
    pub input: char,
    pub to: i32,
    pub output: char,
}

#[allow(dead_code)]
impl MealyTransition {
    pub fn new(from: i32, input: char, to: i32, output: char) -> Self {
        MealyTransition {
            from,
            input,
            to,
            output,
        }
    }
}

/// Mealy machine. Fields are private; use the accessors or `MMBuilder`.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MM {
    states: HashSet<i32>,
    input_alphabet: HashSet<char>,
    output_alphabet: HashSet<char>,
    /// `((from, input) -> (to, output))`. The key is the determinism invariant:
    /// a single entry per state/symbol pair.
    transitions: HashMap<(i32, char), (i32, char)>,
    initial_state: i32,
}

#[allow(dead_code)]
impl MM {
    /// Validates and builds a machine. `transitions` is taken as a list so that
    /// conflicting `(from, input)` pairs are reported instead of being silently
    /// overwritten by `HashMap::insert`.
    pub fn new(
        states: HashSet<i32>,
        input_alphabet: HashSet<char>,
        output_alphabet: HashSet<char>,
        transitions: Vec<MealyTransition>,
        initial_state: i32,
    ) -> Result<Self, String> {
        if !states.contains(&initial_state) {
            return Err(format!(
                "Начальное состояние '{}' не найдено в множестве состояний",
                initial_state
            ));
        }

        let mut map: HashMap<(i32, char), (i32, char)> = HashMap::new();
        for t in &transitions {
            if !states.contains(&t.from) {
                return Err(format!(
                    "Исходное состояние '{}' перехода не найдено в множестве состояний",
                    t.from
                ));
            }
            if !states.contains(&t.to) {
                return Err(format!(
                    "Целевое состояние '{}' перехода не найдено в множестве состояний",
                    t.to
                ));
            }
            if !input_alphabet.contains(&t.input) {
                return Err(format!(
                    "Входной символ '{}' не принадлежит входному алфавиту",
                    t.input
                ));
            }
            if !output_alphabet.contains(&t.output) {
                return Err(format!(
                    "Выходной символ '{}' не принадлежит выходному алфавиту",
                    t.output
                ));
            }
            if map.insert((t.from, t.input), (t.to, t.output)).is_some() {
                return Err(format!(
                    "Повторяющийся переход из состояния '{}' по символу '{}' — машина Мили должна быть детерминированной",
                    t.from, t.input
                ));
            }
        }

        Ok(MM {
            states,
            input_alphabet,
            output_alphabet,
            transitions: map,
            initial_state,
        })
    }

    pub fn builder() -> MMBuilder {
        MMBuilder::new()
    }

    pub fn states(&self) -> &HashSet<i32> {
        &self.states
    }

    pub fn input_alphabet(&self) -> &HashSet<char> {
        &self.input_alphabet
    }

    pub fn output_alphabet(&self) -> &HashSet<char> {
        &self.output_alphabet
    }

    pub fn initial_state(&self) -> i32 {
        self.initial_state
    }

    pub fn transitions(&self) -> &HashMap<(i32, char), (i32, char)> {
        &self.transitions
    }

    /// Target state and emitted output for one reading step; `None` when the
    /// machine has no transition for that `(state, input)` pair.
    pub fn step(&self, from: i32, input: char) -> Option<(i32, char)> {
        self.transitions.get(&(from, input)).copied()
    }
}

/// Fluent builder for [`MM`], mirroring `FABuilder`.
///
/// `transition` registers its states and symbols automatically, so a machine
/// can be described by its initial state and transitions alone.
#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct MMBuilder {
    states: HashSet<i32>,
    input_alphabet: HashSet<char>,
    output_alphabet: HashSet<char>,
    transitions: Vec<MealyTransition>,
    initial_state: Option<i32>,
}

#[allow(dead_code)]
impl MMBuilder {
    pub fn new() -> Self {
        MMBuilder::default()
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

    pub fn input(mut self, symbol: char) -> Self {
        self.input_alphabet.insert(symbol);
        self
    }

    pub fn inputs(mut self, symbols: &[char]) -> Self {
        for &symbol in symbols {
            self.input_alphabet.insert(symbol);
        }
        self
    }

    pub fn output(mut self, symbol: char) -> Self {
        self.output_alphabet.insert(symbol);
        self
    }

    pub fn outputs(mut self, symbols: &[char]) -> Self {
        for &symbol in symbols {
            self.output_alphabet.insert(symbol);
        }
        self
    }

    pub fn set_initial(mut self, state: i32) -> Self {
        self.states.insert(state);
        self.initial_state = Some(state);
        self
    }

    pub fn transition(mut self, from: i32, input: char, to: i32, output: char) -> Self {
        self.states.insert(from);
        self.states.insert(to);
        self.input_alphabet.insert(input);
        self.output_alphabet.insert(output);
        self.transitions.push(MealyTransition::new(from, input, to, output));
        self
    }

    pub fn build(self) -> Result<MM, String> {
        let initial_state = self.initial_state.ok_or("Не указано начальное состояние")?;
        MM::new(
            self.states,
            self.input_alphabet,
            self.output_alphabet,
            self.transitions,
            initial_state,
        )
    }
}
