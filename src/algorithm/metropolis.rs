use crate::algorithm::Algorithm;
use crate::lattice::Lattice;

pub trait Metropolis: Algorithm {
    /// Perform a single sweep over all lattice sites and returns the energy and magnetization
    /// delta. The magnetization delta is split into its (cos, sin) components.
    fn sweep(&mut self, rng: &mut fastrand::Rng) -> (f64, (f64, f64));

    /// Run simulation using the Metropolis Hastings algorithm for the given number of sweeps.
    /// Returns two vectors with the energy and magnetization observables.
    fn metropolis_hastings(
        &mut self,
        rng: &mut fastrand::Rng,
        sweeps: usize,
    ) -> (Vec<f64>, Vec<f64>);
}

impl<T> Algorithm for T
where
    T: Lattice + Metropolis,
{
    /// Runs the Metropolis Hastings algorithm on the lattice for the given number of sweeps.
    /// Returns the energy and magnetization observables.
    fn simulate(&mut self, rng: &mut fastrand::Rng, sweeps: usize) -> (Vec<f64>, Vec<f64>) {
        self.metropolis_hastings(rng, sweeps)
    }
}

impl<T> Metropolis for T
where
    T: Lattice,
{
    fn sweep(&mut self, rng: &mut fastrand::Rng) -> (f64, (f64, f64)) {
        // Prepare change variables
        let (mut chg_energy, mut chg_magnet_cos, mut chg_magnet_sin) = (0.0, 0.0, 0.0);

        // Go over all lattice sites
        for i in 0..self.sites() {
            // Generate random angle [0,2pi) and calculate difference in e and m
            let angle = rng.f64() * crate::constants::MAX_ANGLE;
            let diff_energy = self.energy_diff(i, angle);
            let (diff_magnet_cos, diff_magnet_sin) = self.magnetization_diff(i, angle);

            // Check acceptance ratio for energy difference and update observables and spin
            // if change is accepted
            if self.acceptance(diff_energy) > rng.f64() {
                chg_energy += diff_energy;
                chg_magnet_cos += diff_magnet_cos;
                chg_magnet_sin += diff_magnet_sin;
                self.update_angle(i, angle);
            }
        }

        // Return change in observables
        (chg_energy, (chg_magnet_cos, chg_magnet_sin))
    }

    fn metropolis_hastings(
        &mut self,
        rng: &mut fastrand::Rng,
        sweeps: usize,
    ) -> (Vec<f64>, Vec<f64>) {
        // Prepare results vectors
        let mut energies = Vec::<f64>::with_capacity(sweeps);
        let mut magnets = Vec::<f64>::with_capacity(sweeps);

        // Calculate initial observables and sweeps over lattice
        let (mut cur_energy, mut cur_magnetization) = (self.energy(), self.magnetization());
        for _ in 0..sweeps {
            // Perform sweep and collect running observables
            let (chg_energy, chg_magnetization) = self.sweep(rng);
            cur_energy += chg_energy;
            cur_magnetization.0 += chg_magnetization.0;
            cur_magnetization.1 += chg_magnetization.1;

            // Push current observables to results
            energies.push(self.normalize_per_spin(cur_energy));
            magnets.push(self.normalize_per_spin(f64::sqrt(
                cur_magnetization.0.powi(2) + cur_magnetization.1.powi(2),
            )));
        }

        // Return results
        (energies, magnets)
    }
}
