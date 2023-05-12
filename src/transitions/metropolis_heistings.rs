use rand::Rng;
use super::*;

pub trait MHAcceptance<S: State> {
    fn acceptance_probability(&self, states: &(S, S)) -> f64;
}
pub struct MHTransitionConstructor<P: Transition, A: MHAcceptance<P::State>, G: Rng> {
    pub proposal_distribution: P,
    pub acceptance: A,
    pub generator: G,
}

impl<P: Transition, A: MHAcceptance<P::State>, G: Rng> Transition for MHTransitionConstructor<P, A, G> {
    type State = P::State;
    fn get_next_state(&mut self, state: &Self::State) -> Self::State {
        let candidates = (state.identity(), self.proposal_distribution.get_next_state(state));
        let rng = &mut self.generator;
        if rng.gen::<f64>() < self.acceptance.acceptance_probability(&candidates) {
            candidates.0
        } else {
            candidates.1
        }
    }
}