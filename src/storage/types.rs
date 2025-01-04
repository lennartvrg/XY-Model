#[derive(sqlx::FromRow)]
pub struct Run {
    pub id: i32,
    pub created_at: i32
}

#[derive(sqlx::FromRow)]
pub struct Configuration {
    pub id: i32,
    pub run_id: i32,

    pub size: i32,
    pub temperature: f64,
}

#[derive(sqlx::FromRow)]
pub struct Observable {
    pub configuration_id: i32,
    pub sequence_id: i32,

    pub energy: f64,
    pub energy_squared: f64,
    pub magnetization: f64,
    pub magnetization_squared: f64,
    pub specific_heat: f64,
    pub magnetic_susceptibility: f64,
}
