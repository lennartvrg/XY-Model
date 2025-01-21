use crate::observables::Observable;
use rustfft::{num_complex::Complex, FftPlanner};

pub fn autocorrelation(data: &[impl Observable]) -> (f64, Vec<f64>) {
    let correlation = normalized_autocorrelation_function(data);
    let tau = 0.5
        + correlation
            .iter()
            .skip(1)
            .take_while(|x| **x > 0.0)
            .sum::<f64>();
    (tau, correlation)
}

fn normalized_autocorrelation_function(data: &[impl Observable]) -> Vec<f64> {
    let mean = data.iter().map(|x| x.norm_per_site()).sum::<f64>() / data.len() as f64;
    let series = data
        .iter()
        .map(|x| x.norm_per_site() - mean)
        .collect::<Vec<f64>>();

    let mut planner = FftPlanner::new();
    let fwd = planner.plan_fft_forward(series.len());
    let bwd = planner.plan_fft_inverse(series.len());

    let mut buffer = series
        .iter()
        .map(|x| Complex::new(*x, 0.0))
        .collect::<Vec<Complex<f64>>>();
    fwd.process(&mut buffer);

    buffer = buffer
        .iter()
        .map(|x| x * x.conj())
        .collect::<Vec<Complex<f64>>>();
    bwd.process(&mut buffer);

    buffer.iter().map(|x| x.re / buffer[0].norm()).collect()
}
