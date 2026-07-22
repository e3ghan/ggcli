mod base64;
mod passwd;
mod csv;

pub use csv::CsvOpts;
pub use passwd::PassGenOpts;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Subcommands,
}

#[derive(Debug, Subcommand)]
pub enum Subcommands {
    #[command(name = "csv", about = "Process a CSV file")]
    Csv(CsvOpts),

    #[command(name = "passgen", about = "Generate a password")]
    PassGen(PassGenOpts),
}












