use super::*;

pub trait ClusterUpdater {
    type State: State;
    type Graph;
    fn get_state(&self, state:&Self::State, graph:&Self::Graph) -> Self::State;
    fn get_graph(&self, state:&Self::State, graph:&Self::Graph) -> Self::Graph;
}

pub struct ClusterTransitionConstructor<U: ClusterUpdater> {
    pub graph: U::Graph,
    pub updater: U
}

impl<U: ClusterUpdater> Transition for ClusterTransitionConstructor<U> {
    type State = U::State;
    fn get_next_state(&mut self, state: &Self::State) -> Self::State {
        let new_graph = self.updater.get_graph(&state, &self.graph);
        self.updater.get_state(&state, &new_graph)
    }
}