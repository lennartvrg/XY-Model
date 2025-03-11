use crate::analysis::Observable;
use wide::f64x4;

pub mod lattice_1d;
pub mod lattice_2d;

pub trait Lattice {
    /**
     * The dimensions of the lattice
     */
    const DIM: usize;

    /**
     * Creates a new lattice
     */
    fn new(length: usize, beta: f64) -> Self;

    /**
     * Temperature of the lattice
     */
    fn temperature(&self) -> f64;

    /**
     * Number of sites on the lattice
     */
    fn sites(&self) -> usize;

    /**
     * Flips the spin at index i.
     */
    fn update_angle(&mut self, i: usize, angle: f64);

    /**
     * Calculates the total energy of the lattice.
     */
    fn energy(&self) -> f64;

    /**
     * Calculates the energy difference if one was to flip the spin at index i.
     */
    fn energy_diff(&self, i: usize, angle: f64) -> f64;

    /**
     * Calculates the total magnetization of the lattice.
     */
    fn magnetization(&self) -> (f64, f64) {
        let (mut cos, mut sin) = (0.0, 0.0);
        for i in (0..self.sites()).step_by(4) {
            let data = self.spins();
            let (s, c) = f64x4::new([data[i], data[i + 1], data[i + 2], data[i + 3]]).sin_cos();

            cos += c.reduce_add();
            sin += s.reduce_add();
        }
        (cos, sin)
    }

    /**
     * Calculates the magnetization difference if one was to flip the spin at index i.
     */
    fn magnetization_diff(&self, i: usize, angle: f64) -> (f64, f64);

    /**
     * Calculates the acceptance probability if one was to flip the spin at index i.
     */
    fn acceptance(&self, diff_energy: f64) -> f64;

    /**
     * Normalizes a given observable to a per spin value.
     */
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

    /**
     * Destructs the instance and returns the spins
     */
    fn spins(&self) -> &[f64];
}
