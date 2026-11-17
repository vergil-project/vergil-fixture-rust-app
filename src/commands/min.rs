//! `min NUMBERS...`: print the smallest of some integers.

use std::io::Write;

use anyhow::{Context, Result};

/// The arguments of `min`.
#[derive(Debug, clap::Args)]
pub struct Args {
    /// The integers to compare.
    #[arg(allow_negative_numbers = true)]
    pub numbers: Vec<i64>,
}

/// The smallest of `numbers`.
///
/// # Examples
///
/// ```
/// use vergil_fixture_rust_app::commands::min::smallest;
///
/// assert_eq!(smallest(&[3, -7, 5]).unwrap(), -7);
/// assert!(smallest(&[]).is_err());
/// ```
///
/// # Errors
///
/// Fails when `numbers` is empty, since nothing is the smallest of no numbers.
pub fn smallest(numbers: &[i64]) -> Result<i64> {
    numbers
        .iter()
        .copied()
        .min()
        .context("min needs at least one number")
}

/// Writes the smallest of `args.numbers` to `out`.
///
/// # Errors
///
/// Fails as [`smallest`] does, or when writing to `out` fails.
pub fn run(args: &Args, out: &mut impl Write) -> Result<()> {
    writeln!(out, "{}", smallest(&args.numbers)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::testing::capture;

    const NONE: &str = "min needs at least one number";

    #[test]
    fn picks_the_smallest() {
        assert_eq!(smallest(&[2, 1, 3]).unwrap(), 1);
    }

    #[test]
    fn picks_the_smallest_of_negative_numbers() {
        assert_eq!(smallest(&[-4, -2, -9]).unwrap(), -9);
    }

    #[test]
    fn one_number_is_its_own_smallest() {
        assert_eq!(smallest(&[7]).unwrap(), 7);
    }

    #[test]
    fn handles_the_bounds() {
        assert_eq!(smallest(&[i64::MAX, i64::MIN]).unwrap(), i64::MIN);
        assert_eq!(smallest(&[i64::MAX]).unwrap(), i64::MAX);
    }

    #[test]
    fn rejects_no_numbers() {
        assert_eq!(smallest(&[]).unwrap_err().to_string(), NONE);
    }

    #[test]
    fn run_writes_one_line() {
        let args = Args {
            numbers: vec![40, 2, 42],
        };
        let (result, out) = capture(64, |out| run(&args, out));
        result.unwrap();
        assert_eq!(out, b"2\n");
    }

    #[test]
    fn run_writes_nothing_on_error() {
        let args = Args { numbers: vec![] };
        let (result, out) = capture(64, |out| run(&args, out));
        assert_eq!(result.unwrap_err().to_string(), NONE);
        assert!(out.is_empty());
    }

    #[test]
    fn run_reports_a_write_error() {
        let args = Args {
            numbers: vec![40, 2, 42],
        };
        let (result, out) = capture(0, |out| run(&args, out));
        let kind = result
            .unwrap_err()
            .downcast_ref::<std::io::Error>()
            .map(std::io::Error::kind);
        assert_eq!(kind, Some(std::io::ErrorKind::WriteZero));
        assert!(out.is_empty());
    }
}
