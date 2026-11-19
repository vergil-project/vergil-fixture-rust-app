//! `range NUMBERS...`: print the spread of some integers, the largest less the
//! smallest.

use std::io::Write;

use anyhow::{Context, Result};

/// The arguments of `range`.
#[derive(Debug, clap::Args)]
pub struct Args {
    /// The integers to measure.
    #[arg(allow_negative_numbers = true)]
    pub numbers: Vec<i64>,
}

/// The spread of `numbers`: the largest less the smallest.
///
/// # Examples
///
/// ```
/// use vergil_fixture_rust_app::commands::range::spread;
///
/// assert_eq!(spread(&[3, -7, 5]).unwrap(), 12);
/// assert!(spread(&[]).is_err());
/// assert!(spread(&[i64::MIN, i64::MAX]).is_err());
/// ```
///
/// # Errors
///
/// Fails when `numbers` is empty, since no numbers have a spread, or when the
/// spread does not fit in an `i64`.
pub fn spread(numbers: &[i64]) -> Result<i64> {
    let (&first, rest) = numbers
        .split_first()
        .context("range needs at least one number")?;
    let (smallest, largest) = rest
        .iter()
        .fold((first, first), |(lo, hi), &n| (lo.min(n), hi.max(n)));
    largest
        .checked_sub(smallest)
        .context("the range overflows a 64-bit signed integer")
}

/// Writes the spread of `args.numbers` to `out`.
///
/// # Errors
///
/// Fails as [`spread`] does, or when writing to `out` fails.
pub fn run(args: &Args, out: &mut impl Write) -> Result<()> {
    writeln!(out, "{}", spread(&args.numbers)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::testing::capture;

    const NONE: &str = "range needs at least one number";
    const OVERFLOW: &str = "the range overflows a 64-bit signed integer";

    #[test]
    fn measures_the_spread() {
        assert_eq!(spread(&[2, 9, 3]).unwrap(), 7);
    }

    #[test]
    fn measures_the_spread_of_negative_numbers() {
        assert_eq!(spread(&[-4, -2, -9]).unwrap(), 7);
    }

    #[test]
    fn one_number_has_no_spread() {
        assert_eq!(spread(&[7]).unwrap(), 0);
    }

    #[test]
    fn reaches_the_bounds_without_overflow() {
        assert_eq!(spread(&[0, i64::MAX]).unwrap(), i64::MAX);
        assert_eq!(spread(&[-1, i64::MAX]).unwrap_err().to_string(), OVERFLOW);
        assert_eq!(spread(&[i64::MIN, -1]).unwrap(), i64::MAX);
    }

    #[test]
    fn reports_an_overflow() {
        assert_eq!(
            spread(&[i64::MIN, i64::MAX]).unwrap_err().to_string(),
            OVERFLOW
        );
    }

    #[test]
    fn rejects_no_numbers() {
        assert_eq!(spread(&[]).unwrap_err().to_string(), NONE);
    }

    #[test]
    fn run_writes_one_line() {
        let args = Args {
            numbers: vec![40, 2, 42],
        };
        let (result, out) = capture(64, |out| run(&args, out));
        result.unwrap();
        assert_eq!(out, b"40\n");
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
