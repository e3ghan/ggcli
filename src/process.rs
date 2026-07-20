use std::collections::HashSet;

use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

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

pub fn process_csv(csv_opts: &CsvOpts) -> anyhow::Result<Vec<Value>> {
    if !csv_opts.delimiter.is_ascii() {
        anyhow::bail!("CSV delimiter must be an ASCII character");
    }
    let delimiter = csv_opts.delimiter as u8;

    let mut rdr = ReaderBuilder::new()
        .delimiter(delimiter)
        .has_headers(csv_opts.header)
        .flexible(true)
        .from_path(&csv_opts.input)?;

    // `headers()` always reads the first row. When `has_headers` is false,
    // that row is still returned by `records()`, so use it only to determine
    // the number of columns and generate stable names for the data rows.
    let first_row: Vec<String> = rdr.headers()?.iter().map(str::to_owned).collect();
    let headers = if csv_opts.header {
        first_row
    } else {
        (1..=first_row.len())
            .map(|index| format!("column_{index}"))
            .collect()
    };

    let mut seen = HashSet::new();
    for header in &headers {
        if !seen.insert(header.as_str()) {
            anyhow::bail!("duplicate CSV header: {header}");
        }
    }

    let json_values = rdr
        .records()
        .enumerate()
        .map(|(row_index, record)| -> anyhow::Result<Value> {
            let record = record?;
            if record.len() != headers.len() {
                anyhow::bail!(
                    "row {} has {} fields, expected {}",
                    row_index + 1,
                    record.len(),
                    headers.len()
                );
            }

            let object = headers
                .iter()
                .zip(record.iter())
                .map(|(header, raw)| (header.clone(), infer_json_value(raw)))
                .collect::<Map<String, Value>>();

            Ok(Value::Object(object))
        })
        .collect::<anyhow::Result<Vec<Value>>>()?;

    Ok(json_values)
}

/// Infer JSON scalar/object/array values when a CSV cell contains valid JSON;
/// otherwise preserve the original cell as a string.
fn infer_json_value(raw: &str) -> Value {
    let trimmed = raw.trim();
    serde_json::from_str(trimmed).unwrap_or_else(|_| Value::String(raw.to_owned()))
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::infer_json_value;

    #[test]
    fn infers_valid_json_values() {
        assert_eq!(infer_json_value("42"), json!(42));
        assert_eq!(infer_json_value("3.14"), json!(3.14));
        assert_eq!(infer_json_value("true"), json!(true));
        assert_eq!(infer_json_value("null"), json!(null));
        assert_eq!(infer_json_value("[1, 2]"), json!([1, 2]));
    }

    #[test]
    fn preserves_non_json_values_as_strings() {
        assert_eq!(infer_json_value("hello"), json!("hello"));
        assert_eq!(infer_json_value("001"), json!("001"));
        assert_eq!(infer_json_value(""), json!(""));
    }
}
