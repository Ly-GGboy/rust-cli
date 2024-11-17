use std::path::Path;
use clap::Parser;



#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: Subcommand,
}

#[derive(Debug, Parser)]
pub enum Subcommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
}
//rcli csv -i input.csv -o output.json --header -d ','
#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,

    #[arg(short, long, default_value = "output.json")]
    pub output: String,

    #[arg(short = 'H', long, default_value = "true")]
    pub header: bool,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
}

pub fn verify_input_file(path: &str) -> Result<String, String> {
    if Path::new(path).exists() {
        Ok(path.to_string())
    } else {
        Err(format!("File not found: {}", path))
    }
}
