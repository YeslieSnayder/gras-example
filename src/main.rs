use std::{fs::File, path::PathBuf,
          io::{stdout, BufReader}};
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

    gras_example::find_matches(reader, &args.pattern, &mut stdout())?;

    Ok(())
}

