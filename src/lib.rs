use std::io::{BufRead, BufReader, Read};
use anyhow::{Context, Result};
use log::{info, warn};

/// Works similar to grep. Just finds $pattern in $reader and writes lines with matches to $writer.
pub fn find_matches(reader: BufReader<impl Read>, pattern: &str, mut writer: impl std::io::Write) -> Result<()> {
    let mut count: u32 = 0;
    for line in reader.lines().flatten() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line).with_context(|| {
                warn!("failed to write output");
                "failed to write output"
            })?;
            count += 1;
        }
    }

    info!("{} matches found", count);

    Ok(())
}

#[test]
fn find_a_match() {
    let input = "lorem ipsum\ndolor sit amet";
    let pattern = "lorem";

    let mut writer = Vec::new();

    find_matches(BufReader::new(input.as_bytes()), pattern, &mut writer).expect("can't find matches");

    assert_eq!(writer, b"lorem ipsum\n");
}
