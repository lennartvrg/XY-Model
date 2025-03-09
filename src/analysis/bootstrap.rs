use wide::f64x4;
use crate::utils::{mean, stddev};

pub fn bootstrap(rng: &mut fastrand::Rng, data: &[f64], tau: f64, b: usize) -> (f64, f64) {
    bootstrap_blocked(rng, &thermalize_and_block(data, tau), data.len(), b)
}

pub fn bootstrap_blocked(
    rng: &mut fastrand::Rng,
    blocked: &[f64],
    a: usize,
    b: usize,
) -> (f64, f64) {
    let mut resamples = Vec::with_capacity(b);
    for _ in 0..b {
        resamples.push(resample_blocked(rng, blocked, a));
    }
    (mean(&resamples), stddev(&resamples))
}

fn resample_blocked(rng: &mut fastrand::Rng, blocked: &[f64], a: usize) -> f64 {
    let mut running = 0.0;
    for _ in (0..a).step_by(4) {
        running += f64x4::new([
            blocked[rng.usize(0..blocked.len())],
            blocked[rng.usize(0..blocked.len())],
            blocked[rng.usize(0..blocked.len())],
            blocked[rng.usize(0..blocked.len())]
        ]).reduce_add();
    }
    running / a as f64
}

pub fn thermalize_and_block(data: &[f64], tau: f64) -> Vec<f64> {
    blocking(thermalize(data, tau), tau)
}

fn thermalize(data: &[f64], tau: f64) -> &[f64] {
    &data[(3 * tau.ceil() as usize)..]
}

pub fn blocking(data: &[f64], tau: f64) -> Vec<f64> {
    data.chunks(tau.ceil() as usize).map(mean).collect()
}
