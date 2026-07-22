use clap::Args;
use std::path;
use std::str::FromStr;

#[derive(Debug, Args)]
pub struct CsvOpts {
    #[arg(short, long, help = "Input file path for the CSV file", value_parser = parse_input_file)]
    pub input: String,

    #[arg(short, long, help = "Output path; default output.<format>; bare name gets .<format>")]
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

    #[arg(long ,value_parser = format_parse , default_value = "json", help = "Output format for the Csv file")]
    pub format: OutputFormats,
}


#[derive(Debug, Clone, Copy)]
pub enum OutputFormats {
    Json,
    Yaml,
}


impl From<OutputFormats> for &'static str {
    fn from(format: OutputFormats) -> Self {
        match format {
            OutputFormats::Json => "json",
            OutputFormats::Yaml => "yaml",
        }
    }
}

impl FromStr for OutputFormats {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormats::Json),
            "yaml" => Ok(OutputFormats::Yaml),
            _ => Err(anyhow::anyhow!("Invalid output format: {}", s)),
        }
    }
}


fn parse_input_file(s: &str) -> Result<String, String> {
    if path::Path::new(s).exists() {
        Ok(s.to_string())
    } else {
        Err(format!("Input file '{}' does not exist", s))
    }
}

fn format_parse(format: &str) -> Result<OutputFormats, anyhow::Error> {
    format.parse()
}