
use clap::Parser;
use ggcli::{process_csv, Cli, Subcommands};
use std::fs;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.cmd {
        Subcommands::Csv(csv_opts) => {
            let players = process_csv(&csv_opts)?;
            let content = serde_json::to_string_pretty(&players)?;
            fs::write(csv_opts.output, content)?;
        }
    }

    Ok(())
}
