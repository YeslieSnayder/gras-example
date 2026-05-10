use std::{fs::File, path::PathBuf,
          io::{stdout, Read, BufRead, BufReader}};
use anyhow::{Result, Context};
use clap::Parser;
use log::{info, warn};

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

    find_matches(reader, &args.pattern, &mut stdout())?;

    Ok(())
}

fn find_matches(reader: BufReader<impl Read>, pattern: &str, mut writer: impl std::io::Write) -> Result<()> {
    // let pb = indicatif::ProgressBar::new_spinner();
    let mut i: u32 = 0;
    for line in reader.lines().flatten() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line).with_context(|| {
                warn!("failed to write output");
                "failed to write output"
            })?;
            i += 1;
        }
        // pb.inc(1);
    }
    // pb.finish_with_message("done");

    info!("{} matches found", i);

    Ok(())
}

#[test]
fn find_a_match() {
    let input = "lorem ipsum\ndolor sit amet";
    let pattern = "lorem";

    let mut writer = Vec::new();

    find_matches(BufReader::new(input.as_bytes()), pattern, &mut writer).expect("didn't fail");

    assert_eq!(writer, b"lorem ipsum\n");
}
