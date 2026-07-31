use std::{collections::HashSet, fmt::Debug, hash::Hash};

pub trait Automaton {
    type State: Clone + Eq + Hash + Debug;
    type Symbol: Clone + Eq + Hash + Debug;

    fn accepts(&self, input: &[Self::Symbol]) -> bool;
    fn states(&self) -> &HashSet<Self::State>;
    fn initial_state(&self) -> &Self::State;
    fn accepting_states(&self) -> &HashSet<Self::State>;
    fn alphabet(&self) -> &HashSet<Self::Symbol>;

    fn is_accepting(&self, state: &Self::State) -> bool {
        self.accepting_states().contains(state)
    }
}

pub trait DeterministicAutomaton: Automaton {
    fn next_state(&self, state: &Self::State, symbol: &Self::Symbol) -> Option<&Self::State>;
}

pub trait NondeterministicAutomaton: Automaton {
    fn next_states(&self, state: &Self::State, symbol: &Self::Symbol) -> HashSet<&Self::State>;
    fn epsilon_closure(&self, state: &Self::State) -> HashSet<&Self::State>;
}
