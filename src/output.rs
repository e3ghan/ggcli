use std::collections::BTreeMap;

use serde::Serialize;

use crate::{CsvData, OutputFormat};

#[derive(Serialize)]
struct TomlDocument<'a> {
    rows: Vec<BTreeMap<&'a str, &'a str>>,
}

pub fn format_output(data: &CsvData, format: OutputFormat) -> anyhow::Result<Vec<u8>> {
    match format {
        OutputFormat::Json => Ok(serde_json::to_vec_pretty(&object_rows(data))?),
        OutputFormat::Csv => format_csv(data),
        OutputFormat::Toml => {
            let document = TomlDocument {
                rows: object_rows(data),
            };
            Ok(toml::to_string_pretty(&document)?.into_bytes())
        }
    }
}

fn object_rows(data: &CsvData) -> Vec<BTreeMap<&str, &str>> {
    data.records
        .iter()
        .map(|record| {
            data.headers
                .iter()
                .zip(record.iter())
                .map(|(header, value)| (header.as_str(), value.as_str()))
                .collect()
        })
        .collect()
}

fn format_csv(data: &CsvData) -> anyhow::Result<Vec<u8>> {
    let mut writer = csv::WriterBuilder::new()
        .delimiter(data.delimiter)
        .from_writer(Vec::new());

    if data.has_headers {
        writer.write_record(&data.headers)?;
    }
    for record in &data.records {
        writer.write_record(record)?;
    }

    Ok(writer.into_inner()?)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::format_output;
    use crate::{CsvData, OutputFormat};

    fn sample_data() -> CsvData {
        CsvData {
            headers: vec!["name".into(), "age".into()],
            records: vec![vec!["Ada".into(), "42".into()]],
            has_headers: true,
            delimiter: b',',
        }
    }

    #[test]
    fn formats_json() {
        let output = format_output(&sample_data(), OutputFormat::Json).unwrap();
        let value: serde_json::Value = serde_json::from_slice(&output).unwrap();

        assert_eq!(value, json!([{ "age": "42", "name": "Ada" }]));
    }

    #[test]
    fn formats_csv() {
        let output = format_output(&sample_data(), OutputFormat::Csv).unwrap();

        assert_eq!(String::from_utf8(output).unwrap(), "name,age\nAda,42\n");
    }

    #[test]
    fn formats_toml() {
        let output = format_output(&sample_data(), OutputFormat::Toml).unwrap();
        let value: toml::Value = toml::from_str(&String::from_utf8(output).unwrap()).unwrap();

        assert_eq!(value["rows"][0]["name"].as_str(), Some("Ada"));
        assert_eq!(value["rows"][0]["age"].as_str(), Some("42"));
    }
}
