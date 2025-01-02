use crate::lattice::Lattice;
use rand::Rng;

pub trait Metropolis {
    fn sweep(&mut self, rng: &mut impl Rng) -> (f64, (f64, f64));

    fn metropolis_hastings(&mut self, rng: &mut impl Rng, num_sweeps: usize) -> (f64, f64);
}

impl<T> Metropolis for T
where
    T: Lattice,
{
    fn sweep(&mut self, rng: &mut impl Rng) -> (f64, (f64, f64)) {
        let (mut chg_energy, (mut chg_magnetization_cos, mut chg_magnetization_sin)) = (0.0, (0.0, 0.0));
        for i in 0..self.num_sites() {
            let angle = rng.random_range(0.0..crate::constants::MAX_ANGLE);
            let diff_energy = self.energy_diff(i, angle);
            let (diff_magnetization_cos, diff_magnetization_sin) = self.magnetization_diff(i, angle);

            if self.acceptance(diff_energy) > rng.random() {
                chg_energy += diff_energy;
                chg_magnetization_cos += diff_magnetization_cos;
                chg_magnetization_sin += diff_magnetization_sin;
                self.update_angle(i, angle);
            }
        }
        (chg_energy, (chg_magnetization_cos, chg_magnetization_sin))
    }

    fn metropolis_hastings(&mut self, rng: &mut impl Rng, num_sweeps: usize) -> (f64, f64) {
        let (mut cur_energy, (mut cur_magnetization_cos, mut cur_magnetization_sin)) = (self.energy(), self.magnetization());
        let (mut avg_energy, mut avg_magnetization) = (cur_energy, cur_magnetization_cos.powi(2) + cur_magnetization_sin.powi(2));

        for _ in 0..num_sweeps {
            let (change_energy, (chg_magnetization_cos, chg_magnetization_sin)) = self.sweep(rng);
            cur_energy += change_energy;
            cur_magnetization_cos += chg_magnetization_cos;
            cur_magnetization_sin += chg_magnetization_sin;

            avg_energy += cur_energy;
            avg_magnetization += cur_magnetization_cos.powi(2) + cur_magnetization_sin.powi(2);
        }

        (avg_energy / (self.num_sites() * num_sweeps) as f64, avg_magnetization / (self.num_sites().pow(2) * num_sweeps) as f64)
    }
}
