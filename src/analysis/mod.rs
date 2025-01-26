mod autocorrelation;
mod bootstrap;

pub use autocorrelation::autocorrelation;
pub use bootstrap::bootstrap;
use crate::analysis::bootstrap::bootstrap_blocked;

const SAMPLES: usize = 20_000;

pub struct Observable {
    pub data: Vec<f64>,
    pub mean: f64,
    pub stddev: f64,
    pub tau: f64,
}

impl Observable {
    pub const fn new(data: Vec<f64>, mean: f64, stddev: f64, tau: f64) -> Self {
        Self {
            data,
            mean,
            stddev,
            tau,
        }
    }
}

pub fn complete<F>(drv: F, data: Vec<f64>, temperature: f64) -> (Observable, Observable, Observable)
where
    F: Fn(f64, f64, f64) -> f64,
{
    let data_sqr = data.iter().map(|x| x.powi(2)).collect::<Vec<f64>>();

    let (tau, _) = autocorrelation(&data);
    let (mean, stddev) = bootstrap(&data, tau, SAMPLES);

    let (tau_sqr, _) = autocorrelation(&data_sqr);
    let (mean_sqr, stddev_sqr) = bootstrap(&data_sqr, tau_sqr, SAMPLES);

    let tau_max = f64::max(tau, tau_sqr);
    let data_prep = bootstrap::thermalize_and_block(&data, tau_max);
    let data_sqr_prep = bootstrap::thermalize_and_block(&data_sqr, tau_max);
    let data_derived = data_prep
        .iter()
        .zip(data_sqr_prep.iter())
        .map(|(x, y)| drv(*x, *y, temperature))
        .collect::<Vec<f64>>();
    let (mean_drv, stddev_drv) = bootstrap_blocked(&data_derived, SAMPLES);

    (
        Observable::new(data, mean, stddev, tau),
        Observable::new(data_sqr, mean_sqr, stddev_sqr, tau_sqr),
        Observable::new(data_derived, mean_drv, stddev_drv, tau_max),
    )
}
