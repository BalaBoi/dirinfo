use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn dir_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("dirinfo")?;
    cmd.arg("/dir/does/not/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Directory doesn't exist"));

    Ok(())
}
