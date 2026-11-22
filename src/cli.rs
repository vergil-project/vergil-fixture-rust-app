//! The command-line interface: the top-level parser and its subcommands.

use clap::{Parser, Subcommand};

use crate::commands::{greet, max, mean, min, product, range, sum};

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
    /// Print the largest of NUMBERS (an error when none are given).
    Max(max::Args),
    /// Print the smallest of NUMBERS (an error when none are given).
    Min(min::Args),
    /// Print the product of NUMBERS (1 when none are given).
    Product(product::Args),
    /// Print the largest of NUMBERS less the smallest (an error when none are given).
    Range(range::Args),
    /// Print the mean of NUMBERS, rounded toward zero (an error when none are given).
    Mean(mean::Args),
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
    fn parses_max_with_negative_numbers() {
        let command = parse(&["max", "-4", "2"]);
        assert!(matches!(command, Command::Max(args) if args.numbers == [-4, 2]));
    }

    #[test]
    fn parses_min_with_negative_numbers() {
        let command = parse(&["min", "-4", "2"]);
        assert!(matches!(command, Command::Min(args) if args.numbers == [-4, 2]));
    }

    #[test]
    fn parses_product_with_negative_numbers() {
        let command = parse(&["product", "-4", "2"]);
        assert!(matches!(command, Command::Product(args) if args.numbers == [-4, 2]));
    }

    #[test]
    fn parses_range_with_negative_numbers() {
        let command = parse(&["range", "-4", "2"]);
        assert!(matches!(command, Command::Range(args) if args.numbers == [-4, 2]));
    }

    #[test]
    fn parses_mean_with_negative_numbers() {
        let command = parse(&["mean", "-4", "2"]);
        assert!(matches!(command, Command::Mean(args) if args.numbers == [-4, 2]));
    }

    #[test]
    fn parses_info() {
        let command = parse(&["info"]);
        assert_eq!(format!("{command:?}"), "Info");
    }
}
