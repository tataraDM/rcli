use csv::Reader;
use serde_json::Value;

use crate::OutputFormat;

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();
    for result in reader.records() {
        let record = result?;
        let string_value = headers.iter().zip(record.iter()).collect::<Value>();
        ret.push(string_value);
    }

    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };

    //println!("{}", json_output);
    std::fs::write(output, content)?;
    Ok(())
}
