use crate::analysis::Observable;
use std::ops::Index;
use wide::f64x4;

pub mod lattice_1d;
pub mod lattice_2d;

pub use lattice_1d::Lattice1D;
pub use lattice_2d::Lattice2D;

pub trait Lattice: Index<usize, Output = f64> {
    /// The dimensionality of the lattice.
    const DIM: usize;

    /// Instantiates a new lattice with side length and beta
    fn new(length: usize, beta: f64) -> Self;

    /// Sets a new beta for the lattice.
    fn set_beta(&mut self, beta: f64);

    /// Returns the temperature of the lattice
    fn temperature(&self) -> f64;

    /// Returns the number of lattice sites.
    fn sites(&self) -> usize;

    /// Updates the angle of the spin at index i.
    fn update_angle(&mut self, i: usize, angle: f64);

    /// Calculates the total energy of the lattice.
    fn energy(&self) -> f64;

    /// Calculates the energy difference if one was to flip the spin at index i.
    fn energy_diff(&self, i: usize, angle: f64) -> f64;

    /// Calculates the total magnetization of the lattice by adding up the cosine and sine of
    /// all angles on the lattice. The cos/sin must be squared for the actual magnetization. Due
    /// to the code structure this must happen in code that actually uses the magnetization.
    fn magnetization(&self) -> (f64, f64) {
        let (mut cos, mut sin) = (0.0, 0.0);
        for i in (0..self.sites()).step_by(4) {
            let (s, c) = f64x4::new([self[i], self[i + 1], self[i + 2], self[i + 3]]).sin_cos();

            cos += c.reduce_add();
            sin += s.reduce_add();
        }
        (cos, sin)
    }

    /// Calculates the magnetization difference if one was to change the spin at index i.
    /// Returns the cosine and sine component of the magnetization.
    fn magnetization_diff(&self, i: usize, angle: f64) -> (f64, f64);

    /// Calculates the acceptance probability if one was to flip the spin at index i.
    fn acceptance(&self, diff_energy: f64) -> f64;

    /// Normalizes a given observable to a per spin value.
    fn normalize_per_spin(&self, value: f64) -> f64 {
        value / self.sites() as f64
    }

    fn specific_heat_per_spin(&self, energy: &Observable) -> (f64, f64) {
        (
            (energy.sqr_mean - energy.mean.powi(2)) / self.temperature().powi(2),
            ((energy.sqr_stddev / self.temperature().powi(2)).powi(2)
                + (2.0 * energy.mean * energy.stddev / self.temperature().powi(2)).powi(2))
            .sqrt(),
        )
    }

    fn magnetic_susceptibility_per_spin(&self, magnet: &Observable) -> (f64, f64) {
        (
            (magnet.sqr_mean - magnet.mean.powi(2)) / self.temperature(),
            ((magnet.sqr_stddev / self.temperature()).powi(2)
                + (2.0 * magnet.mean * magnet.stddev / self.temperature()).powi(2))
            .sqrt(),
        )
    }

    /// Serializes the spins into JSON array
    fn serialize(&self) -> String;
}
