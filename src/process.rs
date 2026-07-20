use std::collections::HashSet;
use std::io::Read;

use csv::ReaderBuilder;

use crate::CsvOpts;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CsvData {
    pub headers: Vec<String>,
    pub records: Vec<Vec<String>>,
    pub has_headers: bool,
    pub delimiter: u8,
}

pub fn process_csv(csv_opts: &CsvOpts) -> anyhow::Result<CsvData> {
    let delimiter = validate_delimiter(csv_opts.delimiter)?;
    let input = std::fs::File::open(&csv_opts.input)?;
    process_reader(input, delimiter, csv_opts.header)
}

fn validate_delimiter(delimiter: char) -> anyhow::Result<u8> {
    if !delimiter.is_ascii() || matches!(delimiter, '\0' | '\r' | '\n' | '"') {
        anyhow::bail!("CSV delimiter must be an ASCII character other than NUL, CR, LF, or quote");
    }

    Ok(delimiter as u8)
}

fn process_reader<R: Read>(reader: R, delimiter: u8, has_headers: bool) -> anyhow::Result<CsvData> {
    let mut rdr = ReaderBuilder::new()
        .delimiter(delimiter)
        .has_headers(has_headers)
        .flexible(true)
        .from_reader(reader);

    // `headers()` always reads the first row. When `has_headers` is false,
    // that row is still returned by `records()`, so use it only to determine
    // the number of columns and generate stable names for the data rows.
    let first_row: Vec<String> = rdr.headers()?.iter().map(str::to_owned).collect();
    let headers = if has_headers {
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

    let records = rdr
        .records()
        .enumerate()
        .map(|(row_index, record)| -> anyhow::Result<Vec<String>> {
            let record = record?;
            if record.len() != headers.len() {
                anyhow::bail!(
                    "record {} has {} fields, expected {}",
                    row_index + 1,
                    record.len(),
                    headers.len()
                );
            }

            Ok(record.iter().map(str::to_owned).collect())
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    Ok(CsvData {
        headers,
        records,
        has_headers,
        delimiter,
    })
}

#[cfg(test)]
mod tests {
    use super::{CsvData, process_reader, validate_delimiter};

    #[test]
    fn reads_csv_with_headers() {
        let data = process_reader("name,age\nAda,42\n".as_bytes(), b',', true).unwrap();

        assert_eq!(
            data,
            CsvData {
                headers: vec!["name".into(), "age".into()],
                records: vec![vec!["Ada".into(), "42".into()]],
                has_headers: true,
                delimiter: b',',
            }
        );
    }

    #[test]
    fn generates_column_names_without_headers() {
        let data = process_reader("Ada,42\n".as_bytes(), b',', false).unwrap();

        assert_eq!(data.headers, vec!["column_1", "column_2"]);
        assert_eq!(data.records, vec![vec!["Ada", "42"]]);
        assert!(!data.has_headers);
    }

    #[test]
    fn rejects_invalid_delimiters() {
        assert!(validate_delimiter('，').is_err());
        assert!(validate_delimiter('\n').is_err());
        assert!(validate_delimiter('"').is_err());
    }
}
