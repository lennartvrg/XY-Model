use crate::analysis::Observable;

pub struct Run {
    pub id: i32,
}

pub struct Configuration {
    pub dimension: usize,
    pub temperature: f64,
    pub energy: Observable,
    pub magnetization: Observable,
    pub spins: String,
    pub time: u128,
}

impl Configuration {
    pub const fn new(
        dimension: usize,
        temperature: f64,
        energy: Observable,
        magnetization: Observable,
        spins: String,
        time: u128,
    ) -> Self {
        Self {
            dimension,
            temperature,
            energy,
            magnetization,
            spins,
            time,
        }
    }
}
