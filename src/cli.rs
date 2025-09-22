use clap::{ArgGroup, Args, Parser, Subcommand};

/// Apex F1 CLI
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
    #[command(alias = "s")]
    Standings(StandingsArgs),
}

#[derive(Debug, Args)]
#[command(
    group(
        ArgGroup::new("standings").args(["drivers", "constructors"]).required(true).multiple(false)
)
)]
pub struct StandingsArgs {
    #[arg(short = 'd', long = "drivers")]
    pub drivers: bool,

    #[arg(short = 'c', long = "constructors")]
    pub constructors: bool,

    #[arg(short = 's', long = "season", default_value = "current")]
    pub season: String,

    #[arg(short = 'g', long = "gap")]
    pub gap: bool,
}
