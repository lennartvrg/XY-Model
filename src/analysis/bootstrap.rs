use rand::distr::Uniform;
use rand::Rng;

use crate::utils::{mean, stddev};

pub fn bootstrap(rng: &mut impl Rng, data: &[f64], tau: f64, samples: usize) -> (f64, f64) {
    bootstrap_blocked(rng, &thermalize_and_block(data, tau), samples)
}

pub fn bootstrap_blocked(rng: &mut impl Rng, blocked: &[f64], samples: usize) -> (f64, f64) {
    let mut resamples = Vec::with_capacity(samples);
    let Ok(range) = Uniform::new(0, blocked.len()) else {
        println!("Failed to to form range [0, {})", blocked.len());
        return (0.0, 0.0);
    };

    for _ in 0..samples {
        let mut running = 0.0;
        for idx in rng.sample_iter(range).take(blocked.len()) {
            running += blocked[idx];
        }
        resamples.push(running / blocked.len() as f64);
    }

    (mean(&resamples), stddev(&resamples))
}

pub fn thermalize_and_block(data: &[f64], tau: f64) -> Vec<f64> {
    blocking(&thermalize(data, tau), tau)
}

fn thermalize(data: &[f64], tau: f64) -> Vec<f64> {
    data.iter()
        .skip((3.0 * tau).ceil() as usize)
        .copied()
        .collect::<Vec<_>>()
}

pub fn blocking(data: &[f64], tau: f64) -> Vec<f64> {
    data.chunks(tau.ceil() as usize).map(mean).collect()
}
