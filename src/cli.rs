//! The command-line interface: the top-level parser and its subcommands.

use clap::{Parser, Subcommand};

use crate::commands::{greet, sum};

/// The parsed command line.
#[derive(Debug, Parser)]
#[command(name = "vergil-fixture-rust-app", version, about)]
pub struct Cli {
    /// The subcommand to run.
    #[command(subcommand)]
    pub command: Command,
}

/// The subcommands, one per module under [`crate::commands`].
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Print a greeting for NAME.
    Greet(greet::Args),
    /// Print the sum of NUMBERS (0 when none are given).
    Sum(sum::Args),
    /// Print the program's name and version as JSON.
    Info,
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use super::*;

    fn parse(args: &[&str]) -> Command {
        let argv = std::iter::once("vergil-fixture-rust-app").chain(args.iter().copied());
        Cli::try_parse_from(argv)
            .expect("the command line parses")
            .command
    }

    #[test]
    fn the_command_definition_is_consistent() {
        Cli::command().debug_assert();
    }

    #[test]
    fn parses_greet() {
        let command = parse(&["greet", "Ada"]);
        assert!(matches!(command, Command::Greet(args) if args.name == "Ada"));
    }

    #[test]
    fn parses_sum_with_negative_numbers() {
        let command = parse(&["sum", "-4", "2"]);
        assert!(matches!(command, Command::Sum(args) if args.numbers == [-4, 2]));
    }

    #[test]
    fn parses_info() {
        assert!(matches!(parse(&["info"]), Command::Info));
    }
}
