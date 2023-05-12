use super::*;
pub trait MHTransition<S>: Transition<S> {
    fn couple_candidate_state_and_original_state(&self, state: &S) -> (S, S);
    fn acceptance_probability(&self, candidates: &(S, S)) -> f64;
    fn get_next_state_from_random_f64(&self, state: &S, trial: f64) -> S {
        let candidates = self.couple_candidate_state_and_original_state(state);
        if trial < self.acceptance_probability(&candidates) {
            candidates.0
        } else {
            candidates.1
        }
    }
}