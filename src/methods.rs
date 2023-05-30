pub trait MetropolisHastings {
    type State;
    fn acceptance_probability(&self, state: &Self::State) -> f64;
    fn candidate_state(&self) -> Self::State;
    fn generate_random_number(&mut self) -> f64;
    fn get_next_state(&mut self) {
        let candidate = self.candidate_state();
        if self.generate_random_number() < self.acceptance_probability(&candidate) {
            self.set_state(&candidate);
        }
    }
    fn set_state(&mut self, state: &Self::State);
}

pub trait ClusterUpdate {
    type State;
    type Graph;
    fn get_next_graph(&mut self);
    fn get_next_state_from_graph(&mut self) -> Self::State;
}