use std::fs;

use assert_cmd::Command;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}

#[test]
fn run_basic() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("hello").assert().success();
    Ok(())
}

#[test]
fn run_omit_newline() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("-n")
        .arg("hello")
        .assert()
        .success()
        .stdout(predicate::str::contains("\n").not());
    Ok(())
}

fn run_test_file(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.args(args).assert().success().stdout(expected);
    Ok(())
}

#[test]
fn check_hello1() -> TestResult {
    run_test_file(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn check_hello2() -> TestResult {
    run_test_file(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn check_hello1n() -> TestResult {
    run_test_file(&["Hello  there", "-n"], "tests/expected/hello1.n.txt")
}

#[test]
fn check_hello2n() -> TestResult {
    run_test_file(&["Hello", "there", "-n"], "tests/expected/hello2.n.txt")
}
