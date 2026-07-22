use clap::Parser;
use ggcli::{process_csv, Cli, Subcommands};
use std::{fs};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.cmd {
        Subcommands::Csv(csv_opts) => {
            let (output_path, res_content) = process_csv(&csv_opts)?;
            fs::write(output_path, res_content)?;

        }
    }

    Ok(())
}
