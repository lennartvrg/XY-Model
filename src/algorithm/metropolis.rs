use rand::Rng;

use crate::lattice::Lattice;
use crate::lattice::lattice_2d::Lattice2D;
use crate::observables::{Energy, EnergyObservable, Magnetization, MagnetizationObservable};

pub trait Metropolis {
    fn sweep(&mut self, rng: &mut impl Rng) -> (f64, (f64, f64));

    fn metropolis_hastings(&mut self, rng: &mut impl Rng, num_sweeps: usize) -> Vec<(impl EnergyObservable, impl MagnetizationObservable)>;
}

impl<const N: usize> Metropolis for Lattice2D<N> {
    fn sweep(&mut self, rng: &mut impl Rng) -> (f64, (f64, f64)) {
        let (mut chg_energy, mut chg_magnetization_cos, mut chg_magnetization_sin) = (Default::default(), Default::default(), Default::default());
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

    fn metropolis_hastings(&mut self, rng: &mut impl Rng, num_sweeps: usize) -> Vec<(Energy<N>, Magnetization<N>)> {
        let mut observables = Vec::<(Energy<N>, Magnetization<N>)>::with_capacity(num_sweeps);
        let (mut cur_energy, mut cur_magnetization) = (self.energy(), self.magnetization());

        for _ in 0..num_sweeps {
            let (chg_energy, chg_magnetization) = self.sweep(rng);
            cur_energy += chg_energy;
            cur_magnetization.0 += chg_magnetization.0;
            cur_magnetization.1 += chg_magnetization.1;

            observables.push((Energy::new(cur_energy), Magnetization::new(cur_magnetization)));
        }

        observables
    }
}
