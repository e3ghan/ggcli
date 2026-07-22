use clap::Parser;
use ggcli::{process_csv, process_passgen, Cli, Subcommands};
use std::fs;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.cmd {
        Subcommands::Csv(csv_opts) => {
            let (output_path, response_content) = process_csv(&csv_opts)?;
            fs::write(output_path, response_content)?;
        }
        Subcommands::PassGen(passgen_opts) => {
            let password = process_passgen(&passgen_opts)?;
            println!("{password}");
        }
    }

    Ok(())
}
