mod autocorrelation;
mod bootstrap;

pub use autocorrelation::autocorrelation;
pub use bootstrap::bootstrap;

use crate::observables::Observable;

pub fn complete(data: &[impl Observable]) -> (f64, f64, f64) {
    let (tau, _) = autocorrelation(&data);
    let (mean, stddev) = bootstrap(&data, tau, 500);
    (mean, stddev, tau)
}
