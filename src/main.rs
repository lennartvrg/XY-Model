use std::io::{stdout, Write};
use std::ops::RangeInclusive;
use sqlx::{Acquire, QueryBuilder, Sqlite, Transaction};
use tokio::task::JoinSet;

use crate::algorithm::metropolis::Metropolis;
use crate::lattice::lattice_2d::Lattice2D;
use crate::observables::{EnergyObservable, MagnetizationObservable, Observable};

mod algorithm;
mod constants;
mod lattice;
mod utils;
mod observables;
mod storage;

const SWEEPS: usize = 64_000;

const SCAN: RangeInclusive<usize> = 1..=125;

async fn simulate<const N: usize>(tx: &mut Transaction<'_, Sqlite>, id: i32) {
    let mut tasks = JoinSet::new();
    for (i, t) in SCAN.map(|i| (2 * i) as f64 / 100.0).enumerate() {
        tasks.spawn_blocking(move || {
            let mut rng = rand::rng();
            let mut lattice = Lattice2D::<N>::new(1.0 / t);

            let observables = lattice.metropolis_hastings(&mut rng, SWEEPS);
            let mut std: std::io::StdoutLock<'_> = stdout().lock();

            write!(std, "\r\tLattice {}x{}: {}/{}", N, N, i + 1, SCAN.count()).unwrap();
            std.flush().unwrap();

            (t, observables)
        });
    }

    while let Some(Ok((t, observables))) = tasks.join_next().await {
        let cfg = sqlx::query_as::<_, storage::Configuration>("INSERT INTO configurations (run_id, size, temperature) VALUES ($1, $2, $3) RETURNING *")
            .bind(id).bind(N as i32).bind(t).fetch_one(tx.as_mut()).await.unwrap();

        let mut index = 0;
        for chunk in observables.chunks(32766 / 4) {
            let mut query = QueryBuilder::<Sqlite>::new("INSERT INTO observables (configuration_id, sequence_id, e, m) ");

            query.push_values(chunk, |mut b, v| {
                b.push_bind(cfg.id).push_bind(index).push_bind(v.0.norm_per_site()).push_bind(v.1.norm_per_site());
                index += 1;
            });

            query.build().execute(tx.as_mut()).await.unwrap();
        }
    }
    println!();
}

async fn simulate_all() {
    let mut conn = storage::connect().await;
    let mut tx = conn.begin().await.unwrap();

    let run = sqlx::query_as::<_, storage::Run>("INSERT INTO runs (created_at) VALUES ($1) RETURNING *")
        .bind(utils::unix_time()).fetch_one(&mut *tx).await.unwrap();

    println!("Starting XY model simulations");
    simulate::<8>(&mut tx, run.id).await;
    simulate::<16>(&mut tx, run.id).await;
    simulate::<32>(&mut tx, run.id).await;
    simulate::<64>(&mut tx, run.id).await;

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
