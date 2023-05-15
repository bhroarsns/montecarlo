pub enum Neighboring {
    NearestNeighboring,
    SecondNearestNeighboring,
    Other,
}

pub trait SpinTransition {
    type Distance;
    fn distance(&self, i:usize, j:usize) -> Self::Distance;
    fn from_spin(&mut self, spin_config: &Vec<u64>);
    fn spin_size(&self, i:usize) -> u64;
    fn spins(&self) -> Vec<u64>;

    fn total_spin(&self) -> u64 {
        self.spins().iter().sum()
    }

    fn change_spin(&self, i: usize, value:u64) -> Vec<u64> {
        let mut spins = self.spins();
        spins[i] = value;
        spins
    }
    
    fn change_spins(&self, indices: Vec<usize>, values: Vec<u64>) -> Vec<u64> {
        let mut spins = self.spins();
        for index in indices {
            spins[index] = values[index];
        }
        spins
    }
}

pub trait GibbsWeight {
    type State;
    fn state(&self) -> Self::State;
    fn energy(&self, state: &Self::State) -> f64;
    fn energy_of_current_state(&self) -> f64 {
        self.energy(&self.state())
    }
    fn beta(&self) -> f64;
    fn relative_probability(&self, state: &Self::State) -> f64 {
        (self.beta() * (self.energy(state) - self.energy_of_current_state())).exp()
    }
}