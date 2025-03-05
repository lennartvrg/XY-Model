use clap::Parser;
use std::io::{stdout, Write};
use std::ops::Range;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::task::JoinSet;

use crate::algorithm::metropolis::Metropolis;
use crate::lattice::lattice_1d::Lattice1D;
use crate::lattice::lattice_2d::Lattice2D;
use crate::lattice::Lattice;

mod algorithm;
mod analysis;
mod arguments;
mod constants;
mod lattice;
mod storage;
mod utils;

const SWEEPS: usize = 400_000;

const STEPS: usize = 128;

fn split_range(range: Range<f64>, steps: usize) -> impl Iterator<Item = f64> {
    (1..=steps).map(move |i| range.start + i as f64 * (range.end - range.start) / steps as f64)
}

fn temperature_range() -> impl Iterator<Item = f64> {
    split_range(0.0..0.75, 16)
        .chain(split_range(0.75..1.25, 96))
        .chain(split_range(1.25..2.0, 16))
}

async fn simulate<L>(
    id: i32,
    size: usize,
    storage: &mut storage::Storage,
    range: impl Iterator<Item = f64>,
) where
    L: Lattice,
{
    let counter = Arc::new(AtomicUsize::new(1));

    let mut tasks = JoinSet::new();
    for t in range {
        let counter = counter.clone();
        tasks.spawn_blocking(move || {
            let mut rng = rand::rng();
            let mut lattice = L::new(size, 1.0 / t);

            let start = std::time::Instant::now();
            let (energies, magnets) = lattice.metropolis_hastings(&mut rng, SWEEPS);
            let ms = start.elapsed().as_millis();

            let e = analysis::complete(&mut rng, energies);
            let m = analysis::complete(&mut rng, magnets);

            let mut std: std::io::StdoutLock<'_> = stdout().lock();
            let current = counter.fetch_add(1, Ordering::Relaxed);

            write!(std, "\r\t{}D L = {}^2: {}/{}", L::DIM, size, current, STEPS).unwrap();
            std.flush().unwrap();

            let spins = serde_json::to_string(&lattice.spins()).unwrap();
            storage::Configuration::new(t, e, m, spins, ms)
        });
    }

    let results = tasks.join_all().await;
    storage.insert_results(id, L::DIM, size, &results).await;

    println!();
}

async fn simulate_all(args: arguments::Arguments) {
    let mut storage = storage::Storage::connect().await;
    let run = match storage.get_run(args.run_id).await {
        None => storage.create_run().await,
        Some(run) => run,
    };

    println!("Starting XY model simulations for run {}", run.id);
    for size in args.sizes {
        simulate::<Lattice1D>(run.id, size, &mut storage, split_range(0.0..2.0, STEPS)).await;
        simulate::<Lattice2D>(run.id, size, &mut storage, temperature_range()).await;
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
