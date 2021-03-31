use structopt::StructOpt;

use crate::commands;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
pub struct Opts {
    #[structopt(subcommand)]
    pub subcommand: commands::SubCommand,
}
