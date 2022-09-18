use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicate::str::contains;
use predicates::prelude::*;
use std::error::Error;
use std::process::Command;

#[test]
fn missing_file() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("halfpipers")?;

    cmd.arg("-i").arg("missing-file");
    cmd.assert()
        .failure()
        .stderr(contains("No such file or directory"));

    Ok(())
}

#[test]
fn invalid_file() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("halfpipers")?;

    let file = assert_fs::NamedTempFile::new(".halfpipe.io")?;
    file.write_str("?")?;

    cmd.arg("-i").arg(file.path());
    cmd.assert().failure();

    Ok(())
}

#[test]
fn happy() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("halfpipers")?;

    let file = assert_fs::NamedTempFile::new(".halfpipe.io")?;
    file.write_str(
        "
        pipeline: p
        team: t
        tasks: []
        ",
    )?;

    cmd.arg("-i").arg(file.path());
    cmd.assert().success();

    Ok(())
}
