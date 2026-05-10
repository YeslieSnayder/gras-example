use std::{io::BufRead, io::BufReader, fs::File, path::PathBuf};
use clap::Parser;

/// Command line arguments
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to a file to read
    path: PathBuf,
}

fn main() {
    let args = Cli::parse();

    let file = File::open(&args.path).expect("can't open file");
    let reader = BufReader::new(file);

    for line in reader.lines().flatten() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
