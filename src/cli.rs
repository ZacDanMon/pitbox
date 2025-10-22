use clap::{ArgGroup, Args, Parser, Subcommand};

/// Pitbox F1 CLI
#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    subcommand_required = true,
    arg_required_else_help = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Display driver or constructor standings table.
    #[command(alias = "s")]
    Standings(StandingsArgs),

    /// Display results from a single race.
    #[command(alias = "r")]
    Results(ResultsArgs),

    /// Display results from multiple drivers over a season.
    #[command(alias = "d")]
    Driver(DriverArgs),
}

#[derive(Debug, Args)]
#[command(
    group(
        ArgGroup::new("standings").args(["drivers", "constructors"]).required(true).multiple(false)
)
)]
pub struct StandingsArgs {
    /// Show driver standings.
    #[arg(short = 'd', long = "drivers")]
    pub drivers: bool,

    /// Show constructor standings.
    #[arg(short = 'c', long = "constructors")]
    pub constructors: bool,

    /// The season to display standings from.
    #[arg(short = 's', long = "season", default_value = "current")]
    pub season: String,

    /// Show gap to leader.
    #[arg(short = 'g', long = "gap")]
    pub gap: bool,
}

#[derive(Debug, Args)]
pub struct ResultsArgs {
    /// The season to display results from.
    #[arg(short = 's', long = "season", default_value = "current")]
    pub season: String,

    /// The round to display results from.
    #[arg(short = 'r', long = "round", default_value = "last")]
    pub round: String,
}

#[derive(Debug, Args)]
pub struct DriverArgs {
    /// The season to display driver results from.
    #[arg(short = 's', long = "season", default_value = "current")]
    pub season: String,

    /// The names of drivers to display season results, comma separated.
    /// Can be either the start of a driver's last name, or the 3 character
    /// driver identifier code.
    #[arg(short = 'n', long = "name", value_delimiter = ',', num_args = 1..)]
    pub name: Vec<String>,
}
