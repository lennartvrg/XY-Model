use crate::utils::{mean, stddev};

pub fn bootstrap(rng: &mut fastrand::Rng, data: &[f64], tau: f64, samples: usize) -> (f64, f64) {
    bootstrap_blocked(rng, &thermalize_and_block(data, tau), samples)
}

pub fn bootstrap_blocked(rng: &mut fastrand::Rng, blocked: &[f64], samples: usize) -> (f64, f64) {
    let mut resamples = Vec::with_capacity(samples);
    for _ in 0..samples {
        let mut running = 0.0;
        for idx in rng.choose_multiple(0..blocked.len(), blocked.len()) {
            running += blocked[idx];
        }
        resamples.push(running / blocked.len() as f64);
    }
    (mean(&resamples), stddev(&resamples))
}

pub fn thermalize_and_block(data: &[f64], tau: f64) -> Vec<f64> {
    blocking(&thermalize(data, tau), tau)
}

fn thermalize(data: &[f64], tau: f64) -> &[f64] {
    &data[((3.0 * tau).ceil() as usize)..]
}

pub fn blocking(data: &[f64], tau: f64) -> Vec<f64> {
    data.chunks(tau.ceil() as usize).map(mean).collect()
}
