use clap::Parser;
use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use gethostname::gethostname;

use crate::algorithm::metropolis::Metropolis;
use crate::lattice::lattice_1d::Lattice1D;
use crate::lattice::lattice_2d::Lattice2D;
use crate::lattice::Lattice;
use crate::storage::Configuration;
use crate::utils::range;

mod algorithm;
mod analysis;
mod arguments;
mod constants;
mod lattice;
mod storage;
mod utils;

const STEPS: usize = 128;

const SWEEPS: usize = 1_000_000;

const RESAMPLES: usize = 100_000;

fn weighted_range() -> impl ParallelIterator<Item = f64> {
    range(0.0..0.75, 16)
        .chain(range(0.75..1.25, 96))
        .chain(range(1.25..2.0, 16))
}

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
    let time_mc = start.elapsed().as_millis();

    // Perform bootstrap analysis on observables
    let e = analysis::complete(rng, energies, RESAMPLES);
    let m = analysis::complete(rng, magnets, RESAMPLES);

    // Write console information
    let current = counter.fetch_add(1, Ordering::Relaxed);
    let _ = println!("[{:?}]\tL{}: {}/{}", gethostname(), size, current, STEPS);

    // Serialize spins
    let time_boot = start.elapsed().as_millis() - time_mc;
    Configuration::new(&lattice, e, m, time_mc, time_boot)
}

fn simulate<L, I>(size: usize, range: I) -> Vec<Configuration>
where
    L: Lattice,
    I: ParallelIterator<Item = f64>,
{
    let counter = Arc::new(AtomicUsize::new(1));
    let results = range.map_init(fastrand::Rng::new, |rng, t| {
        simulate_size::<L>(counter.clone(), size, rng, t)
    }).collect::<Vec<_>>();

    println!();
    results
}

fn main() -> Result<(), rusqlite::Error> {
    // Parse CLI arguments and connect to SQLite database
    let args = arguments::Arguments::parse();
    let mut storage = storage::Storage::connect()?;

    // Some debug information for SBATCH
    match std::thread::available_parallelism() {
        Ok(v) => println!("[{:?}] System has {} threads", gethostname(), v),
        Err(v) => println!("[{:?}] Could not fetch system thread count: {}", gethostname(), v),
    };

    // Some debug information for SBATCH
    match std::env::var("RAYON_NUM_THREADS") {
        Ok(v) => println!("[{:?}] RAYON uses {} threads", gethostname(), v),
        Err(v) => println!("[{:?}] Could not fetch RAYON thread count: {}", gethostname(), v),
    }

    // Fetches or creates the current run
    let run = match storage.get_run(args.run_id)? {
        None => storage.create_run()?,
        Some(run) => run,
    };

    // Simulate 1D lattice and store results in SQlite database
    if !args.one.is_empty() {
        println!("[{:?}] Starting 1D XY model simulations for run {}", gethostname(), run.id);
        for size in args.one {
            let configurations = simulate::<Lattice1D, _>(size, range(0.0..2.0, STEPS));
            storage.insert_results(run.id, size, &configurations)?;
        }
    }

    // Simulate 1D lattice and store results in SQlite database
    if !args.two.is_empty() {
        println!("[{:?}] Starting 2D XY model simulations for run {}", gethostname(), run.id);
        for size in args.two {
            let configurations = simulate::<Lattice2D, _>(size, weighted_range());
            storage.insert_results(run.id, size, &configurations)?;
        }
    }
    Ok(())
}
