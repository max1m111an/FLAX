//! Машина Мили: детерминированный преобразователь, выдающий выходной символ на
//! каждом переходе.
//!
//! Выход принадлежит переходу (а не состоянию), поэтому у машины нет ни
//! допускающих состояний, ни eps-переходов: она задаётся набором
//! `(состояния, входной алфавит, выходной алфавит, переходы, начальное состояние)`.
//! Входная и выходная метки перехода читаются/выдаются целиком, как в JFLAP:
//! переход с меткой `123` поглощает три символа за один шаг. При нескольких
//! совпадающих метках используется самая длинная, поэтому машина остаётся
//! детерминированной.

use std::collections::{HashMap, HashSet};

use crate::core::types::{MealyRunStep, match_label};

/// Один переход машины Мили: в состоянии `from`, прочитав `input`, перейти в
/// `to` и выдать `output`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MealyTransition {
    pub from: i32,
    pub input: String,
    pub to: i32,
    pub output: String,
}

#[allow(dead_code)]
impl MealyTransition {
    pub fn new(from: i32, input: String, to: i32, output: String) -> Self {
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
    input_alphabet: HashSet<String>,
    output_alphabet: HashSet<String>,
    /// `((from, input) -> (to, output))`. Ключ обеспечивает инвариант детерминизма:
    /// одна запись на пару состояние/символ.
    transitions: HashMap<(i32, String), (i32, String)>,
    initial_state: i32,
}

#[allow(dead_code)]
impl MM {
    /// Проверяет данные и строит машину. `transitions` принимается списком, чтобы
    /// конфликтующие пары `(from, input)` выдавались как ошибка, а не молча
    /// перезаписывались через `HashMap::insert`.
    pub fn new(
        states: HashSet<i32>,
        input_alphabet: HashSet<String>,
        output_alphabet: HashSet<String>,
        transitions: Vec<MealyTransition>,
        initial_state: i32,
    ) -> Result<Self, String> {
        if !states.contains(&initial_state) {
            return Err(format!(
                "Начальное состояние '{}' не найдено в множестве состояний",
                initial_state
            ));
        }

        let mut map: HashMap<(i32, String), (i32, String)> = HashMap::new();
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
            if map
                .insert((t.from, t.input.clone()), (t.to, t.output.clone()))
                .is_some()
            {
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

    pub fn input_alphabet(&self) -> &HashSet<String> {
        &self.input_alphabet
    }

    pub fn output_alphabet(&self) -> &HashSet<String> {
        &self.output_alphabet
    }

    pub fn initial_state(&self) -> i32 {
        self.initial_state
    }

    pub fn transitions(&self) -> &HashMap<(i32, String), (i32, String)> {
        &self.transitions
    }

    /// Целевое состояние и выходной символ для одного шага чтения; `None`, если у
    /// машины нет перехода с точно такой входной меткой.
    pub fn step(&self, from: i32, input: &str) -> Option<(i32, String)> {
        self.transitions.get(&(from, input.to_string())).cloned()
    }

    /// Ищет из `from` переход, чья входная метка целиком совпадает с фрагментом
    /// входа начиная с `pos`. При нескольких совпадениях берётся самая длинная
    /// метка, поэтому машина остаётся детерминированной. Возвращает метку,
    /// целевое состояние, выход и позицию после прочитанного фрагмента.
    pub fn step_at(
        &self,
        from: i32,
        input: &[String],
        pos: usize,
    ) -> Option<(String, i32, String, usize)> {
        let mut best: Option<(usize, &String, &i32, &String)> = None;
        for ((state, label), (to, out)) in &self.transitions {
            if *state != from {
                continue;
            }
            let Some(next_pos) = match_label(input, pos, label) else {
                continue;
            };
            let len = next_pos - pos;
            if best
                .as_ref()
                .is_none_or(|(best_len, _, _, _)| len > *best_len)
            {
                best = Some((len, label, to, out));
            }
        }
        best.map(|(len, label, to, out)| (label.clone(), *to, out.clone(), pos + len))
    }

    /// Читает `input` из начального состояния: каждый переход поглощает свою
    /// входную метку целиком и выдаёт выходную строку целиком. Останавливается,
    /// как только в текущей позиции нет подходящего перехода (частичный прогон).
    /// Возвращает трассу и полученную выходную строку.
    pub fn run(&self, input: &[String]) -> (Vec<MealyRunStep>, String) {
        let mut steps: Vec<MealyRunStep> = Vec::new();
        let mut output = String::new();
        let mut state = self.initial_state;
        let mut pos = 0;
        while pos < input.len() {
            let Some((label, to, out, next_pos)) = self.step_at(state, input, pos) else {
                break;
            };
            steps.push(MealyRunStep {
                from: state,
                symbol: label,
                to,
                output: out.clone(),
            });
            output.push_str(&out);
            state = to;
            pos = next_pos;
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
    input_alphabet: HashSet<String>,
    output_alphabet: HashSet<String>,
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

    pub fn input(mut self, symbol: String) -> Self {
        self.input_alphabet.insert(symbol);
        self
    }

    pub fn inputs(mut self, symbols: &[String]) -> Self {
        for symbol in symbols {
            self.input_alphabet.insert(symbol.clone());
        }
        self
    }

    pub fn output(mut self, symbol: String) -> Self {
        self.output_alphabet.insert(symbol);
        self
    }

    pub fn outputs(mut self, symbols: &[String]) -> Self {
        for symbol in symbols {
            self.output_alphabet.insert(symbol.clone());
        }
        self
    }

    pub fn set_initial(mut self, state: i32) -> Self {
        self.states.insert(state);
        self.initial_state = Some(state);
        self
    }

    pub fn transition(mut self, from: i32, input: String, to: i32, output: String) -> Self {
        self.states.insert(from);
        self.states.insert(to);
        let transition = MealyTransition::new(from, input, to, output);
        self.input_alphabet.insert(transition.input.clone());
        self.output_alphabet.insert(transition.output.clone());
        self.transitions.push(transition);
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
