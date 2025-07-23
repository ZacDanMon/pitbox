use clap::Parser;

/// F1 Standings CLI
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    /// Show constructor standings for the current season
    #[arg(short = 'c', long = "cstandings")]
    pub constructor_standings: bool,

    /// Show driver standings for the current season
    #[arg(short = 'd', long = "dstandings")]
    pub driver_standings: bool,
}

impl Cli {
    /// Parse std::env::args and return the Cli struct.
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }
}
