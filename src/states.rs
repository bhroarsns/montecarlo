pub mod spinstates;

pub trait State {
    fn identity(&self) -> Self;
}