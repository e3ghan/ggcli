use anyhow::Ok;
use csv::ReaderBuilder;
// use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::path::{Path, PathBuf};

use crate::cli::CsvOpts;

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "PascalCase")]
// pub struct Player {
//     pub name: String,
//     pub position: String,
//     #[serde(rename = "DOB")]
//     pub dob: String,
//     pub nationality: String,
//     #[serde(rename = "Kit Number")]
//     pub number: u8,
// }

pub fn process_csv(csv_opts: &CsvOpts) -> anyhow::Result<(PathBuf, String)> {
    let mut rdr = ReaderBuilder::new()
        .delimiter(csv_opts.delimiter as u8)
        .has_headers(csv_opts.header)
        .from_path(&csv_opts.input)?;

    let headers = rdr.headers().cloned()?;

    let values = rdr
        .records()
        .map(|result| {
            let record = result?;
            let object: Map<String, Value> = headers
                .iter()
                .zip(record.iter())
                .map(|(header, field)| (header.to_string(), Value::String(field.to_string())))
                .collect::<Map<String, Value>>();
            Ok(Value::Object(object))
        })
        .collect::<Result<Vec<Value>, _>>()?;

    let output_path = match &csv_opts.output {
        Some(path) => {
            let path = Path::new(&path);
            if path.extension().is_none() {
                let mut path_ext = path.to_path_buf();
                path_ext.set_extension(Into::<&str>::into(csv_opts.format));
                path_ext
            } else {
                path.to_path_buf()
            }
        }
        None => PathBuf::from(format!("output.{}", Into::<&str>::into(csv_opts.format))),
    };

    let res_content: String = match output_path.extension().and_then(|ext| ext.to_str()) {
        Some("json") => serde_json::to_string_pretty(&values)?,
        Some("yaml") => serde_yaml::to_string(&values)?,
        _ => return Err(anyhow::anyhow!("Invalid output format")),
    };

    Ok((output_path, res_content))
}
