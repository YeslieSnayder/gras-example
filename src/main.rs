use std::{io::BufRead, io::BufReader, fs::File, path::PathBuf};
use anyhow::{Result, Context};
use clap::Parser;
use log::info;

/// Command line arguments
#[derive(Parser, Debug)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to a file to read
    path: PathBuf,

    #[command(flatten)]
    verbosity: clap_verbosity_flag::Verbosity,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    env_logger::Builder::new()
        .filter_level(args.verbosity.into())
        .init();

    let file = File::open(&args.path).
        with_context(|| format!("can't open file {:?}", args.path))?;
    let reader = BufReader::new(file);
    info!("file successfully read");

    // let pb = indicatif::ProgressBar::new_spinner();
    let mut i: u32 = 0;
    for line in reader.lines().flatten() {
        if line.contains(&args.pattern) {
            println!("{}", line);
            i += 1;
        }
        // pb.inc(1);
    }
    // pb.finish_with_message("done");

    info!("{} matches found", i);

    Ok(())
}
