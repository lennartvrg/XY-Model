use clap::Parser;
use std::io::{stdout, Write};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use rayon::prelude::*;

use crate::algorithm::metropolis::Metropolis;
use crate::lattice::lattice_1d::Lattice1D;
use crate::lattice::lattice_2d::Lattice2D;
use crate::lattice::Lattice;
use crate::storage::Configuration;
use crate::utils::split_range;

mod algorithm;
mod analysis;
mod arguments;
mod constants;
mod lattice;
mod storage;
mod utils;

const SWEEPS: usize = 100_000;

const STEPS: usize = 128;

fn temperature_range() -> impl Iterator<Item = f64> {
    split_range(0.0..0.75, 16)
        .chain(split_range(0.75..1.25, 96))
        .chain(split_range(1.25..2.0, 16))
}

fn simulate_size<L>(size: usize, range: impl IntoParallelIterator<Item = f64>) -> Vec<Configuration>
where
    L: Lattice,
{
    let counter = Arc::new(AtomicUsize::new(1));
    range.into_par_iter().map_init(|| fastrand::Rng::new(), |rng, t| {
        // Initialize lattice
        let mut lattice = L::new(size, 1.0 / t);

        // Perform metropolis_hastings and measure time
        let start = std::time::Instant::now();
        let (energies, magnets) = lattice.metropolis_hastings(rng, SWEEPS);
        let ms = start.elapsed().as_millis();

        // Perform bootstrap analysis on observables
        let e = analysis::complete(rng, energies);
        let m = analysis::complete(rng, magnets);

        // Write debug information
        let mut std: std::io::StdoutLock<'_> = stdout().lock();
        let current = counter.fetch_add(1, Ordering::Relaxed);

        write!(std, "\r\t{}D L = {}^2: {}/{}", L::DIM, size, current, STEPS).unwrap();
        std.flush().unwrap();

        // Returns results
        let spins = serde_json::to_string(lattice.spins()).unwrap();
        Configuration::new(L::DIM, t, e, m, spins, ms)
    }).collect::<Vec<_>>()
}

fn main() {
    let args = arguments::Arguments::parse();
    let mut storage = storage::Storage::connect();

    let run = match storage.get_run(args.run_id) {
        None => storage.create_run(),
        Some(run) => run,
    };

    println!("Starting XY model simulations for run {}", run.id);
    for size in args.sizes {
        let mut results = simulate_size::<Lattice1D>(size, split_range(0.0..2.0, STEPS).collect::<Vec<_>>());
        println!();

        results.append(&mut simulate_size::<Lattice2D>(size, temperature_range().collect::<Vec<_>>()));
        println!();

        storage.insert_results(run.id, size, &results);
    }
}
