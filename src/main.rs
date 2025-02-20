use sqlx::{Acquire, Sqlite, Transaction};
use std::io::{stdout, Write};
use std::iter::Map;
use std::ops::RangeInclusive;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::task::JoinSet;

use crate::algorithm::metropolis::Metropolis;
use crate::lattice::lattice_2d::Lattice2D;
use crate::lattice::Lattice;

mod algorithm;
mod analysis;
mod constants;
mod lattice;
mod storage;
mod utils;

const SWEEPS: usize = 300_000;

const SCAN_STEPS: usize = 128;

fn scan_range() -> Map<RangeInclusive<usize>, fn(usize) -> f64> {
    (1..=SCAN_STEPS).map(|i| i as f64 * 2.0 / SCAN_STEPS as f64)
}

async fn simulate<const N: usize>(tx: &mut Transaction<'_, Sqlite>, id: i32) {
    let counter = Arc::new(AtomicUsize::new(1));

    let mut tasks = JoinSet::new();
    for t in scan_range() {
        let counter = counter.clone();
        tasks.spawn_blocking(move || {
            let mut rng = rand::rng();
            let mut lattice = Lattice2D::<N>::new(1.0 / t);

            let (energies, magnets) = lattice.metropolis_hastings(&mut rng, SWEEPS);
            let (e, e_sqr) = analysis::complete(energies);
            let (m, m_sqr) = analysis::complete(magnets);

            let mut std: std::io::StdoutLock<'_> = stdout().lock();
            let current = counter.fetch_add(1, Ordering::Relaxed);

            write!(std, "\r\tL = {}^2: {}/{}", N, current, scan_range().count()).unwrap();
            std.flush().unwrap();

            (t, e, e_sqr, m, m_sqr)
        });
    }

    while let Some(Ok((t, e, e_sqr, m, m_sqr))) = tasks.join_next().await {
        let cv = Lattice2D::<N>::specific_heat_per_spin(e.mean, e_sqr.mean, t);
        let xs = Lattice2D::<N>::magnetic_susceptibility_per_spin(m.mean, m_sqr.mean, t);

        sqlx::query("
            INSERT INTO configurations (run_id, size, temperature, energy, energy_std, energy_tau, energy_sqr, energy_sqr_std, energy_sqr_tau, magnet, magnet_std, magnet_tau, magnet_sqr, magnet_sqr_std, magnet_sqr_tau, specific_heat, magnet_suscept)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
        ").bind(id).bind(N as i32).bind(t).bind(e.mean).bind(e.stddev).bind(e.tau).bind(e_sqr.mean).bind(e_sqr.stddev).bind(e_sqr.tau)
            .bind(m.mean).bind(m.stddev).bind(m.tau).bind(m_sqr.mean).bind(m_sqr.stddev).bind(m_sqr.tau).bind(cv).bind(xs).execute(tx.as_mut()).await.unwrap();
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
    //simulate::<512>(&mut tx, run.id).await;
    //simulate::<1024>(&mut tx, run.id).await;
    //simulate::<2048>(&mut tx, run.id).await;

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
