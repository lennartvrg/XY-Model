mod energy;
mod magnetization;

pub use energy::*;
pub use magnetization::*;

pub trait Observable: Copy + Default {
    fn norm(&self) -> f64;

    fn norm_per_site(&self) -> f64;

    fn norm_sqr(&self) -> f64;

    fn norm_sqr_per_site(&self) -> f64;
}
