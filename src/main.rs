use clap::Parser;
use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use crate::algorithm::metropolis::Metropolis;
use crate::lattice::lattice_1d::Lattice1D;
use crate::lattice::lattice_2d::Lattice2D;
use crate::lattice::Lattice;
use crate::storage::Configuration;
use crate::utils::{host, range};

mod algorithm;
mod analysis;
mod arguments;
mod constants;
mod lattice;
mod storage;
mod utils;

const STEPS: usize = 32;

const SWEEPS: usize = 800_000;

const RESAMPLES: usize = 160_000;

const MAX_DEPTH: usize = 2;

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
    println!("[{}] D{} L{}: {}/{}", host(), L::DIM, size, current, MAX_DEPTH * STEPS);

    // Serialize spins
    let time_boot = start.elapsed().as_millis() - time_mc;
    Configuration::new(&lattice, e, m, time_mc, time_boot)
}

fn simulate<L>(size: usize) -> Vec<Configuration>
where
    L: Lattice
{
    // Result set and counter
    let mut results = Vec::new();
    let counter = Arc::new(AtomicUsize::new(1));

    // Create inital range and loop trough depth
    let mut range = range(0.0..2.0, STEPS);
    for _ in 0..MAX_DEPTH {
        // Simulate lattice and append results
        results.append(&mut range.map_init(fastrand::Rng::new, |rng, t| {
            simulate_size::<L>(counter.clone(), size, rng, t)
        }).collect::<Vec<_>>());

        // Order by magnetic susceptibility
        results.sort_by(|a, b| a.xs.0.total_cmp(&b.xs.0));
        let top = results.iter().rev().take(5).cloned().collect::<Vec<_>>();

        // Get the lower bound of the next range
        let Some(lower) = top.iter().min_by(Configuration::temp_cmp) else {
            break;
        };

        // Get the upper bound of the next range
        let Some(upper) = top.iter().max_by(Configuration::temp_cmp) else {
            break;
        };

        // Generate next range
        range = utils::range(lower.temperature..upper.temperature, STEPS);
    }

    results
}

fn main() -> Result<(), rusqlite::Error> {
    // Parse CLI arguments and connect to SQLite database
    let args = arguments::Arguments::parse();
    let mut storage = storage::Storage::connect()?;

    // Some debug information for SBATCH
    match std::thread::available_parallelism() {
        Ok(v) => println!("[{}] System has {} threads", host(), v),
        Err(v) => println!("[{}] Could not fetch system thread count: {}", host(), v),
    };

    // Some debug information for SBATCH
    match std::env::var("RAYON_NUM_THREADS") {
        Ok(v) => println!("[{}] RAYON uses {} threads", host(), v),
        Err(v) => println!("[{}] Could not fetch RAYON thread count: {}", host(), v),
    }

    // Fetches or creates the current run
    let run = match storage.get_run(args.run_id)? {
        None => storage.create_run()?,
        Some(run) => run,
    };

    // Ensure allocations are registered
    storage.ensure_allocations(run.id, &args.one, &args.two)?;

    // While a next allocation is available => process it
    while let Some((dimension, size)) = storage.next_allocation(run.id, &host())? {
        println!("[{}] Next allocation: D{} L{}", host(), dimension, size);
        let configurations = match dimension {
            1 => simulate::<Lattice1D>(size),
            _ => simulate::<Lattice2D>(size),
        };
        storage.insert_results(run.id, size, &configurations)?;
    }
    Ok(())
}
