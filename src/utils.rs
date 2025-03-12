use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;
use std::ops::Range;
use std::time::{SystemTime, SystemTimeError};

pub fn unix_time() -> Result<i32, SystemTimeError> {
    Ok(SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs() as i32)
}

pub fn hostname() -> String {
    gethostname::gethostname().to_str().unwrap_or("<NONE>").to_owned()
}

pub fn mean(data: &[f64]) -> f64 {
    data.iter().sum::<f64>() / data.len() as f64
}

pub fn stddev(data: &[f64]) -> f64 {
    let mean = mean(data);
    data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / data.len() as f64
}

pub fn range(range: Range<f64>, steps: usize) -> impl ParallelIterator<Item = f64> {
    (1..=steps)
        .into_par_iter()
        .map(move |i| range.start + i as f64 * (range.end - range.start) / steps as f64)
}
