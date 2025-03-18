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

    /// Enables the recording of the development of vortices on the lattice
    #[arg(short = 'v', long = "vortices")]
    pub vortices: Option<usize>,

    /// The lengths of the 1D lattice sides which will be simulated.
    #[arg(short = 'o', long = "one", num_args = 0..)]
    pub one: Vec<usize>,

    /// The lengths of the 2D lattice sides which will be simulated.
    #[arg(short = 't', long = "two", num_args = 0..)]
    pub two: Vec<usize>,
}
