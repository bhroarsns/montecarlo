pub mod methods;
pub mod models;

use std::io::Write;
use std::fmt::Debug;

pub trait Transition {
    fn get_next_state(&mut self);
}

pub fn execute_mcmc<W: Transition, R: Debug, T: Write>(
    transition: &mut W,
    thermalization_step: u64,
    observation_step: u64,
    observe: fn(&W) -> R,
    mut out: T,
) -> Vec<R> {
    for _ in 0..thermalization_step {
        transition.get_next_state();
    }

    let mut result = Vec::new();
    if observation_step > 0 {
        for _ in 0..observation_step {
            transition.get_next_state();
            let tmp = observe(&transition);
            writeln!(out, "{:?}", tmp).unwrap();
            result.push(tmp);
        }
    }
    
    result
}