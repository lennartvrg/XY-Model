use rustfft::{num_complex::Complex, FftPlanner};

pub fn autocorrelation(data: &[f64]) -> (f64, Vec<f64>) {
    let correlation = normalized_autocorrelation_function(data);
    let tau = 0.5
        + correlation
            .iter()
            .skip(1)
            .take_while(|x| **x > 0.0)
            .sum::<f64>();
    (tau, correlation)
}

fn normalized_autocorrelation_function(data: &[f64]) -> Vec<f64> {
    let mean = data.iter().sum::<f64>() / data.len() as f64;
    let series = data.iter().map(|x| x - mean).collect::<Vec<f64>>();

    let mut planner = FftPlanner::new();
    let fwd = planner.plan_fft_forward(series.len());
    let bwd = planner.plan_fft_inverse(series.len());

    let mut buffer = series.iter().map(complex).collect::<Vec<Complex<f64>>>();
    fwd.process(&mut buffer);

    buffer = buffer.iter().map(fold).collect::<Vec<Complex<f64>>>();
    bwd.process(&mut buffer);

    buffer.iter().map(|x| x.re / buffer[0].norm()).collect()
}

const fn complex(x: &f64) -> Complex<f64> {
    Complex::new(*x, 0.0)
}

fn fold(x: &Complex<f64>) -> Complex<f64> {
    x * x.conj()
}
