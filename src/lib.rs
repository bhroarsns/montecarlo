pub mod transitions;
pub mod states;

use crate::transitions::*;
use std::io::Write;

pub fn mcmc_step<T: Transition>(
    current: &T::State,
    transition: &mut T,
) -> T::State {
    transition.get_next_state(current)
}

pub fn execute_mcmc<W: Transition, R: std::fmt::Debug, T: Write>(
    init: &W::State,
    transition: &mut W,
    thermalization_step: u64,
    observation_step: u64,
    observe: fn(&W::State) -> R,
    mut out: T,
) -> Vec<R> {
    let mut state = mcmc_step(init, transition);
    if thermalization_step > 0 {
        for _ in 0..(thermalization_step-1) {
            state = mcmc_step(&state, transition)
        }
    }
    
    let mut result = Vec::new();
    if observation_step > 0 {
        for _ in 0..(observation_step-1) {
            result.push(observe(&state));
            writeln!(out, "{:?}", observe(&state)).unwrap();
            state = mcmc_step(&state, transition);
        }
    }
    result.push(observe(&state));
    
    writeln!(out, "{:?}", observe(&state)).unwrap();
    result
}