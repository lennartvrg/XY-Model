use crate::utils::{mean, stddev};

/// Performs bootstrap analysis on data by first blocking and thermalizing the data using the given
/// tau. For a total of b times this will pick a values from the blocked data with repetition and
/// calculate the mean. From those b means the final mean and sample standard deviation is
/// calculated and returned.
pub fn bootstrap(rng: &mut fastrand::Rng, data: &[f64], tau: f64, b: usize) -> (f64, f64) {
    bootstrap_blocked(rng, &thermalize_and_block(data, tau), data.len(), b)
}

/// Perform bootstrap analysis on blocked data. For a total of b times this will pick a values
/// from the blocked data with repetition and calculate the mean. From those b means the final
/// mean and sample standard deviation is calculated and returned.
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

    let mean = mean(&resamples);
    (mean, stddev(&resamples, mean))
}

/// Picks a random values from the blocked values with repetition and returns the mean.
fn resample_blocked(rng: &mut fastrand::Rng, blocked: &[f64], a: usize) -> f64 {
    let mut running = 0.0;
    for _ in 0..a {
        running += *rng.choice(blocked).unwrap_or(&0.0);
    }
    running * (a as f64).recip()
}

/// Thermalizes and blocks the data using the given tau values.
/// Returns the means of the blocked values.
pub fn thermalize_and_block(data: &[f64], tau: f64) -> Vec<f64> {
    blocking(thermalize(data, tau), tau)
}

/// Thermalizes the data by skipping over the first 3*ceil(tau) elements.
fn thermalize(data: &[f64], tau: f64) -> &[f64] {
    &data[(3 * tau.ceil() as usize)..]
}

/// Blocks the data by dividing it in chunks length ceil(tau).
/// Returns the mean of each chunk
pub fn blocking(data: &[f64], tau: f64) -> Vec<f64> {
    data.chunks(tau.ceil() as usize).map(mean).collect()
}
