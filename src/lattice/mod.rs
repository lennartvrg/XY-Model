pub mod lattice_2d;

pub trait Lattice {
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
    fn magnetization(&self) -> (f64, f64);

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
    fn normalize_per_spin(value: f64) -> f64;

    fn specific_heat_per_spin(e: f64, e_sqr: f64, temperature: f64) -> f64;
}
