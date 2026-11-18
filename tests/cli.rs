//! End-to-end tests: each one runs the built `vergil-fixture-rust-app` binary
//! and checks its exit code, standard output and standard error.

use std::process::{Command, Output};

/// Runs the binary with `args` and returns what it did.
fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_vergil-fixture-rust-app"))
        .args(args)
        .output()
        .expect("the binary runs")
}

fn stdout(output: &Output) -> &str {
    std::str::from_utf8(&output.stdout).expect("stdout is UTF-8")
}

fn stderr(output: &Output) -> &str {
    std::str::from_utf8(&output.stderr).expect("stderr is UTF-8")
}

#[test]
fn greet_prints_a_greeting() {
    let output = run(&["greet", "Ada"]);
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(stdout(&output), "Hello, Ada\n");
    assert_eq!(stderr(&output), "");
}

#[test]
fn greet_rejects_a_blank_name() {
    let output = run(&["greet", "   "]);
    assert_eq!(output.status.code(), Some(1));
    assert_eq!(stdout(&output), "");
    assert_eq!(stderr(&output), "error: the name must not be blank\n");
}

#[test]
fn sum_adds_its_numbers() {
    let output = run(&["sum", "1", "2", "3"]);
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(stdout(&output), "6\n");
}

#[test]
fn sum_takes_negative_numbers() {
    let output = run(&["sum", "-4", "2"]);
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(stdout(&output), "-2\n");
}

#[test]
fn sum_of_no_numbers_is_zero() {
    let output = run(&["sum"]);
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(stdout(&output), "0\n");
}

#[test]
fn sum_reports_an_overflow() {
    let output = run(&["sum", "9223372036854775807", "1"]);
    assert_eq!(output.status.code(), Some(1));
    assert_eq!(stdout(&output), "");
    assert_eq!(
        stderr(&output),
        "error: the sum overflows a 64-bit signed integer\n"
    );
}

#[test]
fn sum_rejects_a_word() {
    let output = run(&["sum", "seven"]);
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(stdout(&output), "");
    assert!(
        stderr(&output).contains("invalid value 'seven'"),
        "stderr: {}",
        stderr(&output)
    );
}

#[test]
fn max_prints_the_largest_number() {
    let output = run(&["max", "-4", "7", "2"]);
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(stdout(&output), "7\n");
    assert_eq!(stderr(&output), "");
}

#[test]
fn max_of_no_numbers_is_an_error() {
    let output = run(&["max"]);
    assert_eq!(output.status.code(), Some(1));
    assert_eq!(stdout(&output), "");
    assert_eq!(stderr(&output), "error: max needs at least one number\n");
}

#[test]
fn min_prints_the_smallest_number() {
    let output = run(&["min", "-4", "7", "2"]);
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(stdout(&output), "-4\n");
    assert_eq!(stderr(&output), "");
}

#[test]
fn min_of_no_numbers_is_an_error() {
    let output = run(&["min"]);
    assert_eq!(output.status.code(), Some(1));
    assert_eq!(stdout(&output), "");
    assert_eq!(stderr(&output), "error: min needs at least one number\n");
}

#[test]
fn product_multiplies_its_numbers() {
    let output = run(&["product", "-4", "3", "2"]);
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(stdout(&output), "-24\n");
    assert_eq!(stderr(&output), "");
}

#[test]
fn product_of_no_numbers_is_one() {
    let output = run(&["product"]);
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(stdout(&output), "1\n");
}

#[test]
fn product_reports_an_overflow() {
    let output = run(&["product", "9223372036854775807", "2"]);
    assert_eq!(output.status.code(), Some(1));
    assert_eq!(stdout(&output), "");
    assert_eq!(
        stderr(&output),
        "error: the product overflows a 64-bit signed integer\n"
    );
}

#[test]
fn info_prints_the_name_and_version_as_json() {
    let output = run(&["info"]);
    assert_eq!(output.status.code(), Some(0));
    let expected = format!(
        "{{\"name\":\"vergil-fixture-rust-app\",\"version\":\"{}\"}}\n",
        env!("CARGO_PKG_VERSION")
    );
    assert_eq!(stdout(&output), expected);
}

#[test]
fn no_subcommand_is_a_usage_error() {
    let output = run(&[]);
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(stdout(&output), "");
    assert!(
        stderr(&output).contains("Usage: vergil-fixture-rust-app <COMMAND>"),
        "stderr: {}",
        stderr(&output)
    );
}

#[test]
fn version_flag_prints_the_version() {
    let output = run(&["--version"]);
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(
        stdout(&output),
        format!("vergil-fixture-rust-app {}\n", env!("CARGO_PKG_VERSION"))
    );
}
