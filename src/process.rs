use csv::Reader;
use anyhow::Result;
use serde_json::Value;

pub fn process_csv(input: &str, output: &str) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret= Vec::with_capacity(128);
    let headers = reader.headers()?.clone();
    for result in reader.records() {
        let player = result?;
        let record = headers.iter().zip(player.iter()).collect::<Value>();
        ret.push(record);
    }
    let json = serde_json::to_string_pretty(&ret)?;
    std::fs::write(output, json)?;
    Ok(())
}
