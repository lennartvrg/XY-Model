use clap::Parser;
use std::io::{stdout, Write};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::task::JoinSet;

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

fn simulate_single<L>(size: usize, temperature: f64, counter: Arc<AtomicUsize>) -> Configuration
where
    L: Lattice,
{
    // Initialize rng generator and lattice
    let mut rng = fastrand::Rng::new().clone();
    let mut lattice = L::new(size, 1.0 / temperature);

    // Perform metropolis_hastings and measure time
    let start = std::time::Instant::now();
    let (energies, magnets) = lattice.metropolis_hastings(&mut rng, SWEEPS);
    let ms = start.elapsed().as_millis();

    // Perform bootstrap analysis on observables
    let e = analysis::complete(&mut rng, energies);
    let m = analysis::complete(&mut rng, magnets);

    // Write debug information
    let mut std: std::io::StdoutLock<'_> = stdout().lock();
    let current = counter.fetch_add(1, Ordering::Relaxed);

    write!(std, "\r\t{}D L = {}^2: {}/{}", L::DIM, size, current, STEPS).unwrap();
    std.flush().unwrap();

    // Returns results
    let spins = serde_json::to_string(lattice.spins()).unwrap();
    Configuration::new(L::DIM, temperature, e, m, spins, ms)
}

async fn simulate_size<L>(size: usize, range: impl Iterator<Item = f64>) -> Vec<Configuration>
where
    L: Lattice,
{
    let counter = Arc::new(AtomicUsize::new(1));
    let mut tasks = JoinSet::new();
    for t in range {
        let counter = counter.clone();
        tasks.spawn_blocking(move || simulate_single::<L>(size, t, counter.clone()));
    }
    tasks.join_all().await
}

async fn simulate_all(args: arguments::Arguments) {
    let mut storage = storage::Storage::connect().await;
    let run = match storage.get_run(args.run_id).await {
        None => storage.create_run().await,
        Some(run) => run,
    };

    println!("Starting XY model simulations for run {}", run.id);
    for size in args.sizes {
        let mut results = simulate_size::<Lattice1D>(size, split_range(0.0..2.0, STEPS)).await;
        println!();

        results.append(&mut simulate_size::<Lattice2D>(size, temperature_range()).await);
        println!();

        storage.insert_results(run.id, size, &results).await;
    }
}

fn main() {
    tokio::runtime::Builder::new_multi_thread()
        .max_blocking_threads(std::thread::available_parallelism().unwrap().get())
        .enable_all()
        .build()
        .unwrap()
        .block_on(async { simulate_all(arguments::Arguments::parse()).await })
}
