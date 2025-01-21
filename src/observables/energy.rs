use crate::observables::Observable;

pub trait EnergyObservable: Observable {
    fn specific_heat_per_side(&self, temperature: f64) -> f64;
}

#[derive(Copy, Clone, Default)]
pub struct Energy<const N: usize> {
    value: f64,
    squared: f64,
}

impl<const N: usize> Energy<N> {
    pub fn new(energy: f64) -> Self {
        Self {
            value: energy,
            squared: energy.powi(2),
        }
    }
}

impl<const N: usize> Observable for Energy<N> {
    fn norm(&self) -> f64 {
        self.value
    }

    fn norm_per_site(&self) -> f64 {
        self.norm() / N.pow(2) as f64
    }

    fn norm_sqr(&self) -> f64 {
        self.squared
    }

    fn norm_sqr_per_site(&self) -> f64 {
        self.norm_sqr() / N.pow(4) as f64
    }
}

impl<const N: usize> EnergyObservable for Energy<N> {
    fn specific_heat_per_side(&self, temperature: f64) -> f64 {
        (self.norm_sqr_per_site() - self.norm_per_site().powi(2)) / temperature.powi(2)
    }
}
