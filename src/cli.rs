use std::path;

use clap::{Args, Parser, Subcommand, ValueEnum};

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
}

#[derive(Debug, Args)]
pub struct CsvOpts {
    #[arg(short, long, help = "Input file path for the CSV file", value_parser = parse_input_file)]
    pub input: String,

    #[arg(short, long, help = "Output file path; defaults to output.<format>")]
    pub output: Option<String>,

    #[arg(
        short,
        long,
        default_value_t = ',',
        help = "Delimiter character for the CSV file"
    )]
    pub delimiter: char,

    #[arg(
        long,
        default_value_t = true,
        action = clap::ArgAction::Set,
        help = "Whether the CSV file has a header row"
    )]
    pub header: bool,

    #[arg(
        short,
        long,
        value_enum,
        ignore_case = true,
        default_value = "json",
        help = "Output format"
    )]
    pub format: OutputFormat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum OutputFormat {
    Json,
    Csv,
    Toml,
}

impl OutputFormat {
    pub const fn extension(self) -> &'static str {
        match self {
            Self::Json => "json",
            Self::Csv => "csv",
            Self::Toml => "toml",
        }
    }
}

impl CsvOpts {
    pub fn output_path(&self) -> String {
        self.output
            .clone()
            .unwrap_or_else(|| format!("output.{}", self.format.extension()))
    }
}
fn parse_input_file(s: &str) -> Result<String, String> {
    if path::Path::new(s).exists() {
        Ok(s.to_string())
    } else {
        Err(format!("Input file '{}' does not exist", s))
    }
}
