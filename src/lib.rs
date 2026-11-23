//! A Vergil test fixture: a small command-line application with no users.
//!
//! It exists so that Vergil's Rust support (validation, CI, the dependency
//! audit, the docs site and the release) has a real application to run
//! against. Each subcommand lives in its own module under [`commands`]: adding
//! one is a new module, a [`Command`] variant and a match arm in [`run`].

pub mod cli;
pub mod commands;

use std::io::Write;

pub use cli::{Cli, Command};

/// Runs `command`, writing its output to `out`.
///
/// # Errors
///
/// Returns the subcommand's error, or an error writing to `out`.
pub fn run(command: Command, out: &mut impl Write) -> anyhow::Result<()> {
    match command {
        Command::Greet(args) => commands::greet::run(&args, out),
        Command::Sum(args) => commands::sum::run(&args, out),
        Command::Max(args) => commands::max::run(&args, out),
        Command::Min(args) => commands::min::run(&args, out),
        Command::Product(args) => commands::product::run(&args, out),
        Command::Range(args) => commands::range::run(&args, out),
        Command::Mean(args) => commands::mean::run(&args, out),
        Command::Count(args) => commands::count::run(&args, out),
        Command::Median(args) => commands::median::run(&args, out),
        Command::Info => commands::info::run(out),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn output(command: Command) -> String {
        let mut out = Vec::new();
        run(command, &mut out).expect("the command succeeds");
        String::from_utf8(out).expect("the output is UTF-8")
    }

    #[test]
    fn run_dispatches_greet() {
        let args = commands::greet::Args {
            name: "Ada".to_owned(),
        };
        assert_eq!(output(Command::Greet(args)), "Hello, Ada\n");
    }

    #[test]
    fn run_dispatches_sum() {
        let args = commands::sum::Args {
            numbers: vec![2, 3],
        };
        assert_eq!(output(Command::Sum(args)), "5\n");
    }

    #[test]
    fn run_dispatches_max() {
        let args = commands::max::Args {
            numbers: vec![2, 3],
        };
        assert_eq!(output(Command::Max(args)), "3\n");
    }

    #[test]
    fn run_dispatches_min() {
        let args = commands::min::Args {
            numbers: vec![2, 3],
        };
        assert_eq!(output(Command::Min(args)), "2\n");
    }

    #[test]
    fn run_dispatches_product() {
        let args = commands::product::Args {
            numbers: vec![2, 3],
        };
        assert_eq!(output(Command::Product(args)), "6\n");
    }

    #[test]
    fn run_dispatches_range() {
        let args = commands::range::Args {
            numbers: vec![2, 3],
        };
        assert_eq!(output(Command::Range(args)), "1\n");
    }

    #[test]
    fn run_dispatches_mean() {
        let args = commands::mean::Args {
            numbers: vec![2, 4],
        };
        assert_eq!(output(Command::Mean(args)), "3\n");
    }

    #[test]
    fn run_dispatches_count() {
        let args = commands::count::Args {
            numbers: vec![2, 4],
        };
        assert_eq!(output(Command::Count(args)), "2\n");
    }

    #[test]
    fn run_dispatches_median() {
        let args = commands::median::Args {
            numbers: vec![9, 2, 4],
        };
        assert_eq!(output(Command::Median(args)), "4\n");
    }

    #[test]
    fn run_dispatches_info() {
        assert!(output(Command::Info).starts_with("{\"name\":\"vergil-fixture-rust-app\""));
    }
}
