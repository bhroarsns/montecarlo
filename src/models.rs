pub trait SpinState {
    fn spin_size(&self) -> u64;
    fn spins(&self) -> Vec<u64>;
    fn from_spin(&self, new_spin_config: Vec<u64>) -> Self;
}

#[allow(dead_code)]
pub struct UniformSpinChain {
    spin_size: u64,
    pub size: u64,
    pub spins: Vec<u64>,
}

#[allow(dead_code)]
impl UniformSpinChain {
    pub fn new(spin_size: u64, size: u64, spins: Vec<u64>) -> Self {
        UniformSpinChain {
            spin_size,
            size,
            spins
        }
    }

    pub fn new_su2(size: u64, spins: Vec<u64>) -> Self {
        UniformSpinChain {
            spin_size: 2,
            size,
            spins
        }
    }

    pub fn new_su3(size: u64, spins: Vec<u64>) -> Self {
        UniformSpinChain {
            spin_size: 3,
            size,
            spins
        }
    }

    pub fn total_magnetization(&self) -> u64 {
        self.spins.iter().sum::<u64>()
    }
}

impl SpinState for UniformSpinChain {
    fn spin_size(&self) -> u64 {
        self.spin_size
    }

    fn spins(&self) -> Vec<u64> {
        self.spins.clone()
    }

    fn from_spin(&self, new_spin_config: Vec<u64>) -> Self {
        Self::new(self.spin_size, self.size, new_spin_config)
    }
}

// pub struct UniformSpin2DLattice {
//     spin_size: u64,
//     pub size: u64,
//     pub spins: Vec<Vec<u64>>,
// }

// #[allow(dead_code)]
// impl UniformSpin2DLattice {
//     pub fn new(spin_size: u64, size: u64, spins: Vec<Vec<u64>>) -> Self {
//         UniformSpin2DLattice {
//             spin_size,
//             size,
//             spins
//         }
//     }

//     pub fn new_su2(size: u64, spins: Vec<Vec<u64>>) -> Self {
//         UniformSpin2DLattice {
//             spin_size: 2,
//             size,
//             spins
//         }
//     }

//     pub fn new_su3(size: u64, spins: Vec<Vec<u64>>) -> Self {
//         UniformSpin2DLattice {
//             spin_size: 3,
//             size,
//             spins
//         }
//     }

//     pub fn total_magnetization(&self) -> u64 {
//         self.spins.iter().flatten().sum::<u64>()
//     }
// }

// impl SpinState for UniformSpin2DLattice {
//     fn spin_size(&self) -> u64 {
//         self.spin_size
//     }

//     fn spins(&self) -> Vec<u64> {
//         self.spins.iter().flatten().map(|x| *x).collect()
//     }
    
//     fn from_spin(&self, new_spin_config: Vec<u64>) -> Self {
//         Self::new(self.spin_size, self.size, new_spin_config)
//     }
// }