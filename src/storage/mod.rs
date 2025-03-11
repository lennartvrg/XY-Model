use rusqlite::Connection;
use rusqlite::TransactionBehavior::Immediate;

mod types;

use crate::utils;
pub use types::*;

const MIGRATION: &str = include_str!("../../migrations/20250103215725_schema.sql");

pub struct Storage(Connection);

impl Storage {
    pub fn connect() -> Result<Self, rusqlite::Error> {
        let conn = Connection::open("output.sqlite")?;
        conn.execute_batch(MIGRATION)?;
        Ok(Self(conn))
    }

    pub fn get_run(&mut self, id: Option<i32>) -> Result<Option<Run>, rusqlite::Error> {
        let params = match id {
            None => return Ok(None),
            Some(v) => (v,),
        };

        let mut stmt = self.0.prepare("SELECT * FROM runs WHERE id = $1")?;
        let result = match stmt.query_map(params, Self::row_to_run)?.next() {
            Some(v) => Ok(Some(v?)),
            _ => Ok(None),
        };
        result
    }

    pub fn create_run(&mut self) -> Result<Run, rusqlite::Error> {
        let tx = self.0.transaction()?;
        let params = (utils::unix_time().unwrap_or_default(),);

        let mut stmt = tx.prepare("INSERT INTO runs (created_at) VALUES ($1) RETURNING *")?;
        let result = match stmt.query_map(params, Self::row_to_run)?.next() {
            Some(v) => Ok(v?),
            _ => panic!("Failed to insert run"),
        };

        drop(stmt);
        tx.commit()?;
        result
    }

    fn row_to_run(row: &rusqlite::Row) -> rusqlite::Result<Run> {
        Ok(Run { id: row.get(0)? })
    }

    pub fn insert_results(
        &mut self,
        id: i32,
        size: usize,
        configurations: &[Configuration],
    ) -> Result<(), rusqlite::Error> {
        let tx = self.0.transaction_with_behavior(Immediate)?;
        {
            let mut stmt = tx.prepare("
                INSERT INTO results (run_id, dimension, size, temperature, energy, energy_std, energy_tau, energy_sqr, energy_sqr_std, energy_sqr_tau, magnet, magnet_std, magnet_tau, magnet_sqr, magnet_sqr_std, magnet_sqr_tau, specific_heat, specific_heat_std, magnet_suscept, magnet_suscept_std, time_mc, time_boot)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22)
            ")?;

            for cfg in configurations {
                stmt.execute(rusqlite::params![
                    id,
                    cfg.dimension as i32,
                    size as i32,
                    cfg.temperature,
                    cfg.energy.mean,
                    cfg.energy.stddev,
                    cfg.energy.tau,
                    cfg.energy.sqr_mean,
                    cfg.energy.sqr_stddev,
                    cfg.energy.sqr_tau,
                    cfg.magnetization.mean,
                    cfg.magnetization.stddev,
                    cfg.magnetization.tau,
                    cfg.magnetization.sqr_mean,
                    cfg.magnetization.sqr_stddev,
                    cfg.magnetization.sqr_tau,
                    cfg.cv.0,
                    cfg.cv.1,
                    cfg.xs.0,
                    cfg.xs.1,
                    cfg.time_mc as i32,
                    cfg.time_boot as i32
                ])?;
            }
        }
        tx.commit()
    }
}
