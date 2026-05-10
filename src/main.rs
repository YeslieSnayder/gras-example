use std::path::PathBuf;
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

    println!("pattern: {:?}, file: {:?}", args.pattern, args.path);
}
