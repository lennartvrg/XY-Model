use std::time::SystemTime;

pub const fn div_rem<const N: usize>(i: usize) -> (usize, usize) {
    (i / N, i % N)
}

pub fn unix_time() -> i32 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i32
}

pub fn mean(data: &[f64]) -> f64 {
    data.iter().sum::<f64>() / data.len() as f64
}

pub fn stddev(data: &[f64]) -> f64 {
    let mean = mean(data);
    data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / data.len() as f64
}
