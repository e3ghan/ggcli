use std::path;

use clap::{ Parser, Subcommand, Args };


#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Subcommands,
}

#[derive(Debug, Subcommand)]
pub enum Subcommands {
    #[command(name = "csv", about = "Process a CSV file")]
    Csv(CsvOpts)
}

#[derive(Debug, Args)]
pub struct CsvOpts {
    #[arg(short, long, help = "Input file path for the CSV file", value_parser = parse_input_file)]
    pub input: String,

    #[arg(short, long, default_value = "output.json", help = "Output file path for the CSV file")]
    pub output: String,

    #[arg(short, long, default_value_t = ',', help = "Delimiter character for the CSV file")]
    pub delimiter: char,

    #[arg(long, default_value_t = true, help = "Whether the CSV file has a header row")]
    pub header: bool,
}

fn parse_input_file(s: &str) -> Result<String, String> {
    if path::Path::new(s).exists() {
        Ok(s.to_string())
    } else {
        Err(format!("Input file '{}' does not exist", s))
    }
}