//! `max NUMBERS...`: print the largest of some integers.

use std::io::Write;

use anyhow::{Context, Result};

/// The arguments of `max`.
#[derive(Debug, clap::Args)]
pub struct Args {
    /// The integers to compare.
    #[arg(allow_negative_numbers = true)]
    pub numbers: Vec<i64>,
}

/// The largest of `numbers`.
///
/// # Examples
///
/// ```
/// use vergil_fixture_rust_app::commands::max::largest;
///
/// assert_eq!(largest(&[3, -7, 5]).unwrap(), 5);
/// assert!(largest(&[]).is_err());
/// ```
///
/// # Errors
///
/// Fails when `numbers` is empty, since nothing is the largest of no numbers.
pub fn largest(numbers: &[i64]) -> Result<i64> {
    numbers
        .iter()
        .copied()
        .max()
        .context("max needs at least one number")
}

/// Writes the largest of `args.numbers` to `out`.
///
/// # Errors
///
/// Fails as [`largest`] does, or when writing to `out` fails.
pub fn run(args: &Args, out: &mut impl Write) -> Result<()> {
    writeln!(out, "{}", largest(&args.numbers)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const NONE: &str = "max needs at least one number";

    #[test]
    fn picks_the_largest() {
        assert_eq!(largest(&[1, 3, 2]).unwrap(), 3);
    }

    #[test]
    fn picks_the_largest_of_negative_numbers() {
        assert_eq!(largest(&[-4, -2, -9]).unwrap(), -2);
    }

    #[test]
    fn one_number_is_its_own_largest() {
        assert_eq!(largest(&[7]).unwrap(), 7);
    }

    #[test]
    fn handles_the_bounds() {
        assert_eq!(largest(&[i64::MIN, i64::MAX]).unwrap(), i64::MAX);
        assert_eq!(largest(&[i64::MIN]).unwrap(), i64::MIN);
    }

    #[test]
    fn rejects_no_numbers() {
        assert_eq!(largest(&[]).unwrap_err().to_string(), NONE);
    }

    #[test]
    fn run_writes_one_line() {
        let mut out = Vec::new();
        let args = Args {
            numbers: vec![40, 42, 2],
        };
        run(&args, &mut out).unwrap();
        assert_eq!(out, b"42\n");
    }

    #[test]
    fn run_writes_nothing_on_error() {
        let mut out = Vec::new();
        let args = Args { numbers: vec![] };
        assert!(run(&args, &mut out).is_err());
        assert!(out.is_empty());
    }
}
