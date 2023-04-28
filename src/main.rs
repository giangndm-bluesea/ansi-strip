use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

use clap::Parser;

/// Simple program to convert ansi to plain-text
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    input: String,

    /// Number of times to greet
    #[arg(short, long)]
    output: String,
}


fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let input_file = File::open(&args.input)?;
    let output_file = File::create(&args.output)?;
    let reader = BufReader::new(input_file);
    let mut writer = BufWriter::new(output_file);

    for line in reader.lines() {
        let stripped_line = strip_ansi_escapes::strip(&line?)?;
        writeln!(writer, "{}", String::from_utf8_lossy(&stripped_line))?;
    }

    Ok(())
}