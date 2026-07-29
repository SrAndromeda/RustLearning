use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

#[test]
fn dies_no_args() -> Result<()> {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}

#[test]
fn run_basic() -> Result<()> {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("hello").assert().success();
    Ok(())
}

#[test]
fn run_omit_newline() -> Result<()> {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("-n")
        .arg("hello")
        .assert()
        .success()
        .stdout(predicate::str::contains("\n").not());
    Ok(())
}

fn run_test_file(args: &[&str], expected_file: &str) -> Result<()> {
    let expected = fs::read_to_string(expected_file)?;
    //expect fill the Result when somthing panics
    let output = Command::cargo_bin("echor")?
        .args(args)
        .output()
        .expect("fail"); //here if the binary panics the output is "fail"
    //here if utf-8 encode fails shows the error, so when the test run it will be self explanatory
    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");

    assert_eq!(stdout, expected);
    Ok(())
}

#[test]
fn check_hello1() -> Result<()> {
    run_test_file(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn check_hello2() -> Result<()> {
    run_test_file(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn check_hello1n() -> Result<()> {
    run_test_file(&["Hello  there", "-n"], "tests/expected/hello1.n.txt")
}

#[test]
fn check_hello2n() -> Result<()> {
    run_test_file(&["Hello", "there", "-n"], "tests/expected/hello2.n.txt")
}
