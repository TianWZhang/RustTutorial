use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("l2_echor")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}

fn runs(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("l2_echor")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn hello1() -> TestResult {
    runs(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn hello2() -> TestResult {
    runs(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn hello3() -> TestResult {
    runs(&["Hello  there", "-n"], "tests/expected/hello1.n.txt")
}

#[test]
fn hello4() -> TestResult {
    runs(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}
