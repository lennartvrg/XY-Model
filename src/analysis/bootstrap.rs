use std::array::TryFromSliceError;
use crate::utils::{mean, stddev};
use wide::f64x4;

pub fn bootstrap(
    rng: &mut fastrand::Rng,
    data: &[f64],
    tau: f64,
    a: usize,
    b: usize,
) -> (f64, f64) {
    bootstrap_blocked(rng, &thermalize_and_block(data, tau), a, b)
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
    while let Some(Ok([a, b, c, d])) = chunk_range::<4>(rng, a).iter().next().copied() {
        running += f64x4::new([blocked[a], blocked[b], blocked[c], blocked[d]]).reduce_add();
    }
    running / a as f64
}

fn chunk_range<const N: usize>(
    rng: &mut fastrand::Rng,
    a: usize,
) -> Vec<Result<[usize; N], TryFromSliceError>> {
    rng.choose_multiple(0..a, a)
        .chunks_exact(N)
        .map(TryInto::<[usize; N]>::try_into)
        .collect::<Vec<_>>()
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
