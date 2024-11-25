use csv::Reader;
use anyhow::Result;
use serde_json::Value;

use crate::opts::OutputFormat;

pub fn process_csv(input: &str, output: &str, format: OutputFormat) -> Result<()> {
    let _ = format;
    let mut reader = Reader::from_path(input)?;
    let mut ret= Vec::with_capacity(128);
    let headers = reader.headers()?.clone();
    for result in reader.records() {
        let player = result?;
        let record = headers.iter().zip(player.iter()).collect::<Value>();
        ret.push(record);
    }
    let content: String = match format {
        OutputFormat::Json =>  serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };
    std::fs::write(output, content)?;
    Ok(())
}
