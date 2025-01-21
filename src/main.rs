use sqlx::{Acquire, Sqlite, Transaction};
use std::io::{stdout, Write};
use std::ops::RangeInclusive;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::task::JoinSet;

use crate::algorithm::metropolis::Metropolis;
use crate::lattice::lattice_2d::Lattice2D;

mod algorithm;
mod analysis;
mod constants;
mod lattice;
mod observables;
mod storage;
mod utils;

const SWEEPS: usize = 64_000;

const SCAN: RangeInclusive<usize> = 1..=100;

async fn simulate<const N: usize>(tx: &mut Transaction<'_, Sqlite>, id: i32) {
    let counter = Arc::new(AtomicUsize::new(1));

    let mut tasks = JoinSet::new();
    for t in SCAN.map(|i| 2.5 * i as f64 / 100.0) {
        let counter = counter.clone();
        tasks.spawn_blocking(move || {
            let mut rng = rand::rng();
            let mut lattice = Lattice2D::<N>::new(1.0 / t);

            let (energies, _) = lattice.metropolis_hastings(&mut rng, SWEEPS);
            let (mean, stddev, tau) = analysis::complete(&energies);

            let mut std: std::io::StdoutLock<'_> = stdout().lock();
            write!(
                std,
                "\r\tLattice {}x{}: {}/{}",
                N,
                N,
                counter.fetch_add(1, Ordering::Relaxed),
                SCAN.count()
            )
            .unwrap();
            std.flush().unwrap();

            (t, mean, stddev, tau)
        });
    }

    while let Some(Ok((t, mean, std, tau))) = tasks.join_next().await {
        sqlx::query("INSERT INTO configurations (run_id, size, temperature, energy, energy_std, energy_tau) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *")
            .bind(id).bind(N as i32).bind(t).bind(mean).bind(std).bind(tau).fetch_one(tx.as_mut()).await.unwrap();
    }
    println!();
}

async fn simulate_all() {
    let mut conn = storage::connect().await;
    let mut tx = conn.begin().await.unwrap();

    let run =
        sqlx::query_as::<_, storage::Run>("INSERT INTO runs (created_at) VALUES ($1) RETURNING *")
            .bind(utils::unix_time())
            .fetch_one(&mut *tx)
            .await
            .unwrap();

    println!("Starting XY model simulations");
    simulate::<8>(&mut tx, run.id).await;
    simulate::<16>(&mut tx, run.id).await;
    simulate::<32>(&mut tx, run.id).await;
    simulate::<64>(&mut tx, run.id).await;
    simulate::<128>(&mut tx, run.id).await;
    simulate::<256>(&mut tx, run.id).await;

    tx.commit().await.unwrap();
}

fn main() {
    tokio::runtime::Builder::new_multi_thread()
        .max_blocking_threads(std::thread::available_parallelism().unwrap().get())
        .enable_all()
        .build()
        .unwrap()
        .block_on(async { simulate_all().await })
}
