use clap::Parser;
use rayon::prelude::*;
use std::io::{stdout, Write};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use crate::algorithm::metropolis::Metropolis;
use crate::lattice::lattice_1d::Lattice1D;
use crate::lattice::lattice_2d::Lattice2D;
use crate::lattice::Lattice;
use crate::storage::Configuration;
use crate::utils::parallel_range;

mod algorithm;
mod analysis;
mod arguments;
mod constants;
mod lattice;
mod storage;
mod utils;

const STEPS: usize = 256;

const SWEEPS: usize = 400_000;

const RESAMPLES: usize = 20_000;

fn simulate_size<L>(
    counter: Arc<AtomicUsize>,
    size: usize,
    rng: &mut fastrand::Rng,
    t: f64,
) -> Configuration
where
    L: Lattice,
{
    // Initialize lattice
    let mut lattice = L::new(size, 1.0 / t);

    // Perform metropolis_hastings and measure time
    let start = std::time::Instant::now();
    let (energies, magnets) = lattice.metropolis_hastings(rng, SWEEPS);

    // Perform bootstrap analysis on observables
    let e = analysis::complete(rng, energies, RESAMPLES);
    let m = analysis::complete(rng, magnets, RESAMPLES);

    // Write console information
    let mut std: std::io::StdoutLock<'_> = stdout().lock();
    let current = counter.fetch_add(1, Ordering::Relaxed);

    write!(std, "\r\tD{} L{}: {}/{}", L::DIM, size, current, STEPS).unwrap();
    std.flush().unwrap();

    // Serialize spins
    let spins = serde_json::to_string(lattice.spins()).unwrap();
    let ms = start.elapsed().as_millis();

    Configuration::new(L::DIM, t, e, m, spins, ms)
}

fn simulate<L>(size: usize) -> Vec<Configuration>
where
    L: Lattice,
{
    let counter = Arc::new(AtomicUsize::new(1));
    parallel_range(0.0..2.0, STEPS).map_init(fastrand::Rng::new, |rng, t| {
            simulate_size::<L>(counter.clone(), size, rng, t)
    }).collect::<Vec<_>>()
}

fn main() {
    // Parse CLI arguments and connect to SQLite database
    let args = arguments::Arguments::parse();
    let mut storage = storage::Storage::connect();

    // Some debug information for SBATCH
    match std::thread::available_parallelism() {
        Ok(v) => println!("System has {} threads", v),
        Err(v) => println!("Could not fetch system thread count: {}", v)
    };

    // Some debug information for SBATCH
    match std::env::var("RAYON_NUM_THREADS") {
        Ok(v) => println!("RAYON uses {} threads", v),
        Err(v) => println!("Could not fetch RAYON thread count: {}", v)
    }

    let run = match storage.get_run(args.run_id) {
        None => storage.create_run(),
        Some(run) => run,
    };

    println!("Starting XY model simulations for run {}", run.id);
    for size in args.sizes {
        // Simulate 1D lattice
        let mut results = simulate::<Lattice1D>(size);
        println!();

        // Simulate 2D lattice
        results.append(&mut simulate::<Lattice2D>(size));
        println!();

        // Store combined results in SQlite database
        storage.insert_results(run.id, size, &results);
    }
}
