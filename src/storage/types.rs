use crate::analysis::Observable;
use crate::lattice::Lattice;
use std::cmp::Ordering;

pub struct Run {
    pub id: i32,
}

#[derive(Clone, Debug)]
pub struct Configuration {
    pub dimension: usize,
    pub temperature: f64,
    pub energy: Observable,
    pub magnetization: Observable,
    pub cv: (f64, f64),
    pub xs: (f64, f64),
    pub time_mc: u128,
    pub time_boot: u128,
}

impl Configuration {
    pub fn new<L>(
        lattice: &L,
        energy: Observable,
        magnetization: Observable,
        time_mc: u128,
        time_boot: u128,
    ) -> Self
    where
        L: Lattice,
    {
        Self {
            dimension: L::DIM,
            temperature: lattice.temperature(),
            cv: lattice.specific_heat_per_spin(&energy),
            xs: lattice.magnetic_susceptibility_per_spin(&magnetization),
            energy,
            magnetization,
            time_mc,
            time_boot,
        }
    }

    pub fn cmp(a: &&Configuration, b: &&Configuration) -> Ordering {
        a.xs.0.total_cmp(&b.xs.0)
    }
}
