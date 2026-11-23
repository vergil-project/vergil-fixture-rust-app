//! `median NUMBERS...`: print the median of some integers, rounded toward zero.

use std::io::Write;

use anyhow::{Context, Result};

/// The arguments of `median`.
#[derive(Debug, clap::Args)]
pub struct Args {
    /// The integers to take the median of.
    #[arg(allow_negative_numbers = true)]
    pub numbers: Vec<i64>,
}

/// The median of `numbers`: the middle number once they are sorted, or, for an
/// even count, the mean of the two middle numbers, rounded toward zero.
///
/// The two middle numbers are added in 128 bits, so the sum cannot overflow,
/// and their mean lies between them, so it fits in an `i64`.
///
/// # Examples
///
/// ```
/// use vergil_fixture_rust_app::commands::median::middle;
///
/// assert_eq!(middle(&[7, 2, 3]).unwrap(), 3);
/// assert_eq!(middle(&[4, 1, 2, 9]).unwrap(), 3);
/// assert_eq!(middle(&[-4, -1]).unwrap(), -2);
/// assert!(middle(&[]).is_err());
/// ```
///
/// # Errors
///
/// Fails when `numbers` is empty, since no numbers have a median.
///
/// # Panics
///
/// Never: the mean of two `i64`s lies within the `i64` range.
pub fn middle(numbers: &[i64]) -> Result<i64> {
    let mut sorted = numbers.to_vec();
    sorted.sort_unstable();
    let half = sorted.len() / 2;
    let upper = *sorted
        .get(half)
        .context("median needs at least one number")?;
    if sorted.len() % 2 == 1 {
        return Ok(upper);
    }
    let lower = sorted[half - 1];
    let mean = (i128::from(lower) + i128::from(upper)) / 2;
    Ok(i64::try_from(mean).expect("the mean of two i64s fits in an i64"))
}

/// Writes the median of `args.numbers` to `out`.
///
/// # Errors
///
/// Fails as [`middle`] does, or when writing to `out` fails.
pub fn run(args: &Args, out: &mut impl Write) -> Result<()> {
    writeln!(out, "{}", middle(&args.numbers)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::testing::capture;

    const NONE: &str = "median needs at least one number";

    #[test]
    fn takes_the_middle_of_an_odd_count() {
        assert_eq!(middle(&[9, -3, 4, 1, 100]).unwrap(), 4);
    }

    #[test]
    fn averages_the_middle_two_of_an_even_count() {
        assert_eq!(middle(&[10, 2, 4, 8]).unwrap(), 6);
    }

    #[test]
    fn rounds_toward_zero() {
        assert_eq!(middle(&[1, 2]).unwrap(), 1);
        assert_eq!(middle(&[-1, -2]).unwrap(), -1);
    }

    #[test]
    fn the_median_of_one_number_is_itself() {
        assert_eq!(middle(&[-7]).unwrap(), -7);
    }

    #[test]
    fn reaches_the_bounds_without_overflow() {
        assert_eq!(middle(&[i64::MAX, i64::MAX]).unwrap(), i64::MAX);
        assert_eq!(middle(&[i64::MIN, i64::MIN]).unwrap(), i64::MIN);
        assert_eq!(middle(&[i64::MIN, i64::MAX]).unwrap(), 0);
    }

    #[test]
    fn rejects_no_numbers() {
        assert_eq!(middle(&[]).unwrap_err().to_string(), NONE);
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
