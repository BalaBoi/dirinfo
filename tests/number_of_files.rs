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

#[test]
fn dir_exists_with_files() -> Result<(), Box<dyn std::error::Error>> {
    let tmpdir = assert_fs::TempDir::new()?;
    let file1 = std::fs::File::create(tmpdir.path().join("temp1"))?;
    file1.set_len(1234)?;
    let file2 = std::fs::File::create(tmpdir.path().join("temp2"))?;
    file2.set_len(1234)?;

    let mut cmd = Command::cargo_bin("dirinfo")?;
    cmd.arg(tmpdir.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Logical size of all the files: 2468"));

    Ok(())
}
