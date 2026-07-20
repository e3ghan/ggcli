use clap::Parser;
use ggcli::{Cli, Subcommands, format_output, process_csv};
use std::fs;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.cmd {
        Subcommands::Csv(csv_opts) => {
            let output_path = csv_opts.output_path();
            let data = process_csv(&csv_opts)?;
            let content = format_output(&data, csv_opts.format)?;
            fs::write(output_path, content)?;
        }
    }

    Ok(())
}
