use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

use crate::structs::automata::{
    Automaton,
    NondeterministicAutomaton
};

/// Специальное значение для eps-перехода
pub const EPSILON: Option<char> = None;

/// Структура NFA (Недетерминированный конечный автомат)
#[derive(Debug, Clone)]
pub struct NFA {
    /// Множество состояний
    states: HashSet<String>,
    /// Алфавит (входные символы)
    alphabet: HashSet<char>,
    /// Функция переходов: (из состояния, символ) -> множество состояний
    /// Если символ = None, это eps-переход
    transitions: HashMap<(String, Option<char>), HashSet<String>>,
    /// Начальное состояние
    initial_state: String,
    /// Множество допускающих состояний
    accepting_states: HashSet<String>,
}

impl NFA {
    /// Создает новый NFA с валидацией
    pub fn new(
        states: HashSet<String>,
        alphabet: HashSet<char>,
        transitions: HashMap<(String, Option<char>), HashSet<String>>,
        initial_state: String,
        accepting_states: HashSet<String>,
    ) -> Result<Self, String> {
        // Проверка существования начального состояния
        if !states.contains(&initial_state) {
            return Err(format!(
                "Начальное состояние '{}' не найдено в множестве состояний",
                initial_state
            ));
        }

        // Проверка всех допускающих состояний
        for state in &accepting_states {
            if !states.contains(state) {
                return Err(format!(
                    "Допускающее состояние '{}' не найдено в множестве состояний",
                    state
                ));
            }
        }

        // Проверка всех переходов
        for ((from, symbol), to_states) in &transitions {
            // Проверка исходного состояния
            if !states.contains(from) {
                return Err(format!(
                    "Исходное состояние '{}' перехода не найдено в множестве состояний",
                    from
                ));
            }

            // Проверка символа (если не ε)
            if let Some(s) = symbol {
                if !alphabet.contains(s) {
                    return Err(format!(
                        "Символ '{}' не принадлежит алфавиту",
                        s
                    ));
                }
            }

            // Проверка всех целевых состояний
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

    /// Создает строитель для NFA
    pub fn builder() -> NFABuilder {
        NFABuilder::new()
    }

    /// Возвращает все достижимые состояния из начального
    pub fn reachable_states(&self) -> HashSet<String> {
        let mut reachable: HashSet<String> = HashSet::new();
        let mut stack: Vec<String> = vec![self.initial_state.clone()];
        
        while let Some(current) = stack.pop() {
            if reachable.contains(&current) {
                continue;
            }
            
            reachable.insert(current.clone());
            
            // Добавляем все состояния, достижимые по любым символам и eps
            for symbol in self.alphabet.iter().map(|&s| Some(s)).chain(std::iter::once(None)) {
                if let Some(next_states) = self.transitions.get(&(current.clone(), symbol)) {
                    for next in next_states {
                        if !reachable.contains(next) {
                            stack.push(next.clone());
                        }
                    }
                }
            }
        }
        
        reachable
    }

    /// Проверяет, пуст ли язык автомата (нет ли принимаемых строк)
    pub fn is_empty(&self) -> bool {
        // Если нет допускающих состояний - язык пуст
        if self.accepting_states.is_empty() {
            return true;
        }
        
        // Проверяем, достижимо ли хотя бы одно допускающее состояние
        let reachable: HashSet<String> = self.reachable_states();
        reachable.iter().any(|s| self.accepting_states.contains(s))
    }

    /// Внутренняя версия epsilon_closure, возвращающая HashSet<String>
    fn epsilon_closure_owned(&self, state: &str) -> HashSet<String> {
        let mut closure: HashSet<String> = HashSet::new();
        let mut stack: Vec<String> = vec![state.to_string()];
        
        while let Some(current) = stack.pop() {
            if closure.contains(&current) {
                continue;
            }
            
            closure.insert(current.clone());
            
            if let Some(next_states) = self.transitions.get(&(current, None)) {
                for next in next_states {
                    if !closure.contains(next) {
                        stack.push(next.clone());
                    }
                }
            }
        }
        
        closure
    }

    /// Внутренняя версия next_states, возвращающая HashSet<String>
    fn next_states_owned(&self, state: &str, symbol: char) -> HashSet<String> {
        self.transitions
            .get(&(state.to_string(), Some(symbol)))
            .cloned()
            .unwrap_or_else(HashSet::new)
    }

    /// Внутренняя версия next_states_with_epsilon
    fn next_states_with_epsilon_owned(&self, state: &str, symbol: char) -> HashSet<String> {
        let direct: HashSet<String> = self.next_states_owned(state, symbol);
        let mut result: HashSet<String> = HashSet::new();
        
        for s in direct {
            result.extend(self.epsilon_closure_owned(&s));
        }
        
        result
    }
}

impl Automaton for NFA {
    type State = String;
    type Symbol = char;
    
    fn accepts(&self, input: &[Self::Symbol]) -> bool {
        // Начинаем с eps-замыкания начального состояния
        let mut current_states: HashSet<String> = self.epsilon_closure_owned(&self.initial_state);
        
        // Обрабатываем каждый символ
        for &symbol in input {
            if !self.alphabet.contains(&symbol) {
                return false; // Символ не из алфавита
            }
            
            // Вычисляем новые состояния
            let mut next_states: HashSet<String> = HashSet::new();
            for state in current_states {
                next_states.extend(self.next_states_with_epsilon_owned(&state, symbol));
            }
            current_states = next_states;
            
            // Если нет доступных состояний, строка не принимается
            if current_states.is_empty() {
                return false;
            }
        }
        
        // Проверяем, есть ли допускающее состояние среди текущих
        current_states.iter().any(|s| self.accepting_states.contains(s))
    }
    
    fn states(&self) -> &HashSet<Self::State> {
        &self.states
    }
    
    fn initial_state(&self) -> &Self::State {
        &self.initial_state
    }
    
    fn accepting_states(&self) -> &HashSet<Self::State> {
        &self.accepting_states
    }
    
    fn alphabet(&self) -> &HashSet<Self::Symbol> {
        &self.alphabet
    }
    
    fn get_transitions(&self) -> &HashMap<(String, Option<char>), HashSet<String>> {
        &self.transitions
    }
}

impl NondeterministicAutomaton for NFA {
    fn epsilon_closure(&self, state: &Self::State) -> HashSet<&String> {
        self.epsilon_closure_owned(state)
            .into_iter()
            .collect::<HashSet<_>>()
            .into_iter()
            .map(|s| self.states.get(&s).unwrap())
            .collect()
    }
    
    fn next_states(&self, state: &Self::State, symbol: &Self::Symbol) -> HashSet<&String> {
        self.transitions
            .get(&(state.clone(), Some(*symbol)))
            .map(|states| states.iter().collect())
            .unwrap_or_else(HashSet::new)
    }
}


#[derive(Debug, Clone, Default)]
pub struct NFABuilder {
    states: HashSet<String>,
    alphabet: HashSet<char>,
    transitions: HashMap<(String, Option<char>), HashSet<String>>,
    initial_state: Option<String>,
    accepting_states: HashSet<String>,
}

impl NFABuilder {
    pub fn new() -> Self {
        NFABuilder::default()
    }
    
    /// Добавляет состояние
    pub fn state(mut self, state: impl Into<String>) -> Self {
        self.states.insert(state.into());
        self
    }
    
    /// Добавляет несколько состояний
    pub fn states(mut self, states: Vec<impl Into<String>>) -> Self {
        for state in states {
            self.states.insert(state.into());
        }
        self
    }
    
    /// Добавляет символ в алфавит
    pub fn symbol(mut self, symbol: char) -> Self {
        self.alphabet.insert(symbol);
        self
    }
    
    /// Добавляет несколько символов
    pub fn symbols(mut self, symbols: Vec<char>) -> Self {
        for symbol in symbols {
            self.alphabet.insert(symbol);
        }
        self
    }
    
    /// Добавляет переход по символу
    pub fn transition(
        mut self,
        from: impl Into<String>,
        symbol: char,
        to: impl Into<String>,
    ) -> Self {
        let from: String = from.into();
        let to: String = to.into();
        
        self.states.insert(from.clone());
        self.states.insert(to.clone());
        self.alphabet.insert(symbol);
        
        self.transitions
            .entry((from, Some(symbol)))
            .or_insert_with(HashSet::new)
            .insert(to);
        
        self
    }
    
    /// Добавляет eps-переход
    pub fn epsilon(
        mut self,
        from: impl Into<String>,
        to: impl Into<String>,
    ) -> Self {
        let from: String = from.into();
        let to: String = to.into();
        
        self.states.insert(from.clone());
        self.states.insert(to.clone());
        
        self.transitions
            .entry((from, EPSILON))
            .or_insert_with(HashSet::new)
            .insert(to);
        
        self
    }
    
    /// Добавляет несколько переходов из одного состояния
    pub fn transitions(
        mut self,
        from: impl Into<String>,
        symbol: char,
        to_states: Vec<impl Into<String>>,
    ) -> Self {
        let from: String = from.into();
        
        self.states.insert(from.clone());
        self.alphabet.insert(symbol);
        
        let entry: &mut HashSet<String> = self.transitions
            .entry((from, Some(symbol)))
            .or_insert_with(HashSet::new);
        
        for to in to_states {
            let to: String = to.into();
            self.states.insert(to.clone());
            entry.insert(to);
        }
        
        self
    }
    
    /// Устанавливает начальное состояние
    pub fn initial(mut self, state: impl Into<String>) -> Self {
        let state: String = state.into();
        self.states.insert(state.clone());
        self.initial_state = Some(state);
        self
    }
    
    /// Добавляет допускающее состояние
    pub fn accepting(mut self, state: impl Into<String>) -> Self {
        let state: String = state.into();
        self.states.insert(state.clone());
        self.accepting_states.insert(state);
        self
    }
    
    /// Добавляет несколько допускающих состояний
    pub fn accepting_states(mut self, states: Vec<impl Into<String>>) -> Self {
        for state in states {
            let state: String = state.into();
            self.states.insert(state.clone());
            self.accepting_states.insert(state);
        }
        self
    }
    
    /// Создает NFA
    pub fn build(self) -> Result<NFA, String> {
        let initial_state: String = self.initial_state
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