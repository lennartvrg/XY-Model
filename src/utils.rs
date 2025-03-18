use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;
use std::ops::Range;
use std::time::SystemTime;

/// Returns the number of seconds since UNIX EPOCH.
pub fn unix_time() -> i32 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i32
}

/// Returns the hostname of the machine the executable is running on.
/// If the hostname cannot be determined it returns '<NONE>'.
pub fn host() -> String {
    gethostname::gethostname()
        .to_str()
        .unwrap_or("<NONE>")
        .to_owned()
}

/// Calculates the mean over a slice of f64 values.
pub fn mean(data: &[f64]) -> f64 {
    data.iter().sum::<f64>() * (data.len() as f64).recip()
}

/// Calculates the sample standard deviation over a slice of f64 values.
/// Takes the mean as an argument so to prevent unnecessary calculations.
pub fn stddev(data: &[f64], mean: f64) -> f64 {
    data.iter().map(|x| (x - mean) * (x - mean)).sum::<f64>() * ((data.len() - 1) as f64).recip()
}

/// Splits the given range into steps and returns a parallel iterator and the step size.
pub fn range_par(range: Range<f64>, steps: usize) -> (impl ParallelIterator<Item = f64>, f64) {
    let stride = (range.end - range.start) / steps as f64;
    (
        (1..=steps)
            .into_par_iter()
            .map(move |i| range.start + i as f64 * stride),
        stride,
    )
}

/// Splits the given range into steps and returns an iterator and the step size.
pub fn range(range: Range<f64>, steps: usize) -> (impl DoubleEndedIterator<Item = f64>, f64) {
    let stride = (range.end - range.start) / steps as f64;
    (
        (1..=steps).map(move |i| range.start + i as f64 * stride),
        stride,
    )
}