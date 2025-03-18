use crate::lattice::Lattice;
use std::ops::Index;
use wide::{f64x2, f64x4};

pub struct Lattice1D {
    beta: f64,
    spins: Box<[f64]>,
}

impl Lattice for Lattice1D {
    const DIM: usize = 1;

    fn new(length: usize, beta: f64) -> Self {
        assert_eq!(length % 4, 0);
        Self {
            beta,
            spins: vec![0.0; length].into_boxed_slice(),
        }
    }

    fn set_beta(&mut self, beta: f64) {
        self.beta = beta;
    }

    fn temperature(&self) -> f64 {
        self.beta.recip()
    }

    fn sites(&self) -> usize {
        self.spins.len()
    }

    fn update_angle(&mut self, i: usize, angle: f64) {
        self.spins[i] = angle;
    }

    fn energy(&self) -> f64 {
        let mut result = 0.0;
        for i in (0..self.sites()).step_by(4) {
            let old = f64x4::splat(self[i]);
            let neighbours = f64x4::new([
                self[(i + 1) % self.sites()],
                self[(i + 2) % self.sites()],
                self[(i + 3) % self.sites()],
                self[(i + 4) % self.sites()],
            ]);
            result += (old - neighbours).cos().reduce_add();
        }
        -result
    }

    fn energy_diff(&self, i: usize, angle: f64) -> f64 {
        let neighbours = f64x2::from([
            self[(i + 1) % self.sites()],
            self[(i + self.sites() - 1) % self.sites()],
        ]);

        let old = f64x2::splat(self[i]);
        let before = (old - neighbours).cos().reduce_add();

        let new = f64x2::splat(angle);
        let after = (new - neighbours).cos().reduce_add();

        before - after
    }

    fn magnetization_diff(&self, i: usize, angle: f64) -> (f64, f64) {
        let (sin, cos) = f64x2::from([angle, std::f64::consts::PI + self[i]]).sin_cos();
        (cos.reduce_add(), sin.reduce_add())
    }

    fn acceptance(&self, diff_energy: f64) -> f64 {
        f64::min(1.0, f64::exp(-self.beta * diff_energy))
    }

    fn serialize(&self) -> String {
        serde_json::to_string(&self.spins).unwrap()
    }
}

impl Index<usize> for Lattice1D {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.spins[index]
    }
}
