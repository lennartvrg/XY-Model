use rand::Rng;

use crate::lattice::lattice_2d::Lattice2D;
use crate::lattice::Lattice;

pub trait Metropolis {
    fn sweep(&mut self, rng: &mut impl Rng) -> (f64, (f64, f64));

    fn metropolis_hastings(&mut self, rng: &mut impl Rng, sweeps: usize) -> (Vec<f64>, Vec<f64>);
}

impl Metropolis for Lattice2D {
    fn sweep(&mut self, rng: &mut impl Rng) -> (f64, (f64, f64)) {
        let (mut chg_energy, mut chg_magnetization_cos, mut chg_magnetization_sin) =
            (Default::default(), Default::default(), Default::default());
        for i in 0..self.sites() {
            let angle = rng.random_range(0.0..crate::constants::MAX_ANGLE);
            let diff_energy = self.energy_diff(i, angle);
            let (diff_magnetization_cos, diff_magnetization_sin) =
                self.magnetization_diff(i, angle);

            if self.acceptance(diff_energy) > rng.random() {
                chg_energy += diff_energy;
                chg_magnetization_cos += diff_magnetization_cos;
                chg_magnetization_sin += diff_magnetization_sin;
                self.update_angle(i, angle);
            }
        }
        (chg_energy, (chg_magnetization_cos, chg_magnetization_sin))
    }

    fn metropolis_hastings(&mut self, rng: &mut impl Rng, sweeps: usize) -> (Vec<f64>, Vec<f64>) {
        let mut energies = Vec::<f64>::with_capacity(sweeps);
        let mut magnetizations = Vec::<f64>::with_capacity(sweeps);

        let (mut cur_energy, mut cur_magnetization) = (self.energy(), self.magnetization());
        for _ in 0..sweeps {
            let (chg_energy, chg_magnetization) = self.sweep(rng);
            cur_energy += chg_energy;
            cur_magnetization.0 += chg_magnetization.0;
            cur_magnetization.1 += chg_magnetization.1;

            energies.push(self.normalize_per_spin(cur_energy));
            magnetizations.push(self.normalize_per_spin(f64::sqrt(
                cur_magnetization.0.powi(2) + cur_magnetization.1.powi(2),
            )));
        }

        (energies, magnetizations)
    }
}
