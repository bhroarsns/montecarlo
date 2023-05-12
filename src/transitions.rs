use crate::states::State;
pub mod cluster;
pub mod metropolis_heistings;

pub trait Transition {
    type State: State;
    fn get_next_state(&mut self, state: &Self::State) -> Self::State;
}