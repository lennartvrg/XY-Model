mod autocorrelation;
mod bootstrap;

pub use autocorrelation::autocorrelation;
pub use bootstrap::bootstrap;

#[derive(Clone)]
pub struct Observable {
    pub mean: f64,
    pub stddev: f64,
    pub tau: f64,

    pub sqr_mean: f64,
    pub sqr_stddev: f64,
    pub sqr_tau: f64,
}

impl Observable {
    pub const fn new(
        mean: f64,
        stddev: f64,
        tau: f64,
        sqr_mean: f64,
        sqr_stddev: f64,
        sqr_tau: f64,
    ) -> Self {
        Self {
            mean,
            stddev,
            tau,
            sqr_mean,
            sqr_stddev,
            sqr_tau,
        }
    }
}

pub fn complete(rng: &mut fastrand::Rng, data: Vec<f64>, resamples: usize) -> Observable {
    let data_sqr = data.iter().map(|x| x.powi(2)).collect::<Vec<f64>>();

    let (tau, _) = autocorrelation(&data);
    let (mean, stddev) = bootstrap(rng, &data, tau, resamples);

    let (tau_sqr, _) = autocorrelation(&data_sqr);
    let (mean_sqr, stddev_sqr) = bootstrap(rng, &data_sqr, tau_sqr, resamples);

    Observable::new(mean, stddev, tau, mean_sqr, stddev_sqr, tau_sqr)
}
