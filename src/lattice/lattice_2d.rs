use crate::lattice::Lattice;
use wide::{f64x2, f64x4};

pub struct Lattice2D<const N: usize> {
    beta: f64,
    spins: [[f64; N]; N],
}

impl<const N: usize> Lattice2D<N> {
    pub const fn new(beta: f64) -> Self {
        Self {
            beta,
            spins: [[0.0; N]; N],
        }
    }

    pub const fn num_sites(&self) -> usize {
        N * N
    }
}

impl<const N: usize> Lattice for Lattice2D<N> {
    fn update_angle(&mut self, i: usize, angle: f64) {
        let (col, row) = (i / N, i % N);
        self.spins[col][row] = angle;
    }

    fn energy(&self) -> f64 {
        let mut result = 0.0;
        for (col, row) in (0..self.num_sites()).map(crate::utils::div_rem::<N>) {
            result += f64::cos(self.spins[col][row] - self.spins[col][(row + 1) % N])
                + f64::cos(self.spins[col][row] - self.spins[(col + 1) % N][row]);
        }
        -result
    }

    fn energy_diff(&self, i: usize, angle: f64) -> f64 {
        let (col, row) = crate::utils::div_rem::<N>(i);
        let neighbours = f64x4::from([
            self.spins[col][(row + 1) % N],
            self.spins[col][(row + N - 1) % N],
            self.spins[(col + 1) % N][row],
            self.spins[(col + N - 1) % N][row],
        ]);

        let old = f64x4::splat(self.spins[col][row]);
        let before = (old - neighbours).cos().reduce_add();

        let new = f64x4::splat(angle);
        let after = (new - neighbours).cos().reduce_add();

        before - after
    }

    fn magnetization(&self) -> (f64, f64) {
        let (mut cos, mut sin) = (0.0, 0.0);
        for (col, row) in (0..self.num_sites()).map(crate::utils::div_rem::<N>) {
            cos += f64::cos(self.spins[col][row]);
            sin += f64::sin(self.spins[col][row]);
        }
        (cos, sin)
    }

    fn magnetization_diff(&self, i: usize, angle: f64) -> (f64, f64) {
        let (col, row) = (i / N, i % N);
        let (sin, cos) =
            f64x2::from([angle, std::f64::consts::PI + self.spins[col][row]]).sin_cos();
        (cos.reduce_add(), sin.reduce_add())
    }

    fn acceptance(&self, diff_energy: f64) -> f64 {
        f64::min(1.0, f64::exp(-self.beta * diff_energy))
    }

    fn normalize_per_spin(value: f64) -> f64 {
        value / N.pow(2) as f64
    }

    fn specific_heat_per_spin(e: f64, e_sqr: f64, temperature: f64) -> f64 {
        (e_sqr - e.powi(2)) / temperature.powi(2)
    }

    fn magnetic_susceptibility_per_spin(m: f64, m_sqr: f64, temperature: f64) -> f64 {
        (m_sqr - m.powi(2)) / temperature
    }
}
