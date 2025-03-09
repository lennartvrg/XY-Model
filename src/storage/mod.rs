mod types;

use rusqlite::Connection;
use crate::lattice::lattice_2d::Lattice2D;
use crate::lattice::Lattice;
use crate::utils;
pub use types::*;

const MIGRATION: &str = include_str!("../../migrations/20250103215725_schema.sql");

pub struct Storage(Connection);

impl Storage {
    pub fn connect() -> Self {
        let conn = Connection::open("output.sqlite?mode=rwc").unwrap();
        conn.execute(MIGRATION, ()).unwrap();
        Self(conn)
    }

    pub fn get_run(&mut self, id: Option<i32>) -> Option<Run> {
        let mut stmt = self.0.prepare("SELECT id FROM runs WHERE id = $1").unwrap();
        let mut rows = stmt.query_map((id?,), |row| Ok(Run {
            id: row.get(0)?
        })).unwrap();
        rows.nth(0).transpose().unwrap()
    }

    pub fn create_run(&mut self) -> Run {
        let mut stmt = self.0.prepare("INSERT INTO runs (created_at) VALUES ($1) RETURNING id").unwrap();
        let mut rows = stmt.query_map((utils::unix_time(),), |row| Ok(Run {
            id: row.get(0)?
        })).unwrap();
        rows.nth(0).transpose().unwrap().unwrap()
    }

    pub fn insert_results(&mut self, id: i32, size: usize, configurations: &[Configuration]) {
        let tx = self.0.transaction().unwrap();
        let mut stmt = tx.prepare("
            INSERT INTO results (run_id, dimension, size, temperature, energy, energy_std, energy_tau, energy_sqr, energy_sqr_std, energy_sqr_tau, magnet, magnet_std, magnet_tau, magnet_sqr, magnet_sqr_std, magnet_sqr_tau, specific_heat, specific_heat_std, magnet_suscept, magnet_suscept_std, spins, duration)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, json($21), $22)
        ").unwrap();

        for cfg in configurations {
            let (cv, cv_std) = Lattice2D::specific_heat_per_spin(
                cfg.energy.mean,
                cfg.energy.stddev,
                cfg.energy.sqr_mean,
                cfg.energy.sqr_stddev,
                cfg.temperature,
            );

            let (xs, xs_std) = Lattice2D::magnetic_susceptibility_per_spin(
                cfg.magnetization.mean,
                cfg.magnetization.stddev,
                cfg.magnetization.sqr_mean,
                cfg.magnetization.sqr_stddev,
                cfg.temperature,
            );

            stmt.execute(rusqlite::params![
                id, cfg.dimension as i32, size as i32, cfg.temperature,
                cfg.energy.mean, cfg.energy.stddev, cfg.energy.tau,
                cfg.energy.sqr_mean, cfg.energy.sqr_stddev, cfg.energy.sqr_tau,
                cfg.magnetization.mean, cfg.magnetization.stddev, cfg.magnetization.tau,
                cfg.magnetization.sqr_mean, cfg.magnetization.sqr_stddev, cfg.magnetization.sqr_tau,
                cv, cv_std, xs, xs_std,  &cfg.spins, cfg.time as i32
            ]).unwrap();
        }
        drop(stmt);
        tx.commit().unwrap();
    }
}
