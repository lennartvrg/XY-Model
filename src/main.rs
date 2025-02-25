use clap::Parser;
use std::io::{stdout, Write};
use std::ops::Range;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::task::JoinSet;

use crate::algorithm::metropolis::Metropolis;
use crate::lattice::lattice_2d::Lattice2D;

mod algorithm;
mod analysis;
mod arguments;
mod constants;
mod lattice;
mod storage;
mod utils;

const SWEEPS: usize = 400_000;

const SCAN_STEPS: usize = 128;

fn split_range(range: Range<f64>, steps: usize) -> Vec<f64> {
    (1..=steps)
        .map(move |i| range.start + i as f64 * (range.end - range.start) / steps as f64)
        .collect::<Vec<_>>()
}

fn temperature_range() -> impl Iterator<Item = f64> {
    split_range(0.0..0.75, 24)
        .into_iter()
        .chain(split_range(0.75..1.25, 80))
        .chain(split_range(1.25..2.0, 24))
}

async fn simulate(size: usize, storage: &mut storage::Storage, id: i32) {
    let counter = Arc::new(AtomicUsize::new(1));

    let mut tasks = JoinSet::new();
    for t in temperature_range() {
        let counter = counter.clone();
        tasks.spawn_blocking(move || {
            let mut rng = rand::rng();
            let mut lattice = Lattice2D::new(size, 1.0 / t);

            let start = std::time::Instant::now();
            let (energies, magnets) = lattice.metropolis_hastings(&mut rng, SWEEPS);

            let e = analysis::complete(&mut rng, energies);
            let m = analysis::complete(&mut rng, magnets);

            let mut std: std::io::StdoutLock<'_> = stdout().lock();
            let current = counter.fetch_add(1, Ordering::Relaxed);

            write!(std, "\r\tL = {}^2: {}/{}", size, current, SCAN_STEPS).unwrap();
            std.flush().unwrap();

            let spins = serde_json::to_string(&lattice.spins()).unwrap();
            let ms = start.elapsed().as_millis();

            storage::Configuration::new(t, e, m, spins, ms)
        });
    }

    storage.insert_results(id, size, &tasks.join_all().await).await;
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
        simulate(size, &mut storage, run.id).await;
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
