use std::{
    collections::{HashMap, HashSet}, fmt::Debug, hash::Hash
};
    
/// Общий трейт для всех конечных автоматов
pub trait Automaton {
    /// Тип состояния (обычно String, usize или enum)
    type State: Clone + Eq + Hash + Debug;
    
    /// Тип символа алфавита (обычно char)
    type Symbol: Clone + Eq + Hash + Debug;
    
    /// Проверяет, принимает ли автомат данную строку
    fn accepts(&self, input: &[Self::Symbol]) -> bool;
    
    /// Возвращает множество всех состояний
    fn states(&self) -> &HashSet<Self::State>;
    
    /// Возвращает начальное состояние
    fn initial_state(&self) -> &Self::State;
    
    /// Возвращает множество допускающих состояний
    fn accepting_states(&self) -> &HashSet<Self::State>;

    /// Возвращает множество переходов
    fn get_transitions(&self) -> &HashMap<(String, Option<char>), HashSet<String>>;
    
    /// Возвращает алфавит
    fn alphabet(&self) -> &HashSet<Self::Symbol>;
    
    /// Проверяет, является ли состояние допускающим
    fn is_accepting(&self, state: &Self::State) -> bool {
        self.accepting_states().contains(state)
    }
}

/// Трейт для детерминированных автоматов
pub trait DeterministicAutomaton: Automaton {
    /// Возвращает следующее состояние по символу (для DFA)
    fn next_state(&self, state: &Self::State, symbol: &Self::Symbol) -> Option<&Self::State>;
}

/// Трейт для недетерминированных автоматов
pub trait NondeterministicAutomaton: Automaton {
    /// Возвращает множество следующих состояний по символу (для NFA)
    fn next_states(&self, state: &Self::State, symbol: &Self::Symbol) -> HashSet<&Self::State>;
    
    /// Возвращает eps-замыкание состояния
    fn epsilon_closure(&self, state: &Self::State) -> HashSet<&Self::State>;
}