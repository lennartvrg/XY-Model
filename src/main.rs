use clap::Parser;
use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use crate::algorithm::{Algorithm, Metropolis};
use crate::lattice::{Lattice, Lattice1D, Lattice2D};
use crate::storage::Configuration;
use crate::utils::{host, range};

mod algorithm;
mod analysis;
mod arguments;
mod constants;
mod lattice;
mod storage;
mod utils;

/// The number of steps the temperature will be divided into.
const STEPS: usize = 64;

/// The number of MC sweeps for the simulation.
const SWEEPS: usize = 1_000_000;

/// The number of resamples (B) for the bootstrap analysis.
const RESAMPLES: usize = 200_000;

/// The maximum depth used for the zooming into the temperature range.
const MAX_DEPTH: usize = 2;

/// The total number of distinct temperature values.
const TOTAL: usize = MAX_DEPTH * STEPS;

/// Simulates the XY model for a given lattice size and temperature. This will do the
/// metropolis hastings algorithm and the bootstrap analysis on the observables. Returns
/// the final configuration with the following observables. e, e^2, m, m^2, Cv, Xs.
fn simulate_size<A, L>(
    counter: Arc<AtomicUsize>,
    size: usize,
    rng: &mut fastrand::Rng,
    t: f64,
) -> Configuration
where
    A: Algorithm + ?Sized,
    L: Lattice,
{
    // Initialize lattice
    let mut lattice = L::new(size, t.recip());

    // Perform metropolis_hastings and measure time
    let start = std::time::Instant::now();
    let (energies, magnets) = lattice.simulate(rng, SWEEPS);
    let time_mc = start.elapsed().as_millis();

    // Perform bootstrap analysis on observables
    let e = analysis::complete(rng, energies, RESAMPLES);
    let m = analysis::complete(rng, magnets, RESAMPLES);

    // Write console information
    let current = counter.fetch_add(1, Ordering::Relaxed);
    println!("[{}] D{} L{}: {}/{}", host(), L::DIM, size, current, TOTAL);

    // Serialize spins
    let time_boot = start.elapsed().as_millis() - time_mc;
    Configuration::new(&lattice, e, m, time_mc, time_boot)
}

fn simulate<A, L>(size: usize) -> Vec<Configuration>
where
    A: Algorithm + ?Sized,
    L: Lattice,
{
    // Result set and counter
    let mut results = Vec::new();
    let counter = Arc::new(AtomicUsize::new(1));

    // Create initial range and loop trough depth
    let (mut range, mut stride) = range(0.0..3.0, STEPS);
    for _ in 0..MAX_DEPTH {
        // Simulate lattice and append results
        let configs = range.map_init(fastrand::Rng::new, |rng, t| {
            simulate_size::<A, L>(counter.clone(), size, rng, t)
        });
        results.append(&mut configs.collect::<Vec<_>>());

        // Get top magnetic susceptibility
        let Some(cfg) = results.iter().filter(Configuration::relevant_tmp).max_by(Configuration::cmp) else {
            break;
        };

        // Generate next range
        (range, stride) = utils::range(
            (cfg.temperature - 3.0 * stride)..(cfg.temperature + 3.0 * stride),
            STEPS,
        );
    }

    // Order by temperature and remove duplicates
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
    while let Some((dimension, size)) = storage.next_allocation(run.id)? {
        println!("[{}] Next allocation: D{} L{}", host(), dimension, size);
        let configurations = match dimension {
            1 => simulate::<dyn Metropolis, Lattice1D>(size),
            _ => simulate::<dyn Metropolis, Lattice2D>(size),
        };
        storage.insert_results(run.id, dimension, size, &configurations)?;
    }
    Ok(())
}
