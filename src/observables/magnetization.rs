use crate::observables::Observable;

pub trait MagnetizationObservable: Observable {
    fn magnetic_susceptibility(&self, temperature: f64) -> f64;
}

#[derive(Copy, Clone, Default)]
pub struct Magnetization<const N: usize> {
    value: f64,
    squared: f64,
}

impl<const N: usize> Magnetization<N> {
    pub fn new((cos, sin): (f64, f64)) -> Self {
        Self {
            value: f64::sqrt(cos * cos + sin * sin),
            squared: cos * cos + sin * sin,
        }
    }
}

impl<const N: usize> Observable for Magnetization<N> {
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

impl<const N: usize> MagnetizationObservable for Magnetization<N> {
    fn magnetic_susceptibility(&self, temperature: f64) -> f64 {
        (self.norm_sqr_per_site() - self.norm_per_site().powi(2)) / temperature
    }
}
