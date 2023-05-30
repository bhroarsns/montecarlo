use std::ops::Add;
use std::fmt::Debug;

pub enum Neighboring {
    Neighboring(u64),
    Other,
}

pub fn distance_for_2d_pbc_nn_lattice(i: usize, j: usize, row: usize, column: usize) -> Neighboring {
    let dist = i.abs_diff(j);
    if dist == 1 {
        Neighboring::Neighboring(1)
    } else if dist == column - 1 {
        if (i % column == 0) || (j % column == 0) {
            Neighboring::Neighboring(1)
        } else {
            Neighboring::Other
        }
    } else if dist == column {
        Neighboring::Neighboring(1)
    } else if dist == column * (row - 1) {
        Neighboring::Neighboring(1)
    } else {
        Neighboring::Other
    }
}

pub fn distance_for_2d_obc_nn_lattice(i: usize, j: usize, _: usize, column: usize) -> Neighboring {
    let dist = i.abs_diff(j);
    if dist == 1 {
        Neighboring::Neighboring(1)
    } else if dist == column {
        Neighboring::Neighboring(1)
    } else {
        Neighboring::Other
    }
}

#[derive(Copy, Clone)]
pub struct SpinVariable {
    size: u64,
    value: u64,
}

impl SpinVariable {
    pub fn new(size: u64, value: u64) -> Self {
        Self { size, value }
    }
    pub fn spin(&self) -> f64 {
        self.value as f64 - (self.size - 1) as f64 / 2.0
    }
    pub fn change(&mut self, up: bool) {
        let up = if self.value == 0 {
            true
        } else if self.value == (self.size - 1) {
            false
        } else {
            up
        };

        if up {
            self.value += 1
        } else {
            self.value -= 1
        }
    }
    pub fn set_value(&mut self, value: u64) {
        self.value = value % self.size
    }
}

impl Add for SpinVariable {
    type Output = f64;
    fn add(self, rhs: Self) -> Self::Output {
        self.spin() + rhs.spin()
    }
}

impl Debug for SpinVariable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.spin())
    }
}

pub trait SpinTransition {
    type Distance;
    fn distance(&self, i:usize, j:usize) -> Self::Distance;
    fn from_spin(&mut self, spin_config: &Vec<SpinVariable>);
    fn spins(&self) -> Vec<SpinVariable>;

    fn total_spin(&self) -> f64 {
        self.spins().iter().map(|s| s.spin()).sum()
    }

    fn change_spin(&self, i: usize, value: u64) -> Vec<SpinVariable> {
        let mut spins = self.spins();
        spins[i].set_value(value);
        spins
    }

    fn change_spins(&self, indices: Vec<usize>, values: Vec<u64>) -> Vec<SpinVariable> {
        let mut spins = self.spins();
        for index in indices {
            spins[index].set_value(values[index]);
        }
        spins
    }

    fn ladder_spin(&self, i: usize, up: bool) -> Vec<SpinVariable> {
        let mut spins = self.spins();
        spins[i].change(up);
        spins
    }
    
    fn ladder_spins(&self, indices: Vec<usize>, ups: Vec<bool>) -> Vec<SpinVariable> {
        let mut spins = self.spins();
        for index in indices {
            spins[index].change(ups[index]);
        }
        spins
    }
}

pub trait GibbsWeight {
    type State;
    fn beta(&self) -> f64;
    fn energy(&self, state: &Self::State) -> f64;
    fn state(&self) -> Self::State;
    fn energy_of_current_state(&self) -> f64 {
        self.energy(&self.state())
    }
    fn relative_probability(&self, state: &Self::State) -> f64 {
        (- self.beta() * (self.energy(state) - self.energy_of_current_state())).exp()
    }
}