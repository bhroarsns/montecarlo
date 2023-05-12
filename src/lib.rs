mod metropolis_heistings;
mod models;

use rand::Rng;

pub trait Transition<S> {
    fn get_next_state_from_random_f64(&self, state: &S, trial:f64) -> S;
}

pub fn mcmc_step<S, T: Transition<S>, G: Rng + ?Sized>(
    current: &S,
    transition: &T,
    generator:&mut G
) -> S {
    let trial: f64 = generator.gen();
    transition.get_next_state_from_random_f64(current, trial)
}

pub fn execute_mcmc<S, T: Transition<S>, G: Rng + ?Sized, R: std::fmt::Debug>(
    init: &S,
    transition: &T,
    thermalization_step: u64,
    observation_step: u64,
    generator:&mut G,
    observe: fn(&S) -> R,
    verbose: bool,
) -> Vec<R> {
    let mut state = mcmc_step(init, transition, generator);
    if thermalization_step > 0 {
        for _ in 0..(thermalization_step-1) {
            state = mcmc_step(&state, transition, generator)
        }
    }
    let mut result = Vec::new();
    if observation_step > 0 {
        for _ in 0..(observation_step-1) {
            result.push(observe(&state));
            if verbose {
                println!("{:?}", observe(&state));
            }
            state = mcmc_step(&state, transition, generator);
        }
    }
    result.push(observe(&state));
    if verbose {
        println!("{:?}", observe(&state));
    }
    result
}