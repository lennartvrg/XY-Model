use crate::lattice::Lattice;

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
}

impl<const N: usize> Lattice for Lattice2D<N> {
    fn update_angle(&mut self, i: usize, angle: f64) {
        let (col, row) = (i / N, i % N);
        self.spins[col][row] = angle;
    }

    fn num_sites(&self) -> usize {
        N * N
    }

    fn energy(&self) -> f64 {
        let mut result = 0.0;
        for (col, row) in (0..(N * N)).map(crate::utils::div_rem::<N>) {
            result += f64::cos(self.spins[col][row] - self.spins[col][(row + 1) % N])
                + f64::cos(self.spins[col][row] - self.spins[(col + 1) % N][row]);
        }
        -result
    }

    fn energy_diff(&self, i: usize, angle: f64) -> f64 {
        let (col, row) = crate::utils::div_rem::<N>(i);
        let before = f64::cos(self.spins[col][row] - self.spins[col][(row + 1) % N])
            + f64::cos(self.spins[col][row] - self.spins[col][(row + N - 1) % N])
            + f64::cos(self.spins[col][row] - self.spins[(col + 1) % N][row])
            + f64::cos(self.spins[col][row] - self.spins[(col + N - 1) % N][row]);

        let after = f64::cos(angle - self.spins[col][(row + 1) % N])
            + f64::cos(angle - self.spins[col][(row + N - 1) % N])
            + f64::cos(angle - self.spins[(col + 1) % N][row])
            + f64::cos(angle - self.spins[(col + N - 1) % N][row]);
        before - after
    }

    fn magnetization(&self) -> (f64, f64) {
        let (mut rcos, mut rsin) = (0.0, 0.0);
        for (col, row) in (0..(N * N)).map(crate::utils::div_rem::<N>) {
            rcos += f64::cos(self.spins[col][row]);
            rsin += f64::sin(self.spins[col][row]);
        }
        (rcos, rsin)
    }

    fn magnetization_diff(&self, i: usize, angle: f64) -> (f64, f64) {
        let (col, row) = (i / N, i % N);
        (f64::cos(angle) - f64::cos(self.spins[col][row]), f64::sin(angle) - f64::sin(self.spins[col][row]))
    }

    fn acceptance(&self, diff_energy: f64) -> f64 {
        f64::min(1.0, f64::exp(-self.beta * diff_energy))
    }
}
