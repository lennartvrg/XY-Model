mod autocorrelation;
mod bootstrap;

pub use autocorrelation::autocorrelation;
pub use bootstrap::bootstrap;

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

pub fn complete(data: Vec<f64>) -> (Observable, Observable) {
    let data_sqr = data.iter().map(|x| x.powi(2)).collect::<Vec<f64>>();

    let (tau, _) = autocorrelation(&data);
    let (mean, stddev) = bootstrap(&data, tau, SAMPLES);

    let (tau_sqr, _) = autocorrelation(&data_sqr);
    let (mean_sqr, stddev_sqr) = bootstrap(&data_sqr, tau_sqr, SAMPLES);

    (
        Observable::new(data, mean, stddev, tau),
        Observable::new(data_sqr, mean_sqr, stddev_sqr, tau_sqr),
    )
}
