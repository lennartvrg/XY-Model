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
            let data: [f64; 4] = self.spins()[i..(i + 4)].try_into().unwrap();
            let (s, c) = f64x4::new(data).sin_cos();

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

    fn specific_heat_per_spin(
        e: f64,
        e_std: f64,
        e_sqr: f64,
        e_sqr_std: f64,
        temperature: f64,
    ) -> (f64, f64) {
        (
            (e_sqr - e.powi(2)) / temperature.powi(2),
            f64::sqrt(
                (e_sqr_std / temperature.powi(2)).powi(2)
                    + (2.0 * e * e_std / temperature.powi(2)).powi(2),
            ),
        )
    }

    fn magnetic_susceptibility_per_spin(
        m: f64,
        m_std: f64,
        m_sqr: f64,
        m_sqr_std: f64,
        temperature: f64,
    ) -> (f64, f64) {
        (
            (m_sqr - m.powi(2)) / temperature,
            f64::sqrt((m_sqr_std / temperature).powi(2) + (2.0 * m * m_std / temperature).powi(2)),
        )
    }

    /**
     * Destructs the instance and returns the spins
     */
    fn spins(&self) -> &[f64];
}
