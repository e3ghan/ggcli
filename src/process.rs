use serde::{Deserialize, Serialize};
use csv::ReaderBuilder;

use crate::CsvOpts;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Player {
    pub name: String,
    pub position: String,
    #[serde(rename = "DOB")]
    pub dob: String,
    pub nationality: String,
    #[serde(rename = "Kit Number")]
    pub number: u8,
}

pub fn process_csv(csv_opts: &CsvOpts) -> anyhow::Result<Vec<Player>> {
    let mut rdr = ReaderBuilder::new()
        .delimiter(csv_opts.delimiter as u8)
        .has_headers(csv_opts.header)
        .from_path(&csv_opts.input)?;

    let players = rdr.deserialize().collect::<Result<Vec<Player>, _>>()?;

    Ok(players)
}