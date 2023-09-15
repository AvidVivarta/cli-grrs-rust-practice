use assert_cmd::prelude::*; 
use predicates::prelude::*;
use assert_fs::prelude::*;
use std::process::Command;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> { 
    let mut cmd = Command::cargo_bin("cli-grrs")?; 
    cmd.arg("-p").arg("foobar").arg("-P").arg("test/file/doesnt/exist");
    cmd.assert().failure().stderr(predicate::str::contains("Could not read file"));
    Ok(())
}

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let  file = assert_fs::NamedTempFile::new("test.txt")?; 
    file.write_str("A test\nActual content\nMore content\nAnother test")?;
    let mut cmd = Command::cargo_bin("cli-grrs")?; 
    cmd.arg("-p").arg("content").arg("-P").arg(file.path());
    cmd.assert().success().stdout(predicate::str::contains("Actual content\nMore content"));
    Ok(())
}
