use crate::analysis::Observable;

#[derive(sqlx::FromRow)]
pub struct Run {
    pub id: i32,
}

pub struct Configuration {
    pub temperature: f64,
    pub energy: Observable,
    pub magnetization: Observable,
    pub spins: String,
    pub time: u128,
}

impl Configuration {
    pub const fn new(
        temperature: f64,
        energy: Observable,
        magnetization: Observable,
        spins: String,
        time: u128,
    ) -> Self {
        Self {
            temperature,
            energy,
            magnetization,
            spins,
            time,
        }
    }
}
