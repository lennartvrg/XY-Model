use crate::lattice::Lattice;
use wide::{f64x2, f64x4};

pub struct Lattice2D {
    size: usize,
    beta: f64,
    spins: Box<[f64]>,
}

impl Lattice2D {
    pub fn new(size: usize, beta: f64) -> Self {
        Self {
            size,
            beta,
            spins: vec![0.0; size * size].into_boxed_slice(),
        }
    }

    pub const fn len(&self) -> usize {
        self.size
    }

    pub fn spins(self) -> Box<[f64]> {
        self.spins
    }
}

impl Lattice for Lattice2D {
    fn sites(&self) -> usize {
        self.spins.len()
    }

    fn update_angle(&mut self, i: usize, angle: f64) {
        self.spins[i] = angle;
    }

    fn energy(&self) -> f64 {
        let mut result = 0.0;
        for i in 0..self.sites() {
            result += f64::cos(self.spins[i] - self.spins[(i + 1) % self.sites()])
                + f64::cos(self.spins[i] - self.spins[(i + self.len()) % self.sites()]);
        }
        -result
    }

    fn energy_diff(&self, i: usize, angle: f64) -> f64 {
        let neighbours = f64x4::from([
            self.spins[(i + 1) % self.sites()],
            self.spins[(i + self.sites() - 1) % self.sites()],
            self.spins[(i + self.len()) % self.sites()],
            self.spins[(i + self.sites() - self.len()) % self.sites()],
        ]);

        let old = f64x4::splat(self.spins[i]);
        let before = (old - neighbours).cos().reduce_add();

        let new = f64x4::splat(angle);
        let after = (new - neighbours).cos().reduce_add();

        before - after
    }

    fn magnetization(&self) -> (f64, f64) {
        let (mut cos, mut sin) = (0.0, 0.0);
        for i in 0..self.sites() {
            cos += f64::cos(self.spins[i]);
            sin += f64::sin(self.spins[i]);
        }
        (cos, sin)
    }

    fn magnetization_diff(&self, i: usize, angle: f64) -> (f64, f64) {
        let (sin, cos) = f64x2::from([angle, std::f64::consts::PI + self.spins[i]]).sin_cos();
        (cos.reduce_add(), sin.reduce_add())
    }

    fn acceptance(&self, diff_energy: f64) -> f64 {
        f64::min(1.0, f64::exp(-self.beta * diff_energy))
    }

    fn normalize_per_spin(&self, value: f64) -> f64 {
        value / self.sites() as f64
    }

    fn specific_heat_per_spin(e: f64, e_sqr: f64, temperature: f64) -> f64 {
        (e_sqr - e.powi(2)) / temperature.powi(2)
    }

    fn magnetic_susceptibility_per_spin(m: f64, m_sqr: f64, temperature: f64) -> f64 {
        (m_sqr - m.powi(2)) / temperature
    }
}
