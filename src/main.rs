use std::fs::File;
use std::sync::RwLock;
use std::io::Write;
use rayon::prelude::*;

use crate::algorithm::metropolis::Metropolis;
use crate::lattice::lattice_2d::Lattice2D;

mod algorithm;
mod constants;
mod lattice;
mod utils;

fn main() {
    let file = RwLock::new(File::create("output.csv").unwrap());
    if let Ok(mut writer) = file.write() {
        writeln!(writer, "T\tE\tM").unwrap();
    }

    (1..=250).into_par_iter().for_each_init(|| rand::rng(), |mut rng, i| {
        let mut lattice = Lattice2D::<128>::new(1.0 / (0.01 * i as f64));
        let (energy, magnetization) = lattice.metropolis_hastings(&mut rng, 32_000);

        if let Ok(mut writer) = file.write() {
            writeln!(writer, "{:.2}\t{:.8}\t{:.8}", 0.01 * i as f64, energy, magnetization).unwrap();
            println!("{:.2}\t{:.8}\t{:.8}", 0.01 * i as f64, energy, magnetization);
        }
    });
}
