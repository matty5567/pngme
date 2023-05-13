use crate::commands;
use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: commands::Command,
}
