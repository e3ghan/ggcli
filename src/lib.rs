mod cli;

pub use cli::{Cli, CsvOpts, OutputFormat, Subcommands};

mod output;

pub use output::format_output;

mod process;

pub use process::{CsvData, process_csv};
