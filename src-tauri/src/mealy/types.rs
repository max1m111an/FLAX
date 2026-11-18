//! Машина Мили: детерминированный преобразователь, выдающий выходной символ на
//! каждом переходе.
//!
//! Выход принадлежит переходу (а не состоянию), поэтому у машины нет ни
//! допускающих состояний, ни eps-переходов: она задаётся набором
//! `(состояния, входной алфавит, выходной алфавит, переходы, начальное состояние)`
//! и детерминирована — для каждой пары `(state, input)` есть не более одного
//! перехода.

use std::collections::{HashMap, HashSet};

use crate::core::types::MealyRunStep;

/// Один переход машины Мили: в состоянии `from`, прочитав `input`, перейти в
/// `to` и выдать `output`.
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

/// Машина Мили. Поля приватные; используйте методы доступа или `MealyBuilder`.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MM {
    states: HashSet<i32>,
    input_alphabet: HashSet<char>,
    output_alphabet: HashSet<char>,
    /// `((from, input) -> (to, output))`. Ключ обеспечивает инвариант детерминизма:
    /// одна запись на пару состояние/символ.
    transitions: HashMap<(i32, char), (i32, char)>,
    initial_state: i32,
}

#[allow(dead_code)]
impl MM {
    /// Проверяет данные и строит машину. `transitions` принимается списком, чтобы
    /// конфликтующие пары `(from, input)` выдавались как ошибка, а не молча
    /// перезаписывались через `HashMap::insert`.
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

    pub fn builder() -> MealyBuilder {
        MealyBuilder::new()
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

    /// Целевое состояние и выходной символ для одного шага чтения; `None`, если у
    /// машины нет перехода для этой пары `(state, input)`.
    pub fn step(&self, from: i32, input: char) -> Option<(i32, char)> {
        self.transitions.get(&(from, input)).copied()
    }

    /// Читает `input` из начального состояния, выдавая по одному выходному символу
    /// на переход. Останавливается, как только для пары `(state, input)` нет
    /// перехода (частичный прогон). Возвращает трассу и полученную выходную строку.
    pub fn run(&self, input: &[char]) -> (Vec<MealyRunStep>, String) {
        let mut steps: Vec<MealyRunStep> = Vec::new();
        let mut output = String::new();
        let mut state = self.initial_state;
        for &symbol in input {
            match self.step(state, symbol) {
                Some((to, out)) => {
                    steps.push(MealyRunStep {
                        from: state,
                        symbol,
                        to,
                        output: out,
                    });
                    output.push(out);
                    state = to;
                }
                None => break,
            }
        }
        (steps, output)
    }
}

/// Fluent-билдер для [`MM`], аналог `FABuilder`.
///
/// `transition` автоматически регистрирует свои состояния и символы, поэтому
/// машину можно описать только начальным состоянием и переходами.
#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct MealyBuilder {
    states: HashSet<i32>,
    input_alphabet: HashSet<char>,
    output_alphabet: HashSet<char>,
    transitions: Vec<MealyTransition>,
    initial_state: Option<i32>,
}

#[allow(dead_code)]
impl MealyBuilder {
    pub fn new() -> Self {
        MealyBuilder::default()
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
        self.transitions
            .push(MealyTransition::new(from, input, to, output));
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
