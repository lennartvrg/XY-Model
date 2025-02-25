use clap::Parser;

#[derive(Parser)]
#[command(
    version = "1.0",
    about = "Simulates the XY model using a metropolis hastings algorithm. Results are written to the output.sqlite database."
)]
pub struct Arguments {
    /// Optionally provide a run id for which the results should be gathered.
    #[arg(short = 'r', long = "run_id")]
    pub run_id: Option<i32>,

    /// The lengths of the lattice sides which will be simulated.
    pub sizes: Vec<usize>,
}
