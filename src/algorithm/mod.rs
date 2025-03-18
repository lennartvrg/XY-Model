pub mod metropolis;

pub use metropolis::Metropolis;

/// Algorithm is the supertrait for all concrete Monte Carlo algorithms
pub trait Algorithm {
    /// Simulates the system using the given random number generate for the given
    /// number of sweeps. Returns the energy and magnetization after said sweeps.
    fn simulate(&mut self, rng: &mut fastrand::Rng, sweeps: usize) -> (Vec<f64>, Vec<f64>);
}
