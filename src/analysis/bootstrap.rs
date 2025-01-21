use crate::observables::Observable;
use rand::distr::Uniform;
use rand::Rng;

pub fn bootstrap(data: &[impl Observable], tau: f64, samples: usize) -> (f64, f64) {
    let thermalized = thermalize(data, tau);
    let blocks = blocking(&thermalized, tau);

    let mut resamples = Vec::with_capacity(samples);
    let range = Uniform::try_from(0..blocks.len()).unwrap();

    for _ in 0..samples {
        let mut running = 0.0;
        for idx in rand::rng().sample_iter(range).take(blocks.len()) {
            running += blocks[idx];
        }
        resamples.push(running / blocks.len() as f64);
    }

    (
        crate::utils::mean(&resamples),
        crate::utils::stddev(&resamples),
    )
}

fn thermalize(data: &[impl Observable], tau: f64) -> Vec<f64> {
    data.iter()
        .skip((3.0 * tau).ceil() as usize)
        .map(|x| x.norm_per_site())
        .collect::<Vec<_>>()
}

fn blocking(data: &[f64], tau: f64) -> Vec<f64> {
    data.chunks(tau.ceil() as usize)
        .map(crate::utils::mean)
        .collect()
}
