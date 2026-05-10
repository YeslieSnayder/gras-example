use assert_cmd::cargo::*;
use predicates::prelude::*;
use assert_fs::fixture::FileWriteStr;

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = cargo_bin_cmd!("gras-example");
    cmd.arg("test").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("A test\nAnother test"));

    Ok(())
}

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = cargo_bin_cmd!("gras-example");

    cmd.arg("foobar").arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("can't open file"));

    Ok(())
}

#[test]
fn content_not_found() -> Result<(), Box<dyn std::error::Error>> {
    let file  = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A test\nMore content\nAnother test")?;

    let mut cmd = cargo_bin_cmd!("gras-example");
    cmd.arg("sample").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::is_empty());

    Ok(())
}
