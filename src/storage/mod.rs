use sqlx::migrate::Migrator;
use sqlx::{Connection, SqliteConnection};

mod types;

use crate::lattice::lattice_2d::Lattice2D;
use crate::lattice::Lattice;
use crate::utils;
pub use types::*;

static MIGRATOR: Migrator = sqlx::migrate!();

pub struct Storage(SqliteConnection);

impl Storage {
    pub async fn connect() -> Self {
        let mut conn = SqliteConnection::connect("output.sqlite?mode=rwc")
            .await
            .unwrap();
        MIGRATOR.run(&mut conn).await.unwrap();
        Self(conn)
    }

    pub async fn get_run(&mut self, id: Option<i32>) -> Option<Run> {
        sqlx::query_as::<_, Run>("SELECT * FROM runs WHERE id = $1")
            .bind(id?)
            .fetch_optional(&mut self.0)
            .await
            .unwrap()
    }

    pub async fn create_run(&mut self) -> Run {
        sqlx::query_as::<_, Run>("INSERT INTO runs (created_at) VALUES ($1) RETURNING *")
            .bind(utils::unix_time())
            .fetch_one(&mut self.0)
            .await
            .unwrap()
    }

    pub async fn insert_results(&mut self, id: i32, size: usize, configurations: &[Configuration]) {
        let mut tx = self.0.begin().await.unwrap();
        for cfg in configurations {
            let cv = Lattice2D::specific_heat_per_spin(
                cfg.energy.mean,
                cfg.energy.sqr_mean,
                cfg.temperature,
            );

            let xs = Lattice2D::magnetic_susceptibility_per_spin(
                cfg.magnetization.mean,
                cfg.magnetization.sqr_mean,
                cfg.temperature,
            );

            sqlx::query("
                INSERT INTO configurations (run_id, size, temperature, energy, energy_std, energy_tau, energy_sqr, energy_sqr_std, energy_sqr_tau, magnet, magnet_std, magnet_tau, magnet_sqr, magnet_sqr_std, magnet_sqr_tau, specific_heat, magnet_suscept, spins, duration)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, json($19))
            ").bind(id).bind(size as i32).bind(cfg.temperature)
                .bind(cfg.energy.mean).bind(cfg.energy.stddev).bind(cfg.energy.tau)
                .bind(cfg.energy.sqr_mean).bind(cfg.energy.sqr_stddev).bind(cfg.energy.sqr_tau)
                .bind(cfg.magnetization.mean).bind(cfg.magnetization.stddev).bind(cfg.magnetization.tau)
                .bind(cfg.magnetization.sqr_mean).bind(cfg.magnetization.sqr_stddev).bind(cfg.magnetization.sqr_tau)
                .bind(cv).bind(xs).bind(&cfg.spins).bind(cfg.time as i32).execute(tx.as_mut()).await.unwrap();
        }
        tx.commit().await.unwrap();
    }
}
